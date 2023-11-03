pub struct Logger {
    logs: Vec<String>,
}

impl Logger {
    pub fn log(message: &str) {
        println!("[LOG]: {message} \n");
    }

    pub fn warn(message: &str) {
        colour::yellow!("[WARN]: {} \n", message);
    }

    pub fn info(message: &str) {
        colour::dark_blue!("[INFO]: {} \n", message);
    }

    pub fn error(message: &str) {
        colour::red!("[ERROR]: {} \n", message);
    }
}
