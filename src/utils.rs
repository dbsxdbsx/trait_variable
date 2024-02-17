use quote::{quote, ToTokens};
use regex::{Captures, Regex};

pub fn process_assignment_expr(re: &Regex, expr: &syn::Expr) -> syn::Expr {
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

    let right = match expr {
        syn::Expr::Assign(assign_expr) => &assign_expr.right,
        syn::Expr::AssignOp(assign_op_expr) => &assign_op_expr.right,
        _ => unreachable!(),
    };
    let right_str = quote!(#right).to_string();
    let new_right_str = re
        .replace_all(&right_str, |caps: &Captures| {
            format!("self._{}()", &caps[1])
        })
        .to_string();

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
