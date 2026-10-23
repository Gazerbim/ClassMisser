use polars::{
    df,
    error::PolarsResult,
    io::{
        SerReader, SerWriter,
        csv::{read::CsvReader, write::CsvWriter},
    },
    prelude::CsvReadOptions,
    series::Series,
};
use std::fs;

pub struct AccountManager {}

impl AccountManager {
    pub fn new() -> AccountManager {
        AccountManager {}
    }

    pub fn login(&self, username: String, password: String) -> bool {
        let polar = CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some("src/data/accounts.csv".into()))
            .unwrap()
            .finish()
            .unwrap();

        let users = polar.column("username").unwrap();
        let index = self.get_user_index(users, &username);
        if index >= 0 {
            let pass = polar
                .column("password")
                .unwrap()
                .get(index as usize)
                .unwrap()
                .to_string();
            let pass = pass.clone()[1..pass.len() - 1].to_string();
            if password == pass {
                return true;
            }
        }
        false
    }

    pub fn create_account(&self, username: String, password: String) -> bool {
        let polar = CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some("src/data/accounts.csv".into()))
            .unwrap()
            .finish()
            .unwrap();
        let users = polar.column("username").unwrap();
        let exists = self.get_user_index(users, &username);
        if exists == -1 {
            self.append_row(username, password).unwrap();
            return true;
        }
        false
    }

    fn append_row(&self, username: String, password: String) -> PolarsResult<()> {
        let file_path = "src/data/accounts.csv".to_string();
        // Read existing file, or start with an empty frame
        let file = fs::File::open(&file_path)?;
        let df = if fs::metadata(&file_path).is_ok() {
            CsvReader::new(file).finish()?
        } else {
            df![
                "username" => [""],
                "password" => [""],
                "points" => [0i64],
            ]?
            .clear() // drop the placeholder row
        };

        // Build the new row
        let new_row = df![
            "username" => [username],
            "password" => [password],
            "points" => [0i64],
        ]?;
        let mut df = df.vstack(&new_row)?;

        // Append and write back
        let mut f = fs::File::create(&file_path)?;
        CsvWriter::new(&mut f)
            .include_header(true)
            .finish(&mut df)?;

        Ok(())
    }

    fn get_user_index(&self, users: &Series, username: &String) -> i32 {
        for i in 0..users.len() {
            let specific_user = users.get(i).unwrap().to_string();
            let specific_user = specific_user.clone()[1..specific_user.len() - 1].to_string();
            if *username == specific_user {
                return i as i32;
            }
        }
        -1
    }
}
