use std::str::FromStr;

#[macro_export]
macro_rules! is_integer {
    () => {
        $crate::is_integer! {"The input must be an integer number."}
    };

    ($message:expr) => {
        &|a| crate::validators::validate_number::<i64>(a, $message)
    };
}

#[macro_export]
macro_rules! is_decimal {
    () => {
        $crate::is_decimal! {"The input must be a decimal number."}
    };

    ($message:expr) => {
        &|a| crate::validators::validate_number::<f64>(a, $message)
    };
}

pub fn validate_number<T>(input: &str, message: &str) -> Result<(), String>
where
    T: FromStr,
{
    match input.parse::<T>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from(message)),
    }
}

#[cfg(test)]
mod tests {
    use crate::validators::validate_number;

    #[test]
    fn is_integer_passes() {
        assert!(validate_number::<i64>("1", "").is_ok());
        assert!(validate_number::<i64>("857", "").is_ok());
        assert!(validate_number::<i64>("4984579", "").is_ok());
    }

    #[test]
    fn is_integer_fails() {
        assert!(validate_number::<i64>("1.0", "").is_err());
        assert!(validate_number::<i64>("857.69", "").is_err());
        assert!(validate_number::<i64>("dg9f78f", "").is_err());
    }

    #[test]
    fn is_decimal_passes() {
        assert!(validate_number::<f64>("1", "").is_ok());
        assert!(validate_number::<f64>("1.0", "").is_ok());
        assert!(validate_number::<f64>("857", "").is_ok());
        assert!(validate_number::<f64>("857.69", "").is_ok());
        assert!(validate_number::<f64>("4984579.457", "").is_ok());
    }

    #[test]
    fn is_decimal_fails() {
        assert!(validate_number::<f64>("a", "").is_err());
        assert!(validate_number::<f64>("987dfyf", "").is_err());
    }
}
