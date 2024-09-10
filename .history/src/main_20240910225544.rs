use clap::{Parser, Subcommand};

#[derive(Debug,Parser)]
#[command(name ="rcli",version,author, about, long_about = None)]
struct Opts {
 
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[command(name = "csv",about="Covert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts{
    #[arg(short,long)]
    input: String,
    #[arg(short,long,default_value = "output.json")]
    output: String,
    #[arg(short,long)]
    delimiter:char,
    #[arg(long,default_value_t = true)]
    header: bool,

}

fn main() {
    let opts = Opts::parse();

    print!("{:?}",opts);

   

    // Continued program logic goes here...
}