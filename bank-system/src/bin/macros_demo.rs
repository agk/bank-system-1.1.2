// bin/macros_demo.rs

use bank_system::tx_chain;
use bank_system::create_users;
use bank_system::Storage;
use bank_system::{Deposit, Transaction, Transfer, Withdraw};
use my_macros::say_hello;

fn main() {
    let mut storage = Storage::new();
    // storage.add_user("Alice".into());
    // storage.add_user("Bob".into());
    create_users!(storage, "Alice", "Bob", "John"); 

    let tx = tx_chain!(
        Deposit { account: "Alice".into(), amount: 500 },
        Transfer { from: "Alice".into(), to: "Bob".into(), amount: 50 },
        Withdraw { account: "Alice".into(), amount: 100 }
    );

    match tx.apply(&mut storage) {
        Ok(_) => println!("Успешно"),
        Err(e) => println!("Ошибка: {:?}", e),
    }

    for (name, balance) in storage.get_all() {
        println!("{} -> {}", name, balance);
    }

    say_hello!("Привет из процедурного макроса!");
    // разворачивается в: println!("{}", "Привет из процедурного макроса!");

}

// cargo expand --bin macros_demo
//     Checking bank-system v0.1.0 (/Users/agk/code_src/src/github.com/bank-system-1.1.2/bank-system)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s

// #![feature(prelude_import)]
// extern crate std;
// #[prelude_import]
// use std::prelude::rust_2024::*;
// use bank_system::tx_chain;
// use bank_system::Storage;
// use bank_system::{Deposit, Transaction, Transfer, Withdraw};
// fn main() {
//     let mut storage = Storage::new();
//     storage.add_user("Alice".into());
//     storage.add_user("Bob".into());
//     let tx = {
//         let tx = Deposit {
//             account: "Alice".into(),
//             amount: 500,
//         };
//         let tx = ::bank_system::TxCombinator {
//             t1: tx,
//             t2: Transfer {
//                 from: "Alice".into(),
//                 to: "Bob".into(),
//                 amount: 50,
//             },
//         };
//         let tx = ::bank_system::TxCombinator {
//             t1: tx,
//             t2: Withdraw {
//                 account: "Alice".into(),
//                 amount: 100,
//             },
//         };
//         tx
//     };
//     match tx.apply(&mut storage) {
//         Ok(_) => {
//             ::std::io::_print(format_args!("Успешно\n"));
//         }
//         Err(e) => {
//             ::std::io::_print(format_args!("Ошибка: {0:?}\n", e));
//         }
//     }
//     for (name, balance) in storage.get_all() {
//         {
//             ::std::io::_print(format_args!("{0} -> {1}\n", name, balance));
//         };
//     }
// }