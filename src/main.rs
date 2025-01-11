// use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;
// use concat_arrays::concat_arrays;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use actix_web::http::header;

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

        // println!("Steps taken: {:?}", merged);
        merged
    }

    fn next_step(&mut self, current_step: usize) -> i8 {
        if current_step == 0 {
            let first_steps = [1, 3, 7, 9];
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

        // println!("Available steps: {:?}", steps);
        steps
    }

    fn current_player(&self) -> Player {
        if (self.step_count() % 2 == 0 && self.first_hand.unwrap_or(false)) || (self.step_count() % 2 != 0 && !self.first_hand.unwrap_or(false)) {
            // self.perfect_player.clone()
            Player::PerfectPlayer
        } else {
            // self.opponent.clone()
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
            return Node {
                step: -1,
                score: 0,
            };
        }

        let mut nodes = vec![];

        for step in self.available_steps() {
            let board = self.try_next_step(step);
            // println!("Try step {}: P - {:?} O - {:?}", step, board.perfect_player, board.opponent);

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

    fn apply_step(&mut self, step: u32) -> &mut Board {
        self.perfect_player.push(step);
        self
    }
}

fn main_local() {
    let current_board = Board {
        // opponent: vec![],
        perfect_player: vec![3],
        opponent: vec![1,4],
        first_hand: None,
    };

    let count = current_board.step_count();
    // println!("Total steps: {}", count);

    let mut board = current_board.clone();
    let step = board.next_step(count);

    if step == -1 {
        println!("Draw!");
    } else {
        let mut next_board = current_board.clone();
        next_board.apply_step(step as u32);

        if current_board.won() {
            println!("Winner - Computer! {:?}", next_board);
        } else if current_board.lose() {
            println!("Winner - Opponent! {:?}", next_board);
        } else {
            println!("Next step: {}", step);
        }
    }
}

#[derive(Serialize, Deserialize)]
struct InputData {
    perfect_player: Vec<u32>,
    opponent: Vec<u32>,
    first_hand: bool,
}

#[derive(Serialize)]
struct OutputData {
    step: i8,
    // status: String,
}

// Function that processes the input data
fn play(input: InputData) -> OutputData {
    println!("Input data: P - {:?} O - {:?} {:?}", input.perfect_player, input.opponent, input.first_hand);
    let current_board = Board {
        // opponent: vec![],
        opponent: input.opponent,
        perfect_player: input.perfect_player,
        first_hand: Some(input.first_hand),
    };

    let count = current_board.step_count();
    // println!("Current steps: {}", count);
    let mut board = current_board.clone();
    let step = board.next_step(count);
    println!("Next step: {}", step);

    //  main_local();

    OutputData {
        step: step,
    }
}

// API handler for processing the input data
async fn handle_play(request: web::Json<InputData>) -> impl Responder {
    let output = play(request.into_inner());
    HttpResponse::Ok().json(output) // Return JSON response
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the HTTP server
    HttpServer::new(|| {
        // App::new()
        App::new()
            // Set up CORS middleware to allow requests from any origin
            .wrap(Cors::default()
                .allowed_origin("http://localhost:3000")  // Allow requests from a specific origin (e.g., your front-end)
                .allowed_methods(vec!["GET", "POST", "OPTIONS"])  // Allowed HTTP methods
                .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])  // Allowed headers
                .max_age(3600)  // Max age for pre-flight requests
            )
            .route("/tictactoe/play", web::post().to(handle_play)) // Define the route for the API
    })
    .bind("127.0.0.1:8383")? // Bind the server to address
    .run()
    .await
}
