use std::fs;

pub struct Parts {
    search_paths: Vec<String>,
    part_names: Vec<String>,
}

impl Parts {
    // constructor
    pub fn new() -> Parts {
        // get part filenames
        let mut parts = Vec::new();
        match fs::read_dir("parts/") {
            Ok(paths) => for path in paths {
                parts.push(path.unwrap().path().to_str().unwrap().to_owned())
            },
            Err(why) => println!("! {:?}", why.kind()),
        }

        Parts { 
            search_paths: vec!["parts/".to_owned()],
            part_names: parts,
        }
    }

    // returns search paths
    pub fn get_search_paths(&self) -> Vec<String> {
        self.search_paths.clone()
    }

    // returns part filenames
    pub fn get_part_names(&self) -> Vec<String> {
        self.part_names.clone()
    }

    // returns only lua parts
    pub fn get_lua_parts(&self) -> Vec<String> {
        let mut luas = Vec::new();
        for part in self.part_names.clone() {
            if part.contains(".lua") {
                luas.push(part)
            }
        }
        luas
    }
}
