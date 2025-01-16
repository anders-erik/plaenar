

pub fn test(){
    println!("HELLO FROM MY FIRST MODULE!")
}

pub mod plaenar_fs;


use std::{env, process};
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

struct Project {
    pub name: String,
    pub dir: plaenar_fs::Directory,
    pub modules: Vec<Module>,
}

impl Project {

    /// Creates Project with an (ideally) previously tested path. <br>
    /// Obviously can't guarantee existence until actuall parsing happens. 
    /// 
    /// TODO : Should return Result with potential error
     pub fn new(path_string: String) -> Result<Project, std::io::Error> {

        // Make sure firectory is ok
        let validated_proj_path = match plaenar_fs::Directory::verify_dir_string(&path_string) {
            Ok(validated_proj_path) => validated_proj_path,
            Err(err) => {
                eprintln!("{:?}", err);
                std::process::exit(1);
            },
        };

        // Create Project
        let mut plaenar_dir = plaenar_fs::Directory::new();
        plaenar_dir.path = validated_proj_path;


        let path = Path::new(&path_string);
        
        // TODO: CLEAN UP 'unwrap'
        // let project_name = String::from(path.to_str().unwrap());
        let project_name = String::from(path.file_name().unwrap().to_str().unwrap());
        plaenar_dir.name = project_name.clone();


        // Parse and print
        plaenar_dir.parse_dir_contents()?;

        
        Ok(Project {
            name: project_name.clone(),
            dir: plaenar_dir,
            modules: Vec::new(),
        })

        // Err((std::io::Error::new(std::io::ErrorKind::InvalidInput, "Path is not a directory")))
    }

    /// Load all project-dirs from the projects directory and pushes them as Project-objects into vector
    pub fn load_module_directories(&mut self){

        // names of directories in module directory
        let module_dir_dir_names = self.dir.get_dirs();
        let dir_path = self.dir.path.clone();

        for dir_name in module_dir_dir_names {
        
            let mut module_dir_string = String::new();
            module_dir_string.push_str(dir_path.as_str());
            module_dir_string.push_str("/");
            module_dir_string.push_str(dir_name.as_str());
            module_dir_string.push_str("/");

            // println!("{project_dir_string}");

            let new_module_object =  match Module::new(module_dir_string) {
                Ok(new_module_object) => new_module_object,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                },
            };

            // println!("{project_dir_string}");

            self.modules.push(new_module_object);


        }

        // self.projects.push( PlaenarProject::new() );
    }


}




struct Module {
    pub name: String,
    pub dir: plaenar_fs::Directory,
}

impl Module {

    /// Creates Project with an (ideally) previously tested path. <br>
    /// Obviously can't guarantee existence until actuall parsing happens. 
    /// 
    /// TODO : Should return Result with potential error
     pub fn new(path_string: String) -> Result<Module, std::io::Error> {

        // Make sure firectory is ok
        let validated_proj_path = match plaenar_fs::Directory::verify_dir_string(&path_string) {
            Ok(validated_proj_path) => validated_proj_path,
            Err(err) => {
                eprintln!("{:?}", err);
                std::process::exit(1);
            },
        };

        // Create Project
        let mut plaenar_dir = plaenar_fs::Directory::new();
        plaenar_dir.path = validated_proj_path;


        let path = Path::new(&path_string);
        
        // TODO: CLEAN UP 'unwrap'
        // let project_name = String::from(path.to_str().unwrap());
        let module_name = String::from(path.file_name().unwrap().to_str().unwrap());
        plaenar_dir.name = module_name.clone();


        // Parse and print
        plaenar_dir.parse_dir_contents()?;

        
        Ok(Module {
            name: module_name.clone(),
            dir: plaenar_dir,
        })

        // Err((std::io::Error::new(std::io::ErrorKind::InvalidInput, "Path is not a directory")))
    }



}

// Holds the configuration for the parse/creation-run
pub struct Plaenar  {
    pub run_scope: RunScope,
    run_verb: RunVerb,

    projects: Vec<Project>,

    aeusb_root_dir: plaenar_fs::Directory,
    aeusb_projects_dir: plaenar_fs::Directory,
}

impl Plaenar {

    pub fn new() -> Self {
        Plaenar {
            run_scope: RunScope::new(),
            run_verb: RunVerb::None,
            projects: Vec::new(),
            aeusb_root_dir: plaenar_fs::Directory::new(),
            aeusb_projects_dir: plaenar_fs::Directory::new(),
        }
    }

    pub fn print_all(&self){
        // TODO: loop through projects and print them
        
        self.aeusb_root_dir.print_dir_contents(0);

        println!("{}", "\nprojects");

        // Print all projects
        let project_count = self.projects.len();
        for i in 0..project_count {
            println!("    {}", self.projects[i].name);
            // self.projects[i].dir.print_dir_contents(8);

            // Print modules
            let module_count = self.projects[i].modules.len();
            for j in 0..module_count {
                println!("        {}", self.projects[i].modules[j].name);
                self.projects[i].modules[j].dir.print_dir_contents(12);
            } 
        }

    }

    /// Load all project-dirs from the projects directory and pushes them as Project-objects into vector
    pub fn load_projects(&mut self){

        // names of directories in projects directory
        let projects_dir_dir_names = self.aeusb_projects_dir.get_dirs();
        let dir_path = self.aeusb_projects_dir.path.clone();

        for dir_name in projects_dir_dir_names {
        
            let mut project_dir_string = String::new();
            project_dir_string.push_str(dir_path.as_str());
            project_dir_string.push_str("/");
            project_dir_string.push_str(dir_name.as_str());
            project_dir_string.push_str("/");

            // println!("{project_dir_string}");

            let mut new_project_object =  match Project::new(project_dir_string) {
                Ok(new_project_object) => new_project_object,
                Err(err) => {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                },
            };

            new_project_object.load_module_directories();

            self.projects.push(new_project_object);


        }

        // self.projects.push( PlaenarProject::new() );
    }

    pub fn find_and_verify_and_load_root_and_project_dirs(&mut self, aeusb_root_argument: String){

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

        let verified_root_dir_string = match plaenar_fs::Directory::verify_dir_string(aeusb_root_dir_string) {
            Ok(returned_string) => returned_string,
            Err(err) => {
                eprintln!("Root directory verification failed : {}", err);
                std::process::exit(1);
            },
        };

        self.aeusb_root_dir.set_name_and_path(String::from("root"), verified_root_dir_string.clone());

        self.aeusb_root_dir.parse_dir_contents();
        




        // PROJECTS DIRECTORY

        let projects_projects_dir_path_string = aeusb_root_dir_string.clone() + "projects";
        let verified_projects_dir_string = match plaenar_fs::Directory::verify_dir_string(&projects_projects_dir_path_string) {
            Ok(returned_string) => returned_string,
            Err(err) => {
                eprintln!("Project directory verification failed : {}", err);
                std::process::exit(1);
            },
        };
        
        self.aeusb_projects_dir.set_name_and_path(String::from("projects"), verified_projects_dir_string.clone());
        
        self.aeusb_projects_dir.parse_dir_contents();
        

        // let projects_root_dir_path = PlaenarDir::verify_root_dir(&projects_root_dir_path_string);

    }

}


enum PlaenarObjectType {
    None,
    Project,
    Module,
    Task,
}

