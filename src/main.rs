mod importer;

use importer::Importer;

fn main() {
    let database_url = "database.sqlite";
    let mut importer = Importer::new(database_url);

    // Example usage
    match importer.add_import_file("6+Holdem-6max-USD-2.00-2.00-201904.walk.txt", "PokerStars") {
        Ok(_) => println!("File added successfully"),
        Err(e) => eprintln!("Error adding file: {}", e),
    }

    match importer.run_import() {
        Ok(stats) => println!("Import completed: {}", stats),
        Err(e) => eprintln!("Import error: {}", e),
    }
}