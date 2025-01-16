

use clap::{Parser, Subcommand};

pub mod plaenar;




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
    Ls {

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




fn main() -> Result<(), std::io::Error> {
    
    // plaenar::test();
    // plaenar_fs::plaenar_fs_test();

    let cli: Cli = Cli::parse();

    // let mut plaenar = plaenar::Plaenar::new();
    let mut plaenar: plaenar::Plaenar;
    
    match &cli.verb {

        Some(Commands::New {aeusb_root_argument, project, module, task }) => {
            println!(" new :  project= {project}");
        }

        Some(Commands::Ls {aeusb_root_argument, project, module, task, output, format }) => {
            
            // println!(aeusb_root_argument);

            // root and project directories are verified and their contents available
            plaenar = plaenar::Plaenar::init(aeusb_root_argument.clone())?;

            
            plaenar.run_scope.load_cli_args(project, module, task);

            // plaenar.find_and_verify_and_load_root_and_project_dirs(aeusb_root_argument.clone());
            

            plaenar.load_projects();

            if output == "stdout" {
                plaenar.print_all();
            }
            

            
        }
        None => {}
    }
    
    Ok(())
}





