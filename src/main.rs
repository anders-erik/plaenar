
use std::{env, fs, io};
use std::path::Path;
use clap::{Parser, Subcommand};

pub mod plaen;



#[derive(Parser)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    verb: Option<Commands>,
}


#[derive(Subcommand)]
enum Commands {

    New {

        #[arg(short='p', long="project", default_value="false", help = "Target the project objects. ")]
        project: bool,

        #[arg(short='m', long="module", default_value="false", help = "Target the module objects. ")]
        module: bool,

        #[arg(short='t', long="task", default_value="false", help = "Target the task objects. ")]
        task: bool,

        #[arg(short='u', long="aeusb-root-dir", default_value="", help = "Aeusb root directory. [overrides $AESAFE envvar]")]
        aeusb_root_argument: String,
    },
    Parse {

        #[arg(short='p', long="project", default_value="false", help = "Target the project objects. ")]
        project: bool,

        #[arg(short='m', long="module", default_value="false", help = "Target the module objects. ")]
        module: bool,

        #[arg(short='t', long="task", default_value="false", help = "Target the task objects. ")]
        task: bool,

        #[arg(short = 'o', long="output", default_value = "stdout")]
        output: String,

        #[arg(short = 'f', long="format", default_value = "plain")]
        format: String,

        #[arg(short='u', long="aeusb-root-dir", default_value="", help = "Aeusb root directory. [overrides $AESAFE envvar]")]
        aeusb_root_argument: String,
    }
}






#[derive(Debug)]
struct RunScope {
    project: bool,
    module: bool,
    task: bool,
}

impl RunScope {

    pub fn new() -> RunScope {
        RunScope {
            project: false,
            module: false,
            task: false,
        }
    }

    pub fn load_cli_args(&mut self, project: &bool, module: &bool, task: &bool) {

        // if any of the scope-flags are active, set scope exactly as specified by flags
        // The default is full scope
        if *project || *module || *task {
            self.project    = project.clone();
            self.module     = module.clone();
            self.task       = task.clone();
        }
        else {
            self.project    = true;
            self.module     = true;
            self.task       = true;
        }
        println!("run_scope = {self:?}");

    }
}

enum RunVerb {
    None,
    Parse,
    New,
}

// Holds the configuration for the parse/creation-run
struct Plaenar  {
    run_scope: RunScope,
    run_verb: RunVerb,
    aeusb_root_dir: PlaenarDir,
    /// Verified directory path
    // projects_root_dir_path: String,
    aeusb_projects_dir: PlaenarDir,
}

impl Plaenar {

    pub fn new() -> Self {
        Plaenar {
            run_scope: RunScope::new(),
            run_verb: RunVerb::None,
            aeusb_root_dir: PlaenarDir::new(),
            aeusb_projects_dir: PlaenarDir::new(),
        }
    }

    pub fn find_and_verify_root_and_project_dir_paths(root_path_arg: String){

    }

}


enum PlaenarObjectType {
    None,
    Project,
    Module,
    Task,
}



#[derive(Debug)]
struct PlaenarDir {
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


    fn parse_dir_contents(&mut self) -> io::Result<()>{

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



    fn print_dir_contents(&mut self, space_indent: u8) {
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




fn main() {
    
    plaen::test();

    let cli: Cli = Cli::parse();

    let mut plaenar = Plaenar::new();
    
    
    match &cli.verb {
        Some(Commands::New {aeusb_root_argument, project, module, task }) => {
            println!(" new :  project= {project}");
        }

        Some(Commands::Parse {aeusb_root_argument, project, module, task, output, format }) => {
            

            plaenar.run_scope.load_cli_args(project, module, task);


            // VERIFY ROOT AND PROJECT DIRS FUNC


            // Determine the root directory of aeusb
            let mut aeusb_root_dir_string: String = String::from("");

            // Look through environment
            let mut aeusb_root_env: String = String::from("");
            let vars: Vec<(String, String)> = env::vars().collect();
            for mut var in vars {
            // println!("{:?}", var);
                if var.0 == "AEUSB" {
                    aeusb_root_env = var.1;
                    // println!("{:?}", var.1);
                }
            }


            // Set root directory
            // The arguments path takes precedence over environment variable!
            if aeusb_root_argument != "" {
                aeusb_root_dir_string = aeusb_root_argument.to_string();
            } else if aeusb_root_env != "" {
                aeusb_root_dir_string = aeusb_root_env;
            } else {
                eprintln!("No aeusb root directory provided.");
                eprintln!("Make sure to proved -u flag or the AEUSB envvar!");
                std::process::exit(1);
            }



            //  ROOT DIRECTORY 

            // New immutable owner of root path candidate
            let aeusb_root_dir_string = &aeusb_root_dir_string;

            let verified_root_dir_string = match PlaenarDir::verify_dir_string(aeusb_root_dir_string) {
                Ok(returned_string) => returned_string,
                Err(err) => {
                    eprintln!("Root directory verification failed : {}", err);
                    std::process::exit(1);
                },
            };

            plaenar.aeusb_root_dir.name = String::from("root");
            plaenar.aeusb_root_dir.path = verified_root_dir_string.clone(); // we drop all previous root strings
            plaenar.aeusb_root_dir.parse_dir_contents();
            plaenar.aeusb_root_dir.print_dir_contents(0);




            // PROJECTS DIRECTORY

            let projects_projects_dir_path_string = aeusb_root_dir_string.clone() + "projects";
            let verified_projects_dir_string = match PlaenarDir::verify_dir_string(&projects_projects_dir_path_string) {
                Ok(returned_string) => returned_string,
                Err(err) => {
                    eprintln!("Project directory verification failed : {}", err);
                    std::process::exit(1);
                },
            };
            
            plaenar.aeusb_projects_dir.name = String::from("projects");
            plaenar.aeusb_projects_dir.path = verified_projects_dir_string.clone(); // we drop all previous root strings
            plaenar.aeusb_projects_dir.parse_dir_contents();
            plaenar.aeusb_projects_dir.print_dir_contents(2);

            // let projects_root_dir_path = PlaenarDir::verify_root_dir(&projects_root_dir_path_string);

            
            println!("{:?}", plaenar.aeusb_projects_dir.dirs)

            
        }
        None => {}
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