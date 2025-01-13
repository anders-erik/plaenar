
use std::{env, fs};

use clap::{Parser, Subcommand};


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
    }
}


enum ObjectType {
    None,
    Project,
    Module,
    Task,
}

#[derive(Debug)]
struct RunScope {
    project: bool,
    module: bool,
    task: bool,
}


// Holds the configuration for the parse/creation-run
struct PlaenarRun  {
    run_scope: RunScope
}

impl PlaenarRun {
    // fn parse_object_type(&mut self, object_type_string: &String) {
    //     if (object_type_string == "project" || object_type_string == "project") {
    //         // println!("PROJET  EJOPIJF SDOFJ")
    //         // self.objectType = ObjectType::Project;
    //     }
    // }

    pub fn parse_projects(&mut self){
        
        // result.inspect(|x: &ReadDir | println!("original: {:?}", x.count() ));
        // result.
        // result.as_mut().
        
        // std::fs::ReadDir

        // println!("is ok? {}", result.is_ok() );
        // println!("is ok? {}", result.ok());

        let mut plaenar_root_dir: String = String::from("/");
        let vars: Vec<(String, String)> = env::vars().collect();
        for mut var in vars {
        // println!("{:?}", var);
            if var.0 == "PLAENAR_ROOT_DIR" {
                plaenar_root_dir = var.1;
                // println!("{:?}", var.1);
            }
        }
        println!("PLAENAR_ROOT_DIR = {}", plaenar_root_dir);
        

        match fs::read_dir(plaenar_root_dir) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            let fullDirPath = path.unwrap().path();
            let dirName = fullDirPath.file_name().unwrap().to_str();
            // let name = Some(&str);
            // let name = String::from_str(dirName);
            // println!("> {:?}", path.unwrap().path());
            // println!("> dirname = {:?}", dirName);
            match dirName {
                Some(name) => println!("{:?}", name),
                None => println!("ERROR"),
            }
        },
    }
        
    }

    fn set_run_scope(&mut self, _run_type: RunScope){
        self.run_scope = _run_type;
    }

}


fn main() {
    
    // let s1: String = String::from("Hola, Mundo!");
    // let s2: String = s1;
    // let s3: &str = "Yello";
    // let s4: &str = "Yello";

    // println!("{s2}");
    // println!("{}", s3);
    // println!("{:?}", s4);

    // println!("Debug: {}", cli.debug);


    

    let cli: Cli = Cli::parse();

    
    // let output = cli.output;
    // let format = cli.format;
    // let object_type = cli.object_type;
    // let format = cli.verb.;
    

    // match cli.debug {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }

    
    // You can check the value provided by positional arguments, or option arguments
    // if let Some(name) = cli.name.as_deref() {
    //     println!("Value for name: {name}");
    // }
    
    
    match &cli.verb {
        Some(Commands::New { project, module, task }) => {
            println!(" new :  project= {project}");
        }

        Some(Commands::Parse { project, module, task, output, format }) => {
            // println!("project={}", project);

            let run_scope: RunScope;
            
            // if any of the scope-flags are active, set scope exactly as specified by flags
            // The default is full scope
            if *project || *module || *task {
                run_scope = RunScope {
                    project: *project,
                    module: *module,
                    task: *task,
                };
            }
            else {
                run_scope = RunScope {
                    project: true,
                    module: true,
                    task: true,
                };
            }

            
            println!("run_scope = {run_scope:?}");


            let mut plaenar_run = PlaenarRun {
                run_scope: run_scope,
            };

            // plaenar_run.parse_projects();
            plaenar_run.parse_projects();
            
            // plaenar_run.set_run_scope(run_scope);



            // plaenarRun.set_run_type(ObjectType::Project);
            // if *project {
            //     // plaenarRun.parse_object_type(object_type);
            //     plae
            // }
            // let objectType: ObjectType;
            // Make sure that a valid input is used
            // if !(object_type == "project" || object_type == "p") && (object_type != "module") && (object_type != "task") {
            //     println!("Not valid Object Type. Exiting!");
            //     std::process::exit(0);
            //     // ::std::process::exit(0);
            // }

            // run_parse(object_type, output, format);
            //     let output = output;
            //     let format = format;
            //     let object_type = object_type;
            // println!("project parse :  object-type= {object_type}, output={output}, format={format}");
        }
        None => {}
    }

    test();
}


fn run_parse(object_type: &String, output: &String, format: &String) {
    // println!("Hola, Mundo!");
    println!("parsing {object_type}s : output={output}, format={format}");
}

fn test() {
    println!("Hola, Mundo!");
}