use quote::quote;
use syn::parse_quote;
use syn::{ItemFn, Stmt};

fn unfold_loop(item: ItemFn) -> ItemFn {
    let mut unfolded = vec![];
    for stmt in &item.block.stmts {
        if let Stmt::Expr(syn::Expr::ForLoop(loop_expr), _s) = stmt {
            if let syn::Expr::Range(range_expr) = &*loop_expr.expr {
                if let (syn::Expr::Lit(start), syn::Expr::Lit(end)) = (
                    range_expr.start.clone().unwrap().as_ref(),
                    range_expr.end.clone().unwrap().as_ref(),
                ) {
                    let start_val = if let syn::Lit::Int(val) = &start.lit {
                        val.base10_parse::<usize>().unwrap()
                    } else {
                        0
                    };
                    let end_val = if let syn::Lit::Int(val) = &end.lit {
                        val.base10_parse::<usize>().unwrap()
                    } else {
                        0
                    };

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

fn transform_loop_with_macro(item: ItemFn) -> ItemFn {
    let mut transformed_stmts = Vec::new();

    for stmt in &item.block.stmts {
        match stmt {
            // For range loop: for i in 0..x { ... }
            Stmt::Expr(syn::Expr::ForLoop(loop_expr), _) => {
                if let syn::Expr::Range(range_expr) = &*loop_expr.expr {
                    if let (syn::Expr::Lit(start), syn::Expr::Lit(end)) = (
                        range_expr.start.clone().unwrap().as_ref(),
                        range_expr.end.clone().unwrap().as_ref(),
                    ) {
                        let loop_body = &loop_expr.body;
                        let start_val = if let syn::Lit::Int(val) = &start.lit {
                            val.base10_parse::<usize>().unwrap()
                        } else {
                            0
                        };
                        let end_val = if let syn::Lit::Int(val) = &end.lit {
                            val.base10_parse::<usize>().unwrap()
                        } else {
                            0
                        };
                        let times = end_val - start_val;
                        let new_stmt: Stmt = parse_quote! { DO!(#loop_body, #times); };
                        transformed_stmts.push(new_stmt);
                    }
                }
            }
            // Unconditional loop: loop { ... }
            Stmt::Expr(syn::Expr::Loop(loop_expr), _) => {
                let loop_body = &loop_expr.body;
                let new_stmt: Stmt = parse_quote! { DO!(#loop_body); };
                transformed_stmts.push(new_stmt);
            }
            _ => {
                transformed_stmts.push(stmt.clone());
            }
        }
    }

    ItemFn {
        block: Box::new(syn::Block {
            stmts: transformed_stmts,
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

pub fn terser_loops() {
    let input_rust_code = "
    fn main() {
        for i in 0..5 {
            println!(\"Hello, world!\");
        }
        loop {
            println!(\"Forever loop!\");
        }
    }";

    let item_fn: ItemFn = syn::parse_str(input_rust_code).unwrap();
    let transformed_fn = transform_loop_with_macro(item_fn);
    let generated_code = quote! { #transformed_fn };
    println!("{}", generated_code);
}

fn main() {
    unfolder();
    terser_loops();
}
