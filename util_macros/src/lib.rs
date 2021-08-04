#[macro_export]
macro_rules! get_argument {
    ($arr:expr, $index:expr) => {
        $arr.get($index).unwrap()
    };
}

#[macro_export]
macro_rules! get_variable {
    ($variable_map:expr, $variable_name:expr) => {
        $variable_map
            .get(&$variable_name.to_string())
            .expect("that variable doesnt exist")
    };
}

#[macro_export]
macro_rules! get_argument_int_if_not_variable {
    ($arr:expr, $variable_map:expr,$index:expr) => {
        match get_argument!($arr, $index).parse::<usize>() {
            Ok(_temp) => _temp,
            Err(_) => *get_variable!($variable_map, get_argument!($arr, $index)),
        }
    };
}
