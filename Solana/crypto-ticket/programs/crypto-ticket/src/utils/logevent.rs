#[macro_export]
macro_rules! log_event {
    ($event_data:expr) => {{
        use base64::{prelude::BASE64_STANDARD, Engine};

        pub fn get_struct_name<T>(_val: &T) -> &'static str {
            std::any::type_name::<T>().split("::").last().unwrap_or("Unknown")
        }

        let struct_name = get_struct_name(&$event_data);
        let serialized_data = &$event_data.try_to_vec()?;
        let encoded = BASE64_STANDARD.encode(&serialized_data);

        msg!("Program event:{}:{:?}", struct_name, encoded);
    }};
}
