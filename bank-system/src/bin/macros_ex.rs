macro_rules! create_vars {
    ($($name:ident),*) => {
        $( let $name = 0; )*   // ← повторение для каждого имени
    };
}

fn main() {
    create_vars!(a, b);
    println!("{a} {b}");   // 0 0
}

// agk@mba-agk bank-system % cargo expand --bin macros_ex
//     Checking bank-system v0.1.0 (/Users/agk/code_src/src/github.com/bank-system-1.1.2/bank-system)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s

// #![feature(prelude_import)]
// extern crate std;
// #[prelude_import]
// use std::prelude::rust_2024::*;
// fn main() {
//     let a = 0;
//     let b = 0;
//     {
//         ::std::io::_print(format_args!("{0} {1}\n", a, b));
//     };
// }