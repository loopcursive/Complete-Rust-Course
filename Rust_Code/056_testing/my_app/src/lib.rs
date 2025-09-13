pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result , 4);
    }

    #[test]
    fn it_works_1() {
        let result = add(2, 2);
        assert_ne!(result , 5);
    }

    
    #[test]
    fn it_works_2() {
        assert!(10 == 100);
    }




}
