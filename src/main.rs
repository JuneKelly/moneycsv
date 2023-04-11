extern crate yaml_rust;

use std::{fs, path::PathBuf};

use clap::Parser;
use yaml_rust::{Yaml, YamlLoader};

const SEPARATOR: &str = ";";

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,
}

fn load_yaml_docs(file_path: PathBuf) -> Vec<Yaml> {
    let source_string = fs::read_to_string(file_path).unwrap();
    let docs = YamlLoader::load_from_str(&source_string).unwrap();
    docs
}

fn print_row(row: Vec<&str>) {
    println!("{}", row.join(SEPARATOR));
}

fn flatten_rows(doc: &Vec<Yaml>) -> Vec<Vec<&str>> {
    let rows: Vec<Vec<&str>> = doc
        .iter()
        .flat_map(|h| {
            let hash = h.as_hash().unwrap();
            hash.iter().flat_map(|(date, spends)| {
                spends.as_vec().unwrap().iter().map(|spend| {
                    let spend_str = spend.as_str().unwrap();
                    let split: Vec<&str> = spend_str.splitn(2, ' ').collect();
                    vec![date.as_str().unwrap(), split[0], split[1]]
                })
            })
        })
        .collect();
    rows
}

fn main() {
    let args = Args::parse();
    let file_path = args.file;

    let docs = load_yaml_docs(file_path);

    let doc = &docs[0];
    let doc_vec = doc.as_vec().unwrap();

    let rows = flatten_rows(doc_vec);

    print_row(vec!["date", "cost", "description"]);
    for row in rows {
        print_row(row);
    }
}
