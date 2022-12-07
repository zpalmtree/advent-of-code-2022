#[derive(Debug, Copy, Clone)]
enum Move {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
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

fn parse_strategy_guide_move(val: &str) -> Move {
    match val {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _         => panic!("Invalid move!"),
    }
}

fn get_round_score(move_a: Move, result: GameResult) -> i32 {
    let game_score = result as i32;

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

        let opponent_move = split.next()
            .expect("Expected opponent move!");

        let player_move = split.next()
            .expect("Expected player move!");

        guide.push(StrategyGuideEntry{
            player_move: parse_strategy_guide_move(player_move),
            opponent_move: parse_strategy_guide_move(opponent_move),
        });
    }

    let mut scores = Vec::new();

    for strategy in guide.iter() {
        let result = play_rps(strategy.player_move, strategy.opponent_move);
        let score = get_round_score(strategy.player_move, result);

        scores.push(score);
    }

    let sum: i32 = scores.iter().sum();

    return sum.to_string();
}
