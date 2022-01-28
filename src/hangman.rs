// use std::io;

static HANGMAN_STAGES: Vec<&str> = vec![
  "+---+\n|   |\n    |\n    |\n    |\n    |\n=========",
  "+---+|   |O   |    |    |    |=========",
  "+---+|   |O   ||   |    |    |=========",
  "+---+|   |O   |/|   |    |    |=========",
  "+---+|   |O   |/|\\  |    |    |=========",
  "+---+|   |O   |/|\\  |/    |    |=========",
  "+---+|   |O   |/|\\  |/ \\  |    |=========",
];

pub mod game {
  struct GameState {
    guesses: u8,
    incorrect: Vec<String>,
    correct: Vec<String>,
  }
  impl GameState {
    fn init() {
      println!("init gamestate");
    }
  }
  const game_state: GameState = GameState {
    guesses: 0,
    incorrect: vec![],
    correct: vec![],
  };

  pub fn start_game() {
    println!("Game started!");
  }
}
