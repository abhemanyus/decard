use std::env::args;
use std::io::Write;
use std::panic::catch_unwind;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, BufWriter},
};

use ical::{
    generator::{Emitter, VcardContact},
    parser::Component,
    VcardParser,
};
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = args().collect();
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    let cards = VcardParser::new(reader);
    let mut named_cards: HashMap<String, VcardContact> = HashMap::new();
    let mut count = 0;
    for card in cards {
        count += 1;
        let mut card = match card {
            Ok(card) => card,
            Err(err) => {
                println!("unable to read card {count}");
                println!("{err:?}");
                continue;
            }
        };
        let name = match card.get_property("FN").and_then(|p| p.value.clone()) {
            Some(name) => name,
            None => {
                println!("No name for {count}");
                continue;
            }
        };
        card.get_property_mut("TEL").and_then(|prop| {
            prop.params.as_mut().map(|params| {
                params
                    .iter_mut()
                    .find(|(k, _)| k == "TYPE")
                    .map(|param| param.1 = vec!["CELL".into()]);
            })
        });
        match named_cards.get_mut(&name) {
            Some(entry) => {
                for new_prop in card.properties {
                    let prop = entry.get_property(&new_prop.name);
                    if prop.is_none() {
                        entry.add_property(new_prop);
                    }
                }
            }
            None => {
                named_cards.insert(name, card);
            }
        }
    }

    println!("Scanned entries: {}", count);
    println!("Unique entries: {}", named_cards.len());

    let file = File::create(&args[2])?;
    let mut out = BufWriter::new(file);
    for card in named_cards.into_values() {
        let res = catch_unwind(|| card.generate());
        match res {
            Ok(card) => {
                write!(out, "{}", card)?;
            }
            Err(_) => {
                println!("{card:?}");
            }
        }
    }
    Ok(())
}
