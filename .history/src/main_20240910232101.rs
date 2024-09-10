use clap::{Parser, Subcommand};
use anyhow::{Result,anyhow};
use std::path::Path;
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
    #[arg(short,long,value_parser = verify_input_file)]
    input: String,
    #[arg(short,long,default_value = "output.json")]
    output: String,
    #[arg(short,long,default_value_t = ',')]
    delimiter:char,
    #[arg(long,default_value_t = true)]
    header: bool,

}

fn verify_input_file(filename: &str) -> Result<String> {
    if Path::new(filename).exists(){
        Ok(filename)
    }else{
        Err(anyhow!(format!("The file '{}' does not exist.", filename)))
    }
}

fn main() {
    let opts = Opts::parse();

    print!("{:?}",opts);

   

    // Continued program logic goes here...
}