use rocksdb::{DB, Options};
use std::path::Path;
use std::io::Cursor;

use std::io::Cursor;
use eetf::{Term, FixInteger, Integer, BigInteger};
use num_bigint::BigInt; // add = "0.4" to Cargo.toml if you want BigInt support
use num_traits::cast::ToPrimitive;

fn decode_height(bytes: &[u8]) -> Option<i64> {
    let term = Term::decode(Cursor::new(bytes)).ok()?;
    match term {
        Term::FixInteger(FixInteger { value }) => Some(value as i64),
        Term::BigInteger(BigInteger { value }) => {
            // Try to downcast BigInt into i64 safely
            value.to_i64()
        }
        _ => None,
    }
}

fn main() {
    // Path to your fabric DB
    let db_path = "/path/to/workdir/db/fabric";

    // Open DB with needed column families
    let cf_names = vec!["entry_by_height|height:entryhash", "sysconf"];

    let opts = Options::default();
    let db = DB::open_cf_for_read_only(&opts, db_path, &cf_names, false)
        .expect("Failed to open RocksDB");

    // Get handle for sysconf column family
    let sysconf_cf = db.cf_handle("sysconf").expect("sysconf CF not found");

    // Try to read "temporal_height"
    if let Ok(Some(value)) = db.get_cf(&sysconf_cf, "temporal_height") {
        // Values in sysconf are term-encoded binaries in Elixir.
        // For simplicity, we just print raw bytes.
        println!("Raw temporal_height bytes: {:?}", value);

        // If you stored as integer term, you may need external decoding.
        // For quick debugging, check if it's directly an integer:
        let term = Term::decode(Cursor::new(&bytes)).expect("decode failed");

    match term {
        Term::FixInteger(FixInteger { value }) => {
            println!("Height = {}", value);
        }
        Term::Integer(Integer { value }) => {
            println!("Height = {}", value);
        }
        other => {
            println!("Unexpected term: {:?}", other);
        }
    }
        //if value.len() == 8 {
        //    let height = i64::from_be_bytes(value.try_into().unwrap());
        //    println!("Current chain height (decoded): {}", height);
        //}
    } else {
        println!("No temporal_height key found, maybe query rooted_tip instead");
    }
}
