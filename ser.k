l:0:"src/lib.rs"
triml:{x^,""}
trimm:({"#[macro_export]"~x}_')
l:triml@trimm l
fill:{,/0{:[1=x-y;y;x+!y-x]}':x}
(&{*+/"{}"=\:-1#x}'l)
"new.rs" 0: l@(fill (&{*+/"{}"=\:-1#x}'l))
