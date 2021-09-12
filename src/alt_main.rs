use std::fs::File;
use std::io::{BufWriter, Write};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short, long)]
    debug: bool,
    #[structopt(short, long, default_value = "https://restcountries.eu/rest/v2/currency/jpy")]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let response = reqwest::get(&args.url)
        .await?
        .text()
        .await?;

    match handle_data(response, Some(args.debug)) {
        Ok(r) => r,
        Err(e) => println!("{}", e),
    }

    Ok(())
}

fn handle_data(data: String, log: Option<bool>) -> Result<(), Box<dyn std::error::Error>> {
    if log.unwrap_or_default() {
        println!("body = {:?}", &data);
    }

    write_to_file(data);
    Ok(())
}

fn write_to_file(data: String) {
    let f = match File::create("stuff.txt") {
        Ok(k) => k, 
        Err(e) => panic!("{}", e), 
    };

    let mut bw = BufWriter::new(f);
    match bw.write_all(data.as_bytes()) {
        Ok(k) => k,
        Err(e) => println!("{}", e),
    };
}

// use std::env;
// use std::fs::File;
// use std::io::{BufWriter, Write};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args: Vec<String> = env::args().collect();

//     let endpoint = &args[1];
//     let include_log = true;

//     let body = reqwest::get(endpoint)
//         .await?
//         .text()
//         .await?;

//     match handle_data(body, Some(include_log)) {
//         Ok(body) => body,
//         Err(e) => println!("{}", e),
//     }
//     Ok(())
// }

// fn handle_data(data: String, log: Option<bool>) -> Result<(), Box<dyn std::error::Error>> {
//     if log.unwrap() {
//         println!("body = {:?}", &data);
//     }

//     write_to_file(data);
//     Ok(())
// }

// fn write_to_file(data: String) {
//     let f = match File::create("stuff.txt") {
//         Ok(k) => k, 
//         Err(e) => panic!("{}", e), 
//     };

//     let mut bw = BufWriter::new(f);
//     match bw.write_all(data.as_bytes()) {
//         Ok(k) => k,
//         Err(e) => println!("{}", e),
//     };
//     println!("File written to local directory");
// }


// async fn get_request() -> (Cli, Result<String, Box<dyn std::error::Error>>) {
//     let args = Cli::from_args();

//     let response = reqwest::get(&args.url)
//         .await?
//         .text()
//         .await?;

//         return (args, Ok(response));
// }
