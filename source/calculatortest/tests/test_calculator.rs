#[path = "../src/calculator.rs"] mod calculator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers_with_success() {
        let expected = 2;
        let result = calculator::add(1, 1);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_add_negative_numvers_with_success() {
        let expected = -10;
        let result = calculator::add(-5, -5);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_sub_positive_numvers_with_success() {
        let expected = 5;
        let result = calculator::sub(10, 5);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_sub_negative_numvers_with_success() {
        let expected = 10;
        let result = calculator::sub(-5, -15);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_divide_positive_numvers_with_success() {
        let expected = 5;
        let result = calculator::divide(10, 2);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_divide_negative_numvers_with_success() {
        let expected = -10;
        let result = calculator::divide(-20, 2);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_multiply_positive_numvers_with_success() {
        let expected = 20;
        let result = calculator::multiply(10, 2);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_multiply_negative_numvers_with_success() {
        let expected = 6;
        let result = calculator::multiply(-2, -3);

        assert_eq!(expected, result);
    }
}