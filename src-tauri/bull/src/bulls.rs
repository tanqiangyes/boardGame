use anyhow::{ensure, Error, Result};
use plate::plates::{Plate, Plates};
use std::fmt;

/// max player number
const MAX_PLAYERS: usize = 10;
const PER_PLAYER_PLATE: usize = 5;
const BULL_NIU: u8 = 10;

/// This is a game plate of bull, we use one plates to deal.
#[derive(Debug, Clone, PartialEq)]
pub struct Bulls(Vec<Plate>);
impl Bulls {
    /// initialize
    pub fn new() -> Bulls {
        let plates = Plates::new().random().clone().plates;
        Bulls(plates.to_vec())
    }

    pub fn deal(&self, player: usize) -> Result<Vec<Bull>, Error> {
        ensure!(
            player <= MAX_PLAYERS,
            "max player number cannot be greater than 10"
        );
        let mut res: Vec<Bull> = Vec::with_capacity(player);

        for p in 0..player {
            let p0 = self
                .0
                .iter()
                .enumerate()
                .filter(|(i, _)| i % player == p)
                .map(|(_, plate)| plate.clone())
                .collect::<Vec<_>>();
            let mut bull: [Plate; 5] = [
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ];
            for i in 0..PER_PLAYER_PLATE {
                bull[i] = p0[i]
            }
            let b = bull.clone();
            res.push(Bull(b));
        }

        Ok(res)
    }
}

impl fmt::Display for Bulls {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut bull = String::new();

        for i in self.0.iter() {
            bull.push_str(i.pcolor.string());
            bull.push_str("  ");
            bull.push_str(i.pvalue.string());
            bull.push_str("\n");
        }

        write!(f, "{}", bull)
    }
}

/// In bull game , every bull can hold five plates, so make this.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Bull([Plate; 5]);
impl Bull {
    pub fn is_small_niu(&self) -> bool {
        self.0.clone().iter().map(|s| s.pvalue.value()).sum::<u8>() <= BULL_NIU
    }

    pub fn is_bomb(&self) -> bool {
        let mut s = self.0.clone();
        s.sort_by_key(|&p| p.pvalue.value());
        s[0].pvalue.value() == s[3].pvalue.value() || s[1].pvalue.value() == s[2].pvalue.value()
    }

    pub fn is_gold_niu(&self) -> bool {
        let mut s = self.0.clone();
        s.sort_by_key(|&p| p.pvalue.value());
        s[0].pvalue.value() > BULL_NIU
    }

    pub fn is_silver_niu(&self) -> bool {
        let mut s = self.0.clone();
        s.sort_by_key(|&p| p.pvalue.value());
        s[0].pvalue.value() == BULL_NIU && s[1].pvalue.value() > BULL_NIU
    }

    pub fn get_niu_by_cards(&self) -> u8 {
        let mut s = self.0.clone();
        s.as_mut().iter_mut().for_each(|p| {
            if p.pvalue.value() >= BULL_NIU {
                *p = Plate {
                    pcolor: p.pcolor,
                    pvalue: BULL_NIU.into(), //do this ,just for cal.
                }
            }
        });
        s.sort_by_key(|&p| p.pvalue.value());
        // println!("{:?}", s.clone());
        let mut lave = s.iter().map(|s| s.pvalue.value()).sum();
        lave = lave % BULL_NIU;
        for i in 0..4 {
            let j = i + 1;
            for k in j..5 {
                // > 10
                if (s[i].pvalue.value() + s[k].pvalue.value()) % 10 == lave {
                    if lave == 0 {
                        return 10;
                    } else {
                        return lave;
                    }
                }
            }
        }
        return 0;
    }

    pub fn get_type_by_cards(&self) -> Result<BType> {
        if self.is_small_niu() {
            return Ok(BType::SMALLNIU);
        }
        if self.is_bomb() {
            return Ok(BType::BOMB);
        }
        if self.is_gold_niu() {
            return Ok(BType::GOLDNIU);
        }
        if self.is_silver_niu() {
            return Ok(BType::SILVERNIU);
        }
        Ok(self.get_niu_by_cards().into())
    }
}

impl fmt::Display for Bull {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut bull = String::new();

        for i in self.0.iter() {
            bull.push_str(i.pcolor.string());
            bull.push_str(" ");
            bull.push_str(i.pvalue.string());
            bull.push_str("\t");
        }

        write!(f, "{}", bull)
    }
}

/// This enum is used to indicate the type of the doobie board
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum BType {
    NOTNIU,    //没牛
    NIU1,      //牛一
    NIU2,      //牛二
    NIU3,      //牛三
    NIU4,      //牛四
    NIU5,      //牛五
    NIU6,      //牛六
    NIU7,      //牛七
    NIU8,      //牛八
    NIU9,      //牛九
    NIUNIU,    //牛牛
    SILVERNIU, //银牛
    GOLDNIU,   //金牛
    BOMB,      //炸弹
    SMALLNIU,  //五小牛
}

impl BType {
    /// English string
    #[allow(dead_code)]
    pub fn en_string(&self) -> &'static str {
        match self {
            BType::NOTNIU => "not_niu",
            BType::NIU1 => "niu_1",
            BType::NIU2 => "niu_2",
            BType::NIU3 => "niu_3",
            BType::NIU4 => "niu_4",
            BType::NIU5 => "niu_5",
            BType::NIU6 => "niu_6",
            BType::NIU7 => "niu_7",
            BType::NIU8 => "niu_8",
            BType::NIU9 => "niu_9",
            BType::NIUNIU => "niu_niu",
            BType::SILVERNIU => "silver_niu",
            BType::GOLDNIU => "gold_niu",
            BType::BOMB => "bomb",
            BType::SMALLNIU => "small_niu",
        }
    }

    /// Chinese string
    #[allow(dead_code)]
    pub fn string(&self) -> &'static str {
        match self {
            BType::NOTNIU => "没牛",
            BType::NIU1 => "牛一",
            BType::NIU2 => "牛二",
            BType::NIU3 => "牛三",
            BType::NIU4 => "牛四",
            BType::NIU5 => "牛五",
            BType::NIU6 => "牛六",
            BType::NIU7 => "牛七",
            BType::NIU8 => "牛八",
            BType::NIU9 => "牛九",
            BType::NIUNIU => "牛牛",
            BType::SILVERNIU => "银牛",
            BType::GOLDNIU => "金牛",
            BType::BOMB => "炸弹",
            BType::SMALLNIU => "五小牛",
        }
    }
}

impl From<u8> for BType {
    fn from(from: u8) -> Self {
        match from {
            0u8 => BType::NOTNIU,
            1u8 => BType::NIU1,
            2u8 => BType::NIU2,
            3u8 => BType::NIU3,
            4u8 => BType::NIU4,
            5u8 => BType::NIU5,
            6u8 => BType::NIU6,
            7u8 => BType::NIU7,
            8u8 => BType::NIU8,
            9u8 => BType::NIU9,
            10u8 => BType::NIUNIU,
            11u8 => BType::SILVERNIU,
            12u8 => BType::GOLDNIU,
            13u8 => BType::BOMB,
            14u8 => BType::SMALLNIU,
            _ => panic!("error palte value {:?}", from),
        }
    }
}

impl From<BType> for u8 {
    fn from(from: BType) -> Self {
        match from {
            BType::NOTNIU => 0u8,
            BType::NIU1 => 1u8,
            BType::NIU2 => 2u8,
            BType::NIU3 => 3u8,
            BType::NIU4 => 4u8,
            BType::NIU5 => 5u8,
            BType::NIU6 => 6u8,
            BType::NIU7 => 7u8,
            BType::NIU8 => 8u8,
            BType::NIU9 => 9u8,
            BType::NIUNIU => 10u8,
            BType::SILVERNIU => 11u8,
            BType::GOLDNIU => 12u8,
            BType::BOMB => 13u8,
            BType::SMALLNIU => 14u8,
        }
    }
}
