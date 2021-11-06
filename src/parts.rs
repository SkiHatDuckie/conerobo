// parts management
use std::fs;

pub struct PartsManager {
    search_paths: Vec<String>,
    parts: Vec<String>,
    mounted_parts: Vec<String>,
}

impl PartsManager {
    // create instance of part managaer
    pub fn new() -> PartsManager {
        // get directories of parts
        let mut p = Vec::new();
        match fs::read_dir("parts/") {
            Ok(paths) => for path in paths {
                p.push(path.unwrap().path().to_str().unwrap().to_owned())
            },
            Err(why) => println!("! {:?}", why.kind()),
        }

        PartsManager { 
            search_paths: vec!["parts/".to_owned()],
            parts: p,
            mounted_parts: Vec::new(),
        }
    }

    // returns search paths
    pub fn get_search_paths(&self) -> Vec<String> {
        self.search_paths.clone()
    }

    // returns part diectories
    pub fn get_part_names(&self) -> Vec<String> {
        self.parts.clone()
    }

    // returns only lua parts
    pub fn get_mounted_lua_parts(&self) -> Vec<String> {
        let mut luas = Vec::new();
        for part in self.mounted_parts.clone() {
            if part.contains(".lua") {
                luas.push(part)
            }
        }

        luas
    }

    // mounts part to core
    pub fn mount(&mut self, part: &str) {
        if self.parts.contains(&part.to_owned()) {
            self.mounted_parts.push(part.to_owned());
            println!("Successfully mounted {} to core", part);
        } else {
            println!("Failed to mount. Reason: Unknown part name \"{}\"", part);
        }
    }
}
