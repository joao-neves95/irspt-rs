#[macro_export]
macro_rules! is_integer {
    () => {
        $crate::is_integer! {"The input must be an integer number."}
    };

    ($message:expr) => {
        &|a| match a.parse::<i64>() {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from($message)),
        }
    };
}

#[macro_export]
macro_rules! is_decimal {
    () => {
        $crate::is_decimal! {"The input must be a decimal number."}
    };

    ($message:expr) => {
        &|a| match a.parse::<f64>() {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from($message)),
        }
    };
}
