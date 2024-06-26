pub struct Hand {
    pub id: Option<i32>,
    pub game_type: String,
    pub hand_text: String,
    // Add other relevant fields
}

impl Hand {
    pub fn new(game_type: String, hand_text: String) -> Self {
        Hand {
            id: None,
            game_type,
            hand_text,
        }
    }
}