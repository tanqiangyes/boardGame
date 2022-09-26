/// a deck of cards, which have 52 plates, and a king and a queen.
/// there have four color, spades, hearts, plum, square, if a plate is king or queen, have no color, in order to show the king and queen, i add two type for color.
/// there have 13 kinds of cards.
/// this crate just a fundentment for plate game.
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::fmt;

/// This enum is used to indicate the suit of the board.
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Serialize, Deserialize)]
pub enum PColor {
    Square, // 方块
    Plum,   // 梅花
    Hearts, // 红心
    Spades, // 黑桃
    Queen,  // 小王
    King,   // 大王
}

impl PColor {
    /// English string
    pub fn en_string(&self) -> &'static str {
        match self {
            PColor::Plum => "plum",
            PColor::Square => "square",
            PColor::Hearts => "hearts",
            PColor::Spades => "spades",
            PColor::Queen => "queen",
            PColor::King => "king",
        }
    }

    /// Chinese string
    pub fn string(&self) -> &'static str {
        match self {
            PColor::Plum => "梅花",
            PColor::Square => "方块",
            PColor::Hearts => "红心",
            PColor::Spades => "黑桃",
            PColor::Queen => "小王",
            PColor::King => "大王",
        }
    }
}

/// This enum is used to represent the value of the card
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Serialize, Deserialize)]
pub enum PValue {
    CardA,     // A
    Card2,     // 2
    Card3,     // 3
    Card4,     // 4
    Card5,     // 5
    Card6,     // 6
    Card7,     // 7
    Card8,     // 8
    Card9,     // 9
    Card10,    // 10
    CardJ,     // J
    CardQ,     // Q
    CardK,     // K
    CardQueen, // queen
    CardKing,  // king
}

impl PValue {
    /// To string
    pub fn string(&self) -> &'static str {
        match self {
            PValue::CardA => "A",
            PValue::Card2 => "2",
            PValue::Card3 => "3",
            PValue::Card4 => "4",
            PValue::Card5 => "5",
            PValue::Card6 => "6",
            PValue::Card7 => "7",
            PValue::Card8 => "8",
            PValue::Card9 => "9",
            PValue::Card10 => "10",
            PValue::CardJ => "J",
            PValue::CardQ => "Q",
            PValue::CardK => "K",
            PValue::CardQueen => "Queen",
            PValue::CardKing => "King",
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            PValue::CardA => 1,
            PValue::Card2 => 2,
            PValue::Card3 => 3,
            PValue::Card4 => 4,
            PValue::Card5 => 5,
            PValue::Card6 => 6,
            PValue::Card7 => 7,
            PValue::Card8 => 8,
            PValue::Card9 => 9,
            PValue::Card10 => 10,
            PValue::CardJ => 11,
            PValue::CardQ => 12,
            PValue::CardK => 13,
            PValue::CardQueen => 14,
            PValue::CardKing => 15,
        }
    }
}

impl From<u8> for PValue {
    fn from(from: u8) -> Self {
        match from {
            1u8 => PValue::CardA,
            2u8 => PValue::Card2,
            3u8 => PValue::Card3,
            4u8 => PValue::Card4,
            5u8 => PValue::Card5,
            6u8 => PValue::Card6,
            7u8 => PValue::Card7,
            8u8 => PValue::Card8,
            9u8 => PValue::Card9,
            10u8 => PValue::Card10,
            11u8 => PValue::CardJ,
            12u8 => PValue::CardQ,
            13u8 => PValue::CardK,
            14u8 => PValue::CardQueen,
            15u8 => PValue::CardKing,
            _ => panic!("error palte value {:?}", from),
        }
    }
}

/// a plate is a card, which have color and value.
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Serialize, Deserialize)]
pub struct Plate {
    pub pvalue: PValue,
    pub pcolor: PColor,
}

impl Plate {
    /// initialize, just for some times need.
    pub fn new(pcolor: PColor, pvalue: PValue) -> Self {
        Plate { pcolor, pvalue }
    }

    /// a format function.
    pub fn string(&self) -> String {
        format!("{} {}", self.pcolor.string(), self.pvalue.string())
    }
}

impl Default for Plate {
    fn default() -> Self {
        Plate {
            pvalue: PValue::Card2,
            pcolor: PColor::Hearts,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Plates {
    pub plates: Vec<Plate>,
}

impl Plates {
    /// init plates
    pub fn new() -> Self {
        let mut plates: Vec<Plate> = Vec::new();
        let end: u8 = 14;
        for i in 1..end {
            plates.push(Plate {
                pcolor: PColor::Plum,
                pvalue: i.into(),
            });
            plates.push(Plate {
                pcolor: PColor::Square,
                pvalue: i.into(),
            });
            plates.push(Plate {
                pcolor: PColor::Hearts,
                pvalue: i.into(),
            });
            plates.push(Plate {
                pcolor: PColor::Spades,
                pvalue: i.into(),
            });
        }
        Plates { plates }
    }

    /// some plate game need king and queen.
    pub fn new_with_queen_king() -> Self {
        let mut plates: Vec<Plate> = Vec::new();
        plates.push(Plate {
            pcolor: PColor::Queen,
            pvalue: 14u8.into(),
        });
        plates.push(Plate {
            pcolor: PColor::King,
            pvalue: 15u8.into(),
        });
        let end: u8 = 14;
        for i in 1..end {
            plates.push(Plate {
                pcolor: PColor::Plum,
                pvalue: i.into(),
            });
            plates.push(Plate {
                pcolor: PColor::Square,
                pvalue: i.into(),
            });
            plates.push(Plate {
                pcolor: PColor::Hearts,
                pvalue: i.into(),
            });
            plates.push(Plate {
                pcolor: PColor::Spades,
                pvalue: i.into(),
            });
        }
        Plates { plates }
    }

    /// shuffle the plates.
    /// we shuffle 3 times.
    pub fn random(&mut self) -> &Plates {
        self.plates.shuffle(&mut thread_rng());
        self.plates.shuffle(&mut thread_rng());
        self.plates.shuffle(&mut thread_rng());
        self
    }

    /// because struct cannot Clone, so make a clone function.
    pub fn clone(&self) -> Self {
        Plates {
            plates: self.plates.clone(),
        }
    }
}

impl fmt::Display for Plates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut plate_str = String::new();

        for i in self.plates.to_vec().iter() {
            plate_str.push_str(i.pcolor.string());
            plate_str.push_str("  ");
            plate_str.push_str(i.pvalue.string());
            plate_str.push_str("\n");
        }

        write!(f, "{}", plate_str)
    }
}
