pub mod bulls;

#[cfg(test)]
mod tests {
    use super::bulls::BType;
    use super::bulls::Bulls;
    use crate::bulls::Bull;
    #[test]
    fn test_bull_init() {
        println!("test_bull_init");
        println!("{}", Bulls::new())
    }

    #[test]
    fn test_bull_deal() {
        println!("test_bull_deal");
        let bulls = Bulls::new();
        for b in bulls.deal(10).unwrap() {
            println!("{}", b)
        }
    }

    #[test]
    fn test_bull_card_type() {
        println!("test_bull_card_type");
        let bulls = Bulls::new();
        for b in &bulls.deal(10).unwrap() {
            let niu: BType = b.get_niu_by_cards().into();
            println!("{}  {}", b, niu.string());
        }
    }

    #[test]
    fn test_bull_compare() {
        println!("test_bull_compare");
        for _i in 0..50 {
            let bulls = Bulls::new();
            let plate = bulls.deal(10).unwrap();

            let niu1: BType = (&plate[0]).get_niu_by_cards().into();
            let niu2: BType = (&plate[1]).get_niu_by_cards().into();
            if niu1 == niu2 && niu1 >= BType::NIU8 {
                println!("equal+++++++++++++++++++++++++++++++++")
            }
            println!(
                "p1:{} niu:{}  p2:{} niu:{}  p1 > p2:{}",
                &plate[0],
                niu1,
                &plate[1],
                niu2,
                Bull::compare(&plate[0], &plate[1]).unwrap()
            )
        }
    }
}
