use clap::{Parser, Subcommand};
use anyhow::{Result,anyhow};
use std::{path::Path};
use csv::Reader;
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
        Ok(filename.into())
    }else{
        Err(anyhow!(format!("The file '{}' does not exist.", filename)))
    }
}

fn csv_to_json(input_file: &str, output_file: &str, delimiter: char, header: bool) -> Result<()> {
    let mut rdr = csv::Reader::from_path(input_file);
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() ->Result<()>{
    let opts = Opts::parse();

    match opts.command {
        SubCommands::Csv(csv_opts) => {
            csv_to_json(&csv_opts.input, &csv_opts.output, csv_opts.delimiter, csv_opts.header)?;
        }
    }

    Ok(())
   

}