pub mod storage;
pub mod transaction;

pub use storage::Storage;
// pub use storage::{Storage, Account, Balance, Name};
pub use transaction::{Transaction, Deposit, Transfer, TxError};

pub type Name = String;
pub type Balance = i64;