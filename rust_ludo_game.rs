use rand::Rng;

const MAX_PLAYERS: usize = 4;
const PIECES_PER_PLAYER: usize = 4;
const BOARD_SIZE: i32 = 10;
const WINNING_POSITION: i32 = 0;

#[derive(Debug, Clone, Copy)]
enum PlayerColor {
    Red,
    Green,
    Yellow,
    Blue,
}

#[derive(Debug, Clone, Copy)]
enum Phase {
    Rolling,
    Moving,
    Waiting,
}

#[derive(Debug)]
struct GameState {
    player_index: usize,
    phase: Phase,
    dice_roll: i32,
}

impl GameState {
    fn new() -> Self {
        Self {
            player_index: 0,
            phase: Phase::Waiting,
            dice_roll: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    position: i32,
    at_home: bool,
    at_end: bool,
}

impl Piece {
    fn new() -> Self {
        Self {
            position: -1,
            at_home: true,
            at_end: false,
        }
    }
}

#[derive(Debug)]
struct Player {
    color: PlayerColor,
    pieces: [Piece; PIECES_PER_PLAYER],
}

impl Player {
    fn new(color: PlayerColor) -> Self {
        Self {
            color,
            pieces: [Piece::new(); PIECES_PER_PLAYER],
        }
    }

    fn can_move(&self, dice_roll: i32) -> bool {
        // Check if any piece that hasn't reached the end can move
        self.pieces.iter().any(|piece| {
            if !piece.at_end {
                if piece.position == -1 && dice_roll == 6 {
                    return true; // Can move piece out of home with a roll of 6
                } else if piece.position != -1 {
                    return true; // Can move any piece already on the board
                }
            }
            false
        })
    }

    fn move_piece(&mut self, dice_roll: i32) -> bool {
        // Move the first eligible piece based on dice roll
        for piece in self.pieces.iter_mut() {
            if piece.at_end {
                continue;
            }
            if piece.position == -1 && dice_roll == 6 {
                piece.position = 0;
                piece.at_home = false;
                return true;
            } else if piece.position != -1 {
                piece.position = (piece.position + dice_roll) % BOARD_SIZE;
                // Check if piece has reached the end
                if piece.position == WINNING_POSITION {
                    piece.at_end = true;
                    println!("Piece has reached the end and is now at final position.");
                }
                return true;
            }
        }
        false
    }

    fn has_won(&self) -> bool {
        self.pieces.iter().all(|piece| piece.at_end)
    }
}

fn roll_dice() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}

fn next_turn(game_state: &mut GameState) {
    game_state.player_index = (game_state.player_index + 1) % MAX_PLAYERS;
    game_state.phase = Phase::Waiting;
    game_state.dice_roll = 0; // Reset dice_roll when switching to the waiting phase
}

fn main() {
    let mut players = [
        Player::new(PlayerColor::Red),
        Player::new(PlayerColor::Green),
        Player::new(PlayerColor::Yellow),
        Player::new(PlayerColor::Blue),
    ];
    let mut game_state = GameState::new();

    loop {
        let current_player = &mut players[game_state.player_index];

        match game_state.phase {
            Phase::Waiting => {
                println!("Player {:?}'s turn", current_player.color);
                game_state.dice_roll = 0;
                game_state.phase = Phase::Rolling;
            }
            Phase::Rolling => {
                game_state.dice_roll = roll_dice();
                println!(
                    "Player {:?} rolled: {}",
                    current_player.color, game_state.dice_roll
                );

                // Decide if player can move based on the roll
                if current_player.can_move(game_state.dice_roll) {
                    game_state.phase = Phase::Moving;
                } else {
                    println!("Player {:?} has no moves", current_player.color);
                    next_turn(&mut game_state);
                }
            }
            Phase::Moving => {
                if current_player.move_piece(game_state.dice_roll) {
                    println!("Player {:?} moved a piece", current_player.color);

                    // Check if the current player has won
                    if current_player.has_won() {
                        println!("Player {:?} wins!", current_player.color);
                        break; // End the game
                    }
                }
                next_turn(&mut game_state);
            }
        }
    }
}

