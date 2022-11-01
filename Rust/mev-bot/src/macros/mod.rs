#[macro_export]
macro_rules! build_address {
    ($addr:tt) => {
        $addr.parse::<ethers::types::Address>().unwrap()
    };
}

#[macro_export]
macro_rules! current_time {
    () => {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    };
}

#[macro_export]
macro_rules! current_time_dur {
    () => {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
    };
}
