extern crate yaml_rust;

use std::{fs, path::PathBuf};

use clap::Parser;
use yaml_rust::YamlLoader;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let file_path = args.file;

    let source_string = fs::read_to_string(file_path).unwrap();

    let docs = YamlLoader::load_from_str(&source_string).unwrap();
    let doc = &docs[0];
    let doc_vec = doc.as_vec().unwrap();

    let rows = doc_vec.iter().flat_map(|h| {
        let hash = h.as_hash().unwrap();
        hash.iter().flat_map(|(date, spends)| {
            spends.as_vec().unwrap().iter().map(|spend| {
                let spend_str = spend.as_str().unwrap();
                let split: Vec<&str> = spend_str.splitn(2, ' ').collect();
                vec![date.as_str().unwrap(), split[0], split[1]]
            })
        })
    });

    println!("{}", "date|cost|description");
    for row in rows {
        println!("{}", row.join("|"));
    }
}
