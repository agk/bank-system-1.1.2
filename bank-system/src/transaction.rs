use crate::storage::Storage;

#[derive(Debug)]
pub enum TxError {
    InsufficientFunds,
    InvalidAccount,
}

pub trait Transaction {
    fn apply(&self, accounts: &mut Storage) -> Result<(), TxError>;
}

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