use std::fs;
use std::io;

// TODO:
// Choose a more permanent location to save files (or add way to pick location)
// Do more with Result than just print to console.
pub fn save_file(filename: String, data: String) {
    let prefix = "test/".to_owned();
    let path = &(prefix + &filename);

    match write_to_file(path, data) {
        Ok(_) => println!("Successfully saved data to {path}"),
        Err(err) => println!("Failed to save data to {path}. Reason: {err}")
    };
}

// TODO:
// Search for files in different directories (or have user lead the way)
// Do more with Result than just print to console.
pub fn load_file(filename: String) -> String {
    let prefix = "test/".to_owned();
    let path = &(prefix + &filename);

    match read_file(&path) {
        Ok(data) => {
            println!("Successfully read data from {path}");
            data
        },
        Err(err) => {
            println!("Failed to read data from {path}. Reason: {err}");
            "".to_owned()
        },
    }
}

fn read_file(path: &String) -> io::Result<String> {
    fs::read_to_string(path)
}

fn write_to_file(path: &String, data: String) -> io::Result<()> {
    fs::write(path, data)
}