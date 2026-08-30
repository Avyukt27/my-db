use std::env;

use my_db::db::Database;

#[test]
fn test_db_insert_and_read() {
    let mut path = env::temp_dir();
    path.push("test_insert.db");
    let _ = std::fs::remove_file(&path);

    let mut db = Database::new(&path).unwrap();

    db.set("username", "Alice").unwrap();
    db.set("password", "password123").unwrap();
    db.set("score", 10).unwrap();
    db.set("ratio", 3.5).unwrap();

    assert_eq!(db.get("username").unwrap().to_str(), "Alice");
    assert_eq!(db.get("score").unwrap().to_str(), "10");

    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_database_persistence() {
    let mut path = env::temp_dir();
    path.push("test_persistence.db");
    let _ = std::fs::remove_file(&path);

    {
        let mut db = Database::new(&path).unwrap();
        db.set("persisted_key", "hello world").unwrap();
    }

    let db_reloaded = Database::new(&path).unwrap();

    assert_eq!(
        db_reloaded.get("persisted_key").unwrap().to_str(),
        "hello world"
    );

    let _ = std::fs::remove_file(&path);
}

#[test]
fn test_database_delete() {
    let mut path = env::temp_dir();
    path.push("test_insert.db");
    let _ = std::fs::remove_file(&path);

    let mut db = Database::new(&path).unwrap();

    db.set("username", "Alice").unwrap();
    db.set("password", "password123").unwrap();
    db.set("score", 10).unwrap();
    db.set("ratio", 3.5).unwrap();

    assert_eq!(db.get("username").unwrap().to_str(), "Alice");

    db.remove("username").unwrap();

    assert!(db.get("username").is_err());
    assert_eq!(db.get("password").unwrap().to_str(), "password123");
}
