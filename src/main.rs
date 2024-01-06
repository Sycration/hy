//use chumsky::prelude::*;
use std::{
    fs::{self, *},
    io::{Error, Read, Write},
    num::ParseIntError,
};

use chrono::{Datelike, Local, TimeZone};
use dialoguer::{Input, MultiSelect, Select};
use home::home_dir;
use serde::{Deserialize, Serialize};
fn main() -> Result<(), std::io::Error> {
    let path = home_dir().unwrap().join(".hy");

    if std::env::args().nth(1) == Some("export".to_string()) {
        let mut file = fs::OpenOptions::new().read(true).open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let data = data
            .lines()
            .map(|s| HyInstance::decode(s).unwrap())
            .collect::<Vec<_>>();
        println!("{}", serde_json::to_string(&data).unwrap());
    } else {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        let data = default_action()?;
        file.write_all(HyInstance::encode(&data).as_bytes())?;
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct HyInstance {
    pub h: u8,
    pub s: Sexuality,
    pub time: String,
}

impl HyInstance {
    pub fn encode(&self) -> String {
        format!(
            "{}#{}#{}\n",
            self.h,
            match self.s {
                Sexuality::FullyTowardsFem => 0,
                Sexuality::MostlyTowardsFem => 1,
                Sexuality::SomewhatTowardsFem => 2,
                Sexuality::Neutral => 3,
                Sexuality::SomewhatTowardsMasc => 4,
                Sexuality::MostlyTowardsMasc => 5,
                Sexuality::FullyTowardsMasc => 6,
            },
            self.time,
        )
    }
    pub fn decode(s: &str) -> Result<Self, std::num::ParseIntError> {
        let nums = s
            .splitn(3, '#')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        Ok(Self {
            h: nums[0].clone().parse()?,
            s: Sexuality::from_num(nums[1].clone().parse()?),
            time: nums[2].clone(),
        })
    }
}
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Sexuality {
    FullyTowardsFem,
    MostlyTowardsFem,
    SomewhatTowardsFem,
    Neutral,
    SomewhatTowardsMasc,
    MostlyTowardsMasc,
    FullyTowardsMasc,
}

impl Sexuality {
    pub fn from_num(x: u8) -> Self {
        match x {
            0 => Sexuality::FullyTowardsFem,
            1 => Sexuality::MostlyTowardsFem,
            2 => Sexuality::SomewhatTowardsFem,
            3 => Sexuality::Neutral,
            4 => Sexuality::SomewhatTowardsMasc,
            5 => Sexuality::MostlyTowardsMasc,
            6 => Sexuality::FullyTowardsMasc,
            _ => unreachable!(),
        }
    }

    pub fn to_num(&self) -> u8 {
        match self {
            Sexuality::FullyTowardsFem => 0,
            Sexuality::MostlyTowardsFem => 1,
            Sexuality::SomewhatTowardsFem => 2,
            Sexuality::Neutral => 3,
            Sexuality::SomewhatTowardsMasc => 4,
            Sexuality::MostlyTowardsMasc => 5,
            Sexuality::FullyTowardsMasc => 6,
        }
    }
}

pub fn default_action() -> Result<HyInstance, Error> {
    let how_much = Input::new()
        .with_prompt("On a scale of 0 (none at all) through 10 (mind overflowing), how attracted have you been today?")
        .validate_with(|input: &String| -> Result<(), &str> {
            match input.parse::<u8>() {
                Ok(0..=10) => Ok(()),
                Ok(_) => Err("Too high, invalid input"),
                Err(_) => Err("This is an invalid input."),
            }
        })
        .interact_text().unwrap().parse().unwrap();
    let choices = vec![
        "Fully attracted to women/girls; being intimate with a guy seems gross.",
        "Mostly attracted to women/girls, but you could be intimate with the right guy.",
        "Marginally more attracted to women/girls than men/boys, but either would be ok.",
        "Neutral; you could be intimate with any gender or not attracted at all.",
        "Marginally more attracted to men/boys than women/girls, but either would be ok.",
        "Mostly attracted to men/boys, but you could be intimate with the right girl.",
        "Fully attracted to men/boys; being intimate with a girl seems gross.",
    ];
    let towards_who = Sexuality::from_num(
        Select::new()
            .items(&choices)
            .with_prompt("Choose the option that fits you best right now.")
            .interact()
            .unwrap()
            .try_into()
            .unwrap(),
    );

    Ok(HyInstance {
        h: how_much,
        s: towards_who,
        time: chrono::Local::now().to_rfc3339(),
    })
}
