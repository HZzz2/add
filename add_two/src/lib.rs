pub fn add_two(n: i32) -> i32 {
    n + 2
}

pub fn square_two(n: i32) -> i32 {
    n * n
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_square_two() {
        //测试平方
        assert_eq!(square_two(2), 4);
        assert_eq!(square_two(4), 16);
        assert_eq!(square_two(5), 255);
    }
}
