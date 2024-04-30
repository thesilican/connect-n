use std::io;

fn main() {
    test_game(3,3,3);  
}

struct Board {
    r: usize,
    c: usize,
    n: usize,
    turn: u32,
    board: Vec<u32>, // 0-empty and 1/2-tokens (assume 1 goes first), bottom row of board is row 0, left-most col is col 0
    column_tops: Vec<usize> // index of first free slot in column (starts at 0)
}

impl Board {
    fn column_is_full(&self, col: usize) -> bool {
        if col >= self.c {
            return false;
        }
        self.column_tops[col] >= self.r
    }

    fn board_is_full(&self) -> bool {
        for i in 0..self.c {
            if !self.column_is_full(i) {
                return false;
            }
        }
        true
    }

    fn place_token(&mut self, col: usize) {
        if col >= self.c || self.column_is_full(col) {
            return;
        }
        self.board[self.column_tops[col]*self.c + col] = self.turn;
        self.column_tops[col] += 1;

        if self.turn == 1 {
            self.turn = 2;
        } else {
            self.turn = 1;
        }
    }

    // returns winner's player num (1/2), else 0
    fn detect_win(&self) -> u32 {
        //horiz
        for r in 0..self.r {
            let mut one = 0;
            let mut two = 0;
            for c in 0..self.c {
                if self.board[r*self.c + c] == 1 {
                    one += 1;
                    two = 0;
                } else if self.board[r*self.c + c] == 2 {
                    two += 1;
                    one = 0;
                } else {
                    one = 0;
                    two = 0;
                }

                if one >= self.n {
                    return 1;
                } else if two >= self.n {
                    return 2;
                }
            }
        }

        //vert
        for c in 0..self.c {
            let mut one = 0;
            let mut two = 0;
            for r in 0..self.r {
                if self.board[r*self.c + c] == 1 {
                    one += 1;
                    two = 0;
                } else if self.board[r*self.c + c] == 2 {
                    two += 1;
                    one = 0;
                } else {
                    one = 0;
                    two = 0;
                }

                if one >= self.n {
                    return 1;
                } else if two >= self.n {
                    return 2;
                }
            }
        }

        //pos diag x axis
        if 0 <= self.c as i32 - self.n as i32 {
            for c in 0..(self.c - self.n + 1) {
                let mut i = 0;
                let mut one = 0;
                let mut two = 0;
                loop {
                    if self.board[i*self.c + c+i] == 1 {
                        one += 1;
                        two = 0;
                    } else if self.board[i*self.c + c+i] == 2 {
                        two += 1;
                        one = 0;
                    } else {
                        one = 0;
                        two = 0;
                    }

                    if one >= self.n {
                        return 1;
                    } else if two >= self.n {
                        return 2;
                    }

                    i += 1;
                    if i >= self.r || c + i >= self.c {
                        break;
                    }
                }
            }
        }

        //pos diag y axis
        if 1 <= self.r as i32 - self.n as i32 {
            for r in 1..(self.r - self.n + 1) {
                let mut i = 0;
                let mut one = 0;
                let mut two = 0;
                loop {
                    if self.board[(r+i)*self.c + i] == 1 {
                        one += 1;
                        two = 0;
                    } else if self.board[(r+i)*self.c + i] == 2 {
                        two += 1;
                        one = 0;
                    } else {
                        one = 0;
                        two = 0;
                    }

                    if one >= self.n {
                        return 1;
                    } else if two >= self.n {
                        return 2;
                    }

                    i += 1;
                    if r + i >= self.r || i >= self.c {
                        break;
                    }
                }
            }
        }

        //neg diag x axis
        if self.n as i32 - 1 >= 0 {
            for c in (self.n-1)..self.c {
                let mut i = 0;
                let mut one = 0;
                let mut two = 0;
                loop {
                    if self.board[i*self.c + c-i] == 1 {
                        one += 1;
                        two = 0;
                    } else if self.board[i*self.c + c-i] == 2 {
                        two += 1;
                        one = 0;
                    } else {
                        one = 0;
                        two = 0;
                    }

                    if one >= self.n {
                        return 1;
                    } else if two >= self.n {
                        return 2;
                    }

                    i += 1;
                    if i >= self.r || (c as i32) - (i as i32) < 0 {
                        break;
                    }
                }
            }
        }

        //neg diag y axis
        if 1 <= self.r as i32 - self.n as i32 {
            for r in 1..(self.r - self.n + 1) {
                let mut i = 0;
                let mut one = 0;
                let mut two = 0;
                loop {
                    if self.board[(r+i)*self.c + (self.c - 1 - i)] == 1 {
                        one += 1;
                        two = 0;
                    } else if self.board[(r+i)*self.c + (self.c - 1 - i)] == 2 {
                        two += 1;
                        one = 0;
                    } else {
                        one = 0;
                        two = 0;
                    }

                    if one >= self.n {
                        return 1;
                    } else if two >= self.n {
                        return 2;
                    }

                    i += 1;
                    if r + i >= self.r || (self.c as i32) - 1 - (i as i32) < 0 {
                        break;
                    }
                }
            }
        }

        0
    }

    fn debug_print(&self) {
        for r in (0..self.r).rev() {
            for c in 0..self.c {
                print!("{} ", self.board[r*self.c + c]);
            }
            println!("");
        }
        println!("");
        for i in 0..self.c {
            print!("{} ", i);
        }
        println!("");
    }
}

fn init_board(row: usize, col: usize, n: usize) -> Board {
    Board {
        r: row,
        c: col,
        n: n,
        turn: 1,
        board: vec![0; col*row],
        column_tops: vec![0; col]
    }
}

fn test_game(row: usize, col: usize, n: usize) {
    let mut b = init_board(row, col, n);
    b.debug_print();
    while b.detect_win() == 0 && !b.board_is_full() {
        println!("Player {} turn: ", b.turn);

        let mut valid_index: bool = false;
        let mut c : i32 = 0;

        while !valid_index {
            let mut inp_c = String::new();
            io::stdin()
                .read_line(&mut inp_c)
                .expect("Failed to read line");

            c = inp_c.trim().parse().expect("enter a number");

            if c < 0 || c >= col as i32 {
                println!("index out of range");
            } else {
                valid_index = true;
            }
        }

        b.place_token(c as usize);
        b.debug_print();
    }
    
    if b.detect_win() != 0 {
        println!("Winner is player {}!", b.detect_win());
    } else {
        println!("Game ended in tie.");
    }  
}
