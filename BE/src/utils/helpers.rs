use std::env;


pub fn get_struct_name<T>(_val: &T) -> &'static str {
    std::any::type_name::<T>().split("::").last().unwrap_or("Unknown")
}

pub fn get_env_var(key: &str) -> String {
    env::var(key).expect(&format!("{} must be set", key))
}