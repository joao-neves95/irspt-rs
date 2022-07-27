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
