# Franca
Macros for writing terser Rust code. This crate contains both a library that exports the macros and a binary that transforms any Rust code to Rust code that uses the exported macros.

Access the online demo hosted on github pages [here](https://x86y.github.io/franca).

## Example code written using Franca macros:
```rs
use franca::*;
pub struct Program {
    pub p: usize,
    pub a: Vec<i64>,
    pub ip: usize,
    pub b: Vec<i64>,
}
impl Program {
    pub fn eval(&mut self, s: Sstr) {
        lm!(st, vec![]; b, 0; a, &mut self.a; p, &mut self.p; ip, &mut self.ip);
        l!(by, s.as_bytes());
        W!(*ip < by.len(), {
            l!(c, _c!(by[*ip]));
            M!(c; ']', {Ic!(nz!(a[*p]), *ip = st.pop().unwrap()); b = *ip};
                  '[', {Ic!(z!(a[*p]), *ip = b); st.push(*ip)};
                  '+', ic!(a[*p]);
                  '-', dc!(a[*p]);
                  '>', ic!(*p);
                  '<', dc!(*p);
                  '.', self.b.push(a[*p]);
                  ',', {};
                  _, unreachable!()
            );
            ic!(*ip);
        })
    }
}

```

