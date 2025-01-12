use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[arg(short, long)]
    apa: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {

    Project {
        #[arg(short, long)]
        list: bool,
    }
    
}

fn main() {
    // let s1 = "Hola, Mundo!";
    let s1: String = String::from("Hola, Mundo!");
    let s2: String = s1;
    let s3: &str = "Yello"; 
    println!("{:?}", s2);
    println!("{s3}");

    let cli: Cli = Cli::parse();
    println!("Debug: {}", cli.debug);

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

    
    match &cli.command {
        Some(Commands::Project { list }) => {
            if *list {
                println!("project list");
            } else {
                println!("unknown project command");
            }
        }
        None => {}
    }
}

