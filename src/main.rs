use std::fs::File;
use std::io::{BufWriter, Write};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short, long)]
    debug: bool,
    #[structopt(
        short,
        long,
        default_value = "https://restcountries.eu/rest/v2/currency/jpy"
    )]
    url: String,
    #[structopt(short, long, default_value = "test_file.txt")]
    file_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let response = reqwest::get(&args.url).await?.text().await?;

    match handle_data(&args.file_name, &response, Some(args.debug)) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    Ok(())
}

fn handle_data(
    file_name: &String,
    data: &String,
    log: Option<bool>,
) -> Result<(), Box<dyn std::error::Error>> {
    if log.unwrap_or_default() {
        match writeln!(std::io::stdout(), "{}", &data) {
            Ok(b) => b,
            Err(e) => panic!("{}", e),
        };
    }

    write_to_file(file_name, data);
    Ok(())
}

fn write_to_file(file_name: &String, data: &String) {
    let f = match File::create(file_name) {
        Ok(k) => k,
        Err(e) => panic!("{}", e),
    };

    let mut bw = BufWriter::new(f);
    match bw.write_all(data.as_bytes()) {
        Ok(k) => k,
        Err(e) => println!("{}", e),
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_data_handling() {
        let file_name: String = "useful_file.txt".to_string();
        let data: String = "".to_string();

        let result = match handle_data(&file_name, &data, Some(true)) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };

        assert_eq!(result, ());
    }

    #[test]
    fn check_writing_to_file() {
        let file_name: String = "megabits".to_string();
        let borrowed_file_name: String = file_name.clone();

        let data: String = "rust in peace".to_string();

        write_to_file(&file_name, &data);

        let r = match std::fs::read_to_string(borrowed_file_name) {
            Ok(k) => k,
            Err(e) => panic!("{}", e),
        };

        let expect: String = String::from(data);

        assert_eq!(r, expect);
    }
}
