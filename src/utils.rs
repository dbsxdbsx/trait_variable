use quote::{quote, ToTokens};
use regex::{Captures, Regex};
use syn::Expr;

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
    let new_right_str = replace_self_field(right);

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

/// 根据匹配模式调整`expr`表达式中的内容，并根据`deref`参数决定是否进行解引用转换
///
/// 当遇到`&mut self.x`模式时，转换为`&mut self._x_mut()`，如果`deref`为`true`，则进一步转换为`&mut (*self._x_mut())`；
/// 当遇到`& self.x`模式时，转换为`& self._x()`，如果`deref`为`true`，则进一步转换为`& (*self._x())`；
/// 当遇到`self.x`模式时，转换为`self._x()`，如果`deref`为`true`，则进一步转换为`(*self._x())`。
/// 不会匹配已经是函数调用的`self.x()`形式。
///
/// # 参数
///
/// * `expr` - 待处理的表达式，实现了`ToTokens` trait
/// * `deref` - 是否进行解引用转换
///
/// # 返回值
///
/// 返回处理后的字符串
pub fn replace_self_field<T: ToTokens>(expr: &T /* , deref: bool */) -> String {
    let expr_str = quote!(#expr).to_string();
    let re = Regex::new(
        r"(&\s*mut\s+self\.)([a-zA-Z_]\w*)|(&\s*self\.)([a-zA-Z_]\w*)|(self\.)([a-zA-Z_]\w*)",
    )
    .unwrap();
    let mut result = String::new();
    let mut last_end = 0;
    for cap in re.captures_iter(&expr_str) {
        let match_start = cap.get(0).unwrap().start();
        let match_end = cap.get(0).unwrap().end();
        // 如果匹配后紧跟`(`，则不进行替换
        if expr_str[match_end..].starts_with('(') {
            continue;
        }
        // 将上一个匹配结束到当前匹配开始之间的文本追加到结果中
        result.push_str(&expr_str[last_end..match_start]);
        match (cap.get(1), cap.get(3), cap.get(5)) {
            (Some(_), _, _) => {
                // 匹配到 &mut self.x
                let name = &cap[2];
                result.push_str(& format!("&mut (*self._{}_mut())", name));
            }
            (_, Some(_), _) => {
                // 匹配到 & self.x
                let name = &cap[4];
                result.push_str(&format!("&(*self._{}())", name));
            }
            (_, _, Some(_)) => {
                // 匹配到 self.x
                let name = &cap[6];
                result.push_str(&format!("(*self._{}())", name));
                // let name = &cap[6];
                // // 这里需要增加逻辑来判断是否应该使用 _mut 后缀
                // let replacement = if 需要可变访问 {
                //     if deref {
                //         format!("(*self._{}_mut())", name)
                //     } else {
                //         format!("self._{}_mut()", name)
                //     }
                // } else {
                //     if deref {
                //         format!("(*self._{}())", name)
                //     } else {
                //         format!("self._{}()", name)
                //     }
                // };
                // result.push_str(&replacement);
            }
            _ => unreachable!(),
        }
        last_end = match_end;
    }
    // 将最后一个匹配结束到字符串末尾之间的文本追加到结果中
    result.push_str(&expr_str[last_end..]);
    result
}
use syn::{visit::Visit, ExprMethodCall, FnArg, PatType, Signature};
/// 检查方法调用是否需要可变引用
fn needs_mutable_access(expr_call: &ExprMethodCall) -> bool {
    // 创建一个访问者来检查方法签名
    struct FindMutVisitor(bool);

    impl<'ast> Visit<'ast> for FindMutVisitor {
        fn visit_signature(&mut self, i: &'ast Signature) {
            // 检查方法的第一个参数是否为`&mut self`
            if let Some(FnArg::Receiver(receiver)) = i.inputs.first() {
                if receiver.mutability.is_some() {
                    self.0 = true;
                }
            }
        }
    }

    let mut visitor = FindMutVisitor(false);
    visitor.visit_expr_method_call(expr_call);
    visitor.0
}
