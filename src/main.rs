use std::env;
use std::collections::HashMap;
use syn::visit::Visit;
use syn::*;
use syn::punctuated::*;
use proc_macro2::*;

struct context {
    pub path: String,
    pub storage: HashMap<String, usize>
    
}

 
pub fn visit_abi<'ast, V>(v: &mut V, node: &'ast Abi)
where
    V: Visit<'ast> + ?Sized,
{
    if let Some(it) = &node.name {
        v.visit_lit_str(it);
    }
}
 
pub fn visit_angle_bracketed_generic_arguments<'ast, V>(
    v: &mut V,
    node: &'ast AngleBracketedGenericArguments,
)
where
    V: Visit<'ast> + ?Sized,
{
    for el in Punctuated::pairs(&node.args) {
        let (it, p) = el.into_tuple();
        v.visit_generic_argument(it);
    }
}
   
pub fn visit_arm<'ast, V>(v: &mut V, node: &'ast Arm)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_pat(&node.pat);
    if let Some(it) = &node.guard {
        v.visit_expr(&*(it).1);
    }
    v.visit_expr(&*node.body);
}
 
pub fn visit_attr_style<'ast, V>(v: &mut V, node: &'ast AttrStyle)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        AttrStyle::Outer => {}
        AttrStyle::Inner(_binding_0) => {
        }
    }
}
 
pub fn visit_attribute<'ast, V>(v: &mut V, node: &'ast Attribute)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_attr_style(&node.style);
    v.visit_path(&node.path);
    // skip!(node.tokens);
}
 
pub fn visit_bare_fn_arg<'ast, V>(v: &mut V, node: &'ast BareFnArg)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    if let Some(it) = &node.name {
        v.visit_ident(&(it).0);
        // // tokens_helper(v, &(it).1.spans);
    }
    v.visit_type(&node.ty);
}
 
pub fn visit_bin_op<'ast, V>(v: &mut V, node: &'ast BinOp)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        _ => {}
        // BinOp::Add(_binding_0) => {
            // // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::Sub(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::Mul(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::Div(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::Rem(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::And(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::Or(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::BitXor(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::BitAnd(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::BitOr(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::Shl(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::Shr(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::Eq(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::Lt(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::Le(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::Ne(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::Ge(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::Gt(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::AddEq(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::SubEq(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::MulEq(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::DivEq(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::RemEq(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::BitXorEq(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::BitAndEq(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::BitOrEq(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::ShlEq(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
        // BinOp::ShrEq(_binding_0) => {
        //     // tokens_helper(v, &_binding_0.spans);
        // }
    }
}
 
pub fn visit_binding<'ast, V>(v: &mut V, node: &'ast Binding)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_ident(&node.ident);
    // // tokens_helper(v, &node.eq_token.spans);
    v.visit_type(&node.ty);
}
   
pub fn visit_block<'ast, V>(v: &mut V, node: &'ast Block)
where
    V: Visit<'ast> + ?Sized,
{
    // // tokens_helper(v, &node.brace_token.span);
    for it in &node.stmts {
        v.visit_stmt(it);
    }
}
 
pub fn visit_bound_lifetimes<'ast, V>(v: &mut V, node: &'ast BoundLifetimes)
where
    V: Visit<'ast> + ?Sized,
{
    // // tokens_helper(v, &node.for_token.span);
    // // tokens_helper(v, &node.lt_token.spans);
    for el in Punctuated::pairs(&node.lifetimes) {
        let (it, p) = el.into_tuple();
        v.visit_lifetime_def(it);
        // if let Some(p) = p {
        //     // tokens_helper(v, &p.spans);
        // }
    }
    // // tokens_helper(v, &node.gt_token.spans);
}
 
pub fn visit_const_param<'ast, V>(v: &mut V, node: &'ast ConstParam)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.const_token.span);
    v.visit_ident(&node.ident);
    // // tokens_helper(v, &node.colon_token.spans);
    v.visit_type(&node.ty);
    // if let Some(it) = &node.eq_token {
    //     // tokens_helper(v, &it.spans);
    // }
    if let Some(it) = &node.default {
        v.visit_expr(it);
    }
}
 
pub fn visit_constraint<'ast, V>(v: &mut V, node: &'ast Constraint)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_ident(&node.ident);
    // // tokens_helper(v, &node.colon_token.spans);
    for el in Punctuated::pairs(&node.bounds) {
        let (it, p) = el.into_tuple();
        v.visit_type_param_bound(it);
        // if let Some(p) = p {
        //     // tokens_helper(v, &p.spans);
        // }
    }
}
pub fn visit_data<'ast, V>(v: &mut V, node: &'ast Data)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        Data::Struct(_binding_0) => {
            v.visit_data_struct(_binding_0);
        }
        Data::Enum(_binding_0) => {
            v.visit_data_enum(_binding_0);
        }
        Data::Union(_binding_0) => {
            v.visit_data_union(_binding_0);
        }
    }
}
// #[cfg(feature = "derive")]
pub fn visit_data_enum<'ast, V>(v: &mut V, node: &'ast DataEnum)
where
    V: Visit<'ast> + ?Sized,
{
    // // tokens_helper(v, &node.enum_token.span);
    // // tokens_helper(v, &node.brace_token.span);
    for el in Punctuated::pairs(&node.variants) {
        let (it, p) = el.into_tuple();
        v.visit_variant(it);
        // if let Some(p) = p {
        //     // tokens_helper(v, &p.spans);
        // }
    }
}
// #[cfg(feature = "derive")]
pub fn visit_data_struct<'ast, V>(v: &mut V, node: &'ast DataStruct)
where
    V: Visit<'ast> + ?Sized,
{
    // // tokens_helper(v, &node.struct_token.span);
    v.visit_fields(&node.fields);
    // if let Some(it) = &node.semi_token {
    //     // tokens_helper(v, &it.spans);
    // }
}
// #[cfg(feature = "derive")]
pub fn visit_data_union<'ast, V>(v: &mut V, node: &'ast DataUnion)
where
    V: Visit<'ast> + ?Sized,
{
    // // tokens_helper(v, &node.union_token.span);
    v.visit_fields_named(&node.fields);
}
// #[cfg(feature = "derive")]
pub fn visit_derive_input<'ast, V>(v: &mut V, node: &'ast DeriveInput)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    v.visit_ident(&node.ident);
    v.visit_generics(&node.generics);
    v.visit_data(&node.data);
}
 
pub fn visit_expr<'ast, V>(v: &mut V, node: &'ast Expr)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        // // Expr::Array(_binding_0) => {
        // //     full!(v.visit_expr_array(_binding_0));
        // // }
        // // Expr::Assign(_binding_0) => {
        // //     full!(v.visit_expr_assign(_binding_0));
        // // }
        // // Expr::AssignOp(_binding_0) => {
        // //     full!(v.visit_expr_assign_op(_binding_0));
        // // }
        // // Expr::Async(_binding_0) => {
        // //     full!(v.visit_expr_async(_binding_0));
        // // }
        // // Expr::Await(_binding_0) => {
        // //     full!(v.visit_expr_await(_binding_0));
        // // }
        // // Expr::Binary(_binding_0) => {
        // //     v.visit_expr_binary(_binding_0);
        // // }
        // // Expr::Block(_binding_0) => {
        // //     full!(v.visit_expr_block(_binding_0));
        // // }
        // // Expr::Box(_binding_0) => {
        // //     full!(v.visit_expr_box(_binding_0));
        // // }
        // // Expr::Break(_binding_0) => {
        // //     full!(v.visit_expr_break(_binding_0));
        // // }
        // // Expr::Call(_binding_0) => {
        // //     v.visit_expr_call(_binding_0);
        // // }
        // // Expr::Cast(_binding_0) => {
        // //     v.visit_expr_cast(_binding_0);
        // // }
        // // Expr::Closure(_binding_0) => {
        // //     full!(v.visit_expr_closure(_binding_0));
        // // }
        // // Expr::Continue(_binding_0) => {
        // //     full!(v.visit_expr_continue(_binding_0));
        // // }
        // // Expr::Field(_binding_0) => {
        // //     v.visit_expr_field(_binding_0);
        // // }
        // // Expr::ForLoop(_binding_0) => {
        // //     full!(v.visit_expr_for_loop(_binding_0));
        // // }
        // // Expr::Group(_binding_0) => {
        // //     full!(v.visit_expr_group(_binding_0));
        // // }
        // // Expr::If(_binding_0) => {
        // //     full!(v.visit_expr_if(_binding_0));
        // // }
        // // Expr::Index(_binding_0) => {
        // //     v.visit_expr_index(_binding_0);
        // // }
        // // Expr::Let(_binding_0) => {
        // //     full!(v.visit_expr_let(_binding_0));
        // // }
        // // Expr::Lit(_binding_0) => {
        // //     v.visit_expr_lit(_binding_0);
        // // }
        // // Expr::Loop(_binding_0) => {
        // //     full!(v.visit_expr_loop(_binding_0));
        // // }
        // // Expr::Macro(_binding_0) => {
        // //     full!(v.visit_expr_macro(_binding_0));
        // // }
        // // Expr::Match(_binding_0) => {
        // //     full!(v.visit_expr_match(_binding_0));
        // // }
        // // Expr::MethodCall(_binding_0) => {
        // //     full!(v.visit_expr_method_call(_binding_0));
        // // }
        // // Expr::Paren(_binding_0) => {
        // //     v.visit_expr_paren(_binding_0);
        // // }
        // // Expr::Path(_binding_0) => {
        // //     v.visit_expr_path(_binding_0);
        // // }
        // // Expr::Range(_binding_0) => {
        // //     full!(v.visit_expr_range(_binding_0));
        // // }
        // // Expr::Reference(_binding_0) => {
        // //     full!(v.visit_expr_reference(_binding_0));
        // // }
        // // Expr::Repeat(_binding_0) => {
        // //     full!(v.visit_expr_repeat(_binding_0));
        // // }
        // // Expr::Return(_binding_0) => {
        // //     full!(v.visit_expr_return(_binding_0));
        // // }
        // // Expr::Struct(_binding_0) => {
        // //     full!(v.visit_expr_struct(_binding_0));
        // // }
        // // Expr::Try(_binding_0) => {
        // //     full!(v.visit_expr_try(_binding_0));
        // // }
        // // Expr::TryBlock(_binding_0) => {
        // //     full!(v.visit_expr_try_block(_binding_0));
        // // }
        // // Expr::Tuple(_binding_0) => {
        // //     full!(v.visit_expr_tuple(_binding_0));
        // // }
        // // Expr::Type(_binding_0) => {
        // //     full!(v.visit_expr_type(_binding_0));
        // // }
        // // Expr::Unary(_binding_0) => {
        // //     v.visit_expr_unary(_binding_0);
        // // }
        // // Expr::Unsafe(_binding_0) => {
        // //     full!(v.visit_expr_unsafe(_binding_0));
        // // }
        // // Expr::Verbatim(_binding_0) => {
        // //     skip!(_binding_0);
        // // }
        // // Expr::While(_binding_0) => {
        // //     full!(v.visit_expr_while(_binding_0));
        // // }
        // // Expr::Yield(_binding_0) => {
        // //     full!(v.visit_expr_yield(_binding_0));
        // // }
        // #[cfg(syn_no_non_exhaustive)]
        _ => unreachable!(),
    }
}
   
pub fn visit_expr_array<'ast, V>(v: &mut V, node: &'ast ExprArray)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.bracket_token.span);
    for el in Punctuated::pairs(&node.elems) {
        let (it, p) = el.into_tuple();
        v.visit_expr(it);
        // if let Some(p) = p {
            // // tokens_helper(v, &p.spans);
        // }
    }
}
   
pub fn visit_expr_assign<'ast, V>(v: &mut V, node: &'ast ExprAssign)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_expr(&*node.left);
    // // tokens_helper(v, &node.eq_token.spans);
    v.visit_expr(&*node.right);
}
   
pub fn visit_expr_assign_op<'ast, V>(v: &mut V, node: &'ast ExprAssignOp)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_expr(&*node.left);
    v.visit_bin_op(&node.op);
    v.visit_expr(&*node.right);
}
   
pub fn visit_expr_async<'ast, V>(v: &mut V, node: &'ast ExprAsync)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.async_token.span);
    // if let Some(it) = &node.capture {
    //     // tokens_helper(v, &it.span);
    // }
    v.visit_block(&node.block);
}
   
pub fn visit_expr_await<'ast, V>(v: &mut V, node: &'ast ExprAwait)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_expr(&*node.base);
    // // tokens_helper(v, &node.dot_token.spans);
    // // tokens_helper(v, &node.await_token.span);
}
 
pub fn visit_expr_binary<'ast, V>(v: &mut V, node: &'ast ExprBinary)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_expr(&*node.left);
    v.visit_bin_op(&node.op);
    v.visit_expr(&*node.right);
}
   
pub fn visit_expr_block<'ast, V>(v: &mut V, node: &'ast ExprBlock)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    if let Some(it) = &node.label {
        v.visit_label(it);
    }
    v.visit_block(&node.block);
}
   
pub fn visit_expr_box<'ast, V>(v: &mut V, node: &'ast ExprBox)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.box_token.span);
    v.visit_expr(&*node.expr);
}
   
pub fn visit_expr_break<'ast, V>(v: &mut V, node: &'ast ExprBreak)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.break_token.span);
    if let Some(it) = &node.label {
        v.visit_lifetime(it);
    }
    if let Some(it) = &node.expr {
        v.visit_expr(&**it);
    }
}
 
pub fn visit_expr_call<'ast, V>(v: &mut V, node: &'ast ExprCall)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_expr(&*node.func);
    // // tokens_helper(v, &node.paren_token.span);
    for el in Punctuated::pairs(&node.args) {
        let (it, p) = el.into_tuple();
        v.visit_expr(it);
        // if let Some(p) = p {
        //     // tokens_helper(v, &p.spans);
        // }
    }
}
 
pub fn visit_expr_cast<'ast, V>(v: &mut V, node: &'ast ExprCast)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_expr(&*node.expr);
    // // tokens_helper(v, &node.as_token.span);
    v.visit_type(&*node.ty);
}
   
pub fn visit_expr_closure<'ast, V>(v: &mut V, node: &'ast ExprClosure)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // if let Some(it) = &node.movability {
    //     // tokens_helper(v, &it.span);
    // }
    // if let Some(it) = &node.asyncness {
    //     // tokens_helper(v, &it.span);
    // }
    // if let Some(it) = &node.capture {
    //     // tokens_helper(v, &it.span);
    // }
    // // tokens_helper(v, &node.or1_token.spans);
    for el in Punctuated::pairs(&node.inputs) {
        let (it, p) = el.into_tuple();
        v.visit_pat(it);
        // if let Some(p) = p {
        //     // tokens_helper(v, &p.spans);
        // }
    }
    // // tokens_helper(v, &node.or2_token.spans);
    v.visit_return_type(&node.output);
    v.visit_expr(&*node.body);
}
   
pub fn visit_expr_continue<'ast, V>(v: &mut V, node: &'ast ExprContinue)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.continue_token.span);
    if let Some(it) = &node.label {
        v.visit_lifetime(it);
    }
}
 
pub fn visit_expr_field<'ast, V>(v: &mut V, node: &'ast ExprField)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_expr(&*node.base);
    // // tokens_helper(v, &node.dot_token.spans);
    v.visit_member(&node.member);
}
   
pub fn visit_expr_for_loop<'ast, V>(v: &mut V, node: &'ast ExprForLoop)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    if let Some(it) = &node.label {
        v.visit_label(it);
    }
    // // tokens_helper(v, &node.for_token.span);
    v.visit_pat(&node.pat);
    // // tokens_helper(v, &node.in_token.span);
    v.visit_expr(&*node.expr);
    v.visit_block(&node.body);
}
   
pub fn visit_expr_group<'ast, V>(v: &mut V, node: &'ast ExprGroup)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.group_token.span);
    v.visit_expr(&*node.expr);
}
   
pub fn visit_expr_if<'ast, V>(v: &mut V, node: &'ast ExprIf)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.if_token.span);
    v.visit_expr(&*node.cond);
    v.visit_block(&node.then_branch);
    if let Some(it) = &node.else_branch {
        // // tokens_helper(v, &(it).0.span);
        v.visit_expr(&*(it).1);
    }
}
 
pub fn visit_expr_index<'ast, V>(v: &mut V, node: &'ast ExprIndex)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_expr(&*node.expr);
    // // tokens_helper(v, &node.bracket_token.span);
    v.visit_expr(&*node.index);
}
   
pub fn visit_expr_let<'ast, V>(v: &mut V, node: &'ast ExprLet)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.let_token.span);
    v.visit_pat(&node.pat);
    // // tokens_helper(v, &node.eq_token.spans);
    v.visit_expr(&*node.expr);
}
 
pub fn visit_expr_lit<'ast, V>(v: &mut V, node: &'ast ExprLit)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_lit(&node.lit);
}
   
pub fn visit_expr_loop<'ast, V>(v: &mut V, node: &'ast ExprLoop)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    if let Some(it) = &node.label {
        v.visit_label(it);
    }
    // // tokens_helper(v, &node.loop_token.span);
    v.visit_block(&node.body);
}
   
pub fn visit_expr_macro<'ast, V>(v: &mut V, node: &'ast ExprMacro)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_macro(&node.mac);
}
   
pub fn visit_expr_match<'ast, V>(v: &mut V, node: &'ast ExprMatch)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.match_token.span);
    v.visit_expr(&*node.expr);
    // // tokens_helper(v, &node.brace_token.span);
    for it in &node.arms {
        v.visit_arm(it);
    }
}
   
pub fn visit_expr_method_call<'ast, V>(v: &mut V, node: &'ast ExprMethodCall)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_expr(&*node.receiver);
    // // tokens_helper(v, &node.dot_token.spans);
    v.visit_ident(&node.method);
    if let Some(it) = &node.turbofish {
        v.visit_method_turbofish(it);
    }
    // // tokens_helper(v, &node.paren_token.span);
    for el in Punctuated::pairs(&node.args) {
        let (it, p) = el.into_tuple();
        v.visit_expr(it);
        // if let Some(p) = p {
        //     // tokens_helper(v, &p.spans);
        // }
    }
}
 
pub fn visit_expr_paren<'ast, V>(v: &mut V, node: &'ast ExprParen)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.paren_token.span);
    v.visit_expr(&*node.expr);
}
 
pub fn visit_expr_path<'ast, V>(v: &mut V, node: &'ast ExprPath)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    if let Some(it) = &node.qself {
        v.visit_qself(it);
    }
    v.visit_path(&node.path);
}
   
pub fn visit_expr_range<'ast, V>(v: &mut V, node: &'ast ExprRange)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    if let Some(it) = &node.from {
        v.visit_expr(&**it);
    }
    v.visit_range_limits(&node.limits);
    if let Some(it) = &node.to {
        v.visit_expr(&**it);
    }
}
   
pub fn visit_expr_reference<'ast, V>(v: &mut V, node: &'ast ExprReference)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.and_token.spans);
    // if let Some(it) = &node.mutability {
    //     // tokens_helper(v, &it.span);
    // }
    v.visit_expr(&*node.expr);
}
   
pub fn visit_expr_repeat<'ast, V>(v: &mut V, node: &'ast ExprRepeat)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.bracket_token.span);
    v.visit_expr(&*node.expr);
    // // tokens_helper(v, &node.semi_token.spans);
    v.visit_expr(&*node.len);
}
   
pub fn visit_expr_return<'ast, V>(v: &mut V, node: &'ast ExprReturn)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.return_token.span);
    if let Some(it) = &node.expr {
        v.visit_expr(&**it);
    }
}
   
pub fn visit_expr_struct<'ast, V>(v: &mut V, node: &'ast ExprStruct)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_path(&node.path);
    // // tokens_helper(v, &node.brace_token.span);
    for el in Punctuated::pairs(&node.fields) {
        let (it, p) = el.into_tuple();
        v.visit_field_value(it);
        // if let Some(p) = p {
        //     // tokens_helper(v, &p.spans);
        // }
    }
    // if let Some(it) = &node.dot2_token {
    //     // tokens_helper(v, &it.spans);
    // }
    if let Some(it) = &node.rest {
        v.visit_expr(&**it);
    }
}
   
pub fn visit_expr_try<'ast, V>(v: &mut V, node: &'ast ExprTry)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_expr(&*node.expr);
    // // tokens_helper(v, &node.question_token.spans);
}
   
pub fn visit_expr_try_block<'ast, V>(v: &mut V, node: &'ast ExprTryBlock)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.try_token.span);
    v.visit_block(&node.block);
}
   
pub fn visit_expr_tuple<'ast, V>(v: &mut V, node: &'ast ExprTuple)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.paren_token.span);
    for el in Punctuated::pairs(&node.elems) {
        let (it, p) = el.into_tuple();
        v.visit_expr(it);
        // if let Some(p) = p {
        //     // tokens_helper(v, &p.spans);
        // }
    }
}
   
pub fn visit_expr_type<'ast, V>(v: &mut V, node: &'ast ExprType)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_expr(&*node.expr);
    // // tokens_helper(v, &node.colon_token.spans);
    v.visit_type(&*node.ty);
}
 
pub fn visit_expr_unary<'ast, V>(v: &mut V, node: &'ast ExprUnary)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_un_op(&node.op);
    v.visit_expr(&*node.expr);
}
   
pub fn visit_expr_unsafe<'ast, V>(v: &mut V, node: &'ast ExprUnsafe)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.unsafe_token.span);
    v.visit_block(&node.block);
}
   
pub fn visit_expr_while<'ast, V>(v: &mut V, node: &'ast ExprWhile)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    if let Some(it) = &node.label {
        v.visit_label(it);
    }
    // // tokens_helper(v, &node.while_token.span);
    v.visit_expr(&*node.cond);
    v.visit_block(&node.body);
}
   
pub fn visit_expr_yield<'ast, V>(v: &mut V, node: &'ast ExprYield)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.yield_token.span);
    if let Some(it) = &node.expr {
        v.visit_expr(&**it);
    }
}
 
pub fn visit_field<'ast, V>(v: &mut V, node: &'ast Field)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    if let Some(it) = &node.ident {
        v.visit_ident(it);
    }
    // if let Some(it) = &node.colon_token {
    //     // tokens_helper(v, &it.spans);
    // }
    v.visit_type(&node.ty);
}
   
pub fn visit_field_pat<'ast, V>(v: &mut V, node: &'ast FieldPat)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_member(&node.member);
    // if let Some(it) = &node.colon_token {
    //     // tokens_helper(v, &it.spans);
    // }
    v.visit_pat(&*node.pat);
}
   
pub fn visit_field_value<'ast, V>(v: &mut V, node: &'ast FieldValue)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_member(&node.member);
    // if let Some(it) = &node.colon_token {
    //     // tokens_helper(v, &it.spans);
    // }
    v.visit_expr(&node.expr);
}
 
pub fn visit_fields<'ast, V>(v: &mut V, node: &'ast Fields)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        Fields::Named(_binding_0) => {
            v.visit_fields_named(_binding_0);
        }
        Fields::Unnamed(_binding_0) => {
            v.visit_fields_unnamed(_binding_0);
        }
        Fields::Unit => {}
    }
}
 
pub fn visit_fields_named<'ast, V>(v: &mut V, node: &'ast FieldsNamed)
where
    V: Visit<'ast> + ?Sized,
{
    // // tokens_helper(v, &node.brace_token.span);
    for el in Punctuated::pairs(&node.named) {
        let (it, p) = el.into_tuple();
        v.visit_field(it);
        // if let Some(p) = p {
        //     // tokens_helper(v, &p.spans);
        // }
    }
}
 
pub fn visit_fields_unnamed<'ast, V>(v: &mut V, node: &'ast FieldsUnnamed)
where
    V: Visit<'ast> + ?Sized,
{
    // // tokens_helper(v, &node.paren_token.span);
    for el in Punctuated::pairs(&node.unnamed) {
        let (it, p) = el.into_tuple();
        v.visit_field(it);
        // if let Some(p) = p {
        //     // tokens_helper(v, &p.spans);
        // }
    }
}
   
pub fn visit_file<'ast, V>(v: &mut V, node: &'ast File)
where
    V: Visit<'ast> + ?Sized,
{
    // skip!(node.shebang);
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    for it in &node.items {
        v.visit_item(it);
    }
}
   
pub fn visit_fn_arg<'ast, V>(v: &mut V, node: &'ast FnArg)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        FnArg::Receiver(_binding_0) => {
            v.visit_receiver(_binding_0);
        }
        FnArg::Typed(_binding_0) => {
            v.visit_pat_type(_binding_0);
        }
    }
}
   
pub fn visit_foreign_item<'ast, V>(v: &mut V, node: &'ast ForeignItem)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        ForeignItem::Fn(_binding_0) => {
            v.visit_foreign_item_fn(_binding_0);
        }
        ForeignItem::Static(_binding_0) => {
            v.visit_foreign_item_static(_binding_0);
        }
        ForeignItem::Type(_binding_0) => {
            v.visit_foreign_item_type(_binding_0);
        }
        ForeignItem::Macro(_binding_0) => {
            v.visit_foreign_item_macro(_binding_0);
        }
        ForeignItem::Verbatim(_binding_0) => {
            // skip!(_binding_0);
        }
        // #[cfg(syn_no_non_exhaustive)]
        _ => unreachable!(),
    }
}
   
pub fn visit_foreign_item_fn<'ast, V>(v: &mut V, node: &'ast ForeignItemFn)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    v.visit_signature(&node.sig);
    // // tokens_helper(v, &node.semi_token.spans);
}
   
pub fn visit_foreign_item_macro<'ast, V>(v: &mut V, node: &'ast ForeignItemMacro)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_macro(&node.mac);
    // if let Some(it) = &node.semi_token {
    //     // tokens_helper(v, &it.spans);
    // }
}
   
pub fn visit_foreign_item_static<'ast, V>(v: &mut V, node: &'ast ForeignItemStatic)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    // // tokens_helper(v, &node.static_token.span);
    if let Some(it) = &node.mutability {
        // // tokens_helper(v, &it.span);
    }
    v.visit_ident(&node.ident);
    // // tokens_helper(v, &node.colon_token.spans);
    v.visit_type(&*node.ty);
    // // tokens_helper(v, &node.semi_token.spans);
}
   
pub fn visit_foreign_item_type<'ast, V>(v: &mut V, node: &'ast ForeignItemType)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    // // tokens_helper(v, &node.type_token.span);
    v.visit_ident(&node.ident);
    // // tokens_helper(v, &node.semi_token.spans);
}
 
pub fn visit_generic_argument<'ast, V>(v: &mut V, node: &'ast GenericArgument)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        GenericArgument::Lifetime(_binding_0) => {
            v.visit_lifetime(_binding_0);
        }
        GenericArgument::Type(_binding_0) => {
            v.visit_type(_binding_0);
        }
        GenericArgument::Const(_binding_0) => {
            v.visit_expr(_binding_0);
        }
        GenericArgument::Binding(_binding_0) => {
            v.visit_binding(_binding_0);
        }
        GenericArgument::Constraint(_binding_0) => {
            v.visit_constraint(_binding_0);
        }
    }
}
   
pub fn visit_generic_method_argument<'ast, V>(
    v: &mut V,
    node: &'ast GenericMethodArgument,
)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        GenericMethodArgument::Type(_binding_0) => {
            v.visit_type(_binding_0);
        }
        GenericMethodArgument::Const(_binding_0) => {
            v.visit_expr(_binding_0);
        }
    }
}
 
pub fn visit_generic_param<'ast, V>(v: &mut V, node: &'ast GenericParam)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        GenericParam::Type(_binding_0) => {
            v.visit_type_param(_binding_0);
        }
        GenericParam::Lifetime(_binding_0) => {
            v.visit_lifetime_def(_binding_0);
        }
        GenericParam::Const(_binding_0) => {
            v.visit_const_param(_binding_0);
        }
    }
}
 
pub fn visit_generics<'ast, V>(v: &mut V, node: &'ast Generics)
where
    V: Visit<'ast> + ?Sized,
{
    if let Some(it) = &node.lt_token {
        // // tokens_helper(v, &it.spans);
    }
    for el in Punctuated::pairs(&node.params) {
        let (it, p) = el.into_tuple();
        v.visit_generic_param(it);
        if let Some(p) = p {
            // // tokens_helper(v, &p.spans);
        }
    }
    if let Some(it) = &node.gt_token {
        // // tokens_helper(v, &it.spans);
    }
    if let Some(it) = &node.where_clause {
        v.visit_where_clause(it);
    }
}
pub fn visit_ident<'ast, V>(v: &mut V, node: &'ast Ident)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_span(&node.span());
}
   
pub fn visit_impl_item<'ast, V>(v: &mut V, node: &'ast ImplItem)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        ImplItem::Const(_binding_0) => {
            v.visit_impl_item_const(_binding_0);
        }
        ImplItem::Method(_binding_0) => {
            v.visit_impl_item_method(_binding_0);
        }
        ImplItem::Type(_binding_0) => {
            v.visit_impl_item_type(_binding_0);
        }
        ImplItem::Macro(_binding_0) => {
            v.visit_impl_item_macro(_binding_0);
        }
        ImplItem::Verbatim(_binding_0) => {
            // skip!(_binding_0);
        }
        // #[cfg(syn_no_non_exhaustive)]
        _ => unreachable!(),
    }
}
   
pub fn visit_impl_item_const<'ast, V>(v: &mut V, node: &'ast ImplItemConst)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    if let Some(it) = &node.defaultness {
        // // tokens_helper(v, &it.span);
    }
    // // tokens_helper(v, &node.const_token.span);
    v.visit_ident(&node.ident);
    // // tokens_helper(v, &node.colon_token.spans);
    v.visit_type(&node.ty);
    // // tokens_helper(v, &node.eq_token.spans);
    v.visit_expr(&node.expr);
    // // tokens_helper(v, &node.semi_token.spans);
}
   
pub fn visit_impl_item_macro<'ast, V>(v: &mut V, node: &'ast ImplItemMacro)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_macro(&node.mac);
    if let Some(it) = &node.semi_token {
        // // tokens_helper(v, &it.spans);
    }
}
   
pub fn visit_impl_item_method<'ast, V>(v: &mut V, node: &'ast ImplItemMethod)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    if let Some(it) = &node.defaultness {
        // // tokens_helper(v, &it.span);
    }
    v.visit_signature(&node.sig);
    v.visit_block(&node.block);
}
   
pub fn visit_impl_item_type<'ast, V>(v: &mut V, node: &'ast ImplItemType)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    if let Some(it) = &node.defaultness {
        // // tokens_helper(v, &it.span);
    }
    // // tokens_helper(v, &node.type_token.span);
    v.visit_ident(&node.ident);
    v.visit_generics(&node.generics);
    // // tokens_helper(v, &node.eq_token.spans);
    v.visit_type(&node.ty);
    // // tokens_helper(v, &node.semi_token.spans);
}
 
pub fn visit_index<'ast, V>(v: &mut V, node: &'ast Index)
where
    V: Visit<'ast> + ?Sized,
{
    // skip!(node.index);
    v.visit_span(&node.span);
}
   
pub fn visit_item<'ast, V>(v: &mut V, node: &'ast Item)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        Item::Const(_binding_0) => {
            v.visit_item_const(_binding_0);
        }
        Item::Enum(_binding_0) => {
            v.visit_item_enum(_binding_0);
        }
        Item::ExternCrate(_binding_0) => {
            v.visit_item_extern_crate(_binding_0);
        }
        Item::Fn(_binding_0) => {
            v.visit_item_fn(_binding_0);
        }
        Item::ForeignMod(_binding_0) => {
            v.visit_item_foreign_mod(_binding_0);
        }
        Item::Impl(_binding_0) => {
            v.visit_item_impl(_binding_0);
        }
        Item::Macro(_binding_0) => {
            v.visit_item_macro(_binding_0);
        }
        Item::Macro2(_binding_0) => {
            v.visit_item_macro2(_binding_0);
        }
        Item::Mod(_binding_0) => {
            v.visit_item_mod(_binding_0);
        }
        Item::Static(_binding_0) => {
            v.visit_item_static(_binding_0);
        }
        Item::Struct(_binding_0) => {
            v.visit_item_struct(_binding_0);
        }
        Item::Trait(_binding_0) => {
            v.visit_item_trait(_binding_0);
        }
        Item::TraitAlias(_binding_0) => {
            v.visit_item_trait_alias(_binding_0);
        }
        Item::Type(_binding_0) => {
            v.visit_item_type(_binding_0);
        }
        Item::Union(_binding_0) => {
            v.visit_item_union(_binding_0);
        }
        Item::Use(_binding_0) => {
            v.visit_item_use(_binding_0);
        }
        Item::Verbatim(_binding_0) => {
            // skip!(_binding_0);
        }
        // #[cfg(syn_no_non_exhaustive)]
        _ => unreachable!(),
    }
}
   
pub fn visit_item_const<'ast, V>(v: &mut V, node: &'ast ItemConst)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    // // tokens_helper(v, &node.const_token.span);
    v.visit_ident(&node.ident);
    // // tokens_helper(v, &node.colon_token.spans);
    v.visit_type(&*node.ty);
    // // tokens_helper(v, &node.eq_token.spans);
    v.visit_expr(&*node.expr);
    // // tokens_helper(v, &node.semi_token.spans);
}
   
pub fn visit_item_enum<'ast, V>(v: &mut V, node: &'ast ItemEnum)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    // // tokens_helper(v, &node.enum_token.span);
    v.visit_ident(&node.ident);
    v.visit_generics(&node.generics);
    // // tokens_helper(v, &node.brace_token.span);
    for el in Punctuated::pairs(&node.variants) {
        let (it, p) = el.into_tuple();
        v.visit_variant(it);
        if let Some(p) = p {
            // // tokens_helper(v, &p.spans);
        }
    }
}
   
pub fn visit_item_extern_crate<'ast, V>(v: &mut V, node: &'ast ItemExternCrate)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    // // tokens_helper(v, &node.extern_token.span);
    // // tokens_helper(v, &node.crate_token.span);
    v.visit_ident(&node.ident);
    if let Some(it) = &node.rename {
        // // tokens_helper(v, &(it).0.span);
        v.visit_ident(&(it).1);
    }
    // // tokens_helper(v, &node.semi_token.spans);
}
   
pub fn visit_item_fn<'ast, V>(v: &mut V, node: &'ast ItemFn)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    v.visit_signature(&node.sig);
    v.visit_block(&*node.block);
}
   
pub fn visit_item_foreign_mod<'ast, V>(v: &mut V, node: &'ast ItemForeignMod)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_abi(&node.abi);
    // // tokens_helper(v, &node.brace_token.span);
    for it in &node.items {
        v.visit_foreign_item(it);
    }
}
   
pub fn visit_item_impl<'ast, V>(v: &mut V, node: &'ast ItemImpl)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    if let Some(it) = &node.defaultness {
        // // tokens_helper(v, &it.span);
    }
    if let Some(it) = &node.unsafety {
        // // tokens_helper(v, &it.span);
    }
    // // tokens_helper(v, &node.impl_token.span);
    v.visit_generics(&node.generics);
    if let Some(it) = &node.trait_ {
        if let Some(it) = &(it).0 {
            // // tokens_helper(v, &it.spans);
        }
        v.visit_path(&(it).1);
        // // tokens_helper(v, &(it).2.span);
    }
    v.visit_type(&*node.self_ty);
    // // tokens_helper(v, &node.brace_token.span);
    for it in &node.items {
        v.visit_impl_item(it);
    }
}
   
pub fn visit_item_macro<'ast, V>(v: &mut V, node: &'ast ItemMacro)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    if let Some(it) = &node.ident {
        v.visit_ident(it);
    }
    v.visit_macro(&node.mac);
    if let Some(it) = &node.semi_token {
        // // tokens_helper(v, &it.spans);
    }
}
   
pub fn visit_item_macro2<'ast, V>(v: &mut V, node: &'ast ItemMacro2)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    // // tokens_helper(v, &node.macro_token.span);
    v.visit_ident(&node.ident);
    // skip!(node.rules);
}
   
pub fn visit_item_mod<'ast, V>(v: &mut V, node: &'ast ItemMod)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    // // tokens_helper(v, &node.mod_token.span);
    v.visit_ident(&node.ident);
    if let Some(it) = &node.content {
        // // tokens_helper(v, &(it).0.span);
        for it in &(it).1 {
            v.visit_item(it);
        }
    }
    if let Some(it) = &node.semi {
        // // tokens_helper(v, &it.spans);
    }
}
   
pub fn visit_item_static<'ast, V>(v: &mut V, node: &'ast ItemStatic)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    // // tokens_helper(v, &node.static_token.span);
    if let Some(it) = &node.mutability {
        // // tokens_helper(v, &it.span);
    }
    v.visit_ident(&node.ident);
    // // tokens_helper(v, &node.colon_token.spans);
    v.visit_type(&*node.ty);
    // // tokens_helper(v, &node.eq_token.spans);
    v.visit_expr(&*node.expr);
    // // tokens_helper(v, &node.semi_token.spans);
}
   
pub fn visit_item_struct<'ast, V>(v: &mut V, node: &'ast ItemStruct)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    // // tokens_helper(v, &node.struct_token.span);
    v.visit_ident(&node.ident);
    v.visit_generics(&node.generics);
    v.visit_fields(&node.fields);
    if let Some(it) = &node.semi_token {
        // // tokens_helper(v, &it.spans);
    }
}
   
pub fn visit_item_trait<'ast, V>(v: &mut V, node: &'ast ItemTrait)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    // if let Some(it) = &node.unsafety {
    //     // tokens_helper(v, &it.span);
    // }
    // if let Some(it) = &node.auto_token {
    //     // tokens_helper(v, &it.span);
    // }
    // // tokens_helper(v, &node.trait_token.span);
    v.visit_ident(&node.ident);
    v.visit_generics(&node.generics);
    // if let Some(it) = &node.colon_token {
    //     // tokens_helper(v, &it.spans);
    // }
    for el in Punctuated::pairs(&node.supertraits) {
        let (it, p) = el.into_tuple();
        v.visit_type_param_bound(it);
        // if let Some(p) = p {
        //     // tokens_helper(v, &p.spans);
        // }
    }
    // // tokens_helper(v, &node.brace_token.span);
    for it in &node.items {
        v.visit_trait_item(it);
    }
}
   
pub fn visit_item_trait_alias<'ast, V>(v: &mut V, node: &'ast ItemTraitAlias)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    // // tokens_helper(v, &node.trait_token.span);
    v.visit_ident(&node.ident);
    v.visit_generics(&node.generics);
    // // tokens_helper(v, &node.eq_token.spans);
    for el in Punctuated::pairs(&node.bounds) {
        let (it, p) = el.into_tuple();
        v.visit_type_param_bound(it);
        if let Some(p) = p {
            // // tokens_helper(v, &p.spans);
        }
    }
    // // tokens_helper(v, &node.semi_token.spans);
}
   
pub fn visit_item_type<'ast, V>(v: &mut V, node: &'ast ItemType)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    // // tokens_helper(v, &node.type_token.span);
    v.visit_ident(&node.ident);
    v.visit_generics(&node.generics);
    // // tokens_helper(v, &node.eq_token.spans);
    v.visit_type(&*node.ty);
    // // tokens_helper(v, &node.semi_token.spans);
}
   
pub fn visit_item_union<'ast, V>(v: &mut V, node: &'ast ItemUnion)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    // // tokens_helper(v, &node.union_token.span);
    v.visit_ident(&node.ident);
    v.visit_generics(&node.generics);
    v.visit_fields_named(&node.fields);
}
   
pub fn visit_item_use<'ast, V>(v: &mut V, node: &'ast ItemUse)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_visibility(&node.vis);
    // // tokens_helper(v, &node.use_token.span);
    if let Some(it) = &node.leading_colon {
        // // tokens_helper(v, &it.spans);
    }
    v.visit_use_tree(&node.tree);
    // // tokens_helper(v, &node.semi_token.spans);
}
   
pub fn visit_label<'ast, V>(v: &mut V, node: &'ast Label)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_lifetime(&node.name);
    // // tokens_helper(v, &node.colon_token.spans);
}
pub fn visit_lifetime<'ast, V>(v: &mut V, node: &'ast Lifetime)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_span(&node.apostrophe);
    v.visit_ident(&node.ident);
}
 
pub fn visit_lifetime_def<'ast, V>(v: &mut V, node: &'ast LifetimeDef)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_lifetime(&node.lifetime);
    if let Some(it) = &node.colon_token {
        // // tokens_helper(v, &it.spans);
    }
    for el in Punctuated::pairs(&node.bounds) {
        let (it, p) = el.into_tuple();
        v.visit_lifetime(it);
        if let Some(p) = p {
            // // tokens_helper(v, &p.spans);
        }
    }
}
pub fn visit_lit<'ast, V>(v: &mut V, node: &'ast Lit)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        Lit::Str(_binding_0) => {
            v.visit_lit_str(_binding_0);
        }
        Lit::ByteStr(_binding_0) => {
            v.visit_lit_byte_str(_binding_0);
        }
        Lit::Byte(_binding_0) => {
            v.visit_lit_byte(_binding_0);
        }
        Lit::Char(_binding_0) => {
            v.visit_lit_char(_binding_0);
        }
        Lit::Int(_binding_0) => {
            v.visit_lit_int(_binding_0);
        }
        Lit::Float(_binding_0) => {
            v.visit_lit_float(_binding_0);
        }
        Lit::Bool(_binding_0) => {
            v.visit_lit_bool(_binding_0);
        }
        Lit::Verbatim(_binding_0) => {
            // skip!(_binding_0);
        }
    }
}
pub fn visit_lit_bool<'ast, V>(v: &mut V, node: &'ast LitBool)
where
    V: Visit<'ast> + ?Sized,
{
    // skip!(node.value);
    v.visit_span(&node.span);
}
pub fn visit_lit_byte<'ast, V>(v: &mut V, node: &'ast LitByte)
where
    V: Visit<'ast> + ?Sized,
{}
pub fn visit_lit_byte_str<'ast, V>(v: &mut V, node: &'ast LitByteStr)
where
    V: Visit<'ast> + ?Sized,
{}
pub fn visit_lit_char<'ast, V>(v: &mut V, node: &'ast LitChar)
where
    V: Visit<'ast> + ?Sized,
{}
pub fn visit_lit_float<'ast, V>(v: &mut V, node: &'ast LitFloat)
where
    V: Visit<'ast> + ?Sized,
{}
pub fn visit_lit_int<'ast, V>(v: &mut V, node: &'ast LitInt)
where
    V: Visit<'ast> + ?Sized,
{}
pub fn visit_lit_str<'ast, V>(v: &mut V, node: &'ast LitStr)
where
    V: Visit<'ast> + ?Sized,
{}
   
pub fn visit_local<'ast, V>(v: &mut V, node: &'ast Local)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // // tokens_helper(v, &node.let_token.span);
    v.visit_pat(&node.pat);
    if let Some(it) = &node.init {
        // // tokens_helper(v, &(it).0.spans);
        v.visit_expr(&*(it).1);
    }
    // // tokens_helper(v, &node.semi_token.spans);
}
 
pub fn visit_macro<'ast, V>(v: &mut V, node: &'ast Macro)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_path(&node.path);
    // // tokens_helper(v, &node.bang_token.spans);
    v.visit_macro_delimiter(&node.delimiter);
    // skip!(node.tokens);
}
 
pub fn visit_macro_delimiter<'ast, V>(v: &mut V, node: &'ast MacroDelimiter)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        MacroDelimiter::Paren(_binding_0) => {
            // // tokens_helper(v, &_binding_0.span);
        }
        MacroDelimiter::Brace(_binding_0) => {
            // // tokens_helper(v, &_binding_0.span);
        }
        MacroDelimiter::Bracket(_binding_0) => {
            // // tokens_helper(v, &_binding_0.span);
        }
    }
}
 
pub fn visit_member<'ast, V>(v: &mut V, node: &'ast Member)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        Member::Named(_binding_0) => {
            v.visit_ident(_binding_0);
        }
        Member::Unnamed(_binding_0) => {
            v.visit_index(_binding_0);
        }
    }
}
 
pub fn visit_meta<'ast, V>(v: &mut V, node: &'ast Meta)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        Meta::Path(_binding_0) => {
            v.visit_path(_binding_0);
        }
        Meta::List(_binding_0) => {
            v.visit_meta_list(_binding_0);
        }
        Meta::NameValue(_binding_0) => {
            v.visit_meta_name_value(_binding_0);
        }
    }
}
 
pub fn visit_meta_list<'ast, V>(v: &mut V, node: &'ast MetaList)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_path(&node.path);
    // tokens_helper(v, &node.paren_token.span);
    for el in Punctuated::pairs(&node.nested) {
        let (it, p) = el.into_tuple();
        v.visit_nested_meta(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
}
 
pub fn visit_meta_name_value<'ast, V>(v: &mut V, node: &'ast MetaNameValue)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_path(&node.path);
    // tokens_helper(v, &node.eq_token.spans);
    v.visit_lit(&node.lit);
}
   
pub fn visit_method_turbofish<'ast, V>(v: &mut V, node: &'ast MethodTurbofish)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.colon2_token.spans);
    // tokens_helper(v, &node.lt_token.spans);
    for el in Punctuated::pairs(&node.args) {
        let (it, p) = el.into_tuple();
        v.visit_generic_method_argument(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
    // tokens_helper(v, &node.gt_token.spans);
}
 
pub fn visit_nested_meta<'ast, V>(v: &mut V, node: &'ast NestedMeta)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        NestedMeta::Meta(_binding_0) => {
            v.visit_meta(_binding_0);
        }
        NestedMeta::Lit(_binding_0) => {
            v.visit_lit(_binding_0);
        }
    }
}
 
pub fn visit_parenthesized_generic_arguments<'ast, V>(
    v: &mut V,
    node: &'ast ParenthesizedGenericArguments,
)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.paren_token.span);
    for el in Punctuated::pairs(&node.inputs) {
        let (it, p) = el.into_tuple();
        v.visit_type(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
    v.visit_return_type(&node.output);
}
   
pub fn visit_pat<'ast, V>(v: &mut V, node: &'ast Pat)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        Pat::Box(_binding_0) => {
            v.visit_pat_box(_binding_0);
        }
        Pat::Ident(_binding_0) => {
            v.visit_pat_ident(_binding_0);
        }
        Pat::Lit(_binding_0) => {
            v.visit_pat_lit(_binding_0);
        }
        Pat::Macro(_binding_0) => {
            v.visit_pat_macro(_binding_0);
        }
        Pat::Or(_binding_0) => {
            v.visit_pat_or(_binding_0);
        }
        Pat::Path(_binding_0) => {
            v.visit_pat_path(_binding_0);
        }
        Pat::Range(_binding_0) => {
            v.visit_pat_range(_binding_0);
        }
        Pat::Reference(_binding_0) => {
            v.visit_pat_reference(_binding_0);
        }
        Pat::Rest(_binding_0) => {
            v.visit_pat_rest(_binding_0);
        }
        Pat::Slice(_binding_0) => {
            v.visit_pat_slice(_binding_0);
        }
        Pat::Struct(_binding_0) => {
            v.visit_pat_struct(_binding_0);
        }
        Pat::Tuple(_binding_0) => {
            v.visit_pat_tuple(_binding_0);
        }
        Pat::TupleStruct(_binding_0) => {
            v.visit_pat_tuple_struct(_binding_0);
        }
        Pat::Type(_binding_0) => {
            v.visit_pat_type(_binding_0);
        }
        Pat::Verbatim(_binding_0) => {
            // skip!(_binding_0);
        }
        Pat::Wild(_binding_0) => {
            v.visit_pat_wild(_binding_0);
        }
        // #[cfg(syn_no_non_exhaustive)]
        _ => unreachable!(),
    }
}
   
pub fn visit_pat_box<'ast, V>(v: &mut V, node: &'ast PatBox)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // tokens_helper(v, &node.box_token.span);
    v.visit_pat(&*node.pat);
}
   
pub fn visit_pat_ident<'ast, V>(v: &mut V, node: &'ast PatIdent)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    if let Some(it) = &node.by_ref {
        // tokens_helper(v, &it.span);
    }
    if let Some(it) = &node.mutability {
        // tokens_helper(v, &it.span);
    }
    v.visit_ident(&node.ident);
    if let Some(it) = &node.subpat {
        // tokens_helper(v, &(it).0.spans);
        v.visit_pat(&*(it).1);
    }
}
   
pub fn visit_pat_lit<'ast, V>(v: &mut V, node: &'ast PatLit)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_expr(&*node.expr);
}
   
pub fn visit_pat_macro<'ast, V>(v: &mut V, node: &'ast PatMacro)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_macro(&node.mac);
}
   
pub fn visit_pat_or<'ast, V>(v: &mut V, node: &'ast PatOr)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    if let Some(it) = &node.leading_vert {
        // tokens_helper(v, &it.spans);
    }
    for el in Punctuated::pairs(&node.cases) {
        let (it, p) = el.into_tuple();
        v.visit_pat(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
}
   
pub fn visit_pat_path<'ast, V>(v: &mut V, node: &'ast PatPath)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    if let Some(it) = &node.qself {
        v.visit_qself(it);
    }
    v.visit_path(&node.path);
}
   
pub fn visit_pat_range<'ast, V>(v: &mut V, node: &'ast PatRange)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_expr(&*node.lo);
    v.visit_range_limits(&node.limits);
    v.visit_expr(&*node.hi);
}
   
pub fn visit_pat_reference<'ast, V>(v: &mut V, node: &'ast PatReference)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // tokens_helper(v, &node.and_token.spans);
    if let Some(it) = &node.mutability {
        // tokens_helper(v, &it.span);
    }
    v.visit_pat(&*node.pat);
}
   
pub fn visit_pat_rest<'ast, V>(v: &mut V, node: &'ast PatRest)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // tokens_helper(v, &node.dot2_token.spans);
}
   
pub fn visit_pat_slice<'ast, V>(v: &mut V, node: &'ast PatSlice)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // tokens_helper(v, &node.bracket_token.span);
    for el in Punctuated::pairs(&node.elems) {
        let (it, p) = el.into_tuple();
        v.visit_pat(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
}
   
pub fn visit_pat_struct<'ast, V>(v: &mut V, node: &'ast PatStruct)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_path(&node.path);
    // tokens_helper(v, &node.brace_token.span);
    for el in Punctuated::pairs(&node.fields) {
        let (it, p) = el.into_tuple();
        v.visit_field_pat(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
    if let Some(it) = &node.dot2_token {
        // tokens_helper(v, &it.spans);
    }
}
   
pub fn visit_pat_tuple<'ast, V>(v: &mut V, node: &'ast PatTuple)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // tokens_helper(v, &node.paren_token.span);
    for el in Punctuated::pairs(&node.elems) {
        let (it, p) = el.into_tuple();
        v.visit_pat(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
}
   
pub fn visit_pat_tuple_struct<'ast, V>(v: &mut V, node: &'ast PatTupleStruct)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_path(&node.path);
    v.visit_pat_tuple(&node.pat);
}
   
pub fn visit_pat_type<'ast, V>(v: &mut V, node: &'ast PatType)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_pat(&*node.pat);
    // tokens_helper(v, &node.colon_token.spans);
    v.visit_type(&*node.ty);
}
   
pub fn visit_pat_wild<'ast, V>(v: &mut V, node: &'ast PatWild)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // tokens_helper(v, &node.underscore_token.spans);
}
 
pub fn visit_path<'ast, V>(v: &mut V, node: &'ast Path)
where
    V: Visit<'ast> + ?Sized,
{
    if let Some(it) = &node.leading_colon {
        // tokens_helper(v, &it.spans);
    }
    for el in Punctuated::pairs(&node.segments) {
        let (it, p) = el.into_tuple();
        v.visit_path_segment(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
}
 
pub fn visit_path_arguments<'ast, V>(v: &mut V, node: &'ast PathArguments)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        PathArguments::None => {}
        PathArguments::AngleBracketed(_binding_0) => {
            v.visit_angle_bracketed_generic_arguments(_binding_0);
        }
        PathArguments::Parenthesized(_binding_0) => {
            v.visit_parenthesized_generic_arguments(_binding_0);
        }
    }
}
 
pub fn visit_path_segment<'ast, V>(v: &mut V, node: &'ast PathSegment)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_ident(&node.ident);
    v.visit_path_arguments(&node.arguments);
}
 
pub fn visit_predicate_eq<'ast, V>(v: &mut V, node: &'ast PredicateEq)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_type(&node.lhs_ty);
    // tokens_helper(v, &node.eq_token.spans);
    v.visit_type(&node.rhs_ty);
}
 
pub fn visit_predicate_lifetime<'ast, V>(v: &mut V, node: &'ast PredicateLifetime)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_lifetime(&node.lifetime);
    // tokens_helper(v, &node.colon_token.spans);
    for el in Punctuated::pairs(&node.bounds) {
        let (it, p) = el.into_tuple();
        v.visit_lifetime(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
}
 
pub fn visit_predicate_type<'ast, V>(v: &mut V, node: &'ast PredicateType)
where
    V: Visit<'ast> + ?Sized,
{
    if let Some(it) = &node.lifetimes {
        v.visit_bound_lifetimes(it);
    }
    v.visit_type(&node.bounded_ty);
    // tokens_helper(v, &node.colon_token.spans);
    for el in Punctuated::pairs(&node.bounds) {
        let (it, p) = el.into_tuple();
        v.visit_type_param_bound(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
}
 
pub fn visit_qself<'ast, V>(v: &mut V, node: &'ast QSelf)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.lt_token.spans);
    v.visit_type(&*node.ty);
    // skip!(node.position);
    if let Some(it) = &node.as_token {
        // tokens_helper(v, &it.span);
    }
    // tokens_helper(v, &node.gt_token.spans);
}
   
pub fn visit_range_limits<'ast, V>(v: &mut V, node: &'ast RangeLimits)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        RangeLimits::HalfOpen(_binding_0) => {
            // tokens_helper(v, &_binding_0.spans);
        }
        RangeLimits::Closed(_binding_0) => {
            // tokens_helper(v, &_binding_0.spans);
        }
    }
}
   
pub fn visit_receiver<'ast, V>(v: &mut V, node: &'ast Receiver)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    if let Some(it) = &node.reference {
        // tokens_helper(v, &(it).0.spans);
        if let Some(it) = &(it).1 {
            v.visit_lifetime(it);
        }
    }
    if let Some(it) = &node.mutability {
        // tokens_helper(v, &it.span);
    }
    // tokens_helper(v, &node.self_token.span);
}
 
pub fn visit_return_type<'ast, V>(v: &mut V, node: &'ast ReturnType)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        ReturnType::Default => {}
        ReturnType::Type(_binding_0, _binding_1) => {
            // tokens_helper(v, &_binding_0.spans);
            v.visit_type(&**_binding_1);
        }
    }
}
   
pub fn visit_signature<'ast, V>(v: &mut V, node: &'ast Signature)
where
    V: Visit<'ast> + ?Sized,
{
    if let Some(it) = &node.constness {
        // tokens_helper(v, &it.span);
    }
    if let Some(it) = &node.asyncness {
        // tokens_helper(v, &it.span);
    }
    if let Some(it) = &node.unsafety {
        // tokens_helper(v, &it.span);
    }
    if let Some(it) = &node.abi {
        v.visit_abi(it);
    }
    // tokens_helper(v, &node.fn_token.span);
    v.visit_ident(&node.ident);
    v.visit_generics(&node.generics);
    // tokens_helper(v, &node.paren_token.span);
    for el in Punctuated::pairs(&node.inputs) {
        let (it, p) = el.into_tuple();
        v.visit_fn_arg(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
    if let Some(it) = &node.variadic {
        v.visit_variadic(it);
    }
    v.visit_return_type(&node.output);
}
pub fn visit_span<'ast, V>(v: &mut V, node: &Span)
where
    V: Visit<'ast> + ?Sized,
{}
   
pub fn visit_stmt<'ast, V>(v: &mut V, node: &'ast Stmt)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        Stmt::Local(_binding_0) => {
            v.visit_local(_binding_0);
        }
        Stmt::Item(_binding_0) => {
            v.visit_item(_binding_0);
        }
        Stmt::Expr(_binding_0) => {
            v.visit_expr(_binding_0);
        }
        Stmt::Semi(_binding_0, _binding_1) => {
            v.visit_expr(_binding_0);
            // tokens_helper(v, &_binding_1.spans);
        }
    }
}
 
pub fn visit_trait_bound<'ast, V>(v: &mut V, node: &'ast TraitBound)
where
    V: Visit<'ast> + ?Sized,
{
    if let Some(it) = &node.paren_token {
        // tokens_helper(v, &it.span);
    }
    v.visit_trait_bound_modifier(&node.modifier);
    if let Some(it) = &node.lifetimes {
        v.visit_bound_lifetimes(it);
    }
    v.visit_path(&node.path);
}
 
pub fn visit_trait_bound_modifier<'ast, V>(v: &mut V, node: &'ast TraitBoundModifier)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        TraitBoundModifier::None => {}
        TraitBoundModifier::Maybe(_binding_0) => {
            // tokens_helper(v, &_binding_0.spans);
        }
    }
}
   
pub fn visit_trait_item<'ast, V>(v: &mut V, node: &'ast TraitItem)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        TraitItem::Const(_binding_0) => {
            v.visit_trait_item_const(_binding_0);
        }
        TraitItem::Method(_binding_0) => {
            v.visit_trait_item_method(_binding_0);
        }
        TraitItem::Type(_binding_0) => {
            v.visit_trait_item_type(_binding_0);
        }
        TraitItem::Macro(_binding_0) => {
            v.visit_trait_item_macro(_binding_0);
        }
        TraitItem::Verbatim(_binding_0) => {
            // skip!(_binding_0);
        }
        // #[cfg(syn_no_non_exhaustive)]
        _ => unreachable!(),
    }
}
   
pub fn visit_trait_item_const<'ast, V>(v: &mut V, node: &'ast TraitItemConst)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // tokens_helper(v, &node.const_token.span);
    v.visit_ident(&node.ident);
    // tokens_helper(v, &node.colon_token.spans);
    v.visit_type(&node.ty);
    if let Some(it) = &node.default {
        // tokens_helper(v, &(it).0.spans);
        v.visit_expr(&(it).1);
    }
    // tokens_helper(v, &node.semi_token.spans);
}
   
pub fn visit_trait_item_macro<'ast, V>(v: &mut V, node: &'ast TraitItemMacro)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_macro(&node.mac);
    if let Some(it) = &node.semi_token {
        // tokens_helper(v, &it.spans);
    }
}
   
pub fn visit_trait_item_method<'ast, V>(v: &mut V, node: &'ast TraitItemMethod)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_signature(&node.sig);
    if let Some(it) = &node.default {
        v.visit_block(it);
    }
    if let Some(it) = &node.semi_token {
        // tokens_helper(v, &it.spans);
    }
}
   
pub fn visit_trait_item_type<'ast, V>(v: &mut V, node: &'ast TraitItemType)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // tokens_helper(v, &node.type_token.span);
    v.visit_ident(&node.ident);
    v.visit_generics(&node.generics);
    if let Some(it) = &node.colon_token {
        // tokens_helper(v, &it.spans);
    }
    for el in Punctuated::pairs(&node.bounds) {
        let (it, p) = el.into_tuple();
        v.visit_type_param_bound(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
    if let Some(it) = &node.default {
        // tokens_helper(v, &(it).0.spans);
        v.visit_type(&(it).1);
    }
    // tokens_helper(v, &node.semi_token.spans);
}
 
pub fn visit_type<'ast, V>(v: &mut V, node: &'ast Type)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        Type::Array(_binding_0) => {
            v.visit_type_array(_binding_0);
        }
        Type::BareFn(_binding_0) => {
            v.visit_type_bare_fn(_binding_0);
        }
        Type::Group(_binding_0) => {
            v.visit_type_group(_binding_0);
        }
        Type::ImplTrait(_binding_0) => {
            v.visit_type_impl_trait(_binding_0);
        }
        Type::Infer(_binding_0) => {
            v.visit_type_infer(_binding_0);
        }
        Type::Macro(_binding_0) => {
            v.visit_type_macro(_binding_0);
        }
        Type::Never(_binding_0) => {
            v.visit_type_never(_binding_0);
        }
        Type::Paren(_binding_0) => {
            v.visit_type_paren(_binding_0);
        }
        Type::Path(_binding_0) => {
            v.visit_type_path(_binding_0);
        }
        Type::Ptr(_binding_0) => {
            v.visit_type_ptr(_binding_0);
        }
        Type::Reference(_binding_0) => {
            v.visit_type_reference(_binding_0);
        }
        Type::Slice(_binding_0) => {
            v.visit_type_slice(_binding_0);
        }
        Type::TraitObject(_binding_0) => {
            v.visit_type_trait_object(_binding_0);
        }
        Type::Tuple(_binding_0) => {
            v.visit_type_tuple(_binding_0);
        }
        Type::Verbatim(_binding_0) => {
            // skip!(_binding_0);
        }
        // #[cfg(syn_no_non_exhaustive)]
        _ => unreachable!(),
    }
}
 
pub fn visit_type_array<'ast, V>(v: &mut V, node: &'ast TypeArray)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.bracket_token.span);
    v.visit_type(&*node.elem);
    // tokens_helper(v, &node.semi_token.spans);
    v.visit_expr(&node.len);
}
 
pub fn visit_type_bare_fn<'ast, V>(v: &mut V, node: &'ast TypeBareFn)
where
    V: Visit<'ast> + ?Sized,
{
    if let Some(it) = &node.lifetimes {
        v.visit_bound_lifetimes(it);
    }
    if let Some(it) = &node.unsafety {
        // tokens_helper(v, &it.span);
    }
    if let Some(it) = &node.abi {
        v.visit_abi(it);
    }
    // tokens_helper(v, &node.fn_token.span);
    // tokens_helper(v, &node.paren_token.span);
    for el in Punctuated::pairs(&node.inputs) {
        let (it, p) = el.into_tuple();
        v.visit_bare_fn_arg(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
    if let Some(it) = &node.variadic {
        v.visit_variadic(it);
    }
    v.visit_return_type(&node.output);
}
 
pub fn visit_type_group<'ast, V>(v: &mut V, node: &'ast TypeGroup)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.group_token.span);
    v.visit_type(&*node.elem);
}
 
pub fn visit_type_impl_trait<'ast, V>(v: &mut V, node: &'ast TypeImplTrait)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.impl_token.span);
    for el in Punctuated::pairs(&node.bounds) {
        let (it, p) = el.into_tuple();
        v.visit_type_param_bound(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
}
 
pub fn visit_type_infer<'ast, V>(v: &mut V, node: &'ast TypeInfer)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.underscore_token.spans);
}
 
pub fn visit_type_macro<'ast, V>(v: &mut V, node: &'ast TypeMacro)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_macro(&node.mac);
}
 
pub fn visit_type_never<'ast, V>(v: &mut V, node: &'ast TypeNever)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.bang_token.spans);
}
 
pub fn visit_type_param<'ast, V>(v: &mut V, node: &'ast TypeParam)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_ident(&node.ident);
    if let Some(it) = &node.colon_token {
        // tokens_helper(v, &it.spans);
    }
    for el in Punctuated::pairs(&node.bounds) {
        let (it, p) = el.into_tuple();
        v.visit_type_param_bound(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
    if let Some(it) = &node.eq_token {
        // tokens_helper(v, &it.spans);
    }
    if let Some(it) = &node.default {
        v.visit_type(it);
    }
}
 
pub fn visit_type_param_bound<'ast, V>(v: &mut V, node: &'ast TypeParamBound)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        TypeParamBound::Trait(_binding_0) => {
            v.visit_trait_bound(_binding_0);
        }
        TypeParamBound::Lifetime(_binding_0) => {
            v.visit_lifetime(_binding_0);
        }
    }
}
 
pub fn visit_type_paren<'ast, V>(v: &mut V, node: &'ast TypeParen)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.paren_token.span);
    v.visit_type(&*node.elem);
}
 
pub fn visit_type_path<'ast, V>(v: &mut V, node: &'ast TypePath)
where
    V: Visit<'ast> + ?Sized,
{
    if let Some(it) = &node.qself {
        v.visit_qself(it);
    }
    v.visit_path(&node.path);
}
 
pub fn visit_type_ptr<'ast, V>(v: &mut V, node: &'ast TypePtr)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.star_token.spans);
    if let Some(it) = &node.const_token {
        // tokens_helper(v, &it.span);
    }
    if let Some(it) = &node.mutability {
        // tokens_helper(v, &it.span);
    }
    v.visit_type(&*node.elem);
}
 
pub fn visit_type_reference<'ast, V>(v: &mut V, node: &'ast TypeReference)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.and_token.spans);
    if let Some(it) = &node.lifetime {
        v.visit_lifetime(it);
    }
    if let Some(it) = &node.mutability {
        // tokens_helper(v, &it.span);
    }
    v.visit_type(&*node.elem);
}
 
pub fn visit_type_slice<'ast, V>(v: &mut V, node: &'ast TypeSlice)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.bracket_token.span);
    v.visit_type(&*node.elem);
}
 
pub fn visit_type_trait_object<'ast, V>(v: &mut V, node: &'ast TypeTraitObject)
where
    V: Visit<'ast> + ?Sized,
{
    if let Some(it) = &node.dyn_token {
        // tokens_helper(v, &it.span);
    }
    for el in Punctuated::pairs(&node.bounds) {
        let (it, p) = el.into_tuple();
        v.visit_type_param_bound(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
}
 
pub fn visit_type_tuple<'ast, V>(v: &mut V, node: &'ast TypeTuple)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.paren_token.span);
    for el in Punctuated::pairs(&node.elems) {
        let (it, p) = el.into_tuple();
        v.visit_type(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
}
 
pub fn visit_un_op<'ast, V>(v: &mut V, node: &'ast UnOp)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        UnOp::Deref(_binding_0) => {
            // tokens_helper(v, &_binding_0.spans);
        }
        UnOp::Not(_binding_0) => {
            // tokens_helper(v, &_binding_0.spans);
        }
        UnOp::Neg(_binding_0) => {
            // tokens_helper(v, &_binding_0.spans);
        }
    }
}
   
pub fn visit_use_glob<'ast, V>(v: &mut V, node: &'ast UseGlob)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.star_token.spans);
}
   
pub fn visit_use_group<'ast, V>(v: &mut V, node: &'ast UseGroup)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.brace_token.span);
    for el in Punctuated::pairs(&node.items) {
        let (it, p) = el.into_tuple();
        v.visit_use_tree(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
}
   
pub fn visit_use_name<'ast, V>(v: &mut V, node: &'ast UseName)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_ident(&node.ident);
}
   
pub fn visit_use_path<'ast, V>(v: &mut V, node: &'ast UsePath)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_ident(&node.ident);
    // tokens_helper(v, &node.colon2_token.spans);
    v.visit_use_tree(&*node.tree);
}
   
pub fn visit_use_rename<'ast, V>(v: &mut V, node: &'ast UseRename)
where
    V: Visit<'ast> + ?Sized,
{
    v.visit_ident(&node.ident);
    // tokens_helper(v, &node.as_token.span);
    v.visit_ident(&node.rename);
}
   
pub fn visit_use_tree<'ast, V>(v: &mut V, node: &'ast UseTree)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        UseTree::Path(_binding_0) => {
            v.visit_use_path(_binding_0);
        }
        UseTree::Name(_binding_0) => {
            v.visit_use_name(_binding_0);
        }
        UseTree::Rename(_binding_0) => {
            v.visit_use_rename(_binding_0);
        }
        UseTree::Glob(_binding_0) => {
            v.visit_use_glob(_binding_0);
        }
        UseTree::Group(_binding_0) => {
            v.visit_use_group(_binding_0);
        }
    }
}
 
pub fn visit_variadic<'ast, V>(v: &mut V, node: &'ast Variadic)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    // tokens_helper(v, &node.dots.spans);
}


 
pub fn visit_variant<'ast, V>(v: &mut V, node: &'ast Variant)
where
    V: Visit<'ast> + ?Sized,
{
    for it in &node.attrs {
        v.visit_attribute(it);
    }
    v.visit_ident(&node.ident);
    v.visit_fields(&node.fields);
    if let Some(it) = &node.discriminant {
        // tokens_helper(v, &(it).0.spans);
        v.visit_expr(&(it).1);
    }
}
 
pub fn visit_vis_crate<'ast, V>(v: &mut V, node: &'ast VisCrate)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.crate_token.span);
}
 
pub fn visit_vis_public<'ast, V>(v: &mut V, node: &'ast VisPublic)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.pub_token.span);
}
 
pub fn visit_vis_restricted<'ast, V>(v: &mut V, node: &'ast VisRestricted)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.pub_token.span);
    // tokens_helper(v, &node.paren_token.span);
    if let Some(it) = &node.in_token {
        // tokens_helper(v, &it.span);
    }
    v.visit_path(&*node.path);
}
 
pub fn visit_visibility<'ast, V>(v: &mut V, node: &'ast Visibility)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        Visibility::Public(_binding_0) => {
            v.visit_vis_public(_binding_0);
        }
        Visibility::Crate(_binding_0) => {
            v.visit_vis_crate(_binding_0);
        }
        Visibility::Restricted(_binding_0) => {
            v.visit_vis_restricted(_binding_0);
        }
        Visibility::Inherited => {}
    }
}
 
pub fn visit_where_clause<'ast, V>(v: &mut V, node: &'ast WhereClause)
where
    V: Visit<'ast> + ?Sized,
{
    // tokens_helper(v, &node.where_token.span);
    for el in Punctuated::pairs(&node.predicates) {
        let (it, p) = el.into_tuple();
        v.visit_where_predicate(it);
        if let Some(p) = p {
            // tokens_helper(v, &p.spans);
        }
    }
}
 
pub fn visit_where_predicate<'ast, V>(v: &mut V, node: &'ast WherePredicate)
where
    V: Visit<'ast> + ?Sized,
{
    match node {
        WherePredicate::Type(_binding_0) => {
            v.visit_predicate_type(_binding_0);
        }
        WherePredicate::Lifetime(_binding_0) => {
            v.visit_predicate_lifetime(_binding_0);
        }
        WherePredicate::Eq(_binding_0) => {
            v.visit_predicate_eq(_binding_0);
        }
    }
}

fn record_count(name: String, storage: &mut HashMap<String, usize>) {
    let count = storage.entry(name).or_insert(0);
    *count += 1;
}
fn record_count2(name: &str, storage: &mut HashMap<String, usize>) {
    let count = storage.entry(String::from(name)).or_insert(0);
    *count += 1;
}

// viist all the node and count the node number for each node type
impl<'ast> Visit<'ast> for context {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        // let name = i.sig.ident.to_string();
        let key = String::from("ItemFn");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        for attr in &i.attrs {
            self.visit_attribute(attr);
        }
        self.visit_visibility(&i.vis);
        self.visit_signature(&i.sig);
        self.visit_block(&i.block);
    }
    fn visit_abi(&mut self, i: &'ast Abi) {
        let key = String::from("Abi");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_abi(self, i);
    }
    fn visit_angle_bracketed_generic_arguments(
        &mut self,
        i: &'ast AngleBracketedGenericArguments,
    ) {
        let key = String::from("AngleBracketedGenericArguments");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_angle_bracketed_generic_arguments(self, i);
    }
    fn visit_arm(&mut self, i: &'ast Arm) {
        let key = String::from("Arm");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_arm(self, i);
    }
     
    fn visit_attr_style(&mut self, i: &'ast AttrStyle) {
        let key = String::from("AttrStyle");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_attr_style(self, i);
    }
     
    fn visit_attribute(&mut self, i: &'ast Attribute) {
        let key = String::from("Attribute");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_attribute(self, i);
    }
     
    fn visit_bare_fn_arg(&mut self, i: &'ast BareFnArg) {
        let key = String::from("BareFnArg");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_bare_fn_arg(self, i);
    }
     
    fn visit_bin_op(&mut self, i: &'ast BinOp) {
        let key = String::from("BinOp");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_bin_op(self, i);
    }
     
    fn visit_binding(&mut self, i: &'ast Binding) {
        let key = String::from("Binding");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_binding(self, i);
    }
     
    fn visit_block(&mut self, i: &'ast Block) {
        let key = String::from("Block");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_block(self, i);
    }
     
    fn visit_bound_lifetimes(&mut self, i: &'ast BoundLifetimes) {
        let key = String::from("BoundLifetimes");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_bound_lifetimes(self, i);
    }
     
    fn visit_const_param(&mut self, i: &'ast ConstParam) {
        let key = String::from("ConstParam");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_const_param(self, i);
    }
     
    fn visit_constraint(&mut self, i: &'ast Constraint) {
        let key = String::from("Constraint");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_constraint(self, i);
    }
    fn visit_data(&mut self, i: &'ast Data) {
        let key = String::from("Data");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_data(self, i);
    }
    fn visit_data_enum(&mut self, i: &'ast DataEnum) {
        let key = String::from("DataEnum");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_data_enum(self, i);
    }
    fn visit_data_struct(&mut self, i: &'ast DataStruct) {
        let key = String::from("DataStruct");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_data_struct(self, i);
    }
    fn visit_data_union(&mut self, i: &'ast DataUnion) {
        let key = String::from("DataUnion");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_data_union(self, i);
    }
    fn visit_derive_input(&mut self, i: &'ast DeriveInput) {
        let key = String::from("DeriveInput");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_derive_input(self, i);
    }
     
    fn visit_expr(&mut self, i: &'ast Expr) {
        let key = String::from("Expr");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr(self, i);
    }
     
    fn visit_expr_array(&mut self, i: &'ast ExprArray) {
        let key = String::from("ExprArray");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_array(self, i);
    }
     
    fn visit_expr_assign(&mut self, i: &'ast ExprAssign) {
        let key = String::from("ExprAssign");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_assign(self, i);
    }
     
    fn visit_expr_assign_op(&mut self, i: &'ast ExprAssignOp) {
        let key = String::from("ExprAssignOp");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_assign_op(self, i);
    }
     
    fn visit_expr_async(&mut self, i: &'ast ExprAsync) {
        let key = String::from("ExprAsync");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_async(self, i);
    }
     
    fn visit_expr_await(&mut self, i: &'ast ExprAwait) {
        let key = String::from("ExprAwait");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_await(self, i);
    }
     
    fn visit_expr_binary(&mut self, i: &'ast ExprBinary) {
        let key = String::from("ExprBinary");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;

        visit_expr_binary(self, i);
    }
     
    fn visit_expr_block(&mut self, i: &'ast ExprBlock) {
        let key = String::from("ExprBlock");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_block(self, i);
    }
     
    fn visit_expr_box(&mut self, i: &'ast ExprBox) {
        let key = String::from("ExprBox");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_box(self, i);
    }
     
    fn visit_expr_break(&mut self, i: &'ast ExprBreak) {
        let key = String::from("ExprBreak");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_break(self, i);
    }
     
    fn visit_expr_call(&mut self, i: &'ast ExprCall) {
        let key = String::from("ExprCall");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_call(self, i);
    }
     
    fn visit_expr_cast(&mut self, i: &'ast ExprCast) {
        let key = String::from("ExprCast");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_cast(self, i);
    }
     
    fn visit_expr_closure(&mut self, i: &'ast ExprClosure) {
        let key = String::from("ExprClosure");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_closure(self, i);
    }
     
    fn visit_expr_continue(&mut self, i: &'ast ExprContinue) {
        let key = String::from("ExprContinue");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_continue(self, i);
    }
     
    fn visit_expr_field(&mut self, i: &'ast ExprField) {
        let key = String::from("ExprField");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;

        visit_expr_field(self, i);
    }
     
    fn visit_expr_for_loop(&mut self, i: &'ast ExprForLoop) {
        let key = String::from("ExprForLoop");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_for_loop(self, i);
    }
     
    fn visit_expr_group(&mut self, i: &'ast ExprGroup) {
        let key = String::from("ExprGroup");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_group(self, i);
    }
     
    fn visit_expr_if(&mut self, i: &'ast ExprIf) {
        let key = String::from("ExprIf");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_if(self, i);
    }
     
    fn visit_expr_index(&mut self, i: &'ast ExprIndex) {
        let key = String::from("ExprIndex");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_index(self, i);
    }
     
    fn visit_expr_let(&mut self, i: &'ast ExprLet) {
        let key = String::from("ExprLet");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_let(self, i);
    }
     
    fn visit_expr_lit(&mut self, i: &'ast ExprLit) {
        let key = String::from("ExprLit");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_lit(self, i);
    }
     
    fn visit_expr_loop(&mut self, i: &'ast ExprLoop) {
        let key = String::from("ExprLoop");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_loop(self, i);
    }
     
    fn visit_expr_macro(&mut self, i: &'ast ExprMacro) {
        let key = String::from("ExprMacro");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_macro(self, i);
    }
     
    fn visit_expr_match(&mut self, i: &'ast ExprMatch) {
        let key = String::from("ExprMatch");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_match(self, i);
    }
     
    fn visit_expr_method_call(&mut self, i: &'ast ExprMethodCall) {
        let key = String::from("ExprMethodCall");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_method_call(self, i);
    }
     
    fn visit_expr_paren(&mut self, i: &'ast ExprParen) {
        let key = String::from("ExprParen");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_paren(self, i);
    }
     
    fn visit_expr_path(&mut self, i: &'ast ExprPath) {
        let key = String::from("ExprPath");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_path(self, i);
    }
     
    fn visit_expr_range(&mut self, i: &'ast ExprRange) {
        let key = String::from("ExprRange");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_range(self, i);
    }
     
    fn visit_expr_reference(&mut self, i: &'ast ExprReference) {
        let key = String::from("ExprReference");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_reference(self, i);
    }
     
    fn visit_expr_repeat(&mut self, i: &'ast ExprRepeat) {
        let key = String::from("ExprRepeat");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_repeat(self, i);
    }
     
    fn visit_expr_return(&mut self, i: &'ast ExprReturn) {
        let key = String::from("ExprReturn");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_return(self, i);
    }
     
    fn visit_expr_struct(&mut self, i: &'ast ExprStruct) {
        let key = String::from("ExprStruct");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_struct(self, i);
    }
     
    fn visit_expr_try(&mut self, i: &'ast ExprTry) {
        let key = String::from("ExprTry");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_try(self, i);
    }
     
    fn visit_expr_try_block(&mut self, i: &'ast ExprTryBlock) {
        let key = String::from("ExprTryBlock");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_try_block(self, i);
    }
     
    fn visit_expr_tuple(&mut self, i: &'ast ExprTuple) {
        let key = String::from("ExprTuple");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_tuple(self, i);
    }
     
    fn visit_expr_type(&mut self, i: &'ast ExprType) {
        let key = String::from("ExprType");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_type(self, i);
    }
     
    fn visit_expr_unary(&mut self, i: &'ast ExprUnary) {
        let key = String::from("ExprUnary");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_unary(self, i);
    }
     
    fn visit_expr_unsafe(&mut self, i: &'ast ExprUnsafe) {
        let key = String::from("ExprUnsafe");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_unsafe(self, i);
    }
     
    fn visit_expr_while(&mut self, i: &'ast ExprWhile) {
        let key = String::from("ExprWhile");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_while(self, i);
    }
     
    fn visit_expr_yield(&mut self, i: &'ast ExprYield) {
        let key = String::from("ExprYield");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_expr_yield(self, i);
    }
     
    fn visit_field(&mut self, i: &'ast Field) {
        let key = String::from("Field");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_field(self, i);
    }
     
    fn visit_field_pat(&mut self, i: &'ast FieldPat) {
        let key = String::from("FieldPat");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_field_pat(self, i);
    }
     
    fn visit_field_value(&mut self, i: &'ast FieldValue) {
        let key = String::from("FieldValue");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_field_value(self, i);
    }
     
    fn visit_fields(&mut self, i: &'ast Fields) {
        let key = String::from("Fields");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_fields(self, i);
    }
     
    fn visit_fields_named(&mut self, i: &'ast FieldsNamed) {
        let key = String::from("FieldsNamed");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_fields_named(self, i);
    }
     
    fn visit_fields_unnamed(&mut self, i: &'ast FieldsUnnamed) {
        let key = String::from("FieldsUnnamed");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_fields_unnamed(self, i);
    }
     
    fn visit_file(&mut self, i: &'ast File) {
        let key = String::from("File");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_file(self, i);
    }
     
    fn visit_fn_arg(&mut self, i: &'ast FnArg) {
        let key = String::from("FnArg");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_fn_arg(self, i);
    }
     
    fn visit_foreign_item(&mut self, i: &'ast ForeignItem) {
        let key = String::from("ForeignItem");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_foreign_item(self, i);
    }
     
    fn visit_foreign_item_fn(&mut self, i: &'ast ForeignItemFn) {
        let key = String::from("ForeignItemFn");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_foreign_item_fn(self, i);
    }
     
    fn visit_foreign_item_macro(&mut self, i: &'ast ForeignItemMacro) {
        let key = String::from("ForeignItemMacro");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_foreign_item_macro(self, i);
    }
     
    fn visit_foreign_item_static(&mut self, i: &'ast ForeignItemStatic) {
        let key = String::from("ForeignItemStatic");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_foreign_item_static(self, i);
    }
     
    fn visit_foreign_item_type(&mut self, i: &'ast ForeignItemType) {
        let key = String::from("ForeignItemType");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_foreign_item_type(self, i);
    }
     
    fn visit_generic_argument(&mut self, i: &'ast GenericArgument) {
        let key = String::from("GenericArgument");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_generic_argument(self, i);
    }
     
    fn visit_generic_method_argument(&mut self, i: &'ast GenericMethodArgument) {
        let key = String::from("GenericMethodArgument");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_generic_method_argument(self, i);
    }
     
    fn visit_generic_param(&mut self, i: &'ast GenericParam) {
        let key = String::from("GenericParam");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_generic_param(self, i);
    }
     
    fn visit_generics(&mut self, i: &'ast Generics) {
        let key = String::from("Generics");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_generics(self, i);
    }
    fn visit_ident(&mut self, i: &'ast Ident) {
        let key = String::from("Ident");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_ident(self, i);
    }
     
    fn visit_impl_item(&mut self, i: &'ast ImplItem) {
        let key = String::from("ImplItem");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_impl_item(self, i);
    }
     
    fn visit_impl_item_const(&mut self, i: &'ast ImplItemConst) {
        let key = String::from("ImplItemConst");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_impl_item_const(self, i);
    }
     
    fn visit_impl_item_macro(&mut self, i: &'ast ImplItemMacro) {
        let key = String::from("ImplItemMacro");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_impl_item_macro(self, i);
    }
     
    fn visit_impl_item_method(&mut self, i: &'ast ImplItemMethod) {
        let key = String::from("ImplItemMethod");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_impl_item_method(self, i);
    }
     
    fn visit_impl_item_type(&mut self, i: &'ast ImplItemType) {
        let key = String::from("ImplItemType");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_impl_item_type(self, i);
    }
     
    fn visit_index(&mut self, i: &'ast Index) {
        let key = String::from("Index");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_index(self, i);
    }
     
    fn visit_item(&mut self, i: &'ast Item) {
        let key = String::from("Item");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item(self, i);
    }
     
    fn visit_item_const(&mut self, i: &'ast ItemConst) {
        let key = String::from("ItemConst");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item_const(self, i);
    }
     
    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        let key = String::from("ItemEnum");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item_enum(self, i);
    }
     
    fn visit_item_extern_crate(&mut self, i: &'ast ItemExternCrate) {
        let key = String::from("ItemExternCrate");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item_extern_crate(self, i);
    }
     
    // fn visit_item_fn(&mut self, i: &'ast ItemFn) {
    //     visit_item_fn(self, i);
    // }
     
    fn visit_item_foreign_mod(&mut self, i: &'ast ItemForeignMod) {
        let key = String::from("ItemForeignMod");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item_foreign_mod(self, i);
    }
     
    fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
        let key = String::from("ItemImpl");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item_impl(self, i);
    }
     
    fn visit_item_macro(&mut self, i: &'ast ItemMacro) {
        let key = String::from("ItemMacro");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item_macro(self, i);
    }
     
    fn visit_item_macro2(&mut self, i: &'ast ItemMacro2) {
        let key = String::from("ItemMacro2");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item_macro2(self, i);
    }
     
    fn visit_item_mod(&mut self, i: &'ast ItemMod) {
        let key = String::from("ItemMod");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item_mod(self, i);
    }
     
    fn visit_item_static(&mut self, i: &'ast ItemStatic) {
        let key = String::from("ItemStatic");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item_static(self, i);
    }
     
    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        let key = String::from("ItemStruct");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item_struct(self, i);
    }
     
    fn visit_item_trait(&mut self, i: &'ast ItemTrait) {
        let key = String::from("ItemTrait");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item_trait(self, i);
    }
     
    fn visit_item_trait_alias(&mut self, i: &'ast ItemTraitAlias) {
        let key = String::from("ItemTraitAlias");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item_trait_alias(self, i);
    }
     
    fn visit_item_type(&mut self, i: &'ast ItemType) {
        let key = String::from("ItemType");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item_type(self, i);
    }
     
    fn visit_item_union(&mut self, i: &'ast ItemUnion) {
        let key = String::from("ItemUnion");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item_union(self, i);
    }
     
    fn visit_item_use(&mut self, i: &'ast ItemUse) {
        let key = String::from("ItemUse");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_item_use(self, i);
    }
     
    fn visit_label(&mut self, i: &'ast Label) {
        let key = String::from("Label");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_label(self, i);
    }
    fn visit_lifetime(&mut self, i: &'ast Lifetime) {
        let key = String::from("Lifetime");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_lifetime(self, i);
    }
     
    fn visit_lifetime_def(&mut self, i: &'ast LifetimeDef) {
        let key = String::from("LifetimeDef");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_lifetime_def(self, i);
    }
    fn visit_lit(&mut self, i: &'ast Lit) {
        let key = String::from("Lit");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_lit(self, i);
    }
    fn visit_lit_bool(&mut self, i: &'ast LitBool) {
        let key = String::from("LitBool");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_lit_bool(self, i);
    }
    fn visit_lit_byte(&mut self, i: &'ast LitByte) {
        let key = String::from("LitByte");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_lit_byte(self, i);
    }
    fn visit_lit_byte_str(&mut self, i: &'ast LitByteStr) {
        let key = String::from("LitByteStr");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_lit_byte_str(self, i);
    }
    fn visit_lit_char(&mut self, i: &'ast LitChar) {
        let key = String::from("LitChar");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_lit_char(self, i);
    }
    fn visit_lit_float(&mut self, i: &'ast LitFloat) {
        let key = String::from("LitFloat");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_lit_float(self, i);
    }
    fn visit_lit_int(&mut self, i: &'ast LitInt) {
        let key = String::from("LitInt");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_lit_int(self, i);
    }
    fn visit_lit_str(&mut self, i: &'ast LitStr) {
        let key = String::from("LitStr");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_lit_str(self, i);
    }
     
    fn visit_local(&mut self, i: &'ast Local) {
        let key = String::from("Local");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_local(self, i);
    }
     
    fn visit_macro(&mut self, i: &'ast Macro) {
        let key = String::from("Macro");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_macro(self, i);
    }
     
    fn visit_macro_delimiter(&mut self, i: &'ast MacroDelimiter) {
        let key = String::from("MacroDelimiter");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_macro_delimiter(self, i);
    }
     
    fn visit_member(&mut self, i: &'ast Member) {
        let key = String::from("Member");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_member(self, i);
    }
     
    fn visit_meta(&mut self, i: &'ast Meta) {
        let key = String::from("Meta");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_meta(self, i);
    }
     
    fn visit_meta_list(&mut self, i: &'ast MetaList) {
        let key = String::from("MetaList");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_meta_list(self, i);
    }
     
    fn visit_meta_name_value(&mut self, i: &'ast MetaNameValue) {
        let key = String::from("MetaNameValue");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_meta_name_value(self, i);
    }
     
    fn visit_method_turbofish(&mut self, i: &'ast MethodTurbofish) {
        let key = String::from("MethodTurbofish");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_method_turbofish(self, i);
    }
     
    fn visit_nested_meta(&mut self, i: &'ast NestedMeta) {
        let key = String::from("NestedMeta");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_nested_meta(self, i);
    }
     
    fn visit_parenthesized_generic_arguments(
        &mut self,
        i: &'ast ParenthesizedGenericArguments,
    ) {
        let key = String::from("ParenthesizedGenericArguments");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_parenthesized_generic_arguments(self, i);
    }
     
    fn visit_pat(&mut self, i: &'ast Pat) {
        let key = String::from("Pat");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_pat(self, i);
    }
     
    fn visit_pat_box(&mut self, i: &'ast PatBox) {
        let key = String::from("PatBox");
        let count = self.storage.entry(key).or_insert(0);
        *count += 1;
        visit_pat_box(self, i);
    }

     
    fn visit_pat_ident(&mut self, i: &'ast PatIdent) {
        let key = String::from("PatIdent");
        record_count(key, &mut self.storage);
        visit_pat_ident(self, i);
    }
     
    fn visit_pat_lit(&mut self, i: &'ast PatLit) {
        let key = String::from("PatLit");
        record_count(key, &mut self.storage);
        visit_pat_lit(self, i);
    }
     
    fn visit_pat_macro(&mut self, i: &'ast PatMacro) {
        record_count2("PatMacro", &mut self.storage);
        visit_pat_macro(self, i);
    }
     
    fn visit_pat_or(&mut self, i: &'ast PatOr) {
        record_count2("PatOr", &mut self.storage);
        visit_pat_or(self, i);
    }
     
    fn visit_pat_path(&mut self, i: &'ast PatPath) {
        record_count2("PatPath", &mut self.storage);
        visit_pat_path(self, i);
    }
     
    fn visit_pat_range(&mut self, i: &'ast PatRange) {
        record_count2("PatRange", &mut self.storage);
        visit_pat_range(self, i);
    }
     

    fn visit_pat_reference(&mut self, i: &'ast PatReference) {
        record_count2("PatReference", &mut self.storage);
        visit_pat_reference(self, i);
    }
     
    fn visit_pat_rest(&mut self, i: &'ast PatRest) {
        record_count2("PatRest", &mut self.storage);
        visit_pat_rest(self, i);
    }
     
    fn visit_pat_slice(&mut self, i: &'ast PatSlice) {
        record_count2("PatSlice", &mut self.storage);
        visit_pat_slice(self, i);
    }
     
    fn visit_pat_struct(&mut self, i: &'ast PatStruct) {
        record_count2("PatStruct", &mut self.storage);
        visit_pat_struct(self, i);
    }
     
    fn visit_pat_tuple(&mut self, i: &'ast PatTuple) {
        record_count2("PatTuple", &mut self.storage);
        visit_pat_tuple(self, i);
    }
     
    fn visit_pat_tuple_struct(&mut self, i: &'ast PatTupleStruct) {
        record_count2("PatTupleStruct", &mut self.storage);
        visit_pat_tuple_struct(self, i);
    }
     
    fn visit_pat_type(&mut self, i: &'ast PatType) {
        record_count2("PatType", &mut self.storage);
        visit_pat_type(self, i);
    }
     
    fn visit_pat_wild(&mut self, i: &'ast PatWild) {
        record_count2("PatWild", &mut self.storage);
        visit_pat_wild(self, i);
    }
     
    fn visit_path(&mut self, i: &'ast Path) {
        record_count2("Path", &mut self.storage);
        visit_path(self, i);
    }
     
    fn visit_path_arguments(&mut self, i: &'ast PathArguments) {
        record_count2("PathArguments", &mut self.storage);
        visit_path_arguments(self, i);
    }
     
    fn visit_path_segment(&mut self, i: &'ast PathSegment) {
        record_count2("PathSegment", &mut self.storage);
        visit_path_segment(self, i);
    }
     
    fn visit_predicate_eq(&mut self, i: &'ast PredicateEq) {
        record_count2("PredicateEq", &mut self.storage);
        visit_predicate_eq(self, i);
    }
     
    fn visit_predicate_lifetime(&mut self, i: &'ast PredicateLifetime) {
        record_count2("PredicateLifetime", &mut self.storage);
        visit_predicate_lifetime(self, i);
    }
     
    fn visit_predicate_type(&mut self, i: &'ast PredicateType) {
        record_count2("PredicateType", &mut self.storage);
        visit_predicate_type(self, i);
    }
     
    fn visit_qself(&mut self, i: &'ast QSelf) {
        record_count2("QSelf", &mut self.storage);
        visit_qself(self, i);
    }
     
    fn visit_range_limits(&mut self, i: &'ast RangeLimits) {
        record_count2("RangeLimits", &mut self.storage);
        visit_range_limits(self, i);
    }
     
    fn visit_receiver(&mut self, i: &'ast Receiver) {
        record_count2("Receiver", &mut self.storage);
        visit_receiver(self, i);
    }
     
    fn visit_return_type(&mut self, i: &'ast ReturnType) {
        record_count2("ReturnType", &mut self.storage);
        visit_return_type(self, i);
    }
     
    fn visit_signature(&mut self, i: &'ast Signature) {
        record_count2("Signature", &mut self.storage);
        visit_signature(self, i);
    }
    fn visit_span(&mut self, i: &Span) {
        record_count2("Span", &mut self.storage);
        visit_span(self, i);
    }
     
    fn visit_stmt(&mut self, i: &'ast Stmt) {
        record_count2("Stmt", &mut self.storage);
        visit_stmt(self, i);
    }
     
    fn visit_trait_bound(&mut self, i: &'ast TraitBound) {
        record_count2("TraitBound", &mut self.storage);
        visit_trait_bound(self, i);
    }
     
    fn visit_trait_bound_modifier(&mut self, i: &'ast TraitBoundModifier) {
        record_count2("TraitBoundModifier", &mut self.storage);
        visit_trait_bound_modifier(self, i);
    }
     
    fn visit_trait_item(&mut self, i: &'ast TraitItem) {
        record_count2("TraitItem", &mut self.storage);
        visit_trait_item(self, i);
    }
     
    fn visit_trait_item_const(&mut self, i: &'ast TraitItemConst) {
        record_count2("TraitItemConst", &mut self.storage);
        visit_trait_item_const(self, i);
    }
     
    fn visit_trait_item_macro(&mut self, i: &'ast TraitItemMacro) {
        record_count2("TraitItemMacro", &mut self.storage);
        visit_trait_item_macro(self, i);
    }
     
    fn visit_trait_item_method(&mut self, i: &'ast TraitItemMethod) {
        record_count2("TraitItemMethod", &mut self.storage);
        visit_trait_item_method(self, i);
    }
     
    fn visit_trait_item_type(&mut self, i: &'ast TraitItemType) {
        record_count2("TraitItemType", &mut self.storage);
        visit_trait_item_type(self, i);
    }
     
    fn visit_type(&mut self, i: &'ast Type) {
        record_count2("Type", &mut self.storage);
        visit_type(self, i);
    }
    fn visit_type_array(&mut self, i: &'ast TypeArray) {
        record_count2("TypeArray", &mut self.storage);
        visit_type_array(self, i);
    }
    fn visit_type_bare_fn(&mut self, i: &'ast TypeBareFn) {
        record_count2("TypeBareFn", &mut self.storage);
        visit_type_bare_fn(self, i);
    }
    fn visit_type_group(&mut self, i: &'ast TypeGroup) {
        record_count2("TypeGroup", &mut self.storage);
        visit_type_group(self, i);
    }
    fn visit_type_impl_trait(&mut self, i: &'ast TypeImplTrait) {
        record_count2("TypeImplTrait", &mut self.storage);
        visit_type_impl_trait(self, i);
    }
    fn visit_type_infer(&mut self, i: &'ast TypeInfer) {
        record_count2("TypeInfer", &mut self.storage);
        visit_type_infer(self, i);
    }
    fn visit_type_macro(&mut self, i: &'ast TypeMacro) {
        record_count2("TypeMacro", &mut self.storage);
        visit_type_macro(self, i);
    }
    fn visit_type_never(&mut self, i: &'ast TypeNever) {
        record_count2("TypeNever", &mut self.storage);
        visit_type_never(self, i);
    }
    fn visit_type_param(&mut self, i: &'ast TypeParam) {
        record_count2("TypeParam", &mut self.storage);
        visit_type_param(self, i);
    }
    fn visit_type_param_bound(&mut self, i: &'ast TypeParamBound) {
        record_count2("TypeParamBound", &mut self.storage);
        visit_type_param_bound(self, i);
    }
    fn visit_type_paren(&mut self, i: &'ast TypeParen) {
        record_count2("TypeParen", &mut self.storage);
        visit_type_paren(self, i);
    }
    fn visit_type_path(&mut self, i: &'ast TypePath) {
        record_count2("TypePath", &mut self.storage);
        visit_type_path(self, i);
    }
    fn visit_type_ptr(&mut self, i: &'ast TypePtr) {
        record_count2("TypePtr", &mut self.storage);
        visit_type_ptr(self, i);
    }
    fn visit_type_reference(&mut self, i: &'ast TypeReference) {
        record_count2("TypeReference", &mut self.storage);
        visit_type_reference(self, i);
    }
    fn visit_type_slice(&mut self, i: &'ast TypeSlice) {
        record_count2("TypeSlice", &mut self.storage);
        visit_type_slice(self, i);
    }
    fn visit_type_trait_object(&mut self, i: &'ast TypeTraitObject) {
        record_count2("TypeTraitObject", &mut self.storage);
        visit_type_trait_object(self, i);
    }
    fn visit_type_tuple(&mut self, i: &'ast TypeTuple) {
        record_count2("TypeTuple", &mut self.storage);
        visit_type_tuple(self, i);
    }
    fn visit_un_op(&mut self, i: &'ast UnOp) {
        record_count2("UnOp", &mut self.storage);
        visit_un_op(self, i);
    }
    fn visit_use_glob(&mut self, i: &'ast UseGlob) {
        record_count2("UseGlob", &mut self.storage);
        visit_use_glob(self, i);
    }
    fn visit_use_group(&mut self, i: &'ast UseGroup) {
        record_count2("UseGroup", &mut self.storage);
        visit_use_group(self, i);
    }
    fn visit_use_name(&mut self, i: &'ast UseName) {
        record_count2("UseName", &mut self.storage);
        visit_use_name(self, i);
    }
    fn visit_use_path(&mut self, i: &'ast UsePath) {
        record_count2("UsePath", &mut self.storage);
        visit_use_path(self, i);
    }
    fn visit_use_rename(&mut self, i: &'ast UseRename) {
        record_count2("UseRename", &mut self.storage);
        visit_use_rename(self, i);
    }
    fn visit_use_tree(&mut self, i: &'ast UseTree) {
        record_count2("UseTree", &mut self.storage);
        visit_use_tree(self, i);
    }
    fn visit_variadic(&mut self, i: &'ast Variadic) {
        record_count2("Variadic", &mut self.storage);
        visit_variadic(self, i);
    }
    fn visit_variant(&mut self, i: &'ast Variant) {
        record_count2("Variant", &mut self.storage);
        visit_variant(self, i);
    }
    fn visit_vis_crate(&mut self, i: &'ast VisCrate) {
        record_count2("VisCrate", &mut self.storage);
        visit_vis_crate(self, i);
    }
    fn visit_vis_public(&mut self, i: &'ast VisPublic) {
        record_count2("VisPublic", &mut self.storage);
        visit_vis_public(self, i);
    }
    fn visit_vis_restricted(&mut self, i: &'ast VisRestricted) {
        record_count2("VisRestricted", &mut self.storage);
        visit_vis_restricted(self, i);
    }
    fn visit_visibility(&mut self, i: &'ast Visibility) {
        record_count2("Visibility", &mut self.storage);
        visit_visibility(self, i);
    }
    fn visit_where_clause(&mut self, i: &'ast WhereClause) {
        record_count2("WhereClause", &mut self.storage);
        visit_where_clause(self, i);
    }
    fn visit_where_predicate(&mut self, i: &'ast WherePredicate) {
        record_count2("WherePredicate", &mut self.storage);
        visit_where_predicate(self, i);
    }
}


impl context {
    pub fn new(p: String) -> context {
        let mut ctx = context {
            path: p,
            storage: HashMap::new()
        };
        // initial all the node count to 0
        ctx.storage.insert("ItemFn".to_string(), 0);
        ctx.storage.insert("Abi".to_string(), 0);
        ctx.storage.insert("AngleBracketedGenericArguments".to_string(), 0);
        ctx.storage.insert("Arm".to_string(), 0);
        ctx.storage.insert("AttrStyle".to_string(), 0);
        ctx.storage.insert("Attribute".to_string(), 0);
        ctx.storage.insert("BareFnArg".to_string(), 0);
        ctx.storage.insert("Binding".to_string(), 0);
        ctx.storage.insert("Block".to_string(), 0);
        ctx.storage.insert("Body".to_string(), 0);
        ctx.storage.insert("Captured".to_string(), 0);
        ctx.storage.insert("ConstParam".to_string(), 0);
        ctx.storage.insert("Constraint".to_string(), 0);
        ctx.storage.insert("Data".to_string(), 0);
        ctx.storage.insert("DataEnum".to_string(), 0);
        ctx.storage.insert("DataStruct".to_string(), 0);
        ctx.storage.insert("DataUnion".to_string(), 0);
        ctx.storage.insert("DeriveInput".to_string(), 0);
        ctx.storage.insert("Expr".to_string(), 0);
        ctx.storage.insert("ExprArray".to_string(), 0);
        ctx.storage.insert("ExprAssign".to_string(), 0);
        ctx.storage.insert("ExprAssignOp".to_string(), 0);
        ctx.storage.insert("ExprAsync".to_string(), 0);
        ctx.storage.insert("ExprAwait".to_string(), 0);
        ctx.storage.insert("ExprBinary".to_string(), 0);
        ctx.storage.insert("ExprBlock".to_string(), 0);
        ctx.storage.insert("ExprBox".to_string(), 0);
        ctx.storage.insert("ExprBreak".to_string(), 0);
        ctx.storage.insert("ExprCall".to_string(), 0);
        ctx.storage.insert("ExprCast".to_string(), 0);
        ctx.storage.insert("ExprClosure".to_string(), 0);
        ctx.storage.insert("ExprContinue".to_string(), 0);
        ctx.storage.insert("ExprField".to_string(), 0);
        ctx.storage.insert("ExprForLoop".to_string(), 0);
        ctx.storage.insert("ExprGroup".to_string(), 0);
        ctx.storage.insert("ExprIf".to_string(), 0);
        ctx.storage.insert("ExprIndex".to_string(), 0);
        ctx.storage.insert("ExprLet".to_string(), 0);
        ctx.storage.insert("ExprLit".to_string(), 0);
        ctx.storage.insert("ExprLoop".to_string(), 0);
        ctx.storage.insert("ExprMacro".to_string(), 0);
        ctx.storage.insert("ExprMatch".to_string(), 0);
        ctx.storage.insert("ExprMethodCall".to_string(), 0);
        ctx.storage.insert("ExprParen".to_string(), 0);
        ctx.storage.insert("ExprPath".to_string(), 0);
        ctx.storage.insert("ExprRange".to_string(), 0);
        ctx.storage.insert("ExprReference".to_string(), 0);
        ctx.storage.insert("ExprRepeat".to_string(), 0);
        ctx.storage.insert("ExprRet".to_string(), 0);
        ctx.storage.insert("ExprStruct".to_string(), 0);
        ctx.storage.insert("ExprTry".to_string(), 0);
        ctx.storage.insert("ExprTryBlock".to_string(), 0);
        ctx.storage.insert("ExprTuple".to_string(), 0);
        ctx.storage.insert("ExprType".to_string(), 0);
        ctx.storage.insert("ExprUnary".to_string(), 0);
        ctx.storage.insert("ExprUnsafe".to_string(), 0);
        ctx.storage.insert("ExprWhile".to_string(), 0);
        ctx.storage.insert("ExprYield".to_string(), 0);
        ctx.storage.insert("Field".to_string(), 0);
        ctx.storage.insert("FieldPat".to_string(), 0);
        ctx.storage.insert("FieldValue".to_string(), 0);
        ctx.storage.insert("Fields".to_string(), 0);
        ctx.storage.insert("FieldsNamed".to_string(), 0);
        ctx.storage.insert("FieldsUnnamed".to_string(), 0);
        ctx.storage.insert("File".to_string(), 0);
        ctx.storage.insert("FnArg".to_string(), 0);
        ctx.storage.insert("ForeignItem".to_string(), 0);
        ctx.storage.insert("ForeignItemFn".to_string(), 0);
        ctx.storage.insert("ForeignItemMacro".to_string(), 0);
        ctx.storage.insert("ForeignItemStatic".to_string(), 0);
        ctx.storage.insert("ForeignItemType".to_string(), 0);
        ctx.storage.insert("GenericArgument".to_string(), 0);
        ctx.storage.insert("GenericMethodArgument".to_string(), 0);
        ctx.storage.insert("GenericParam".to_string(), 0);
        ctx.storage.insert("Generics".to_string(), 0);
        ctx.storage.insert("Ident".to_string(), 0);
        ctx.storage.insert("ImplItem".to_string(), 0);
        ctx.storage.insert("ImplItemConst".to_string(), 0);
        ctx.storage.insert("ImplItemMacro".to_string(), 0);
        ctx.storage.insert("ImplItemMethod".to_string(), 0);
        ctx.storage.insert("ImplItemType".to_string(), 0);
        ctx.storage.insert("Index".to_string(), 0);
        ctx.storage.insert("Item".to_string(), 0);
        ctx.storage.insert("ItemConst".to_string(), 0);
        ctx.storage.insert("ItemEnum".to_string(), 0);
        ctx.storage.insert("ItemExternCrate".to_string(), 0);
        ctx.storage.insert("ItemForeignMod".to_string(), 0);
        ctx.storage.insert("ItemImpl".to_string(), 0);
        ctx.storage.insert("ItemMacro".to_string(), 0);
        ctx.storage.insert("ItemMacro2".to_string(), 0);
        ctx.storage.insert("ItemMod".to_string(), 0);
        ctx.storage.insert("ItemStatic".to_string(), 0);
        ctx.storage.insert("ItemStruct".to_string(), 0);
        ctx.storage.insert("ItemTrait".to_string(), 0);
        ctx.storage.insert("ItemTraitAlias".to_string(), 0);
        ctx.storage.insert("ItemType".to_string(), 0);
        ctx.storage.insert("ItemUnion".to_string(), 0);
        ctx.storage.insert("ItemUse".to_string(), 0);
        ctx.storage.insert("Label".to_string(), 0);
        ctx.storage.insert("Lifetime".to_string(), 0);
        ctx.storage.insert("LifetimeDef".to_string(), 0);
        ctx.storage.insert("Lit".to_string(), 0);
        ctx.storage.insert("LitBool".to_string(), 0);
        ctx.storage.insert("LitByte".to_string(), 0);
        ctx.storage.insert("LitByteStr".to_string(), 0);
        ctx.storage.insert("LitChar".to_string(), 0);
        ctx.storage.insert("LitFloat".to_string(), 0);
        ctx.storage.insert("LitInt".to_string(), 0);
        ctx.storage.insert("LitStr".to_string(), 0);
        ctx.storage.insert("Local".to_string(), 0);
        ctx.storage.insert("Macro".to_string(), 0);
        ctx.storage.insert("MacroDelimiter".to_string(), 0);
        ctx.storage.insert("Member".to_string(), 0);
        ctx.storage.insert("Meta".to_string(), 0);
        ctx.storage.insert("MetaList".to_string(), 0);
        ctx.storage.insert("MetaNameValue".to_string(), 0);
        ctx.storage.insert("MethodTurbofish".to_string(), 0);
        ctx.storage.insert("NestedMeta".to_string(), 0);
        ctx.storage.insert("ParenthesizedGenericArguments".to_string(), 0);
        ctx.storage.insert("Pat".to_string(), 0);
        ctx.storage.insert("PatBox".to_string(), 0);
        ctx.storage.insert("PatIdent".to_string(), 0);
        ctx.storage.insert("PatLit".to_string(), 0);
        ctx.storage.insert("PatMacro".to_string(), 0);
        ctx.storage.insert("PatOr".to_string(), 0);
        ctx.storage.insert("PatPath".to_string(), 0);
        ctx.storage.insert("PatRange".to_string(), 0);
        ctx.storage.insert("PatReference".to_string(), 0);
        ctx.storage.insert("PatRest".to_string(), 0);
        ctx.storage.insert("PatSlice".to_string(), 0);
        ctx.storage.insert("PatStruct".to_string(), 0);
        ctx.storage.insert("PatTuple".to_string(), 0);
        ctx.storage.insert("PatTupleStruct".to_string(), 0);
        ctx.storage.insert("PatType".to_string(), 0);
        ctx.storage.insert("PatWild".to_string(), 0);
        ctx.storage.insert("Path".to_string(), 0);
        ctx.storage.insert("PathArguments".to_string(), 0);
        ctx.storage.insert("PathSegment".to_string(), 0);
        ctx.storage.insert("PredicateEq".to_string(), 0);
        ctx.storage.insert("PredicateLifetime".to_string(), 0);
        ctx.storage.insert("PredicateType".to_string(), 0);
        ctx.storage.insert("QSelf".to_string(), 0);
        ctx.storage.insert("RangeLimits".to_string(), 0);
        ctx.storage.insert("Receiver".to_string(), 0);
        ctx.storage.insert("ReturnType".to_string(), 0);
        ctx.storage.insert("Signature".to_string(), 0);
        ctx.storage.insert("Span".to_string(), 0);
        ctx.storage.insert("Stmt".to_string(), 0);
        ctx.storage.insert("TraitBound".to_string(), 0);
        ctx.storage.insert("TraitBoundModifier".to_string(), 0);
        ctx.storage.insert("TraitItem".to_string(), 0);
        ctx.storage.insert("TraitItemConst".to_string(), 0);
        ctx.storage.insert("TraitItemMacro".to_string(), 0);
        ctx.storage.insert("TraitItemMethod".to_string(), 0);
        ctx.storage.insert("TraitItemType".to_string(), 0);
        ctx.storage.insert("Type".to_string(), 0);
        ctx.storage.insert("TypeArray".to_string(), 0);
        ctx.storage.insert("TypeBareFn".to_string(), 0);
        ctx.storage.insert("TypeGroup".to_string(), 0);
        ctx.storage.insert("TypeImplTrait".to_string(), 0);
        ctx.storage.insert("TypeInfer".to_string(), 0);
        ctx.storage.insert("TypeMacro".to_string(), 0);
        ctx.storage.insert("TypeNever".to_string(), 0);
        ctx.storage.insert("TypeParam".to_string(), 0);
        ctx.storage.insert("TypeParamBound".to_string(), 0);
        ctx.storage.insert("TypeParen".to_string(), 0);
        ctx.storage.insert("TypePath".to_string(), 0);
        ctx.storage.insert("TypePtr".to_string(), 0);
        ctx.storage.insert("TypeReference".to_string(), 0);
        ctx.storage.insert("TypeSlice".to_string(), 0);
        ctx.storage.insert("TypeTraitObject".to_string(), 0);
        ctx.storage.insert("TypeTuple".to_string(), 0);
        ctx.storage.insert("UnOp".to_string(), 0);
        ctx.storage.insert("UseGlob".to_string(), 0);
        ctx.storage.insert("UseGroup".to_string(), 0);
        ctx.storage.insert("UseName".to_string(), 0);
        ctx.storage.insert("UsePath".to_string(), 0);
        ctx.storage.insert("UseRename".to_string(), 0);
        ctx.storage.insert("UseTree".to_string(), 0);
        ctx.storage.insert("Variadic".to_string(), 0);
        ctx.storage.insert("Variant".to_string(), 0);
        ctx.storage.insert("VisCrate".to_string(), 0);
        ctx.storage.insert("VisPublic".to_string(), 0);
        ctx.storage.insert("VisRestricted".to_string(), 0);
        ctx.storage.insert("Visibility".to_string(), 0);
        ctx.storage.insert("WhereClause".to_string(), 0);
        ctx.storage.insert("WherePredicate".to_string(), 0);
        return ctx;




    }

    pub fn start(&mut self) {
        // read all the file in a dir of path
         for entry in std::fs::read_dir(&self.path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            // let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let file_content = std::fs::read_to_string(&path).unwrap();

            if let Ok(file_ast) = parse_file(&file_content) {
                self.visit_file(&file_ast);
            }
         }
    }

    pub fn print_result(&self) {
        for (key, value) in &self.storage {
            println!("{}: {}", key, value);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut ctx = context::new(path.clone());
    ctx.start();
    ctx.print_result();
}
