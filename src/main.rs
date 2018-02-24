extern crate backoff;
#[macro_use]
extern crate failure;
extern crate rayon;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod battle;
use battle::{Battle, BattleResultMap};

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

use reqwest::Client;
use failure::Error;
use rayon::prelude::*;

use std::{thread, time};

fn get_battle_result(client: &Client, index: u32) -> Result<(u32, Battle), Error> {
    println!("fetching data for battle #{:?}", index);
    let url = format!("http://elmaonline.net/API/battleinfo/{}", index);
    let mut battle: Battle = client.get(&url).send()?.json()?;
    if battle.error.is_none() {
        let url_results = format!("http://elmaonline.net/API/battle/{}", index);
        battle.results = client.get(&url_results).send()?.json()?;
    }
    Ok((index, battle))
}

fn main() {
    let wait: std::time::Duration = time::Duration::from_millis(1000);
    let mut data = String::new();
    let mut battles: HashMap<u32, Battle>;
    match File::open("results.json") {
        Err(_) => {
            battles = HashMap::new();
        }
        Ok(mut file) => {
            file.read_to_string(&mut data)
                .expect("Unable to read string");
            battles = serde_json::from_str(&data).unwrap()
        }
    };

    let client = Client::new();
    let new_battles: HashMap<u32, Battle> = (1..2000 as u32)
        .into_par_iter()
        .filter(|n| !&battles.contains_key(&n))
        .map(|n| {
            thread::sleep(wait);
            get_battle_result(&client, n)
        })
        .filter(|battle| {
            if battle.is_ok() {
                return true;
            } else {
                println!("error: {:?}", battle.as_ref().unwrap_err());
                return false;
            }
        })
        .map(|battle| battle.unwrap())
        .collect();

    if !new_battles.is_empty() {
        battles.extend(new_battles);

        let mut f = File::create("results.json").expect("Unable to create file");
        f.write_all(serde_json::to_string_pretty(&battles).unwrap().as_bytes())
            .expect("Unable to write data");
    }
}
