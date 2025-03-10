use clap::Parser;
use errors::GenError;
use std::fs;

mod errors;
mod generator;
mod models;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'i', long)]
    input: String,
    #[arg(short = 'c', long)]
    count: u32,
}

fn main() {
    let args = Args {
        input: String::from("./input.json"),
        count: 10,
    }; //Args::parse();
    let input_file = fs::read_to_string(&args.input)
        .map_err(GenError::from)
        .and_then(|content| {
            serde_json::from_str::<models::InputDefinition>(content.as_str())
                .map_err(GenError::from)
        })
        .expect(format!("Failed to read input file at {}", &args.input).as_str());
    let generated = generator::generate(args.count, &input_file.fields)
        .map_err(GenError::from)
        .expect("Failed to generate data");
    let mut writer = csv::Writer::from_writer(std::io::stdout());
    writer
        .write_record(&generated.header)
        .expect("Failed to write header");
    for row in generated.rows {
        writer.write_record(row).expect("Failed to write row");
    }
    writer.flush().expect("Failed to flush output");
}
