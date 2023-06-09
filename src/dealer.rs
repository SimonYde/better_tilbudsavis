use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::EnumIter;
#[derive(
    Hash, Debug, EnumIter, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy,
)]
pub enum Dealer {
    Rema1000,
    Netto,
    DagliBrugsen,
    SuperBrugsen,
    Aldi,
    Bilka,
    Coop365,
    Irma,
    Føtex,
    Lidl,
    Meny,
    Kvickly,
    Spar,
}

impl Dealer {
    pub fn id(&self) -> &'static str {
        match self {
            Dealer::Rema1000 => "11deC",
            Dealer::Netto => "9ba51",
            Dealer::DagliBrugsen => "d311fg",
            Dealer::Aldi => "98b7e",
            Dealer::Bilka => "93f13",
            Dealer::Coop365 => "DWZE1w",
            Dealer::Irma => "d432U",
            Dealer::Føtex => "bdf5A",
            Dealer::Lidl => "71c90",
            Dealer::Meny => "267e1m",
            Dealer::Kvickly => "c1edq",
            Dealer::Spar => "88ddE",
            Dealer::SuperBrugsen => "0b1e8",
        }
    }
}
impl FromStr for Dealer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match s.trim().to_lowercase().as_str() {
            "bilka" => Dealer::Bilka,
            "coop365" => Dealer::Coop365,
            "lidl" => Dealer::Lidl,
            "rema1000" => Dealer::Rema1000,
            "spar" => Dealer::Spar,
            "meny" => Dealer::Meny,
            "føtex" => Dealer::Føtex,
            "irma" => Dealer::Irma,
            "aldi" => Dealer::Aldi,
            "netto" => Dealer::Netto,
            "kvickly" => Dealer::Kvickly,
            "daglibrugsen" | "dagli'brugsen" => Dealer::DagliBrugsen,
            "superbrugsen" => Dealer::SuperBrugsen,
            _ => return Err(()),
        };
        Ok(value)
    }
}
