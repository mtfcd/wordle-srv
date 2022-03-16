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

pub fn insert_guess(problem_id: i64, guess: &str, line: usize) -> Result<i64> {
    let conn = Connection::open("wordle.db")?;

    conn.execute(
        "CREATE TABLE if not exists guesses (
                  id              INTEGER PRIMARY KEY,
                  problem_id      INTEGER,
                  line            INTEGER,
                  guess           TEXT NOT NULL,
                  date            TEXT,
                  UNIQUE          (problem_id, line)
            )",
        [],
    )?;
    conn.execute(
        "INSERT INTO guesses (problem_id, guess, line, date) 
        VALUES (?1, ?2, ?3, ?4)
        ON CONFLICT(problem_id, line)
        DO UPDATE SET guess=?2",
        params![problem_id, guess, line, Utc::now()],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_guesses(problem_id: i64) -> Result<Vec<String>> {
    let conn = Connection::open("wordle.db")?;
    let mut stmt = conn.prepare(
        "SELECT guess FROM guesses WHERE problem_id = ?1
            ORDER BY line"
    )?;
    let rows = stmt.query_map(params![problem_id], |row| {
        row.get(0)
    })?;

    let mut guesses = Vec::new();
    for name_result in rows {
        guesses.push(name_result?);
    }

    Ok(guesses)
}

pub fn find_word(word: &str) -> Result<String> {
    dbg!(word);
    let conn = Connection::open("wordle.db")?;
    let mut stmt = conn.prepare(
        "SELECT word FROM words WHERE word = ?1"
    )?;
    let row = stmt.query_row(params![word], |row| {
        row.get(0)
    })?;

    Ok(row)
}

#[test]
fn test_db(){
    let id = insert_problem("word").unwrap();
    let p = get_problem_by_id(id).unwrap();
    assert_eq!(p.word, "word");
}


#[test]
fn test_find_word(){
    let p = find_word("word");
    assert_eq!(p.unwrap(), "word");
}