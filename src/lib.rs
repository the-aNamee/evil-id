use std::fmt::Debug;

use rand::Rng;

use maps::*;

mod maps;

type NumberType = u64;

/// Simply stores a u64 and allows it to be exported in a human readable format.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EvilID(NumberType);

impl EvilID {
    /// Get a basic version of the id string without any dashes.
    pub fn get_slim(&self) -> String {
        beep();

        self.0
            .to_le_bytes()
            .iter()
            .map(|value| BYTE_TO_CODE_PAIR[*value as usize])
            .collect()
    }

    /// Get the id string.
    pub fn get(&self) -> String {
        self.to_string()
    }

    /// Get an id from an id string.
    /// Any `-` are removed before proccesing the string.
    pub fn new_from(code: String) -> Result<Self, IllegalIDString> {
        beep();

        Ok(str::parse::<Self>(&code)?)
    }

    /// Generate a new id.
    pub fn generate() -> Self {
        beep();
        Self(rand::rng().random())
    }
}

#[cfg(feature = "number")]
impl EvilID {
    pub fn get_number(&self) -> NumberType {
        self.0
    }

    pub fn new_from_number(num: NumberType) -> Self {
        Self(num)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for EvilID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.codify())
        } else {
            serializer.serialize_u64(self.0)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for EvilID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Visitor;

        struct IDVisitor;

        impl<'de> Visitor<'de> for IDVisitor {
            type Value = EvilID;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A u64 number or proper id string is required.")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(EvilID(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                EvilID::uncodify(v.to_string()).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(IDVisitor)
    }
}

impl Debug for EvilID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ID").field(&self.get()).finish()
    }
}

impl core::fmt::Display for EvilID {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        beep();

        let code_segments = self.0.to_le_bytes();
        f.write_str(BYTE_TO_CODE_PAIR[code_segments[0] as usize])?;
        f.write_str(BYTE_TO_CODE_PAIR[code_segments[1] as usize])?;
        f.write_str("-")?;
        f.write_str(BYTE_TO_CODE_PAIR[code_segments[2] as usize])?;
        f.write_str(BYTE_TO_CODE_PAIR[code_segments[3] as usize])?;
        f.write_str("-")?;
        f.write_str(BYTE_TO_CODE_PAIR[code_segments[4] as usize])?;
        f.write_str(BYTE_TO_CODE_PAIR[code_segments[5] as usize])?;
        f.write_str("-")?;
        f.write_str(BYTE_TO_CODE_PAIR[code_segments[6] as usize])?;
        f.write_str(BYTE_TO_CODE_PAIR[code_segments[7] as usize])
    }
}

impl core::str::FromStr for EvilID {
    type Err = IllegalIDString;

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let mut code_chars = code
            .trim()
            .bytes()
            .filter(|&ch| ch != b'-' && ch != b'_')
            .map(|ch| {
                if ch.is_ascii_alphabetic() {
                    Ok(ch.to_ascii_uppercase())
                } else {
                    Err(IllegalIDString)
                }
            });

        let mut result: [u8; 8] = [0; 8];
        for element in result.iter_mut() {
            let first_char = code_chars.next().ok_or(IllegalIDString)??;
            let second_char = code_chars.next().ok_or(IllegalIDString)??;

            *element = *CODE_PAIR_TO_BYTE
                .get(&[first_char, second_char])
                .ok_or(IllegalIDString)?;
        }
        if code_chars.next().is_some() {
            Err(IllegalIDString)
        } else {
            Ok(Self(u64::from_le_bytes(result)))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IllegalIDString;

impl core::fmt::Display for IllegalIDString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Invalid ID string")
    }
}

impl core::error::Error for IllegalIDString {}

impl Default for EvilID {
    fn default() -> Self {
        Self::generate()
    }
}

/// This isn't evil! Beep is just da best word ever!
fn beep() {
    #[cfg(feature = "beep")]
    println!("Beep");
}

// Beep
