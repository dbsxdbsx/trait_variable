use std::fmt;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use regex::{Captures, Regex};

use syn::{parse_str, BinOp, Expr, FnArg, Receiver, Signature, TraitItemFn};

/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓custom Assignment Object↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
pub struct MyAssignExpr {
    #[allow(unused)]
    attrs: Vec<syn::Attribute>,
    left: TokenStream,
    op: TokenStream,
    right: TokenStream,
}
impl fmt::Display for MyAssignExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "l:`{}` op:`{}` r:`{}`", self.left, self.op, self.right)
    }
}
pub enum MyAssignOp {
    AssignOp(MyAssignExpr),
    NoAssignOp,
}
impl From<syn::Expr> for MyAssignOp {
    fn from(expr: syn::Expr) -> Self {
        match expr {
            Expr::Assign(assign) => {
                let attrs = assign.attrs.clone();
                let left = assign.left.to_token_stream();
                let op = assign.eq_token.to_token_stream();
                let right = assign.right.to_token_stream();
                MyAssignOp::AssignOp(MyAssignExpr {
                    attrs,
                    left,
                    op,
                    right,
                })
            }
            Expr::Binary(binary)
                if matches!(
                    binary.op,
                    BinOp::AddAssign(_)
                        | BinOp::SubAssign(_)
                        | BinOp::MulAssign(_)
                        | BinOp::DivAssign(_)
                        | BinOp::RemAssign(_)
                        | BinOp::BitXorAssign(_)
                        | BinOp::BitAndAssign(_)
                        | BinOp::BitOrAssign(_)
                        | BinOp::ShlAssign(_)
                        | BinOp::ShrAssign(_)
                ) =>
            {
                let attrs = binary.attrs.clone();
                let left = binary.left.to_token_stream();
                let op = binary.op.to_token_stream();
                let right = binary.right.to_token_stream();
                MyAssignOp::AssignOp(MyAssignExpr {
                    attrs,
                    left,
                    op,
                    right,
                })
            }
            _ => MyAssignOp::NoAssignOp,
        }
    }
}
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑custom Assignment Object↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

pub fn parse_assignment_expr(expr: MyAssignExpr, is_method_mut: bool) -> Expr {
    // 1. left side
    let new_left_str = replace_self_field(&expr.left, is_method_mut, true);
    // 2. right side
    let new_right_str = replace_self_field(&expr.right, is_method_mut, false);
    // 3. rebuild the final expression and return
    let new_expr_str = format!("{} {} {}", new_left_str, expr.op, new_right_str);
    syn::parse_str(&new_expr_str).expect("Failed to parse new expr")
}

/// Check if a string contains `ref mut` adjacent to each other
pub fn is_ref_mut(s: &str) -> bool {
    let parts = s.split_whitespace();
    let mut found_ref = false;
    for part in parts {
        if found_ref && part == "mut" {
            return true;
        }
        found_ref = part == "ref";
    }
    false
}

/// Replaces all occurrences of `self.field`, `&self.field`, and `&mut self.field` in the given expression
/// with their corresponding getter/setter method calls.
///
/// # Arguments
///
/// * `expr` - The expression to replace `self.field` occurrences in, as a `ToTokens` implementor.
/// * `is_method_mut` - A boolean indicating whether the method being called is mutable or not.
///
/// # Returns
///
/// A `String` containing the modified expression with `self.field` occurrences replaced.
/// ```
pub fn replace_self_field<T: ToTokens>(
    expr: &T,
    is_method_mut: bool,
    is_left_ref_mut: bool, // TODO: change to Option and refine doc
) -> String {
    let raw_expr_str = quote!(#expr).to_string();
    let re = Regex::new(
        r"(&\s*mut\s+self\.)([a-zA-Z_]\w*)|(&\s*self\.)([a-zA-Z_]\w*)|(self\.)([a-zA-Z_]\w*)",
    )
    .unwrap();

    let mut result = String::new();
    let mut last_end = 0;
    for cap in re.captures_iter(&raw_expr_str) {
        let match_start = cap.get(0).unwrap().start();
        let match_end = cap.get(0).unwrap().end();
        // 如果匹配后紧跟`(`，则不进行替换，因为这说明其原本就是个合法的函数调用
        if raw_expr_str[match_end..].starts_with('(') {
            continue;
        }
        // 将上一个匹配结束到当前匹配开始之间的文本追加到结果中
        result.push_str(&raw_expr_str[last_end..match_start]);
        match (cap.get(1), cap.get(3), cap.get(5)) {
            // match `&mut self.x`
            (Some(_), _, _) => {
                let name = &cap[2];
                result.push_str(&format!("&mut (*self._{}_mut())", name));
            }
            // match `&self.x`
            (_, Some(_), _) => {
                let name = &cap[4];
                result.push_str(&format!("&(*self._{}())", name));
            }
            // match `self.x`
            (_, _, Some(_)) => {
                let name = &cap[6];
                let trailing_fn_expr =
                    extract_trailing_expr_until_to_1st_fn(&raw_expr_str, match_end);
                // convert to mut or non-mut version accordingly
                if is_method_mut {
                    // NOTE: techniqically, no need to care `is_method_mut`,
                    // but since it is hard to know if the field trailing fn is a mutalbe methond,
                    // so here for simplicity, decided to treat it as a mutable reference by both the root
                    // method mutablity(`is_method_mut`) and the exitance of trailing fn expr.
                    // then the field is being used as a mutable reference for simplicity
                    if (!trailing_fn_expr.is_empty()) || is_left_ref_mut {
                        result.push_str(&format!("(*self._{}_mut())", name));
                    } else {
                        result.push_str(&format!("(*self._{}())", name));
                    }
                } else {
                    result.push_str(&format!("(*self._{}())", name));
                }
            }
            _ => unreachable!(
                "fn `replace_self_field`: Regex should not match without a capture group"
            ),
        }
        last_end = match_end;
    }
    // 将最后一个匹配结束到字符串末尾之间的文本追加到结果中
    result.push_str(&raw_expr_str[last_end..]);

    result
}

fn extract_trailing_expr_until_to_1st_fn(raw_expr_str: &str, match_end: usize) -> &str {
    let expr_after_name = &raw_expr_str[match_end..];
    if !expr_after_name.starts_with('.') {
        return "";
    }
    //
    let mut parentheses_count = 0;
    for (i, c) in expr_after_name.char_indices() {
        match c {
            '(' => parentheses_count += 1,
            ')' => {
                parentheses_count -= 1;
                if parentheses_count == 0 {
                    return &expr_after_name[..=i];
                }
            }
            _ => {}
        }
    }
    //
    ""
}

/// 检查一个方法签名是否表示一个可变方法。
/// 返回 `Some(true)` 表示可变方法,`Some(false)` 表示不可变方法,
/// `None` 表示无效签名或不是一个与 `self` 参数相关的方法。
pub fn is_method_mut(method_signature: &str) -> Option<bool> {
    let sig: Result<Signature, _> = parse_str(method_signature);

    match sig {
        Ok(sig) => {
            sig.inputs.iter().next().and_then(|first_arg| {
                match first_arg {
                    FnArg::Receiver(Receiver {
                        mutability,
                        reference,
                        ..
                    }) => {
                        // 如果是 `self`,返回 None
                        if reference.is_none() {
                            None
                        } else {
                            // 如果有 mutability,则为 Some(true),否则为 Some(false)
                            Some(mutability.is_some())
                        }
                    }
                    FnArg::Typed(_) => {
                        // 其他情况不考虑为方法,返回 None
                        None
                    }
                }
            })
        }
        Err(_) => None, // 解析失败返回 None
    }
}

pub fn is_trait_method_mutable(method: &TraitItemFn) -> bool {
    let method_sig = &method.sig;
    let method_sig_str = quote!(#method_sig).to_string();
    is_method_mut(&method_sig_str).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_method_mut() {
        assert_eq!(
            is_method_mut("fn change_value(&mut self, value: i32)"),
            Some(true)
        );
        assert_eq!(is_method_mut("fn read_value(&self) -> i32"), Some(false));
        assert_eq!(is_method_mut("fn into_value(self) -> i32"), None);
        assert_eq!(is_method_mut("fn not_a_method(value: i32) -> i32"), None);
        assert_eq!(is_method_mut("fn update(& mut self)"), Some(true));
        assert_eq!(is_method_mut("fn invalid(&wrong self)"), None);
    }

    #[test]
    fn test_regex_capture_with_1st_function_call() {
        let re = Regex::new(
            r"(&\s*mut\s+self\.)([a-zA-Z_]\w*)|(&\s*self\.)([a-zA-Z_]\w*)|(self\.)([a-zA-Z_]\w*)",
        )
        .unwrap();

        // case 1
        let raw_str = "self.v";
        // 1.1 check trait field capture
        let captures = re.captures(raw_str).unwrap();
        assert_eq!(captures.get(0).unwrap().as_str(), "self.v");
        assert_eq!(captures.get(6).unwrap().as_str(), "v");
        // 1.2 check the trailing (fn) expression capture
        let target_trail =
            extract_trailing_expr_until_to_1st_fn(raw_str, captures.get(0).unwrap().end());
        assert_eq!(target_trail, "");

        // case 2
        let raw_str = "self.v.c";
        // 2.1 check trait field capture
        let captures = re.captures(raw_str).unwrap();
        assert_eq!(captures.get(0).unwrap().as_str(), "self.v");
        assert_eq!(captures.get(6).unwrap().as_str(), "v");
        // 2.2 check the trailing (fn) expression capture
        let target_trail =
            extract_trailing_expr_until_to_1st_fn(raw_str, captures.get(0).unwrap().end());
        assert_eq!(target_trail, "");

        // case 3
        let raw_str = "self.v.c.push(1).iter()";
        // 3.1 check trait field capture
        let captures = re.captures(raw_str).unwrap();
        assert_eq!(captures.get(0).unwrap().as_str(), "self.v");
        assert_eq!(captures.get(6).unwrap().as_str(), "v");
        // 3.2 check the trailing (fn) expression capture
        let target_trail =
            extract_trailing_expr_until_to_1st_fn(raw_str, captures.get(0).unwrap().end());
        assert_eq!(target_trail, ".c.push(1)");
    }

    #[test]
    fn test_is_ref_mut() {
        assert!(is_ref_mut("ref mut"));
        assert!(is_ref_mut("ref   mut"));
        assert!(is_ref_mut("  ref   mut  "));
        assert!(is_ref_mut("(_, ref  mut s, _)"));
        assert!(!is_ref_mut("refmut"));
        assert!(!is_ref_mut("aref mut"));
        assert!(!is_ref_mut("ref mutxx"));
        assert!(!is_ref_mut(""));
        assert!(!is_ref_mut("hello world"));
    }
}
