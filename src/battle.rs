#![allow(non_snake_case)]
use std::fmt::Display;
use std::str::FromStr;
use std::collections::HashMap;

// use serde::de::{self, Deserialize, Deserializer};
// use serde::Serializer;
//
// fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
// where
//     T: FromStr,
//     T::Err: Display,
//     D: Deserializer<'de>,
// {
//     let s = String::deserialize(deserializer)?;
//     T::from_str(&s).map_err(de::Error::custom)
// }
//
// fn to_str<S, T>(x: &T, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
//     T: ToString,
// {
//     serializer.serialize_str(&x.to_string().clone())
// }

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Battle {
    // KuskiIndex: u32,
    // LevelIndex: u32,
    // BattleType: String,
    // SeeOthers: u8,
    // SeeTimes: u8,
    // AllowStarter: u8,
    // AcceptBugs: u8,
    // NoVolt: u8,
    // NoTurn: u8,
    // OneTurn: u8,
    // NoBrake: u8,
    // NoThrottle: u8,
    // AlwaysThrottle: u8,
    // Drunk: u8,
    // OneWheel: u8,
    // Multi: u8,
    // Started: String,
    // Duration: u32,
    // Countdown: u32,
    #[serde(default)] Aborted: u8,
    #[serde(default)] Finished: u8,
    // InQueue: u8,
    // RecFileName: Option<String>,
    // #[serde(default)] LevelName: String,
    // #[serde(default)] LongName: String,
    // BattleIndex: u32,
    // Apples: bool,
    // Order: String,
    #[serde(default)] pub results: BattleResultMap,
    #[serde(default)] pub error: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BattleResultMap(HashMap<String, BattleResult>);

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BattleResult {
    #[serde(default)] number: u32,
    #[serde(default)] kuski: String,
    #[serde(default)] kuskiindex: u32,
    // team: String,
    // nation: String,
    // time: String,
    // timeindex: u32,
}
