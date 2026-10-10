pub struct Config {
    pub title: String,
    pub height: u32,
    pub width: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: String::from("Rorka App"),
            height: 600,
            width: 800,
        }
    }
}
