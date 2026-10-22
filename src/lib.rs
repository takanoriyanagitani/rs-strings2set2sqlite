use std::io;
use std::path::Path;

use io::BufRead;

use rusqlite::Connection;

use rusqlite::Statement;
use rusqlite::Transaction;

pub fn str2sqlite(s: &str, stmt: &mut Statement) -> Result<usize, io::Error> {
    stmt.execute([s]).map_err(io::Error::other)
}

pub fn brdr2strings2sqlite<R>(mut brdr: R, stmt: &mut Statement) -> Result<usize, io::Error>
where
    R: BufRead,
{
    let mut tot: usize = 0;
    let mut buf: String = String::default();
    loop {
        buf.clear();

        let sz: usize = brdr.read_line(&mut buf)?;
        if 0 == sz {
            return Ok(tot);
        }

        let nolf: &str = buf.trim_end();

        let neo: usize = str2sqlite(nolf, stmt)?;
        tot += neo;
    }
}

pub fn brdr2strings2sqlite_path<R, P>(
    brdr: R,
    qcreate: &str,
    qinsert: &str,
    path2sqlite: P,
) -> Result<usize, io::Error>
where
    R: BufRead,
    P: AsRef<Path>,
{
    if path2sqlite.as_ref().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "empty path to the sqlite db file got",
        ));
    }
    let mut con: Connection = Connection::open(path2sqlite).map_err(io::Error::other)?;
    con.execute(qcreate, ()).map_err(io::Error::other)?;

    let tx: Transaction = con.transaction().map_err(io::Error::other)?;
    let mut stmt: Statement = tx.prepare(qinsert).map_err(io::Error::other)?;
    let tot: usize = brdr2strings2sqlite(brdr, &mut stmt)?;
    stmt.finalize().map_err(io::Error::other)?;
    tx.commit().map_err(io::Error::other)?;
    con.close().map_err(|t| t.1).map_err(io::Error::other)?;
    Ok(tot)
}

pub fn stdin2strings2sqlite_path<P>(
    qcreate: &str,
    qinsert: &str,
    path2sqlite: P,
) -> Result<usize, io::Error>
where
    P: AsRef<Path>,
{
    brdr2strings2sqlite_path(io::stdin().lock(), qcreate, qinsert, path2sqlite)
}

pub const CREATE_QUERY_DEFAULT: &str = r#"
    CREATE TABLE IF NOT EXISTS strings(
        key TEXT PRIMARY KEY
    )
    WITHOUT ROWID
    ;
"#;

pub const INSERT_QUERY_DEFAULT: &str = r#"
    INSERT INTO strings
    VALUES (?1)
    ON CONFLICT(key)
    DO NOTHING
"#;

pub fn stdin2strings2sqlite_path_default<P>(path2sqlite: P) -> Result<usize, io::Error>
where
    P: AsRef<Path>,
{
    stdin2strings2sqlite_path(CREATE_QUERY_DEFAULT, INSERT_QUERY_DEFAULT, path2sqlite)
}
