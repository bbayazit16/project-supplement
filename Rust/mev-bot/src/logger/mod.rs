pub mod logger {

    pub fn info(message: &str) {
        println!(
            "\x1b[32m\x1b[1mINFO \x1b[0m[{}] \x1b[1m{}\x1b[0m",
            base_time(),
            message
        );
    }

    pub fn warn(message: &str) {
        println!(
            "\x1b[33m\x1b[1mWARN \x1b[0m[{}] \x1b[1m{}\x1b[0m",
            base_time(),
            message
        );
    }

    pub fn fatal(message: &str) {
        println!(
            "\x1b[31m\x1b[1mFATAL \x1b[0m[{}] \x1b[1m{}\x1b[0m",
            base_time(),
            message
        );
        std::process::exit(1);
    }

    fn base_time() -> String {
        chrono::Local::now()
            .format("%d/%m/%y %H:%M:%S.%3f")
            .to_string()
    }
}
