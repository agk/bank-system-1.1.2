use crate::storage::Storage;

#[derive(Debug)]
pub enum TxError {
    InsufficientFunds,
    InvalidAccount,
}

pub trait Transaction {
    fn apply(&self, accounts: &mut Storage) -> Result<(), TxError>;
}

trait Describable {
    fn describe(&self) -> String;
}

// Deposit — тип по умолчанию, атрибут не нужен
#[derive(Transaction)]
pub struct Deposit {
    pub account: String,
    pub amount: i64,
}

// impl Transaction for Deposit {
//     fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
//         *storage.accounts.entry(self.account.clone()).or_insert(0) += self.amount;
//         Ok(())
//     }
// }

impl Transaction for Deposit {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        match storage.accounts.get_mut(&self.account) {
            Some(acc) => {
                acc.balance += self.amount;   // ← меняем ПОЛЕ balance
                Ok(())
            }
            None => Err(TxError::InvalidAccount),
        }
    }
}

#[derive(Transaction)]
#[transaction("transfer")]
pub struct Transfer {
    pub from: String,
    pub to: String,
    pub amount: i64,
} 

// impl Transaction for Transfer {
//     fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
//         let from_balance = storage.accounts.entry(self.from.clone()).or_insert(0);
//         if *from_balance < self.amount {
//             return Err(TxError::InsufficientFunds);
//         }
//         *from_balance -= self.amount;
//         *storage.accounts.entry(self.to.clone()).or_insert(0) += self.amount;
//         Ok(())
//     }
// }
impl Transaction for Transfer {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        let from_balance = storage
            .accounts
            .get(&self.from)
            .ok_or(TxError::InvalidAccount)?
            .balance;

        if from_balance < self.amount {
            return Err(TxError::InsufficientFunds);
        }

        if !storage.accounts.contains_key(&self.to) {
            return Err(TxError::InvalidAccount);
        }

        storage.accounts.get_mut(&self.from).unwrap().balance -= self.amount;
        storage.accounts.get_mut(&self.to).unwrap().balance += self.amount;
        Ok(())
    }
}

#[derive(Transaction)]
#[transaction("withdraw")]
pub struct Withdraw {
    pub account: String,
    pub amount: i64,
}

impl Transaction for Withdraw {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        let balance = storage
            .accounts
            .get(&self.account)
            .ok_or(TxError::InvalidAccount)?
            .balance;

        if balance < self.amount {
            return Err(TxError::InsufficientFunds);
        }
        storage.accounts.get_mut(&self.account).unwrap().balance -= self.amount;
        Ok(())
    }

}

impl<A: Transaction, B: Transaction> Transaction for TxCombinator<A, B> {
    fn apply(&self, storage: &mut Storage) -> Result<(), TxError> {
        self.t1.apply(storage)?;   // сначала левая транзакция
        self.t2.apply(storage)     // потом правая
    }
}

#[derive(Debug)]
pub struct TxCombinator<A, B> {
    pub t1: A,
    pub t2: B,
}

impl<A, B> TxCombinator<A, B> {
    pub fn new(t1: A, t2: B) -> Self {
        TxCombinator { t1, t2 }
    }
}

#[macro_export]
macro_rules! impl_add {
    ( $( ($lhs:ty, $rhs:ty) ),* $(,)? ) => {
        $(
            impl std::ops::Add<$rhs> for $lhs {
                type Output = $crate::TxCombinator<$lhs, $rhs>;

                fn add(self, rhs: $rhs) -> Self::Output {
                    $crate::TxCombinator { t1: self, t2: rhs }
                }
            }
        )*
    };
}
// transaction.rs
// use crate::impl_add;

impl_add! {
    (Deposit, Transfer),
    (Transfer, Deposit),
    (Deposit, Deposit),
    (Transfer, Transfer),
}

#[macro_export]
macro_rules! tx_chain {
    ( $first:expr $(, $rest:expr )* $(,)? ) => {{
        let tx = $first;
        $(
            let tx = $crate::TxCombinator { t1: tx, t2: $rest };
        )*
        tx
    }};
}

#[macro_export]
macro_rules! create_users {
    ($storage:expr, $( $name:expr ),+ $(,)?) => {
        $(
            $storage.add_user($name.to_string());
        )+
    };
}