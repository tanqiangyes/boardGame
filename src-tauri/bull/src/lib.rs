pub mod bulls;

#[cfg(test)]
mod tests {
    use super::bulls::BType;
    use super::bulls::Bulls;
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
            // println!("{}", b);
            let a = &(b.clone());
            let niu: BType = a.get_niu_by_cards().into();
            println!("{}  {}", a, niu.string());
        }
    }
}
