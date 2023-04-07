extern crate yaml_rust;

use yaml_rust::{YamlLoader};

fn main() {
    let example_doc =
"
- 2023-01-01:
  - 21.45 sweets
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

    for val in doc_vec.iter() {
        let h = val.as_hash().unwrap();
        println!("Entry: {:?}", h);
        for (k, v) in h.iter() {
            println!("Date: {:?}", k);
            for spend in v.as_vec().unwrap().iter() {
                println!("  Spend {:?}", spend);
            }
        }
    }


}
