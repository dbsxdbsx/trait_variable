use quote::{quote, ToTokens};
use regex::{Captures, Regex};
use syn::{parse_quote, Expr};

pub fn process_assignment_expr(re: &Regex, expr: &Expr) -> Expr {
    // 1. left side
    let left = match expr {
        syn::Expr::Assign(assign_expr) => &assign_expr.left,
        syn::Expr::AssignOp(assign_op_expr) => &assign_op_expr.left,
        _ => unreachable!(),
    };
    let left_str = quote!(#left).to_string();
    let new_left_str = re
        .replace_all(&left_str, |caps: &Captures| {
            format!("(*self._{}_mut())", &caps[1])
        })
        .to_string();

    // 2. right side
    let right = match expr {
        syn::Expr::Assign(assign_expr) => &assign_expr.right,
        syn::Expr::AssignOp(assign_op_expr) => &assign_op_expr.right,
        _ => unreachable!(),
    };
    // 2.1 deal with conditional expression pattern
    let new_right_str = if let Expr::If(if_expr) = right.as_ref() {
        let new_cond_str = replace_self_field(&if_expr.cond, true);
        let new_then_branch_str = replace_self_field(&if_expr.then_branch, true);
        let new_else_branch_str = if let Some((_, else_expr)) = &if_expr.else_branch {
            replace_self_field(else_expr, true)
        } else {
            "".into()
        };
        // rebuild if block string
        let mut new_if_block_str = format!("if {} {{ {} }}", new_cond_str, new_then_branch_str);
        if !new_else_branch_str.is_empty() {
            new_if_block_str = format!("{} else {{ {} }}", new_if_block_str, new_else_branch_str);
        }
        new_if_block_str
    } else {
        replace_self_field(right, false)
    };

    // 3. rebuild the final expression and return
    let new_expr_str = match expr {
        syn::Expr::Assign(_) => format!("{} = {}", new_left_str, new_right_str),
        syn::Expr::AssignOp(assign_op_expr) => format!(
            "{} {} {}",
            new_left_str,
            assign_op_expr.op.to_token_stream(),
            new_right_str
        ),
        _ => unreachable!(),
    };
    syn::parse_str(&new_expr_str).expect("Failed to parse new expr")
}

/// 替换表达式中的 `self` 字段访问
///
/// # 参数
///
/// * `expr` - 实现了 `ToTokens` trait 的表达式
/// * `deref` - 是否对 `self` 进行解引用
///
/// # 返回值
///
/// 返回替换后的表达式字符串
/// 替换表达式中的 `self.<field>` 模式，除非它后面紧跟着一个左括号 `(`
pub fn replace_self_field<T: ToTokens>(expr: &T, deref: bool) -> String {
    let re = Regex::new(r"\bself\.([a-zA-Z_]\w*)").unwrap();
    let expr_str = quote!(#expr).to_string();
    let mut new_expr_str = String::new();
    let mut last_end = 0;

    for cap in re.captures_iter(&expr_str) {
        let match_start = cap.get(0).unwrap().start();
        let match_end = cap.get(0).unwrap().end();

        // if followed with `(`, no need replacement
        if expr_str[match_end..].chars().next() != Some('(') {
            let replacement = if deref {
                format!("(*self._{}())", &cap[1])
            } else {
                format!("self._{}()", &cap[1])
            };
            new_expr_str.push_str(&expr_str[last_end..match_start]);
            new_expr_str.push_str(&replacement);
            last_end = match_end;
        }
    }
    new_expr_str.push_str(&expr_str[last_end..]);

    new_expr_str
}
