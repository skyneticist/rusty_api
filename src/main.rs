use futures::executor::block_on;
use std::fs::File;
use std::io::{BufWriter, Write};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short, long)]
    debug: bool,
    #[structopt(short, long, default_value = "https://api.spacexdata.com/v3/capsules")]
    url: String,
    #[structopt(short, long, default_value = "test_file.txt")]
    file_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let my_future = get_links(&args);
    let my_response = block_on(my_future);

    match handle_data(
        &args.file_name,
        &my_response.unwrap_or_default(),
        Some(args.debug),
    ) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    let rockets_filename: String = "spacex_all_rockets.txt".to_string();

    let spacex_future = get_projects();
    let future_response = block_on(spacex_future);

    match handle_data(
        &rockets_filename,
        &future_response.unwrap_or_default(),
        Some(true),
    ) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
    
    Ok(())
}

async fn get_links(args: &Cli) -> Result<String, Box<dyn std::error::Error>> {
    let response: String = match reqwest::get(&args.url).await?.text().await {
        Ok(r) => r,
        Err(e) => e.to_string(),
    };
    return Ok(response);
}

async fn get_projects() -> Result<String, Box<dyn std::error::Error>> {
    let spacex_rockets_url: String = "https://api.spacexdata.com/v3/rockets".to_string();
    let response: String = match reqwest::get(spacex_rockets_url).await?.text().await {
        Ok(r) => r,
        Err(e) => e.to_string(),
    };
    return Ok(response);
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

    // attempting to write an async tokio runtime for
    // handling async tokio calls in various tests below
    // -------------------------------------------------
    // fn run_one_call<F>(f: F) -> Result<F::Item, F::Error>
    // where
    //     F: IntoFuture,
    //     F::Future: Send + 'static,
    //     F::Item: Send + 'static,
    //     F::Error: Send + 'static,
    // {
    //     let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create runtime");
    //     runtime.block_on(f.into_future())
    // }

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

        let result = match std::fs::read_to_string(borrowed_file_name) {
            Ok(k) => k,
            Err(e) => panic!("{}", e),
        };

        let expected: String = String::from(data);

        assert_eq!(result, expected);
    }

    // #[test]
    // fn test_get_projects() {
    //     let result = block_on(get_projects());
    //     let result = result.unwrap_or_default();

    //     assert_eq!();
    // }
}
