use std::path::{Path, PathBuf};

use syn::visit::Visit;
use syn::{parse_file, Ident, Macro};
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

pub(crate) struct TraitPathFinder {
    crate_root: PathBuf,
    trait_name: String,
    searched: bool,
    cur_check_path: String,
    trait_path: String,
}

impl TraitPathFinder {
    pub fn new(trait_name: String) -> Self {
        Self {
            crate_root: caller_crate_root(),
            trait_name,
            searched: false,
            cur_check_path: "".to_string(),
            trait_path: "".to_string(),
        }
    }

    pub fn get_trait_import_statement(&mut self) -> String {
        if self.get_trait_def_path().is_empty() {
            return "".to_string();
        }

        let crate_root_path = Path::new(&self.crate_root);
        let trait_path_buf = PathBuf::from(&self.trait_path);
        // Try to remove the `crate_root` part from `trait_path` and get the relative path
        let relative_path = trait_path_buf
            .strip_prefix(crate_root_path)
            .expect("trait_path does not start with crate_root")
            .to_str()
            .expect("path is not valid UTF-8");
        // Convert path separators to Rust module path separators `::`
        let module_path = relative_path
            .replace(std::path::MAIN_SEPARATOR, "::")
            .replace(".rs", "");
        // Format as use crate::<module_path>::<trait_name>; statement
        format!("use crate::{}::{};", module_path, self.trait_name)
    }

    pub fn get_trait_def_path(&mut self) -> String {
        if self.searched {
            return self.trait_path.clone();
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
            if !self.trait_path.is_empty() {
                break;
            };
        }
        // set searched flag and return
        self.searched = true;
        self.trait_path.clone()
    }
}

impl<'ast> Visit<'ast> for TraitPathFinder {
    fn visit_macro(&mut self, mac: &'ast Macro) {
        let last_seg = mac.path.segments.last().unwrap();
        if last_seg.ident != "trait_variable" {
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
        for i in 0..idents.len().min(3) {
            if idents[i] == "trait" && idents[i + 1] == self.trait_name {
                // println!("found trait: {:?}", self.trait_name);
                // put the trait path into self.trait_path
                self.trait_path = self.cur_check_path.clone();
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
    // positive case
    let mut trait_searcher = TraitPathFinder::new("ComplexTrait".to_string());
    let caller_path = trait_searcher.get_trait_def_path();
    assert!(caller_path.ends_with("trait_variable\\tests\\complex.rs"));
    let import_statment = trait_searcher.get_trait_import_statement();
    assert_eq!(import_statment, "use crate::tests::complex::ComplexTrait;");
    // negative case
    let mut trait_searcher = TraitPathFinder::new("NoExistedTrait".to_string());
    assert!(trait_searcher.get_trait_import_statement().is_empty());
}
