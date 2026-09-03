use std::{env, path::PathBuf};

use my_db::{db::Database, parser::parse_command};

pub struct TestDb {
    pub db: Database,
    path: PathBuf,
}

impl TestDb {
    pub fn new(test_name: &str) -> Self {
        let mut path = env::temp_dir();
        path.push(format!(
            "{}_{}_{}.db",
            test_name,
            std::process::id(),
            rand::random::<u16>()
        ));
        let _ = std::fs::remove_file(&path);

        let mut db = Database::new(&path).unwrap();
        db.set("username", "Alice").unwrap();
        db.set("password", "password123").unwrap();
        db.set("score", 10).unwrap();
        db.set("ratio", 3.5).unwrap();

        Self { db, path }
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

#[test]
fn test_db_insert_and_read() {
    let db = TestDb::new("insert-and-read");

    assert_eq!(db.db.get("username").unwrap().to_str(), "Alice");
    assert_eq!(db.db.get("score").unwrap().to_str(), "10");
}

#[test]
fn test_database_persistence() {
    {
        let mut db = TestDb::new("persistence");
        db.db.set("persisted_key", "hello world").unwrap();
    }

    let db_reloaded = TestDb::new("persistence");

    assert_eq!(
        db_reloaded.db.get("persisted_key").unwrap().to_str(),
        "hello world"
    );
}

#[test]
fn test_database_delete() {
    let mut db = TestDb::new("delete");
    assert_eq!(db.db.get("username").unwrap().to_str(), "Alice");
    db.db.remove("username").unwrap();
    assert!(db.db.get("username").is_err());
    assert_eq!(db.db.get("password").unwrap().to_str(), "password123");
}

#[test]
fn test_database_cmd_get() {
    let mut db = TestDb::new("cmd-get");
    assert!(parse_command(&mut db.db, "GET username").is_ok_and(|val| val == "Alice"));
    assert!(parse_command(&mut db.db, "GET user_1").is_err());
}

#[test]
fn test_database_cmd_set() {
    let mut db = TestDb::new("cmd-set");
    assert!(parse_command(&mut db.db, "SET username Arthur").is_ok_and(|val| val == "Ok"));
    assert!(parse_command(&mut db.db, "SET").is_err());
}

#[test]
fn test_database_cmd_del() {
    let mut db = TestDb::new("cmd-del");
    let _ = parse_command(&mut db.db, "DEL username");
    assert!(parse_command(&mut db.db, "GET username").is_err());
}
