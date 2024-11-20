
pub fn get_struct_name<T>(_val: &T) -> &'static str {
    std::any::type_name::<T>().split("::").last().unwrap_or("Unknown")
}