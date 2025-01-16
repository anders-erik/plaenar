

pub fn test(){
    println!("HELLO FROM MY FIRST MODULE!")
}

pub mod plaenar_fs;


use std::ffi::{OsStr, OsString};
use std::{env, process};
use std::path::{Path, PathBuf};
use std::io;


#[derive(Debug)]
pub struct RunScope {
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

struct Project {
    pub name: String,
    pub dir: plaenar_fs::Directory,
    pub task_file: plaenar_fs::TaskFile,
    pub sub_projects: Vec<Project>,
}

impl Project {

    /// Creates Project with an (ideally) previously tested path. <br>
    /// Obviously can't guarantee existence until actuall parsing happens. 
    /// 
    /// TODO : Should return Result with potential error
     pub fn new(_name: String, _dir: plaenar_fs::Directory) -> Result<Project, std::io::Error> {
        let project_path = _dir.path.clone();
        
        let _task_file = plaenar_fs::TaskFile::new(project_path.clone(), _name.clone());

        Ok(Project {
            name: _name.clone(),
            dir: _dir,
            task_file: _task_file,
            sub_projects: Vec::new(),
        })

        // Err((std::io::Error::new(std::io::ErrorKind::InvalidInput, "Path is not a directory")))
    }

    /// Load all project-dirs from the projects directory and pushes them as Project-objects into vector
    pub fn load_sub_projects(&mut self) -> Result<(), std::io::Error> {




        // names of directories in projects directory
        // let projects_dir_dir_names = self.projects_dir.get_dirs();
        let project_entries = self.dir.get_dir_entries()?;
        // let projects_root_path = self.projects_dir.path.clone();

        for project_entry in project_entries {

            let project_name = project_entry.file_name().into_string().unwrap();
            let project_path = project_entry.path();
        
            let mut project_dir = plaenar_fs::Directory::new(&project_name, &project_path, &self.dir.path);

            if project_path.is_file() {
                continue;
            }
            
            project_dir.parse_dir_contents();


            let mut new_project_object =  match Project::new(project_name, project_dir) {
                Ok(new_project_object) => new_project_object,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                },
            };

            new_project_object.load_sub_projects();

            self.sub_projects.push(new_project_object);


        }

        Ok(())


    }


}



// Holds the configuration for the parse/creation-run
pub struct Plaenar  {
    pub run_scope: RunScope,
    run_verb: RunVerb,

    projects: Vec<Project>,

    root_dir: plaenar_fs::Directory,
    projects_dir: plaenar_fs::Directory,
}

impl Plaenar {

    // pub fn new() -> Self {
    //     Plaenar {
    //         run_scope: RunScope::new(),
    //         run_verb: RunVerb::None,
    //         projects: Vec::new(),
    //         root_dir: plaenar_fs::Directory::new(),
    //         projects_dir: plaenar_fs::Directory::new(),
    //     }
    // }

    pub fn init(aeusb_root_argument: String) -> io::Result<Self> {

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
        let aeusb_root_path_buf = PathBuf::from(aeusb_root_dir_string.clone());



        // let verified_root_dir_string = match plaenar_fs::Directory::verify_dir_string(aeusb_root_dir_string) {
        //     Ok(returned_string) => returned_string,
        //     Err(err) => {
        //         eprintln!("Root directory verification failed : {}", err);
        //         std::process::exit(1);
        //     },
        // };
        if !aeusb_root_path_buf.is_dir() {
            eprintln!("No directory found at provided root location : {}", aeusb_root_path_buf.to_str().unwrap());
            println!("Exiting.");
            process::exit(1);
        };

        let verified_root_path = aeusb_root_path_buf;

        let mut root_dir = plaenar_fs::Directory::new(&String::from("root"), &verified_root_path, &PathBuf::from(""));

        // self.root_dir.set_name_and_paths(String::from("root"), verified_root_path, PathBuf::from(""));

        root_dir.parse_dir_contents();
        




        // PROJECTS DIRECTORY

        
        let projects_dir_path_string = aeusb_root_dir_string.clone();
        let projects_dir_path = PathBuf::from(projects_dir_path_string + "projects");
        if !projects_dir_path.is_dir() {
            eprintln!("No project directory found in provided root location : {}", projects_dir_path.to_str().unwrap());
            println!("Exiting.");
            process::exit(1);
        };

        let verified_projects_path = projects_dir_path;
        
        let mut projects_dir = plaenar_fs::Directory::new(&String::from("projects"), &verified_projects_path, &verified_root_path);
        // self.projects_dir.set_name_and_paths(String::from("projects"), verified_projects_path);
        
        projects_dir.parse_dir_contents();
        

        // let projects_root_dir_path = PlaenarDir::verify_root_dir(&projects_root_dir_path_string);

        // Ok(())
        

        Ok(Plaenar {
            run_scope: RunScope::new(),
            run_verb: RunVerb::None,
            projects: Vec::new(),
            root_dir: root_dir,
            projects_dir: projects_dir,
        })
    }

    pub fn print_all(&self){
        // TODO: loop through projects and print them
        
        self.root_dir.print_dir_contents(0);

        println!("{}", "\nprojects");

        // Print all projects
        let project_count = self.projects.len();
        for i in 0..project_count {
            println!("    {}", self.projects[i].name);
            // self.projects[i].dir.print_dir_contents(8);

            // Print modules
            let module_count = self.projects[i].sub_projects.len();
            for j in 0..module_count {
                println!("        {}", self.projects[i].sub_projects[j].name);
                self.projects[i].sub_projects[j].dir.print_dir_contents(12);
            } 
        }

    }

    /// Load all project-dirs from the projects directory and pushes them as Project-objects into vector
    pub fn load_projects(&mut self) -> Result<(), std::io::Error>{

        // names of directories in projects directory
        // let projects_dir_dir_names = self.projects_dir.get_dirs();
        let projects_root_entries = self.projects_dir.get_dir_entries()?;
        // let projects_root_path = self.projects_dir.path.clone();

        for project_entry in projects_root_entries {

            let project_name = project_entry.file_name().into_string().unwrap();
            let project_path = project_entry.path();
            
            if project_path.is_file() {
                continue;
            }

            let mut project_dir = plaenar_fs::Directory::new(&project_name, &project_path, &self.projects_dir.path);

            project_dir.parse_dir_contents();


            let mut new_project_object =  match Project::new(project_name, project_dir) {
                Ok(new_project_object) => new_project_object,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                },
            };

            new_project_object.load_sub_projects();

            self.projects.push(new_project_object);


        }
        Ok(())
        // self.projects.push( PlaenarProject::new() );
    }


}


enum PlaenarObjectType {
    None,
    Project,
    Module,
    Task,
}

