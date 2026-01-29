pub struct AppState {
    pub messages: Vec<String>,
    pub input: String,
    pub hp: i32,
    pub max_hp: i32,
    pub stamina: i32,
    pub max_stamina: i32,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            messages: vec![
                "Welcome to DOGMUD!".to_string(),
                "Type commands and press Enter.".to_string(),
                "Press 'q' to quit.".to_string(),
            ],
            input: String::new(),
            hp: 100,
            max_hp: 100,
            stamina: 100,
            max_stamina: 100,
        }
    }
}
