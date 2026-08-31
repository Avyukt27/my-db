use crate::{db::Database, db_error::DbError};

pub fn parse_command(db: &mut Database, command: &str) -> Result<String, DbError> {
    let mut parts = command.split_whitespace();
    let cmd = parts
        .next()
        .ok_or_else(|| DbError::ParseError(command.to_owned()))?;

    match cmd.to_uppercase().as_str() {
        "SET" => {
            let key = sanitize_input(
                parts
                    .next()
                    .ok_or_else(|| DbError::ParseError("Missing key for SET".to_owned()))?,
            );
            let value = sanitize_input(
                parts
                    .next()
                    .ok_or_else(|| DbError::ParseError("Missing value for SET".to_owned()))?,
            );
            db.set(key, value)?;
            Ok("Ok".to_owned())
        }
        "GET" => {
            let key = sanitize_input(
                parts
                    .next()
                    .ok_or_else(|| DbError::ParseError("Missing key for GET".to_owned()))?,
            );
            let value = db.get(key)?;
            Ok(format!("{}", value.to_str()))
        }
        "DEL" => {
            let key = sanitize_input(
                parts
                    .next()
                    .ok_or_else(|| DbError::ParseError("Missing key for DEL".to_owned()))?,
            );
            let value = db
                .remove(&key)?
                .ok_or_else(|| DbError::KeyNotFound(key.to_owned()))?;
            Ok(format!("{}", value.to_str()))
        }
        "COMPACT" => {
            db.compact()?;
            Ok("Ok".to_owned())
        }
        _ => Err(DbError::ParseError(command.to_owned())),
    }
}

fn sanitize_input(input: &str) -> String {
    input.chars().filter(|c| !c.is_control()).collect()
}
