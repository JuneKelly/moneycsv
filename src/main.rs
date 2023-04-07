extern crate yaml_rust;

use yaml_rust::{YamlLoader};

fn main() {
    let example_doc =
"
- 2023-01-01:
  - 21.45 sweets and wine
  - 3.67 groceries
- 2023-01-02:
  - 15.98 things
  - 210.34 toys
";

    let docs = YamlLoader::load_from_str(example_doc).unwrap();
    let doc = &docs[0];

    println!("Doc: {:?}", doc);

    let doc_vec = doc.as_vec().unwrap();
    println!("As vector: {:?}", doc_vec);

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

    println!(">> Rows:");
    for row in rows {
        println!("{:?}", row);
    }

}
