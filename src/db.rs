use rusqlite::{params, Connection, Result};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Problem {
    pub id: i64,
    pub word: String,
    pub date: DateTime<Utc>,
}

pub fn insert_problem(word: &str) -> Result<i64> {
    let conn = Connection::open("wordle.db")?;

    conn.execute(
        "CREATE TABLE if not exists problem (
                  id              INTEGER PRIMARY KEY,
                  word            TEXT NOT NULL,
                  date            TEXT
                  )",
        [],
    )?;
    conn.execute(
        "INSERT INTO problem (word, date) VALUES (?1, ?2)",
        params![word, Utc::now()],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_problem_by_id(id: i64) -> Result<Problem> {
    let conn = Connection::open("wordle.db")?;
    let mut stmt = conn.prepare("SELECT id, word, date FROM problem WHERE id = ?1")?;
    let problem = stmt.query_row(params![id], |row| {
        Ok(Problem {
            id: row.get(0)?,
            word: row.get(1)?,
            date: row.get(2)?,
        })
    })?;

    Ok(problem)
}


#[test]
fn test_db(){
    let id = insert_problem("word").unwrap();
    let p = get_problem_by_id(id).unwrap();
    assert_eq!(p.word, "word");
}