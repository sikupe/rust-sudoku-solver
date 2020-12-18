use json;
use json::JsonValue;
use std::convert::TryInto;

#[derive(Clone, Copy)]
struct Sudoku {
    sudoku: [[u8; 9]; 9]
}

impl Sudoku {
    fn is_set(&self, x: usize, y: usize) -> bool {
        self.sudoku[y][x] > 0
    }

    fn try_value(&self, x: usize, y: usize, val: u8) -> Sudoku {
        let mut sudoku = Sudoku {
            sudoku: self.sudoku.clone()
        };
        sudoku.sudoku[y][x] = val;
        sudoku
    }

    fn is_valid(&self, x: usize, y: usize) -> bool {
        let lower_x = (x / 3) * 3;
        let lower_y = (y / 3) * 3;

        let value = self.sudoku[y][x];

        for i in lower_x..(lower_x + 3) {
            for j in lower_y..(lower_y + 3) {
                if i != x && j != y && self.sudoku[j][i] == value {
                    return false;
                }
            }
        }

        for i in 0..9 {
            if i != x && self.sudoku[y][i] == value {
                return false;
            }
        }

        for j in 0..9 {
            if j != y && self.sudoku[j][x] == value {
                return false;
            }
        }

        true
    }

    fn from_str(input: &str) -> Sudoku {
        let mut sudoku = [[0; 9]; 9];
        let lines = input.split('\n');
        for (x, line) in lines.into_iter().enumerate() {
            let numbers = line.trim().chars();
            for (y, number) in numbers.enumerate() {
                sudoku[x][y] = number.to_string().parse().expect("No valid sudoku provided")
            }
        }
        Sudoku {
            sudoku
        }
    }

    fn from_json_str(json_input: &str) -> Sudoku {
        let parsed = json::parse(json_input).unwrap();
        if let JsonValue::Array(vector) = parsed {
            let array_of_vectors: [JsonValue; 9] = vector.try_into().expect("Vec to array conversion did not work");
            let mut array_of_arrays: [[u8; 9]; 9] = [[0; 9]; 9];
            for (y, vector) in array_of_vectors.iter().enumerate() {
                if let JsonValue::Array(vector) = vector {
                    if vector.len() != 9 {
                        panic!("Json is not valid, subarrays to big!");
                    }
                    for (x, value) in vector.iter().enumerate() {
                        array_of_arrays[y][x] = value.as_u8().expect("Sudoku must contain numbers");
                    }
                } else {
                    panic!("Wrong format");
                }
            }
            return Sudoku {
                sudoku: array_of_arrays
            };
        }
        panic!("Json did not contain an array");
    }
}

impl ToString for Sudoku {
    fn to_string(&self) -> String {
        let mut out = String::new();
        for y in 0..9 {
            out.push('|');
            for x in 0..9 {
                out.push_str(self.sudoku[y][x].to_string().as_str());
                out.push('|');
            }
            out.push('\n');
        }
        out
    }
}

fn partial_solution(sudoku: Sudoku, index: usize) -> Option<Sudoku> {
    let x = index % 9;
    let y = index / 9;
    let mut solution = None;

    if index == 9 * 9 {
        solution = Some(sudoku)
    } else if sudoku.is_set(x, y) {
        solution = partial_solution(sudoku, index + 1);
    } else {
        for value in 1..10 {
            let trial = sudoku.try_value(x, y, value);
            // clear_screen();
            // println!("{}", trial.to_string());
            if trial.is_valid(x, y) {
                if let Some(result) = partial_solution(trial, index + 1) {
                    solution = Some(result);
                    break;
                }
            }
        }
    }
    solution
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}


fn main() {
    let sudoku = read_sudoku();

    let result = partial_solution(sudoku, 0);
    if let Some(res) = result {
        println!("{}", res.to_string());
    } else {
        println!("No result found");
    }
}

fn read_sudoku() -> Sudoku {
    Sudoku {
        sudoku: [[0, 0, 0, 0, 0, 0, 0, 0, 5], [1, 3, 5, 2, 0, 0, 0, 8, 0], [6, 0, 0, 1, 0, 0, 0, 0, 0], [0, 0, 0, 0, 5, 0, 0, 0, 0], [0, 4, 6, 0, 0, 0, 0, 0, 2], [0, 9, 0, 3, 0, 0, 0, 0, 6], [0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 9, 0, 4, 0, 0, 0], [9, 6, 4, 0, 0, 0, 8, 2, 1]]
    }
}
