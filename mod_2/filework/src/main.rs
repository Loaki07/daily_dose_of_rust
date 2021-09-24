extern crate filework;

use failure::Error;
use filework::*;

fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let trans = get_transactions_b("test_data/transactions.json")?;
    for t in trans {
        println!("{:?}", t);
    }

    // First trans
    let t = get_first_transaction_for("test_data/transactions.json", "Matt");
    match t {
        Ok(v) => println!("Found transaction: {:?}", v),
        Err(e) => println!("Error: {}, Backtrace: {}", e, e.backtrace()),
    }    
    Ok(())
}
