// src/importer/mod.rs


use diesel::prelude::*;
use std::fmt;

impl fmt::Display for ImportStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stored: {}, Duplicates: {}, Partial: {}, Skipped: {}, Errors: {}, Time taken: {:.2}s",
               self.stored, self.duplicates, self.partial, self.skipped, self.errors, self.time_taken)
    }
}

pub struct Importer {
    settings: ImportSettings,
    database: SqliteConnection,
    file_list: Vec<ImportFile>,
}

#[derive(Debug, Default)]
struct ImportSettings {
    hand_count: u32,
    drop_indexes: String,
    drop_hud_cache: String,
    stars_archive: bool,
    ftp_archive: bool,
    test_data: bool,
    cache_hhc: bool,
}

#[derive(Debug)]
struct ImportFile {
    path: String,
    site: String,
    file_type: FileType,
    file_id: Option<i32>,
}

#[derive(Debug)]
enum FileType {
    HandHistory,
    Summary,
    Both,
}

impl Importer {
    pub fn new(database_url: &str) -> Self {
        let database = SqliteConnection::establish(database_url)
            .expect("Error connecting to database");
        
        Importer {
            settings: ImportSettings::default(),
            database,
            file_list: Vec::new(),
        }
    }

    pub fn add_import_file(&mut self, _filename: &str, _site: &str) -> Result<(), String> {
        // Implement file addition logic here
        Ok(())
    }

    pub fn run_import(&mut self) -> Result<ImportStats, String> {
        // Implement import logic here
        Ok(ImportStats::default())
    }

    // Add other methods as needed
}

#[derive(Debug, Default)]
pub struct ImportStats {
    stored: u32,
    duplicates: u32,
    partial: u32,
    skipped: u32,
    errors: u32,
    time_taken: f64,
}



