pub mod plate;

#[cfg(test)]
mod tests {
    use super::plate::Plates;
    #[test]
    fn test_plate_random() {
        let mut plate = Plates::new();
        println!("test_plate_random");
        println!("{}", plate.random());
    }

    #[test]
    fn test_initial_plate() {
        println!("test_initial_plate");
        println!("{}", Plates::new());
    }
}
