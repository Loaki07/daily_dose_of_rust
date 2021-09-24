mod error;

pub use error::TransactionError;
use serde_derive::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

/// Error Handling using traditional match combinators
pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, String> {
    // Reading a file from a directory
    let s = match std::fs::read_to_string(fname) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };

    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    Ok(t)
}

/// Error handling using "map_err" and "and_then"
/// "and_then" executes only if there is a positive result
pub fn get_transactions_b(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    // std::fs::read_to_string(fname)
    //     .map_err(|e| e.into())
    //     .and_then(|res| serde_json::from_str(&res).map_err(|e| e.into()))

    // Much optimized way of handling error
    // the "?" will only work if you have implamented the From trait
    // for all the types of errors
    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
}

pub fn get_first_transaction_for(fname: &str, uname: &str) -> Result<Transaction, failure::Error> {
    let trans = get_transactions_b(fname)?;
    for t in trans {
        if t.from == uname {
           return Ok(t);
        }
    }
    Err(TransactionError::Message("Could not find transaction with that name").into())
}