use rusqlite::Row;
use serde::{Deserialize, Serialize};
use std::result::Result;
use std::{error::Error, str::FromStr};
use strum_macros::{Display, EnumString};

use super::record::Record;

#[derive(Debug, Deserialize, Serialize, EnumString, Clone, Copy, Display, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize, Default, Serialize, EnumString, Clone, Copy, Display, PartialEq)]
pub enum DisputeStatus {
    #[default]
    None,
    Disputed,
    Resolved,
    Chargedback,
}

pub const CORRECTING_TRANSACTION_TYPES: [TransactionType; 3] = [
    TransactionType::Dispute,
    TransactionType::Resolve,
    TransactionType::Chargeback,
];

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub struct Transaction {
    #[serde(rename(deserialize = "type"))]
    pub tx_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>,
    #[serde(default)]
    pub dispute_status: DisputeStatus,
}

impl Transaction {
    pub fn tx_id_to_check(self: &Transaction) -> Option<u32> {
        if CORRECTING_TRANSACTION_TYPES.contains(&self.tx_type) {
            Some(self.tx)
        } else {
            None
        }
    }

    fn update_dispute_status(&self, disp_st: DisputeStatus) -> Self {
        Transaction {
            tx_type: self.tx_type,
            client: self.client,
            tx: self.tx,
            amount: self.amount,
            dispute_status: disp_st,
        }
    }

    fn process_deposit(&self, current_rec: &Record) -> Record {
        Record {
            client: current_rec.client,
            available: current_rec.available + self.amount.unwrap(),
            held: current_rec.held,
            total: current_rec.total + self.amount.unwrap(),
            locked: current_rec.locked,
        }
    }

    fn process_withdrawal(&self, current_rec: &Record) -> Option<Record> {
        // do we have enough funds?
        if (current_rec.available - self.amount.unwrap()) < 0.0 {
            None
        } else {
            Some(Record {
                client: current_rec.client,
                available: current_rec.available - self.amount.unwrap(),
                held: current_rec.held,
                total: current_rec.total - self.amount.unwrap(),
                locked: current_rec.locked,
            })
        }
    }

    fn process_dispute(
        &self,
        current_rec: &Record,
        disputed_txn: &Transaction,
    ) -> (Option<Record>, Option<Transaction>) {
        (
            Some(Record {
                client: current_rec.client,
                available: current_rec.available - disputed_txn.amount.unwrap(),
                held: current_rec.held + disputed_txn.amount.unwrap(),
                total: current_rec.total,
                locked: current_rec.locked,
            }),
            Some(disputed_txn.update_dispute_status(DisputeStatus::Disputed)),
        )
    }

    fn process_resolve(
        &self,
        current_rec: &Record,
        txn_to_resolve: &Transaction,
    ) -> (Option<Record>, Option<Transaction>) {
        let new_rec = Record {
            client: current_rec.client,
            available: current_rec.available + txn_to_resolve.amount.unwrap(),
            held: current_rec.held - txn_to_resolve.amount.unwrap(),
            total: current_rec.total,
            locked: current_rec.locked,
        };
        let updated_txn = txn_to_resolve.update_dispute_status(DisputeStatus::Resolved);
        (Some(new_rec), Some(updated_txn))
    }

    fn process_chargeback(
        &self,
        current_rec: &Record,
        chargeback: &Transaction,
    ) -> (Option<Record>, Option<Transaction>) {
        (
            Some(Record {
                client: current_rec.client,
                available: current_rec.available,
                held: current_rec.held - chargeback.amount.unwrap(),
                total: current_rec.total - chargeback.amount.unwrap(),
                locked: Some(1),
            }),
            Some(chargeback.update_dispute_status(DisputeStatus::Chargedback)),
        )
    }

    fn is_valid_transaction(self: &Transaction, txn_to_check: Option<Transaction>) -> bool {
        match self.tx_type {
            // only check that the amount field is present
            TransactionType::Deposit => self.amount.is_some(),
            TransactionType::Withdrawal => self.amount.is_some(),

            // more checks
            TransactionType::Dispute => self.is_valid_correcting_txn(txn_to_check),
            TransactionType::Resolve => {
                self.is_valid_correcting_txn(txn_to_check)
                    && txn_to_check.unwrap().dispute_status == DisputeStatus::Disputed
            }
            TransactionType::Chargeback => {
                self.is_valid_correcting_txn(txn_to_check)
                    && txn_to_check.unwrap().dispute_status == DisputeStatus::Disputed
            }
        }
    }

    #[inline]
    fn is_valid_correcting_txn(self: &Transaction, txn_to_check: Option<Transaction>) -> bool {
        txn_to_check.is_some()
            && txn_to_check.unwrap().client == self.client
            && txn_to_check.unwrap().amount.is_some()
    }

    pub fn process(
        self: &Transaction,
        current_rec: &Record,
        transaction_to_check: Option<Transaction>,
    ) -> Result<(Option<Record>, Option<Transaction>), Box<dyn Error>> {
        if !self.is_valid_transaction(transaction_to_check) {
            return Ok((None, None)); // means this is an error and we ignore it
        }

        match self.tx_type {
            TransactionType::Deposit => Ok((Some(self.process_deposit(current_rec)), None)),
            TransactionType::Withdrawal => Ok((self.process_withdrawal(current_rec), None)),
            TransactionType::Dispute => {
                Ok(self.process_dispute(current_rec, &transaction_to_check.unwrap()))
            }
            TransactionType::Resolve => {
                Ok(self.process_resolve(current_rec, &transaction_to_check.unwrap()))
            }
            TransactionType::Chargeback => {
                Ok(self.process_chargeback(current_rec, &transaction_to_check.unwrap()))
            }
        }
    }
}

impl From<&Row<'_>> for Transaction {
    fn from(row: &Row) -> Self {
        let txn_type_str: String = row.get(2).unwrap();
        let disp_st_str: String = row.get(3).unwrap();
        Transaction {
            tx: row.get(0).unwrap(),
            client: row.get(1).unwrap(),
            tx_type: TransactionType::from_str(txn_type_str.as_str()).unwrap(),
            dispute_status: DisputeStatus::from_str(disp_st_str.as_str()).unwrap(),
            amount: row.get(4).unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DisputeStatus, Transaction, TransactionType};
    use crate::processor::record::Record;

    #[test]
    fn test_tx_id_to_check() {
        assert_eq!(
            None,
            get_test_transaction(TransactionType::Deposit).tx_id_to_check()
        );
        assert_eq!(
            None,
            get_test_transaction(TransactionType::Withdrawal).tx_id_to_check()
        );
        assert_eq!(
            Some(1),
            get_test_correction(TransactionType::Dispute).tx_id_to_check()
        );
        assert_eq!(
            Some(1),
            get_test_correction(TransactionType::Resolve).tx_id_to_check()
        );
        assert_eq!(
            Some(1),
            get_test_correction(TransactionType::Chargeback).tx_id_to_check()
        );
    }

    #[test]
    fn test_process_deposit() {
        let test_deposit = get_test_transaction(TransactionType::Deposit);
        let current_rec = make_unlocked_record(0.0, 0.0, 0.0);
        let expected_result = make_unlocked_record(0.0001, 0.0, 0.0001);
        assert_eq!(expected_result, test_deposit.process_deposit(&current_rec));
    }

    #[test]
    fn test_process_withdrawal_success() {
        let test_deposit = get_test_transaction(TransactionType::Withdrawal);
        let current_rec = make_unlocked_record(0.0001, 0.0, 0.0001);
        let expected_result = make_unlocked_record(0.0, 0.0, 0.0);
        assert_eq!(
            Some(expected_result),
            test_deposit.process_withdrawal(&current_rec)
        );
    }

    #[test]
    fn test_process_withdrawal_fail() {
        let test_deposit = get_test_transaction(TransactionType::Withdrawal);
        let current_rec = make_unlocked_record(0.0, 0.0, 0.0);
        assert_eq!(None, test_deposit.process_withdrawal(&current_rec));
    }

    #[test]
    fn test_process_dispute() {
        let test_dipute = make_undisputed_txn(TransactionType::Dispute, 1, 1, None);
        let disputed_txn = make_undisputed_txn(TransactionType::Withdrawal, 1, 1, Some(20.00));

        let current_rec = make_unlocked_record(100.0, 20.50, 120.50);
        let expected_result = make_unlocked_record(80.0, 40.50, 120.50);

        let (result, txn_to_update) = test_dipute.process_dispute(&current_rec, &disputed_txn);
        assert_eq!(Some(expected_result), result);
        assert_eq!(
            Some(disputed_txn.update_dispute_status(DisputeStatus::Disputed)),
            txn_to_update
        );
    }

    #[test]
    fn test_process_resolve() {
        let test_resolve = make_undisputed_txn(TransactionType::Resolve, 1, 1, None);
        let txn_to_resolve = make_undisputed_txn(TransactionType::Withdrawal, 1, 1, Some(20.00));

        let current_rec = make_unlocked_record(80.0, 40.50, 120.50);
        let expected_result = make_unlocked_record(100.0, 20.50, 120.50);

        let (result, txn_to_update) = test_resolve.process_resolve(&current_rec, &txn_to_resolve);
        assert_eq!(Some(expected_result), result);
        assert_eq!(
            Some(txn_to_resolve.update_dispute_status(DisputeStatus::Resolved)),
            txn_to_update
        );
    }

    #[test]
    fn test_process_chargeback() {
        let test_chargeback = make_undisputed_txn(TransactionType::Chargeback, 1, 1, None);
        let chargeback = make_undisputed_txn(TransactionType::Withdrawal, 1, 1, Some(20.00));

        let current_rec = make_unlocked_record(80.0, 40.50, 120.50);
        let expected_result = Record {
            client: 1,
            available: 80.0,
            held: 20.50,
            total: 100.50,
            locked: Some(1),
        };

        let (result, txn_to_update) = test_chargeback.process_chargeback(&current_rec, &chargeback);
        assert_eq!(Some(expected_result), result);
        assert_eq!(
            Some(chargeback.update_dispute_status(DisputeStatus::Chargedback)),
            txn_to_update
        );
    }

    #[test]
    fn test_is_invalid_corrective_transaction() {
        let test_dispute = get_test_correction(TransactionType::Dispute);
        let test_resolve = get_test_correction(TransactionType::Resolve);
        let test_chargeback = get_test_correction(TransactionType::Chargeback);
        let corrective_transactions = [test_dispute, test_resolve, test_chargeback];

        // the transaction to correct is missing
        corrective_transactions.map(|tx| {
            assert!(!tx.is_valid_transaction(None));
        });

        // the transaction to correct doesn't have correct client_id
        let tx_to_correct = make_undisputed_txn(TransactionType::Withdrawal, 2, 1, None);
        corrective_transactions.map(|tx| {
            assert!(!tx.is_valid_transaction(Some(tx_to_correct)));
        });

        // the transaction to correct misses the amount
        let tx_to_correct = make_undisputed_txn(TransactionType::Withdrawal, 1, 1, None);
        corrective_transactions.map(|tx| {
            assert!(!tx.is_valid_transaction(Some(tx_to_correct)));
        });
    }

    #[test]
    fn test_is_valid_correcting_transaction() {
        let test_dispute = get_test_correction(TransactionType::Dispute);
        let test_resolve = get_test_correction(TransactionType::Resolve);
        let test_chargeback = get_test_correction(TransactionType::Chargeback);
        let correcting_transactions = [test_dispute, test_resolve, test_chargeback];

        // the transaction to correct looks fine
        let tx_to_correct = get_test_transaction(TransactionType::Deposit);
        correcting_transactions.map(|tx| {
            assert!(tx.is_valid_correcting_txn(Some(tx_to_correct)));
        });
    }

    #[test]
    fn test_is_valid_resolve_or_chargeback() {
        let test_resolve = get_test_correction(TransactionType::Resolve);
        let test_chargeback = get_test_correction(TransactionType::Chargeback);
        let test_deposit = Transaction {
            client: 1,
            tx_type: TransactionType::Deposit,
            tx: 1,
            amount: Some(2.134),
            dispute_status: DisputeStatus::Disputed,
        };

        assert!(test_resolve.is_valid_transaction(Some(test_deposit)));
        assert!(test_chargeback.is_valid_transaction(Some(test_deposit)));
    }

    #[test]
    fn test_is_invalid_resolve_or_chargeback() {
        let test_resolve = get_test_correction(TransactionType::Resolve);
        let test_chargeback = get_test_correction(TransactionType::Chargeback);
        let test_deposit = Transaction {
            client: 1,
            tx_type: TransactionType::Deposit,
            tx: 1,
            amount: Some(2.134),
            dispute_status: DisputeStatus::None,
        };

        assert!(!test_resolve.is_valid_transaction(Some(test_deposit)));
        assert!(!test_chargeback.is_valid_transaction(Some(test_deposit)));
    }

    #[test]
    fn test_is_invalid_regular_transaction() {
        // the transaction misses the amount
        let test_deposit = make_undisputed_txn(TransactionType::Deposit, 1, 1, None);
        let test_withdrawal = make_undisputed_txn(TransactionType::Withdrawal, 1, 1, None);

        assert!(!test_deposit.is_valid_transaction(None));
        assert!(!test_withdrawal.is_valid_transaction(None));
    }

    #[test]
    fn test_is_valid_regular_transaction() {
        // the transaction has the amount
        let test_deposit = make_undisputed_txn(TransactionType::Deposit, 1, 1, Some(1.0101));
        let test_withdrawal = make_undisputed_txn(TransactionType::Withdrawal, 1, 1, Some(1.0101));

        assert!(test_deposit.is_valid_transaction(None));
        assert!(test_withdrawal.is_valid_transaction(None));
    }

    fn make_undisputed_txn(
        tx_type: TransactionType,
        client: u16,
        tx: u32,
        amount: Option<f64>,
    ) -> Transaction {
        Transaction {
            tx_type,
            client,
            tx,
            amount,
            dispute_status: DisputeStatus::None,
        }
    }
    fn make_unlocked_record(available: f64, held: f64, total: f64) -> Record {
        Record {
            client: 1,
            available,
            held,
            total,
            locked: None,
        }
    }
    fn get_test_transaction(tx_type: TransactionType) -> Transaction {
        make_undisputed_txn(tx_type, 1, 1, Some(0.0001))
    }
    fn get_test_correction(tx_type: TransactionType) -> Transaction {
        make_undisputed_txn(tx_type, 1, 1, None)
    }
}
