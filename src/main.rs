use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[arg(short, long)]
    scope: Option<String>,

    #[arg(short = 'o', long="output", default_value = "stdout")]
    output: String,

    #[arg(short = 'f', long="format", default_value = "plain")]
    format: String,

    #[arg(short='t', long="type", help = "The type of planner object to create [project, module, task]")]
    object_type: String,
    

    #[command(subcommand)]
    verb: Option<Commands>,

}

#[derive(Subcommand)]
enum Commands {
    New {},
    Parse {}
}

fn main() {
    // let s1 = "Hola, Mundo!";
    let s1: String = String::from("Hola, Mundo!");
    let s2: String = s1;
    let s3: &str = "Yello";
    let s4: &str = "Yello";

    println!("{:?}", s2);
    println!("{s3}");
    println!("{}", s4);

    let cli: Cli = Cli::parse();
    println!("Debug: {}", cli.debug);

    let output = cli.output;
    let format = cli.format;
    let object_type = cli.object_type;
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
        Some(Commands::Parse { }) => {
            println!("project parse output={output}, format={format}");
        }
        Some(Commands::New { }) => {
            println!("project new :  type= {object_type}, output={output}, format={format}");
        }
        None => {}
    }

    test();
}


fn test() {
    println!("Hola, Mundo!");
}