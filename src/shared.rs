use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Player {
    Opponent,
    PerfectPlayer,
}

struct Node {
    step: i8,
    score: i16,
}

#[derive(Debug, Clone)]
struct Board {
    opponent: Vec<u32>,
    perfect_player: Vec<u32>,
    first_hand: Option<bool>,
}

const BASE_SCORE: i16 = 10;

impl Board {
    fn winning_combinations() -> [[u32; 3]; 8] {
        [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
            [1, 4, 7],
            [2, 5, 8],
            [3, 6, 9],
            [1, 5, 9],
            [3, 5, 7],
        ]
    }

    fn draw(&self) -> bool {
        self.step_count() == 9
    }

    fn won(&self) -> bool {
        Self::can_win(self.perfect_player.clone())
    }

    fn lose(&self) -> bool {
        Self::can_win(self.opponent.clone())
    }

    fn can_win(selected: Vec<u32>) -> bool {
        if selected.len() < 3 {
            return false;
        }

        for combination in Self::winning_combinations().iter() {
            if combination.iter().all(|&step| selected.contains(&step)) {
                return true;
            }
        }

        false
    }

    fn step_count(&self) -> usize {
        Self::occupied_steps(&self.opponent, &self.perfect_player).len()
    }

    fn occupied_steps<'a>(opponent: &'a Vec<u32>, perfect_player: &'a Vec<u32>) -> Vec<u32> {
        let mut merged = opponent.clone();
        merged.extend(perfect_player);
        merged
    }

    fn next_step(&mut self, current_step: usize) -> i8 {
        if current_step == 0 {
            // Opening Book Optimization
            let first_steps = [1, 3, 5, 7, 9];
            let mut rng = thread_rng();
            *first_steps.choose(&mut rng).unwrap() // Always safe
        } else {
            self.minimax(0).step
        }
    }

    fn available_steps(&self) -> Vec<u32> {
        let occupied = Self::occupied_steps(&self.opponent, &self.perfect_player);
        let mut steps = vec![];
        for i in 1..=9 {
            if !occupied.contains(&i) {
                steps.push(i);
            }
        }
        steps
    }

    fn current_player(&self) -> Player {
        if (self.step_count() % 2 == 0 && self.first_hand.unwrap_or(false))
            || (self.step_count() % 2 != 0 && !self.first_hand.unwrap_or(false))
        {
            Player::PerfectPlayer
        } else {
            Player::Opponent
        }
    }

    fn try_next_step(&self, step: u32) -> Board {
        let mut board = self.clone(); // Clone the current board to keep `self` immutable
        if self.current_player() == Player::Opponent {
            board.opponent.push(step);
        } else {
            board.perfect_player.push(step);
        }
        board
    }

    fn minimax(&self, depth: u8) -> Node {
        if self.draw() {
            return Node { step: -1, score: 0 };
        }

        let mut nodes = vec![];

        for step in self.available_steps() {
            let board = self.try_next_step(step);
            if board.won() {
                return Node {
                    step: step as i8,
                    score: BASE_SCORE - depth as i16,
                };
            }

            if board.lose() {
                return Node {
                    step: step as i8,
                    score: depth as i16 - BASE_SCORE,
                };
            }

            let score = board.minimax(depth + 1).score;

            nodes.push(Node {
                step: step as i8,
                score: score,
            });
        }

        if depth % 2 == 0 {
            let max = nodes.iter().max_by_key(|node| node.score).unwrap();
            return Node {
                step: max.step,
                score: max.score,
            };
        }

        let min = nodes.iter().min_by_key(|node| node.score).unwrap();
        Node {
            step: min.step,
            score: min.score,
        }
    }
}

pub fn play(perfect_player: Vec<u32>, opponent: Vec<u32>, first_hand: bool) -> i32 {
    let current_board = Board {
        perfect_player: perfect_player,
        opponent: opponent,
        first_hand: Some(first_hand),
    };

    let count = current_board.step_count();
    let mut board = current_board.clone();
    let step = board.next_step(count);

    step as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_step_count() {
        let board = Board {
            perfect_player: vec![],
            opponent: vec![],
            first_hand: Some(true),
        };
        assert_eq!(board.step_count(), 0);

        let board2 = Board {
            perfect_player: vec![],
            opponent: vec![1],
            first_hand: None,
        };
        assert_eq!(board2.step_count(), 1);
    }

    #[test]
    fn board_has_winner() {
        let board1 = Board {
            perfect_player: vec![1, 2, 3],
            opponent: vec![],
            first_hand: None,
        };
        assert_eq!(board1.won(), true);

        let board2 = Board {
            perfect_player: vec![4, 5, 6],
            opponent: vec![],
            first_hand: None,
        };
        assert_eq!(board2.won(), true);

        let board3 = Board {
            perfect_player: vec![7, 8, 9],
            opponent: vec![],
            first_hand: None,
        };
        assert_eq!(board3.won(), true);

        let board4 = Board {
            perfect_player: vec![1, 4, 7],
            opponent: vec![],
            first_hand: None,
        };
        assert_eq!(board4.won(), true);

        let board5 = Board {
            perfect_player: vec![2, 5, 8],
            opponent: vec![],
            first_hand: None,
        };
        assert_eq!(board5.won(), true);

        let board6 = Board {
            perfect_player: vec![3, 6, 9],
            opponent: vec![],
            first_hand: None,
        };
        assert_eq!(board6.won(), true);

        let board7 = Board {
            perfect_player: vec![1, 5, 9],
            opponent: vec![],
            first_hand: None,
        };
        assert_eq!(board7.won(), true);

        let board8 = Board {
            perfect_player: vec![3, 5, 7],
            opponent: vec![],
            first_hand: None,
        };
        assert_eq!(board8.won(), true);
    }

    #[test]
    fn board_has_loser() {
        let board1 = Board {
            perfect_player: vec![],
            opponent: vec![1, 2, 3],
            first_hand: None,
        };
        assert_eq!(board1.won(), false);

        let board2 = Board {
            perfect_player: vec![],
            opponent: vec![4, 5, 6],
            first_hand: None,
        };
        assert_eq!(board2.won(), false);

        let board3 = Board {
            perfect_player: vec![],
            opponent: vec![7, 8, 9],
            first_hand: None,
        };
        assert_eq!(board3.won(), false);

        let board4 = Board {
            perfect_player: vec![],
            opponent: vec![1, 4, 7],
            first_hand: None,
        };
        assert_eq!(board4.won(), false);

        let board5 = Board {
            perfect_player: vec![],
            opponent: vec![2, 5, 8],
            first_hand: None,
        };
        assert_eq!(board5.won(), false);

        let board6 = Board {
            perfect_player: vec![],
            opponent: vec![3, 6, 9],
            first_hand: None,
        };
        assert_eq!(board6.won(), false);

        let board7 = Board {
            perfect_player: vec![],
            opponent: vec![1, 5, 9],
            first_hand: None,
        };
        assert_eq!(board7.won(), false);

        let board8 = Board {
            perfect_player: vec![],
            opponent: vec![3, 5, 7],
            first_hand: None,
        };
        assert_eq!(board8.won(), false);
    }

    #[test]
    fn board_is_draw() {
        let board1 = Board {
            perfect_player: vec![2, 5, 6, 7],
            opponent: vec![1, 3, 4, 8, 9],
            first_hand: None,
        };
        assert_eq!(board1.draw(), true);

        let board2 = Board {
            perfect_player: vec![],
            opponent: vec![],
            first_hand: Some(true),
        };
        assert_eq!(board2.draw(), false);
    }

    #[test]
    fn board_occuiped_steps() {
        let board = Board {
            perfect_player: vec![1, 2, 3],
            opponent: vec![4, 5, 6],
            first_hand: None,
        };
        assert_eq!(
            Board::occupied_steps(&board.opponent, &board.perfect_player),
            vec![4, 5, 6, 1, 2, 3]
        );
    }

    #[test]
    fn board_available_steps() {
        let board = Board {
            perfect_player: vec![1, 2, 3],
            opponent: vec![4, 5, 6],
            first_hand: None,
        };
        assert_eq!(board.available_steps(), vec![7, 8, 9]);
    }

    #[test]
    fn board_current_player() {
        let board = Board {
            perfect_player: vec![1, 2, 3],
            opponent: vec![4, 5, 6],
            first_hand: Some(true),
        };
        assert_eq!(board.current_player(), Player::PerfectPlayer);

        let board2 = Board {
            perfect_player: vec![],
            opponent: vec![],
            first_hand: None,
        };
        assert_eq!(board2.current_player(), Player::Opponent);

        let board3 = Board {
            perfect_player: vec![1, 2, 3],
            opponent: vec![4, 5, 6],
            first_hand: Some(false),
        };
        assert_eq!(board3.current_player(), Player::Opponent);

        let board4 = Board {
            perfect_player: vec![],
            opponent: vec![],
            first_hand: Some(true),
        };
        assert_eq!(board4.current_player(), Player::PerfectPlayer);
    }

    #[test]
    fn board_try_next_step() {
        let board = Board {
            perfect_player: vec![1, 2, 3],
            opponent: vec![4, 5, 6],
            first_hand: Some(true),
        };
        let new_board = board.try_next_step(7);
        assert_eq!(new_board.perfect_player, vec![1, 2, 3, 7]);
    }

    #[test]
    fn board_minimax() {
        let first_move = Board {
            perfect_player: vec![],
            opponent: vec![],
            first_hand: Some(true),
        };
        let node = first_move.minimax(0);
        assert_eq!([1, 3, 5, 7, 9].contains(&node.step), true);

        let draw = Board {
            perfect_player: vec![2, 5, 6, 7],
            opponent: vec![1, 3, 4, 8, 9],
            first_hand: None,
        };
        let node = draw.minimax(0);
        assert_eq!(node.step, -1);
    }
}
