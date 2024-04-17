use std::path::{Path, PathBuf};

use proc_macro2::TokenTree;
use syn::visit::Visit;
use syn::{parse_file, Attribute, Ident, ItemStruct, Macro, Meta};
use walkdir::WalkDir;

fn caller_crate_root() -> PathBuf {
    let crate_name =
        std::env::var("CARGO_PKG_NAME").expect("failed to read ENV var `CARGO_PKG_NAME`!");
    let current_dir = std::env::current_dir().expect("failed to unwrap env::current_dir()!");
    let search_entry = format!("name=\"{crate_name}\"");
    for entry in WalkDir::new(&current_dir)
        .into_iter()
        .filter_entry(|e| !e.file_name().eq_ignore_ascii_case("target"))
    {
        let Ok(entry) = entry else { continue };
        if !entry.file_type().is_file() {
            continue;
        }
        let Some(file_name) = entry.path().file_name() else {
            continue;
        };
        if !file_name.eq_ignore_ascii_case("Cargo.toml") {
            continue;
        }
        let Ok(cargo_toml) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        if cargo_toml
            .chars()
            .filter(|&c| !c.is_whitespace())
            .collect::<String>()
            .contains(search_entry.as_str())
        {
            return entry.path().parent().unwrap().to_path_buf();
        }
    }
    current_dir
}

pub(crate) struct PathFinder {
    crate_root: PathBuf,
    name: String,
    searched: bool,
    cur_check_path: String,
    path: String,
    for_struct: bool,
}

impl PathFinder {
    pub fn new(trait_name: String, for_struct: bool) -> Self {
        Self {
            crate_root: caller_crate_root(),
            name: trait_name,
            searched: false,
            cur_check_path: "".to_string(),
            path: "".to_string(),
            for_struct,
        }
    }

    pub fn get_trait_import_statement(&mut self) -> String {
        if self.get_def_path().is_empty() {
            return "".to_string();
        }

        let crate_root_path = Path::new(&self.crate_root);
        let trait_path_buf = PathBuf::from(&self.path);
        // Try to remove the `crate_root` part from `trait_path` and get the relative path
        let relative_path = trait_path_buf
            .strip_prefix(crate_root_path)
            .expect("trait_path does not start with crate_root")
            .to_str()
            .expect("path is not valid UTF-8");
        // Convert path separators to Rust module path separators `::`
        let module_path = relative_path
            .replace(std::path::MAIN_SEPARATOR, "::")
            .replace("::mod.rs", "")
            .replace(".rs", "");
        // Format as use crate::<module_path>::<trait_name>; statement
        format!("use crate::{}::{};", module_path, self.name).replace("crate::src::", "crate::")
    }

    pub fn get_def_path(&mut self) -> String {
        if self.searched {
            return self.path.clone();
        }
        // do search
        for entry in WalkDir::new(&self.crate_root)
            .into_iter()
            .filter_entry(|e| !e.file_name().eq_ignore_ascii_case("target"))
        {
            let Ok(entry) = entry else { continue };
            if !entry.path().is_file() {
                continue;
            }
            let Some(ext) = entry.path().extension() else {
                continue;
            };
            if !ext.eq_ignore_ascii_case("rs") {
                continue;
            }
            let Ok(rust_source) = std::fs::read_to_string(entry.path()) else {
                continue;
            };

            self.cur_check_path = entry.path().to_string_lossy().to_string();

            let file = parse_file(&rust_source).unwrap();
            // dbg!("===searching file: {:?}", entry.path());
            self.visit_file(&file);
            if !self.path.is_empty() {
                break;
            };
        }
        // set searched flag and return
        self.searched = true;
        self.path.clone()
    }
}

impl<'ast> Visit<'ast> for PathFinder {
    fn visit_item_struct(&mut self, struct_item: &'ast ItemStruct) {
        if struct_item.ident == self.name {
            // println!("found struct: {:?}", self.name);
            self.path = self.cur_check_path.clone();
        }
    }

    fn visit_macro(&mut self, mac: &'ast Macro) {
        let last_seg = mac.path.segments.last().unwrap();
        let check_macro_prefix = if self.for_struct {
            "trait_var"
        } else {
            "trait_variable"
        };

        if last_seg.ident != check_macro_prefix {
            return;
        }
        // Convert the macro body tokens into a vector of Ident
        let idents: Vec<Ident> = mac
            .tokens
            .clone()
            .into_iter()
            .filter_map(|tt| match tt {
                proc_macro2::TokenTree::Ident(ident) => Some(ident),
                _ => None,
            })
            .collect();
        // Check for the presence of 'trait' keyword followed by the desired trait name
        // If matched, it should appear at the beginning of the macro invocation within 3 ident tokens
        let check_prefix = if self.for_struct { "struct" } else { "trait" };
        for i in 0..idents.len().min(3) {
            if idents[i] == check_prefix && idents[i + 1] == self.name {
                // println!("found trait: {:?}", self.trait_name);
                self.path = self.cur_check_path.clone();
                break;
            }
        }
    }
}

#[test]
fn test_caller_crate_root() {
    let crate_root = caller_crate_root();
    println!("crate_root: {:?}", crate_root);
    assert!(crate_root.ends_with("trait_variable"));
}

#[test]
fn test_trait_path_finder() {
    // positive case for trait finder
    let mut trait_searcher = PathFinder::new("ComplexTrait".to_string(), false);
    let caller_path = trait_searcher.get_def_path();
    assert!(caller_path.ends_with("trait_variable\\tests\\complex.rs"));
    let import_statment = trait_searcher.get_trait_import_statement();
    assert_eq!(import_statment, "use crate::tests::complex::ComplexTrait;");
    // positive case for struct finder
    let mut struct_searcher = PathFinder::new("MyStructForBasic".to_string(), true);
    let caller_path = struct_searcher.get_def_path();
    assert!(caller_path.ends_with("trait_variable\\tests\\basic.rs"));
    // negative case
    let mut trait_searcher = PathFinder::new("NoExistedTrait".to_string(), false);
    assert!(trait_searcher.get_trait_import_statement().is_empty());
}

#[test]
fn test_mod_and_src_path_for_trait_path_finder() {
    // 1. test import_statement with `mod.rs`
    let mut trait_searcher = PathFinder::new("PracticalTrait".to_string(), false);
    let caller_path = trait_searcher.get_def_path();
    assert!(caller_path.ends_with("trait_variable\\tests\\common\\mod.rs"));
    let import_statment = trait_searcher.get_trait_import_statement();
    assert_eq!(import_statment, "use crate::tests::common::PracticalTrait;");
    // 2. test import_statement with `src` folder
    let crate_root_path = Path::new(&trait_searcher.crate_root);
    trait_searcher.path = crate_root_path
        .join("src")
        .join("common")
        .join("mod.rs")
        .to_string_lossy()
        .to_string();
    let import_statment = trait_searcher.get_trait_import_statement();
    assert_eq!(import_statment, "use crate::common::PracticalTrait;");
}
