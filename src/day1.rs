///
///```
///use starter::day1::add_two;
///
///let x = 1;
///assert_eq!(x + 2, add_two(x));
///```
///
pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }

    #[test]
    fn it_should_add_two() {
        assert_eq!(4, add_two(2));
    }
}

