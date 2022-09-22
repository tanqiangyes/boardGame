pub mod plates;

#[cfg(test)]
mod tests {
    use super::plates::Plates;
    #[test]
    fn test_plate_no_king_random() {
        let mut plate = Plates::new();
        println!("test_plate_no_king_random");
        println!("{}", plate.random());
    }

    #[test]
    fn test_initial_no_king_plate() {
        println!("test_initial_no_king_plate");
        println!("{}", Plates::new());
    }

    #[test]
    fn test_initial_king_plate() {
        println!("test_initial_king_plate");
        println!("{}", Plates::new_with_queen_king());
    }

    #[test]
    fn test_initial_king_random() {
        let mut plate = Plates::new_with_queen_king();
        println!("test_initial_king_random");
        println!("{}", plate.random());
    }
}
