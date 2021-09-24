extern crate filework;

use filework::*;

fn main() -> Result<(), TransactionError> {
    println!("Hello, world!");
    let trans =
        get_transactions_b("test_data/transactions.json")?;
    for t in trans {
        println!("{:?}", t);
    }

    // First trans
    let t = get_first_transaction_for("test_data/transactions.json", "Matt").ok_or("could not get first transaction")?;
    println!("Matt's first transaction: {:?}", t);
    Ok(())
}
