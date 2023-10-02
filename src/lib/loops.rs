use quote::quote;
use syn::parse_quote;
use syn::{ItemFn, Stmt};
use wasm_bindgen::prelude::wasm_bindgen;

macro_rules! parse_syn_int {
    ($v: ident) => {
        if let syn::Lit::Int(val) = &$v.lit {
            val.base10_parse::<usize>().unwrap()
        } else {
            0
        }
    };
}

fn transform_stmt(stmt: &Stmt) -> Vec<Stmt> {
    match stmt {
        Stmt::Local(syn::Local {
            attrs: _,
            let_token: _,
            pat,
            init,
            semi_token: _,
        }) => {
            let init = init.as_ref().unwrap();
            let expr = &init.expr;
            if let syn::Pat::Ident(ident) = pat {
                if ident.mutability.is_some() {
                    return vec![parse_quote! { lm!(#expr); }];
                } else {
                    return vec![parse_quote! { l!(#expr); }];
                }
            }
        }
        Stmt::Expr(syn::Expr::ForLoop(loop_expr), _) => {
            if let syn::Expr::Range(range_expr) = &*loop_expr.expr {
                if let (syn::Expr::Lit(start), syn::Expr::Lit(end)) = (
                    range_expr.start.clone().unwrap().as_ref(),
                    range_expr.end.clone().unwrap().as_ref(),
                ) {
                    let loop_body = &loop_expr.body;
                    let start_val = parse_syn_int![start];
                    let end_val = parse_syn_int![end];
                    let times = end_val - start_val;
                    let new_body = transform_block(loop_body);
                    return vec![parse_quote! { DO!(#new_body, #times); }];
                }
            }
        }
        Stmt::Expr(syn::Expr::Loop(loop_expr), _) => {
            let new_body = transform_block(&loop_expr.body);
            return vec![parse_quote! { DO!(#new_body); }];
        }
        Stmt::Expr(syn::Expr::While(loop_expr), _) => {
            let cond = &loop_expr.cond;
            let new_body = transform_block(&loop_expr.body);
            return vec![parse_quote! { W!(#cond, { #new_body }); }];
        }
        Stmt::Expr(syn::Expr::If(if_expr), _) => {
            let inner = &if_expr.cond;
            let then_body = transform_block(&if_expr.then_branch);
            if if_expr.else_branch.is_some() {
                let else_body = &*if_expr.else_branch.as_ref().unwrap().1;
                return vec![
                    parse_quote! { I!(#inner,  #then_body ); },
                    parse_quote! { I!(#inner,  #else_body ); },
                ];
            }
            return vec![parse_quote! { I!(#inner, { #then_body }); }];
        }
        Stmt::Expr(syn::Expr::Match(match_expr), _) => {
            let expr = &match_expr.expr;
            let arms = &match_expr.arms;
            let patterns = arms.iter().map(|arm| &arm.pat);
            let bodies = arms.iter().map(|arm| &arm.body);
            let transformed: Vec<_> = patterns
                .zip(bodies)
                .map(|(pat, body)| {
                    quote! { #pat , #body }
                })
                .collect();
            return vec![parse_quote! { M!(#expr; #(#transformed);*); }];
        }
        _ => return vec![stmt.clone()],
    }
    vec![]
}

fn transform_block(block: &syn::Block) -> syn::Block {
    let mut transformed_stmts = Vec::new();
    for stmt in &block.stmts {
        transformed_stmts.extend(transform_stmt(stmt));
    }

    syn::Block {
        stmts: transformed_stmts,
        ..block.clone()
    }
}

pub fn transform_loop_with_macro(item: ItemFn) -> ItemFn {
    let transformed_block = transform_block(&item.block);
    ItemFn {
        block: Box::new(transformed_block),
        ..item
    }
}

pub fn unfold_loop(item: ItemFn) -> ItemFn {
    let mut unfolded = vec![];
    for stmt in &item.block.stmts {
        if let Stmt::Expr(syn::Expr::ForLoop(loop_expr), _s) = stmt {
            if let syn::Expr::Range(range_expr) = &*loop_expr.expr {
                if let (syn::Expr::Lit(start), syn::Expr::Lit(end)) = (
                    range_expr.start.clone().unwrap().as_ref(),
                    range_expr.end.clone().unwrap().as_ref(),
                ) {
                    let start_val = parse_syn_int![start];
                    let end_val = parse_syn_int![end];
                    for _ in start_val..end_val {
                        for loop_stmt in &loop_expr.body.stmts {
                            unfolded.push(loop_stmt.clone());
                        }
                    }
                    continue;
                }
            }
        }
        unfolded.push(stmt.clone());
    }

    ItemFn {
        block: Box::new(syn::Block {
            stmts: unfolded,
            brace_token: item.block.brace_token,
        }),
        ..item
    }
}

pub fn unfolder() {
    let input_rust_code = "
    fn main() {
        for i in 0..5 {
            println!(\"Hello, world!\");
        }
    }";
    let item_fn = syn::parse_str(input_rust_code).unwrap();
    let unfolded_fn = unfold_loop(item_fn);
    let generated_code = quote! { #unfolded_fn };
    println!("{generated_code}");
}

#[wasm_bindgen]
pub fn mkterse(i: String) -> String {
    let item_fn: ItemFn = syn::parse_str(&i).unwrap();
    let transformed_fn = transform_loop_with_macro(item_fn);
    let generated_code = quote! { #transformed_fn };
    let f: syn::File = syn::parse2(generated_code).unwrap();
    let pretty = prettyplease::unparse(&f);
    pretty.to_string()
}
