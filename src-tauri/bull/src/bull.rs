struct Bull {

}


/// This enum is used to indicate the type of the doobie board
#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq)]
enum BType {
    NOT_NIU,    //没牛
    NIU_1,      //牛一
    NIU_2,      //牛二
    NIU_3,      //牛三
    NIU_4,      //牛四
    NIU_5,      //牛五
    NIU_6,      //牛六
    NIU_7,      //牛七
    NIU_8,      //牛八
    NIU_9,      //牛九
    NIU_NIU,    //牛牛
    SILVER_NIU, //银牛
    GOLD_NIU,   //金牛
    BOMB,       //炸弹
    SMALL_NIU,  //五小牛
}

impl BType {
    /// English string
    pub fn en_string(&self) -> &'static str {
        match self {
            NOT_NIU => "not_niu",
            NIU_1 => "niu_1",
            NIU_2 => "niu_2",
            NIU_3 => "niu_3",
            NIU_4 => "niu_4",
            NIU_5 => "niu_5",
            NIU_6 => "niu_6",
            NIU_7 => "niu_7",
            NIU_8 => "niu_8",
            NIU_9 => "niu_9",
            NIU_NIU => "niu_niu",
            SILVER_NIU => "silver_niu",
            GOLD_NIU => "gold_niu",
            BOMB => "bomb",
            SMALL_NIU => "small_niu",
        }
    }

    /// Chinese string
    pub fn string(&self) -> &'static str {
        match self {
            CType::NOT_NIU => "没牛",
            CType::NIU_1 => "牛一",
            CType::NIU_2 => "牛二",
            CType::NIU_3 => "牛三",
            CType::NIU_4 => "牛四",
            CType::NIU_5 => "牛五",
            CType::NIU_6 => "牛六",
            CType::NIU_7 => "牛七",
            CType::NIU_8 => "牛八",
            CType::NIU_9 => "牛九",
            CType::NIU_NIU => "牛牛",
            CType::SILVER_NIU => "银牛",
            CType::GOLD_NIU => "金牛",
            CType::BOMB => "炸弹",
            CType::SMALL_NIU => "五小牛",
        }
    }
}