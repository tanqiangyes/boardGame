// pub mod outcome;
// pub mod player;
// pub mod rooms;

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    #[test]
    fn it_works() {
        for i in 0..100 {
            println!("{}", Uuid::new_v4().to_string())
        }
    }
}
