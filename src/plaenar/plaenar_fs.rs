
pub fn plaenar_fs_test(){
    println!("plaenar_fs_test");
}


use std::fs::read_dir;
use std::{fs, io};
use std::path::{Path, PathBuf};




#[derive(Debug)]
pub struct Directory {
    pub name: String,
    /// e.g. /path/to/projects/create-library/ is the  path to the 'crate-library' project
    pub path: PathBuf,
    pub parent_path: PathBuf,
    dir_names: Vec<String>,
    file_names: Vec<String>,
}
// impl Clone for PlaenarDir {
//     fn clone(&self) -> Self {
//         let mut new_dirs = Vec::new();

//         PlaenarDir {
//             name: self.name.clone(),
//             path: self.path.clone(),
            
//         }
//     }
// }
impl Directory {

    // pub fn new() -> Directory {
    //     Directory {
    //         name: String::from(""),
    //         path: PathBuf::from(""),
    //         parent_path: PathBuf::from(""),
    //         dir_names: Vec::new(),
    //         file_names: Vec::new(),
    //     }
    // }

     pub fn new(_name: &String, _path: &PathBuf, _parent_path: &PathBuf) -> Directory {
        Directory {
            name: _name.clone(),
            path: _path.clone(),
            parent_path: _parent_path.clone(),
            dir_names: Vec::new(),
            file_names: Vec::new(),
        }
    }



    pub fn get_dirs(&self) -> Vec<String> {
        return self.dir_names.clone();
    }
    
    pub fn get_dir_entries(&self) -> io::Result<Vec<fs::DirEntry>> {    
        
        // Grab an easily handled vector of directory entries
        // let entries = match fs::read_dir(self.path.clone() ) {

        //     Ok(entries) => entries.collect::<Result<Vec<_>, io::Error>>()?,
        //     Err(err) => {
        //         let path_path_buf = self.path.clone();
        //         // To lossy string conversion is ok for now -- the printing is the mort important right now..
        //         eprintln!("Failed to read directory entry {} \n {}",path_path_buf.to_string_lossy().to_string() , err);
        //         return Err(err);
        //     },
            
        // };

        

        let dir_read = read_dir(self.path.clone())?;
        let dir_vector = dir_read.collect::<Result<Vec<_>, io::Error>>()?;

        Ok(dir_vector)
        
    }
    pub fn get_files(&self) -> Vec<String> {
        return self.file_names.clone();
    }

    pub fn set_name_and_paths(&mut self, new_name: String, new_path: PathBuf, new_parent_path: PathBuf) {
        self.name = new_name.clone();
        // self.path = PathBuf::from(new_path.clone());
        self.path = new_path;
        self.parent_path = new_parent_path;
    }

    /// Returns the verified string. 
    /// TODO: Return an os-indepenedent path object instead!
    /// NOTE: This might be performing superfluous checks, but the basic contents should still be checked to verify that it is in fact a plaenarDir.
    pub fn verify_dir_string(path_string: &String) ->  Result<PathBuf, io::Error>  {
    // pub fn verify_dir_string(path_string: &String) ->  Result<&Path, io::Error>  {

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

        Ok(PathBuf::from(string_to_return))
        // Ok(path)

    }


    pub fn parse_dir_contents(&mut self) -> io::Result<()>{

        // Grab an easily handled vector of directory entries
        let entries = match fs::read_dir(self.path.clone() ) {

            Ok(entries) => entries.collect::<Result<Vec<_>, io::Error>>()?,
            Err(err) => {
                let path_path_buf = self.path.clone();
                // To lossy string conversion is ok for now -- the printing is the mort important right now..
                eprintln!("Failed to read directory entry {} \n {}",path_path_buf.to_string_lossy().to_string() , err);
                return Err(err);
            },
            
        };


        // Put files and dirs in their respective PlaenarDir-vector
        for entry in entries {
            let file_type = entry.file_type()?;

            // Flags
            let is_file = file_type.is_file();
            let is_dir = file_type.is_dir();

            // jumping through hoops
            let entry_name = entry.file_name().to_string_lossy().into_owned();
            let entry_name_tmp = entry_name.clone();


            // Moving actual data
            if is_file {

                self.file_names.push(entry_name_tmp);

            } else if is_dir {

                self.dir_names.push(entry_name_tmp);
            }
            
        }

        Ok(())
        
    }



    pub fn print_dir_contents(&self, space_indent: u8) {
        let dir_names = &self.dir_names;
        let file_names = &self.file_names;

        // Indent
        let mut indent_string = String::new();
        let mut i: u8 = 0;
        while i < space_indent {
            indent_string.push(' ');
            i = i + 1;
        }

        for dir_name in dir_names {
            println!("{}_ {}", indent_string, dir_name);
        }

        for file_name in file_names {
            println!("{}{}", indent_string, file_name);
        }

    }



    

}

#[test]
fn test_verify_dir_string() {

    match Directory::verify_dir_string(&String::from("./src")) {
        Ok(_) => println!("Directory is valid!"),
        Err(err) => {
            eprintln!("Directory verification failed: {}", err);
        },
    }

}






#[derive(Debug)]
struct File {
    path: String,
    name: String,
    file_type: FileType,
    contents: String,
}

#[derive(Debug)]
enum FileType {
    Unknown,
    tasks,
    media,
    markdown,
}