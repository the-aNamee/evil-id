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
                0 => "MS",
                1 => "TP",
                2 => "XJ",
                3 => "ZU",
                4 => "AW",
                5 => "HL",
                6 => "LT",
                7 => "UT",
                8 => "OJ",
                9 => "MI",
                10 => "TW",
                11 => "QQ",
                12 => "BN",
                13 => "SL",
                14 => "JN",
                15 => "LG",
                16 => "PJ",
                17 => "LH",
                18 => "MA",
                19 => "WB",
                20 => "CZ",
                21 => "VJ",
                22 => "NO",
                23 => "MP",
                24 => "HP",
                25 => "KX",
                26 => "UL",
                27 => "JZ",
                28 => "CI",
                29 => "TR",
                30 => "FP",
                31 => "OT",
                32 => "NN",
                33 => "IQ",
                34 => "NS",
                35 => "SV",
                36 => "JC",
                37 => "ME",
                38 => "FE",
                39 => "MK",
                40 => "SK",
                41 => "IK",
                42 => "WM",
                43 => "SD",
                44 => "QC",
                45 => "MD",
                46 => "CL",
                47 => "DN",
                48 => "CE",
                49 => "KO",
                50 => "LI",
                51 => "BB",
                52 => "AD",
                53 => "FS",
                54 => "OD",
                55 => "SC",
                56 => "RC",
                57 => "NY",
                58 => "JV",
                59 => "GI",
                60 => "VG",
                61 => "HG",
                62 => "HT",
                63 => "HB",
                64 => "MO",
                65 => "NF",
                66 => "BE",
                67 => "HY",
                68 => "IY",
                69 => "NG",
                70 => "BR",
                71 => "UX",
                72 => "ES",
                73 => "SP",
                74 => "RB",
                75 => "PU",
                76 => "GT",
                77 => "OW",
                78 => "QH",
                79 => "HV",
                80 => "LL",
                81 => "PC",
                82 => "ZS",
                83 => "IX",
                84 => "OQ",
                85 => "WO",
                86 => "EI",
                87 => "TH",
                88 => "XL",
                89 => "PV",
                90 => "WJ",
                91 => "MU",
                92 => "DY",
                93 => "YA",
                94 => "XT",
                95 => "UR",
                96 => "PX",
                97 => "FF",
                98 => "CU",
                99 => "UM",
                100 => "KL",
                101 => "BU",
                102 => "AC",
                103 => "AI",
                104 => "HE",
                105 => "YG",
                106 => "UQ",
                107 => "IA",
                108 => "IH",
                109 => "OU",
                110 => "OS",
                111 => "XD",
                112 => "UH",
                113 => "KV",
                114 => "DP",
                115 => "GH",
                116 => "AZ",
                117 => "DI",
                118 => "HH",
                119 => "TM",
                120 => "BC",
                121 => "SQ",
                122 => "IC",
                123 => "EB",
                124 => "QO",
                125 => "NR",
                126 => "CC",
                127 => "ZH",
                128 => "DV",
                129 => "TT",
                130 => "FU",
                131 => "LU",
                132 => "PB",
                133 => "RK",
                134 => "VV",
                135 => "CB",
                136 => "AJ",
                137 => "CN",
                138 => "UY",
                139 => "LJ",
                140 => "DZ",
                141 => "KG",
                142 => "FT",
                143 => "JD",
                144 => "GZ",
                145 => "MZ",
                146 => "EC",
                147 => "GN",
                148 => "QD",
                149 => "BT",
                150 => "SW",
                151 => "WF",
                152 => "UG",
                153 => "EP",
                154 => "EU",
                155 => "AL",
                156 => "SR",
                157 => "NH",
                158 => "VW",
                159 => "SX",
                160 => "WU",
                161 => "CW",
                162 => "TS",
                163 => "XU",
                164 => "ET",
                165 => "XS",
                166 => "IL",
                167 => "JU",
                168 => "SZ",
                169 => "EE",
                170 => "BO",
                171 => "GL",
                172 => "YC",
                173 => "QP",
                174 => "JL",
                175 => "GY",
                176 => "VB",
                177 => "GC",
                178 => "XV",
                179 => "LV",
                180 => "KK",
                181 => "ZB",
                182 => "HK",
                183 => "EZ",
                184 => "CS",
                185 => "IJ",
                186 => "ZO",
                187 => "GJ",
                188 => "ZJ",
                189 => "PT",
                190 => "GG",
                191 => "PL",
                192 => "NB",
                193 => "XB",
                194 => "XI",
                195 => "KC",
                196 => "UK",
                197 => "JE",
                198 => "LO",
                199 => "LF",
                200 => "XQ",
                201 => "OX",
                202 => "XW",
                203 => "SY",
                204 => "DT",
                205 => "TZ",
                206 => "MC",
                207 => "AM",
                208 => "AG",
                209 => "VH",
                210 => "DW",
                211 => "YV",
                212 => "GW",
                213 => "MF",
                214 => "WK",
                215 => "LS",
                216 => "WW",
                217 => "FJ",
                218 => "RG",
                219 => "SE",
                220 => "MW",
                221 => "WP",
                222 => "IT",
                223 => "IM",
                224 => "YI",
                225 => "KY",
                226 => "XP",
                227 => "IG",
                228 => "RW",
                229 => "YE",
                230 => "VC",
                231 => "UA",
                232 => "ZL",
                233 => "II",
                234 => "CD",
                235 => "CY",
                236 => "DQ",
                237 => "UW",
                238 => "MH",
                239 => "XZ",
                240 => "ZV",
                241 => "ND",
                242 => "BF",
                243 => "GO",
                244 => "SN",
                245 => "HN",
                246 => "MX",
                247 => "BM",
                248 => "UI",
                249 => "CX",
                250 => "MG",
                251 => "FZ",
                252 => "VM",
                253 => "XR",
                254 => "NW",
                255 => "OE",
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
        let code = code.replace("-", "").replace("_", "").to_uppercase();

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
                "MS" => 0,
                "TP" => 1,
                "XJ" => 2,
                "ZU" => 3,
                "AW" => 4,
                "HL" => 5,
                "LT" => 6,
                "UT" => 7,
                "OJ" => 8,
                "MI" => 9,
                "TW" => 10,
                "QQ" => 11,
                "BN" => 12,
                "SL" => 13,
                "JN" => 14,
                "LG" => 15,
                "PJ" => 16,
                "LH" => 17,
                "MA" => 18,
                "WB" => 19,
                "CZ" => 20,
                "VJ" => 21,
                "NO" => 22,
                "MP" => 23,
                "HP" => 24,
                "KX" => 25,
                "UL" => 26,
                "JZ" => 27,
                "CI" => 28,
                "TR" => 29,
                "FP" => 30,
                "OT" => 31,
                "NN" => 32,
                "IQ" => 33,
                "NS" => 34,
                "SV" => 35,
                "JC" => 36,
                "ME" => 37,
                "FE" => 38,
                "MK" => 39,
                "SK" => 40,
                "IK" => 41,
                "WM" => 42,
                "SD" => 43,
                "QC" => 44,
                "MD" => 45,
                "CL" => 46,
                "DN" => 47,
                "CE" => 48,
                "KO" => 49,
                "LI" => 50,
                "BB" => 51,
                "AD" => 52,
                "FS" => 53,
                "OD" => 54,
                "SC" => 55,
                "RC" => 56,
                "NY" => 57,
                "JV" => 58,
                "GI" => 59,
                "VG" => 60,
                "HG" => 61,
                "HT" => 62,
                "HB" => 63,
                "MO" => 64,
                "NF" => 65,
                "BE" => 66,
                "HY" => 67,
                "IY" => 68,
                "NG" => 69,
                "BR" => 70,
                "UX" => 71,
                "ES" => 72,
                "SP" => 73,
                "RB" => 74,
                "PU" => 75,
                "GT" => 76,
                "OW" => 77,
                "QH" => 78,
                "HV" => 79,
                "LL" => 80,
                "PC" => 81,
                "ZS" => 82,
                "IX" => 83,
                "OQ" => 84,
                "WO" => 85,
                "EI" => 86,
                "TH" => 87,
                "XL" => 88,
                "PV" => 89,
                "WJ" => 90,
                "MU" => 91,
                "DY" => 92,
                "YA" => 93,
                "XT" => 94,
                "UR" => 95,
                "PX" => 96,
                "FF" => 97,
                "CU" => 98,
                "UM" => 99,
                "KL" => 100,
                "BU" => 101,
                "AC" => 102,
                "AI" => 103,
                "HE" => 104,
                "YG" => 105,
                "UQ" => 106,
                "IA" => 107,
                "IH" => 108,
                "OU" => 109,
                "OS" => 110,
                "XD" => 111,
                "UH" => 112,
                "KV" => 113,
                "DP" => 114,
                "GH" => 115,
                "AZ" => 116,
                "DI" => 117,
                "HH" => 118,
                "TM" => 119,
                "BC" => 120,
                "SQ" => 121,
                "IC" => 122,
                "EB" => 123,
                "QO" => 124,
                "NR" => 125,
                "CC" => 126,
                "ZH" => 127,
                "DV" => 128,
                "TT" => 129,
                "FU" => 130,
                "LU" => 131,
                "PB" => 132,
                "RK" => 133,
                "VV" => 134,
                "CB" => 135,
                "AJ" => 136,
                "CN" => 137,
                "UY" => 138,
                "LJ" => 139,
                "DZ" => 140,
                "KG" => 141,
                "FT" => 142,
                "JD" => 143,
                "GZ" => 144,
                "MZ" => 145,
                "EC" => 146,
                "GN" => 147,
                "QD" => 148,
                "BT" => 149,
                "SW" => 150,
                "WF" => 151,
                "UG" => 152,
                "EP" => 153,
                "EU" => 154,
                "AL" => 155,
                "SR" => 156,
                "NH" => 157,
                "VW" => 158,
                "SX" => 159,
                "WU" => 160,
                "CW" => 161,
                "TS" => 162,
                "XU" => 163,
                "ET" => 164,
                "XS" => 165,
                "IL" => 166,
                "JU" => 167,
                "SZ" => 168,
                "EE" => 169,
                "BO" => 170,
                "GL" => 171,
                "YC" => 172,
                "QP" => 173,
                "JL" => 174,
                "GY" => 175,
                "VB" => 176,
                "GC" => 177,
                "XV" => 178,
                "LV" => 179,
                "KK" => 180,
                "ZB" => 181,
                "HK" => 182,
                "EZ" => 183,
                "CS" => 184,
                "IJ" => 185,
                "ZO" => 186,
                "GJ" => 187,
                "ZJ" => 188,
                "PT" => 189,
                "GG" => 190,
                "PL" => 191,
                "NB" => 192,
                "XB" => 193,
                "XI" => 194,
                "KC" => 195,
                "UK" => 196,
                "JE" => 197,
                "LO" => 198,
                "LF" => 199,
                "XQ" => 200,
                "OX" => 201,
                "XW" => 202,
                "SY" => 203,
                "DT" => 204,
                "TZ" => 205,
                "MC" => 206,
                "AM" => 207,
                "AG" => 208,
                "VH" => 209,
                "DW" => 210,
                "YV" => 211,
                "GW" => 212,
                "MF" => 213,
                "WK" => 214,
                "LS" => 215,
                "WW" => 216,
                "FJ" => 217,
                "RG" => 218,
                "SE" => 219,
                "MW" => 220,
                "WP" => 221,
                "IT" => 222,
                "IM" => 223,
                "YI" => 224,
                "KY" => 225,
                "XP" => 226,
                "IG" => 227,
                "RW" => 228,
                "YE" => 229,
                "VC" => 230,
                "UA" => 231,
                "ZL" => 232,
                "II" => 233,
                "CD" => 234,
                "CY" => 235,
                "DQ" => 236,
                "UW" => 237,
                "MH" => 238,
                "XZ" => 239,
                "ZV" => 240,
                "ND" => 241,
                "BF" => 242,
                "GO" => 243,
                "SN" => 244,
                "HN" => 245,
                "MX" => 246,
                "BM" => 247,
                "UI" => 248,
                "CX" => 249,
                "MG" => 250,
                "FZ" => 251,
                "VM" => 252,
                "XR" => 253,
                "NW" => 254,
                "OE" => 255,
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
