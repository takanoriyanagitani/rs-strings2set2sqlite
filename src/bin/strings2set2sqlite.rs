use std::io;
use std::process::ExitCode;

fn io_envkey2val(key: &'static str) -> impl Fn() -> String {
    move || std::env::var(key).unwrap_or_default()
}

fn io_path2sqlite() -> impl Fn() -> String {
    io_envkey2val("ENV_SQLITE_DB_FILE_PATH")
}

fn io_stdin2strings2sqlite2changed() -> impl FnMut() -> Result<usize, io::Error> {
    || {
        let path2sqlite: String = io_path2sqlite()();
        rs_strings2set2sqlite::stdin2strings2sqlite_path_default(path2sqlite)
    }
}

fn io_changed2stdout() -> impl FnMut(usize) -> Result<(), io::Error> {
    move |changed: usize| {
        println!("changed: {changed}");
        Ok(())
    }
}

fn io_main() -> impl FnOnce() -> Result<(), io::Error> {
    || {
        let changed: usize = io_stdin2strings2sqlite2changed()()?;
        io_changed2stdout()(changed)?;
        Ok(())
    }
}

fn sub() -> Result<(), io::Error> {
    io_main()()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
