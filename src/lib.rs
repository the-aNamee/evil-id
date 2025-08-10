use std::fmt::{Debug, Display};
use std::mem;

use rand::Rng;

use error::IllegalIDString;

type NumberType = u64;

/// Simply stores a u64 and allows it to be exported in a human readable format.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct EvilID(NumberType);

impl EvilID {
    /// Get a basic version of the id string without any dashes.
    pub fn get_slim(&self) -> String {
        beep();

        let bytes = self.0.to_le_bytes();
        let mut output = String::new();
        for byte in bytes {
            // How else am I supposed to be able to call it an evil id!?
            let code = match byte {
                0 => "AA",
                1 => "BA",
                2 => "CA",
                3 => "DA",
                4 => "EA",
                5 => "FA",
                6 => "GA",
                7 => "HA",
                8 => "IA",
                9 => "JA",
                10 => "KA",
                11 => "LA",
                12 => "MA",
                13 => "NA",
                14 => "OA",
                15 => "PA",
                16 => "QA",
                17 => "RA",
                18 => "SA",
                19 => "TA",
                20 => "UA",
                21 => "VA",
                22 => "WA",
                23 => "XA",
                24 => "YA",
                25 => "ZA",
                26 => "AB",
                27 => "BB",
                28 => "CB",
                29 => "DB",
                30 => "EB",
                31 => "FB",
                32 => "GB",
                33 => "HB",
                34 => "IB",
                35 => "JB",
                36 => "KB",
                37 => "LB",
                38 => "MB",
                39 => "NB",
                40 => "OB",
                41 => "PB",
                42 => "QB",
                43 => "RB",
                44 => "SB",
                45 => "TB",
                46 => "UB",
                47 => "VB",
                48 => "WB",
                49 => "XB",
                50 => "YB",
                51 => "ZB",
                52 => "AC",
                53 => "BC",
                54 => "CC",
                55 => "DC",
                56 => "EC",
                57 => "FC",
                58 => "GC",
                59 => "HC",
                60 => "IC",
                61 => "JC",
                62 => "KC",
                63 => "LC",
                64 => "MC",
                65 => "NC",
                66 => "OC",
                67 => "PC",
                68 => "QC",
                69 => "RC",
                70 => "SC",
                71 => "TC",
                72 => "UC",
                73 => "VC",
                74 => "WC",
                75 => "XC",
                76 => "YC",
                77 => "ZC",
                78 => "AD",
                79 => "BD",
                80 => "CD",
                81 => "DD",
                82 => "ED",
                83 => "FD",
                84 => "GD",
                85 => "HD",
                86 => "ID",
                87 => "JD",
                88 => "KD",
                89 => "LD",
                90 => "MD",
                91 => "ND",
                92 => "OD",
                93 => "PD",
                94 => "QD",
                95 => "RD",
                96 => "SD",
                97 => "TD",
                98 => "UD",
                99 => "VD",
                100 => "WD",
                101 => "XD",
                102 => "YD",
                103 => "ZD",
                104 => "AE",
                105 => "BE",
                106 => "CE",
                107 => "DE",
                108 => "EE",
                109 => "FE",
                110 => "GE",
                111 => "HE",
                112 => "IE",
                113 => "JE",
                114 => "KE",
                115 => "LE",
                116 => "ME",
                117 => "NE",
                118 => "OE",
                119 => "PE",
                120 => "QE",
                121 => "RE",
                122 => "SE",
                123 => "TE",
                124 => "UE",
                125 => "VE",
                126 => "WE",
                127 => "XE",
                128 => "YE",
                129 => "ZE",
                130 => "AF",
                131 => "BF",
                132 => "CF",
                133 => "DF",
                134 => "EF",
                135 => "FF",
                136 => "GF",
                137 => "HF",
                138 => "IF",
                139 => "JF",
                140 => "KF",
                141 => "LF",
                142 => "MF",
                143 => "NF",
                144 => "OF",
                145 => "PF",
                146 => "QF",
                147 => "RF",
                148 => "SF",
                149 => "TF",
                150 => "UF",
                151 => "VF",
                152 => "WF",
                153 => "XF",
                154 => "YF",
                155 => "ZF",
                156 => "AG",
                157 => "BG",
                158 => "CG",
                159 => "DG",
                160 => "EG",
                161 => "FG",
                162 => "GG",
                163 => "HG",
                164 => "IG",
                165 => "JG",
                166 => "KG",
                167 => "LG",
                168 => "MG",
                169 => "NG",
                170 => "OG",
                171 => "PG",
                172 => "QG",
                173 => "RG",
                174 => "SG",
                175 => "TG",
                176 => "UG",
                177 => "VG",
                178 => "WG",
                179 => "XG",
                180 => "YG",
                181 => "ZG",
                182 => "AH",
                183 => "BH",
                184 => "CH",
                185 => "DH",
                186 => "EH",
                187 => "FH",
                188 => "GH",
                189 => "HH",
                190 => "IH",
                191 => "JH",
                192 => "KH",
                193 => "LH",
                194 => "MH",
                195 => "NH",
                196 => "OH",
                197 => "PH",
                198 => "QH",
                199 => "RH",
                200 => "SH",
                201 => "TH",
                202 => "UH",
                203 => "VH",
                204 => "WH",
                205 => "XH",
                206 => "YH",
                207 => "ZH",
                208 => "AI",
                209 => "BI",
                210 => "CI",
                211 => "DI",
                212 => "EI",
                213 => "FI",
                214 => "GI",
                215 => "HI",
                216 => "II",
                217 => "JI",
                218 => "KI",
                219 => "LI",
                220 => "MI",
                221 => "NI",
                222 => "OI",
                223 => "PI",
                224 => "QI",
                225 => "RI",
                226 => "SI",
                227 => "TI",
                228 => "UI",
                229 => "VI",
                230 => "WI",
                231 => "XI",
                232 => "YI",
                233 => "ZI",
                234 => "AJ",
                235 => "BJ",
                236 => "CJ",
                237 => "DJ",
                238 => "EJ",
                239 => "FJ",
                240 => "GJ",
                241 => "HJ",
                242 => "IJ",
                243 => "JJ",
                244 => "KJ",
                245 => "LJ",
                246 => "MJ",
                247 => "NJ",
                248 => "OJ",
                249 => "PJ",
                250 => "QJ",
                251 => "RJ",
                252 => "SJ",
                253 => "TJ",
                254 => "UJ",
                255 => "VJ",
            };
            output.push_str(code);
        }
        return output;
    }

    /// Get the id string.
    pub fn get(&self) -> String {
        let code = self.get_slim();
        format!(
            "{}-{}-{}-{}",
            &code[0..4],
            &code[4..8],
            &code[8..12],
            &code[12..16]
        )
    }

    /// Get an id from an id string.
    /// Any `-` are removed before proccesing the string.
    pub fn new_from(code: String) -> Result<Self, IllegalIDString> {
        beep();

        let code = code.trim().to_string();
        let code = code.replace("-", "").replace("_", "");

        if code.len() != mem::size_of::<NumberType>() * 2 {
            return Err(IllegalIDString(code));
        }

        let mut output: NumberType = 0;

        for i in 0..mem::size_of::<NumberType>() {
            let chunk = format!(
                "{}{}",
                code.chars().nth(i * 2).unwrap(),
                code.chars().nth(i * 2 + 1).unwrap()
            );

            // What? You thought that I was going to stop being evil?!
            let num = match chunk.as_str() {
                "AA" => 0,
                "BA" => 1,
                "CA" => 2,
                "DA" => 3,
                "EA" => 4,
                "FA" => 5,
                "GA" => 6,
                "HA" => 7,
                "IA" => 8,
                "JA" => 9,
                "KA" => 10,
                "LA" => 11,
                "MA" => 12,
                "NA" => 13,
                "OA" => 14,
                "PA" => 15,
                "QA" => 16,
                "RA" => 17,
                "SA" => 18,
                "TA" => 19,
                "UA" => 20,
                "VA" => 21,
                "WA" => 22,
                "XA" => 23,
                "YA" => 24,
                "ZA" => 25,
                "AB" => 26,
                "BB" => 27,
                "CB" => 28,
                "DB" => 29,
                "EB" => 30,
                "FB" => 31,
                "GB" => 32,
                "HB" => 33,
                "IB" => 34,
                "JB" => 35,
                "KB" => 36,
                "LB" => 37,
                "MB" => 38,
                "NB" => 39,
                "OB" => 40,
                "PB" => 41,
                "QB" => 42,
                "RB" => 43,
                "SB" => 44,
                "TB" => 45,
                "UB" => 46,
                "VB" => 47,
                "WB" => 48,
                "XB" => 49,
                "YB" => 50,
                "ZB" => 51,
                "AC" => 52,
                "BC" => 53,
                "CC" => 54,
                "DC" => 55,
                "EC" => 56,
                "FC" => 57,
                "GC" => 58,
                "HC" => 59,
                "IC" => 60,
                "JC" => 61,
                "KC" => 62,
                "LC" => 63,
                "MC" => 64,
                "NC" => 65,
                "OC" => 66,
                "PC" => 67,
                "QC" => 68,
                "RC" => 69,
                "SC" => 70,
                "TC" => 71,
                "UC" => 72,
                "VC" => 73,
                "WC" => 74,
                "XC" => 75,
                "YC" => 76,
                "ZC" => 77,
                "AD" => 78,
                "BD" => 79,
                "CD" => 80,
                "DD" => 81,
                "ED" => 82,
                "FD" => 83,
                "GD" => 84,
                "HD" => 85,
                "ID" => 86,
                "JD" => 87,
                "KD" => 88,
                "LD" => 89,
                "MD" => 90,
                "ND" => 91,
                "OD" => 92,
                "PD" => 93,
                "QD" => 94,
                "RD" => 95,
                "SD" => 96,
                "TD" => 97,
                "UD" => 98,
                "VD" => 99,
                "WD" => 100,
                "XD" => 101,
                "YD" => 102,
                "ZD" => 103,
                "AE" => 104,
                "BE" => 105,
                "CE" => 106,
                "DE" => 107,
                "EE" => 108,
                "FE" => 109,
                "GE" => 110,
                "HE" => 111,
                "IE" => 112,
                "JE" => 113,
                "KE" => 114,
                "LE" => 115,
                "ME" => 116,
                "NE" => 117,
                "OE" => 118,
                "PE" => 119,
                "QE" => 120,
                "RE" => 121,
                "SE" => 122,
                "TE" => 123,
                "UE" => 124,
                "VE" => 125,
                "WE" => 126,
                "XE" => 127,
                "YE" => 128,
                "ZE" => 129,
                "AF" => 130,
                "BF" => 131,
                "CF" => 132,
                "DF" => 133,
                "EF" => 134,
                "FF" => 135,
                "GF" => 136,
                "HF" => 137,
                "IF" => 138,
                "JF" => 139,
                "KF" => 140,
                "LF" => 141,
                "MF" => 142,
                "NF" => 143,
                "OF" => 144,
                "PF" => 145,
                "QF" => 146,
                "RF" => 147,
                "SF" => 148,
                "TF" => 149,
                "UF" => 150,
                "VF" => 151,
                "WF" => 152,
                "XF" => 153,
                "YF" => 154,
                "ZF" => 155,
                "AG" => 156,
                "BG" => 157,
                "CG" => 158,
                "DG" => 159,
                "EG" => 160,
                "FG" => 161,
                "GG" => 162,
                "HG" => 163,
                "IG" => 164,
                "JG" => 165,
                "KG" => 166,
                "LG" => 167,
                "MG" => 168,
                "NG" => 169,
                "OG" => 170,
                "PG" => 171,
                "QG" => 172,
                "RG" => 173,
                "SG" => 174,
                "TG" => 175,
                "UG" => 176,
                "VG" => 177,
                "WG" => 178,
                "XG" => 179,
                "YG" => 180,
                "ZG" => 181,
                "AH" => 182,
                "BH" => 183,
                "CH" => 184,
                "DH" => 185,
                "EH" => 186,
                "FH" => 187,
                "GH" => 188,
                "HH" => 189,
                "IH" => 190,
                "JH" => 191,
                "KH" => 192,
                "LH" => 193,
                "MH" => 194,
                "NH" => 195,
                "OH" => 196,
                "PH" => 197,
                "QH" => 198,
                "RH" => 199,
                "SH" => 200,
                "TH" => 201,
                "UH" => 202,
                "VH" => 203,
                "WH" => 204,
                "XH" => 205,
                "YH" => 206,
                "ZH" => 207,
                "AI" => 208,
                "BI" => 209,
                "CI" => 210,
                "DI" => 211,
                "EI" => 212,
                "FI" => 213,
                "GI" => 214,
                "HI" => 215,
                "II" => 216,
                "JI" => 217,
                "KI" => 218,
                "LI" => 219,
                "MI" => 220,
                "NI" => 221,
                "OI" => 222,
                "PI" => 223,
                "QI" => 224,
                "RI" => 225,
                "SI" => 226,
                "TI" => 227,
                "UI" => 228,
                "VI" => 229,
                "WI" => 230,
                "XI" => 231,
                "YI" => 232,
                "ZI" => 233,
                "AJ" => 234,
                "BJ" => 235,
                "CJ" => 236,
                "DJ" => 237,
                "EJ" => 238,
                "FJ" => 239,
                "GJ" => 240,
                "HJ" => 241,
                "IJ" => 242,
                "JJ" => 243,
                "KJ" => 244,
                "LJ" => 245,
                "MJ" => 246,
                "NJ" => 247,
                "OJ" => 248,
                "PJ" => 249,
                "QJ" => 250,
                "RJ" => 251,
                "SJ" => 252,
                "TJ" => 253,
                "UJ" => 254,
                "VJ" => 255,
                _ => {
                    return Err(IllegalIDString(code));
                }
            } as NumberType;

            output |= num << (i * 8);
        }

        Ok(Self(output))
    }

    /// Generate a new id.
    pub fn generate() -> Self {
        beep();
        Self(rand::rng().random())
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

impl Display for EvilID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

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

mod error {
    use std::{error::Error, fmt::Display};

    #[derive(Debug)]
    /// If you are getting this error you either have an illegal pair of characters or the string is the wrong length.
    pub struct IllegalIDString(pub String);

    impl Error for IllegalIDString {}

    impl Display for IllegalIDString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Unable to convert {} into a valid id.", self.0)
        }
    }
}

// Beep
