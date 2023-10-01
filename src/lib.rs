use wasm_bindgen::prelude::wasm_bindgen;
use quote::quote;
use syn::parse_quote;
use syn::{ItemFn, Stmt};

/// forks
#[macro_export]
macro_rules! atop {
    ($a: expr, $b: expr, $c: expr) => {
        $a($b($c))
    };
}
#[macro_export]
macro_rules! t {
    ($a: expr, $b: expr, $c: expr) => {
        |d| $b($a(&d), $c(&d))
    };
}
#[macro_export]
macro_rules! m {
    ($a: expr, $b: expr, $c: expr) => {
        $a.iter().map(|d| $b(d, $c)).collect()
    };
}
#[macro_export]
macro_rules! mm {
    ($a: expr, $b: expr, $c: expr) => {
        $a.iter().map(|d| $b(d, $c)).collect()
    };
}

/// looping
#[macro_export]
macro_rules! DO {
    ($code: expr) => {
        loop {
            $code
        }
    };
    ($code: expr, $x: expr) => {
        for _ in 0..($x) {
            $code
        }
    };
}
#[macro_export]
macro_rules! W {
    ($cond:expr, $code: expr) => {
        while $cond {
            $code
        }
    };
}

/// assigning
#[macro_export]
macro_rules! l { ($($i: ident, $code: expr);+) => { $(let $i = $code;)+ }; }
#[macro_export]
macro_rules! lm { ($($i: ident, $code: expr);+) => { $(let mut $i = $code;)+ }; }

/// conditionals
#[macro_export]
macro_rules! I {
    ($cond: expr, $code: expr) => {
        if $cond {
            $code
        }
    };
}
#[macro_export]
macro_rules! Ic {
    ($cond: expr, $code: expr) => {
        if $cond {
            $code;
            continue;
        }
    };
}
#[macro_export]
macro_rules! Ib {
    ($cond: expr, $code: expr) => {
        if $cond {
            $code;
            break;
        }
    };
}
#[macro_export]
macro_rules! tf {
    ($a: expr) => {
        $a = !$a;
    };
}
#[macro_export]
macro_rules! tt {
    ($a: expr) => {
        $a = true;
    };
}
#[macro_export]
macro_rules! ff {
    ($a: expr) => {
        $a = false;
    };
}
#[macro_export]
macro_rules! nz {
    ($i: expr) => {
        $i != 0
    };
}
#[macro_export]
macro_rules! z {
    ($i: expr) => {
        $i == 0
    };
}
#[macro_export]
macro_rules! ls {
    (s $v: ident, $e: expr, $c: expr) => {
        if let Some($v) = $e {
            $c;
        } else {
        }
    };
    (e $v: ident, $e: expr, $c: expr) => {
        if let Err($v) = $e {
            $c;
        } else {
        }
    };
}
/// if let some

/// inc
#[macro_export]
macro_rules! ic {
    ($i: expr) => {
        $i += 1;
    };
}
#[macro_export]
macro_rules! dc {
    ($i: expr) => {
        $i -= 1;
    };
}

/// iters
#[macro_export]
macro_rules! _m {
    ($l: expr, $m: expr) => {
        $l.map($m)
    };
}
#[macro_export]
macro_rules! _i {
    ($l: expr) => {
        $l.iter()
    };
}
#[macro_export]
macro_rules! c {
    ($l: expr) => {
        $l.collect::<Vec<i32>>()
    };
}
#[macro_export]
macro_rules! _n {
    ($code: expr) => {
        $code.next()
    };
}
#[macro_export]
macro_rules! _u {
    ($code: expr) => {
        $code.unwrap()
    };
}
#[macro_export]
macro_rules! _nu {
    ($code: expr) => {
        _u!(_n!($code))
    };
} // unwrap next

/// IO
#[macro_export]
macro_rules! D { ($($code:expr),+) => { $(print!("{:?} ", $code);)+ println!(); }; }
#[macro_export]
macro_rules! O { ($($code:expr),+) => { $(print!("{} ", $code);)+ println!(); }; }
#[macro_export]
macro_rules! Fc {
    ($code: expr) => {
        std::fs::File::create($code)
    };
}
#[macro_export]
macro_rules! Fr {
    ($code: expr) => {
        std::fs::read($code)
    };
}

/// structs
#[macro_export]
macro_rules! S {
    ($n: ident ) => { struct $n {} };
    ($n: ident, $($a: ident, $c: ty),+) => { struct $n { $($a: $c,)+ } };
}
#[macro_export]
macro_rules! _I {
    ($name: ty, $fn: ident) => {
        impl $name {
            fn $fn() {}
        }
    };
}
#[macro_export]
macro_rules! f { ($t: ty, $n: ident, $(($var: ident: $p: ty)),+, ex $e: expr) => { fn $n($($var: $p),+) -> $t { $e } }; }

#[macro_export]
macro_rules! br {
    ($expr: expr) => {{
        $expr;
        continue;
    }};
}

#[macro_export]
macro_rules! M {($e:expr; $($p:pat,$v:expr);+) => (match $e {$($p => $v),+});}
#[macro_export]
macro_rules! Cn {
    ($code: expr) => {
        std::process::Command::new($code)
    };
}

///casting
#[macro_export]
macro_rules! _c {
    ($code: expr) => {
        $code as char
    };
}

pub const T: bool = true;
pub const F: bool = false;
pub type Sstr = &'static str;

fn transform_stmt(stmt: &Stmt) -> Vec<Stmt> {
    match stmt {
        Stmt::Expr(syn::Expr::ForLoop(loop_expr), _) => {
            if let syn::Expr::Range(range_expr) = &*loop_expr.expr {
                if let (syn::Expr::Lit(start), syn::Expr::Lit(end)) = (
                    range_expr.start.clone().unwrap().as_ref(),
                    range_expr.end.clone().unwrap().as_ref(),
                ) {
                    let loop_body = &loop_expr.body;
                    let start_val = if let syn::Lit::Int(val) = &start.lit {
                        val.base10_parse::<usize>().unwrap_or(0)
                    } else {
                        0
                    };
                    let end_val = if let syn::Lit::Int(val) = &end.lit {
                        val.base10_parse::<usize>().unwrap_or(0)
                    } else {
                        0
                    };
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
pub fn terser_loops(i: String) -> String {
    let input_rust_code = if i.is_empty() {
        "
    fn main() {
        for i in 0..5 {
            println!(\"Hello, world!\");
        }
        loop {
            println!(\"Forever loop!\");
        }
    }"
    } else {
        &i
    };

    let item_fn: ItemFn = syn::parse_str(input_rust_code).unwrap();
    let transformed_fn = transform_loop_with_macro(item_fn);
    let generated_code = quote! { #transformed_fn };
    let f: syn::File = syn::parse2(generated_code).unwrap();
    let pretty = prettyplease::unparse(&f);
    pretty.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_struct() {
        // no vars
        S!(B);
        let _b = B {};
        // one var
        S!(N, a, i64);
        let n = N { a: 12 };
        assert_eq!(n.a, 12);
        // two vars
        S!(M, a, usize, b, i32);
        let m = M { a: 1, b: 19 };
        assert_eq!(m.a, 1);
        assert_eq!(m.b, 19);
    }
    #[test]
    fn test_do() {
        let mut i = 0;
        DO!(i += 1, 10);
        assert_eq!(i, 10);
    }
    #[test]
    fn test_assgn() {
        l!(i, 10);
        assert_eq!(i, 10);
        lm!(j, 1);
        DO!(j += j, 10);
        assert_eq!(j, 1024);
    }

    #[test]
    fn test_map() {
        l!(i, [1, 2, 3, 4]);
        l!(j, c!(_m!(_i!(i), |i| i * 2)));
        assert_eq!(j, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_let_binding() {
        let a = Some(5);
        // let an: Option<i32> = None;
        let b: Result<i32, i32> = Ok(2);
        // let bn: Result<i32, &str> = Err("error");
        ls!(s  c, a, assert_eq!(c, 5));
        // ls!(s  cn, an);
        ls!(e  d, b, assert_eq!(d, 2));
        // ls!(e  dn, bn);
    }

    #[test]
    fn test_impl() {
        struct S {}
        _I!(S, foo);
        S::foo();
        f!(i64, foo, (a: i64), (b: i64), ex a+b);
        foo(12, 123);
        assert_eq!(foo(12, 123), 12 + 123);
    }
}
