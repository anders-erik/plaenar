
pub fn plaenar_fs_test(){
    println!("plaenar_fs_test");
}
        
use std::{env, fs, io};
use std::path::Path;


#[derive(Debug)]
pub struct PlaenarDir {
    path: String,
    name: String,
    dirs: Vec<PlaenarDir>,
    files: Vec<PlaenarFile>,
}
impl PlaenarDir {

    pub fn new() -> PlaenarDir {
        PlaenarDir {
            name: String::from(""),
            path: String::from(""),
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    pub fn set_name_and_path(&mut self, new_name: String, new_path: String) {
        self.name = new_name.clone();
        self.path = new_path.clone();
    }

    pub fn verify_dir_string(path_string: &String) ->  Result<String, io::Error>  {

        let path = Path::new(path_string);

        
        if !path.exists () {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Path does not exist"));
        }
        if !path.is_dir() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path is not a directory"));
        }

        // Try accessing the directory to check permissions
        fs::read_dir(path)?; // Propagate any io::Error directly

        let string_to_return = match path.to_str() {
            Some(string) => string,
            None => return Err(io::Error::new(io::ErrorKind::InvalidData, "Path is not a directory")),
        };

        Ok(String::from(string_to_return))

    }


    pub fn parse_dir_contents(&mut self) -> io::Result<()>{

        // Grab an easily handled vector of directory entries
        let dirs = match fs::read_dir(self.path.clone() ) {

            Ok(entries) => entries.collect::<Result<Vec<_>, io::Error>>()?,
            Err(err) => {
                eprintln!("Failed to read directory: {}", err);
                return Err(err);
            },
            
        };

        // Put files and dirs in their respective PlaenarDir-vector
        for entry in dirs {
            let file_type = entry.file_type()?;

            // Flags
            let is_file = file_type.is_file();
            let is_dir = file_type.is_dir();

            // jumping through hoops
            let dir_name = entry.file_name().to_string_lossy().into_owned();
            let dir_name_tmp = dir_name.clone();
            let dir_name_slice = dir_name_tmp.as_str();
            // println!("{:?}", dir_name);


            // Moving actual data
            if is_file {

                self.files.push(PlaenarFile {
                    name: dir_name,
                    path: self.path.clone() + dir_name_slice,
                    file_type: PleanarFileType::Unknown,
                    contents: String::new(),
                });

            } else if is_dir {

                self.dirs.push(PlaenarDir {
                    name: dir_name,
                    path: self.path.clone() + dir_name_slice,
                    dirs: Vec::new(),
                    files: Vec::new(),
                });

            }
            
        }

        Ok(())
        
    }



    pub fn print_dir_contents(&mut self, space_indent: u8) {
        let dirs = &self.dirs;
        let files = &self.files;

        // Indent
        let mut indent_string = String::new();
        let mut i: u8 = 0;
        while i < space_indent {
            indent_string.push(' ');
            i = i + 1;
        }

        for dir in dirs {
            println!("{}{}", indent_string, dir.name);
        }

        for file in files {
            println!("{}{}", indent_string, file.name);
        }

    }



    

}

#[test]
fn test_verify_dir_string() {

    match PlaenarDir::verify_dir_string(&String::from("./src")) {
        Ok(_) => println!("Directory is valid!"),
        Err(err) => {
            eprintln!("Directory verification failed: {}", err);
        },
    }

}



#[derive(Debug)]
struct PlaenarFile {
    path: String,
    name: String,
    file_type: PleanarFileType,
    contents: String,
}

#[derive(Debug)]
enum PleanarFileType {
    Unknown,
    tasks,
    media,
    markdown,
}