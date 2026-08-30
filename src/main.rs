use std::io;

use crate::db::DataBase;

mod db;
mod db_types;

fn main() -> io::Result<()> {
    let mut db = DataBase::new("db.db")?;
    if let Some(value) = db.get("user_1") {
        println!("{:?}", value);
    } else {
        println!("Value not found");
    }
    db.set("user_1", "DEF")?;
    if let Some(value) = db.get("user_1") {
        println!("{:?}", value);
    } else {
        println!("Value not found");
    }
    db.set("user_1", 3)?;
    if let Some(value) = db.get("user_1") {
        println!("{:?}", value);
    } else {
        println!("Value not found");
    }

    Ok(())
}
