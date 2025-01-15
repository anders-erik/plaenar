

pub fn test(){
    println!("HELLO FROM MY FIRST MODULE!")
}

pub mod plaenar_fs;


use std::{env, fs, io};
use std::path::Path;


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

struct PlaenarProject {
    name: String,
    dir: plaenar_fs::PlaenarDir,
}

impl PlaenarProject {
    pub fn new() -> PlaenarProject {
        PlaenarProject {
            name: String::from(""),
            dir: plaenar_fs::PlaenarDir::new(),
        }
    }
    
    pub fn load_fs(&mut self, path: String) {
        // self.dir.path = path;
        // self.dir.parse_dir_contents();
    }
}

// Holds the configuration for the parse/creation-run
pub struct Plaenar  {
    pub run_scope: RunScope,
    run_verb: RunVerb,

    projects: PlaenarProject,

    aeusb_root_dir: plaenar_fs::PlaenarDir,
    aeusb_projects_dir: plaenar_fs::PlaenarDir,
}

impl Plaenar {

    pub fn new() -> Self {
        Plaenar {
            run_scope: RunScope::new(),
            run_verb: RunVerb::None,
            projects: PlaenarProject::new(),
            aeusb_root_dir: plaenar_fs::PlaenarDir::new(),
            aeusb_projects_dir: plaenar_fs::PlaenarDir::new(),
        }
    }

    pub fn print(&self){
        // TODO: loop through projects and print them
        // println!("{:?}", self.aeusb_projects_dir.dirs)
    }

    pub fn find_and_verify_and_load_root_and_project_dirs(&mut self, aeusb_root_argument: String){

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

        let verified_root_dir_string = match plaenar_fs::PlaenarDir::verify_dir_string(aeusb_root_dir_string) {
            Ok(returned_string) => returned_string,
            Err(err) => {
                eprintln!("Root directory verification failed : {}", err);
                std::process::exit(1);
            },
        };

        self.aeusb_root_dir.set_name_and_path(String::from("root"), verified_root_dir_string.clone());

        self.aeusb_root_dir.parse_dir_contents();
        self.aeusb_root_dir.print_dir_contents(0);




        // PROJECTS DIRECTORY

        let projects_projects_dir_path_string = aeusb_root_dir_string.clone() + "projects";
        let verified_projects_dir_string = match plaenar_fs::PlaenarDir::verify_dir_string(&projects_projects_dir_path_string) {
            Ok(returned_string) => returned_string,
            Err(err) => {
                eprintln!("Project directory verification failed : {}", err);
                std::process::exit(1);
            },
        };
        
        self.aeusb_projects_dir.set_name_and_path(String::from("projects"), verified_projects_dir_string.clone());
        
        self.aeusb_projects_dir.parse_dir_contents();
        self.aeusb_projects_dir.print_dir_contents(2);

        // let projects_root_dir_path = PlaenarDir::verify_root_dir(&projects_root_dir_path_string);

    }

}


enum PlaenarObjectType {
    None,
    Project,
    Module,
    Task,
}

