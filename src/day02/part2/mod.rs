#[derive(Debug, Copy, Clone)]
enum Move {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Move {
    fn from_i32(value: i32) -> Move {
        match value {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Debug)]
enum GameResult {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

#[derive(Debug)]
struct StrategyGuideEntry {
    player_move: Move,

    opponent_move: Move,
}

fn parse_strategy_guide_result(val: &str) -> GameResult {
    match val {
        "X" => GameResult::Loss,
        "Y" => GameResult::Draw,
        "Z" => GameResult::Win,
        _   => panic!("Invalid result!"),
    }
}

fn parse_strategy_guide_move(val: &str) -> Move {
    match val {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _   => panic!("Invalid move!"),
    }
}

fn get_player_move(opponent_move: Move, game_result: GameResult) -> Move {
    match game_result {
        GameResult::Draw  => opponent_move,
        GameResult::Win   => Move::from_i32((opponent_move as i32 + 1) % 3),
        GameResult::Loss  => Move::from_i32((opponent_move as i32 + 2) % 3),
    }
}

fn get_round_score(move_a: Move, result: GameResult) -> u32 {
    let game_score = result as u32;

    let move_score = match move_a {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    game_score + move_score
}

fn play_rps(move_a: Move, move_b: Move) -> GameResult {
    let difference = (move_a as i32) - (move_b as i32);

    /*
    2 - 0 = 2 = loss
    1 - 2 = -1 = loss
    0 - 1 = -1 = loss

    2 - 1 = 1 = win
    1 - 0 = 1 = win
    0 - 2 = -2 = win

    2 - 2 = 0 = draw
    1 - 1 = 0 = draw
    0 - 0 = 0 = draw */
    match difference {
        -1 | 2 => GameResult::Loss,
        0      => GameResult::Draw,
        -2 | 1 => GameResult::Win,
        _      => panic!("Unexpected moves!"),
    }
}

pub fn solution(input: Vec<String>) -> String {
    let mut guide = Vec::new();

    for next_line in input.iter() {
        let mut split = next_line.split_whitespace();

        let opponent_move = parse_strategy_guide_move(split.next()
            .expect("Expected opponent move!"));

        let game_result = parse_strategy_guide_result(split.next()
            .expect("Expected game result!"));

        guide.push(StrategyGuideEntry{
            player_move: get_player_move(opponent_move, game_result),
            opponent_move: opponent_move,
        });
    }

    let mut scores = Vec::new();

    for strategy in guide.iter() {
        let result = play_rps(strategy.player_move, strategy.opponent_move);
        let score = get_round_score(strategy.player_move, result);

        scores.push(score);
    }

    let sum: u32 = scores.iter().sum();

    return sum.to_string();
}
