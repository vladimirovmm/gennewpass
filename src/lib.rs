use anyhow::{bail, Error};
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const FORMAT_0: &str = "1234567890";
const FORMAT_A: &str = "abcdefghijkmnopqrstuvwxyz";
const FORMAT_AA: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ";
const FORMAT_S: &str = "-_";

#[derive(Default, Debug, Clone)]
pub enum CharacterVariants {
    F0AaS,

    #[default]
    F16,
}

impl CharacterVariants {
    pub fn gen(&self) -> String {
        match self {
            CharacterVariants::F0AaS => {
                gen(&format!("{FORMAT_0}{FORMAT_A}{FORMAT_AA}{FORMAT_S}"), 8)
            }
            CharacterVariants::F16 => {
                let mask = format!("{FORMAT_A}{FORMAT_AA}");

                { 0..4 }
                    .map(|_| gen(&mask, 4))
                    .collect::<Vec<String>>()
                    .join("-")
            }
        }
    }
}

impl FromStr for CharacterVariants {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let variant = match s {
            "0Aas" => CharacterVariants::F0AaS,
            "16" => CharacterVariants::F16,
            _ => bail!("Неверный формат"),
        };

        Ok(variant)
    }
}

impl Display for CharacterVariants {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CharacterVariants::F0AaS => "0Aas",
            CharacterVariants::F16 => "16",
        };
        write!(f, "{s}")
    }
}

fn gen(mask: &str, len: usize) -> String {
    let mask_len = mask.len() - 1;

    { 0..len }
        .filter_map(|_| {
            let rand_char_num: usize = rand::thread_rng().gen_range(0..mask_len);
            mask.chars().nth(rand_char_num)
        })
        .collect::<String>()
}
