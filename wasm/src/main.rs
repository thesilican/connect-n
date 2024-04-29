use std::io;

fn main() {
    test_game(3, 3, 3);    
}

enum CountTypes {
    H,
    V,
    PosDiag,
    NegDiag
}

struct ConnectCounts {
    row: usize,
    col: usize,
    h: Vec<Vec<u32>>,
    v: Vec<Vec<u32>>,
    pos_diag: Vec<Vec<u32>>,
    neg_diag: Vec<Vec<u32>>
}

trait SafeCountsIndexing {
    fn get(&self, count_type: CountTypes, i: i32, j: i32) -> u32;
}

impl SafeCountsIndexing for ConnectCounts {
    fn get(&self, count_type: CountTypes, i: i32, j: i32) -> u32 {
        if i < 0 || j < 0 {
            return 0;
        }
        let i_s = i as usize;
        let j_s = j as usize;

        if i_s >= self.row || j_s >= self.col {
            return 0;
        }

        match count_type {
            CountTypes::H => self.h[i_s][j_s],
            CountTypes::V => self.v[i_s][j_s],
            CountTypes::PosDiag => self.pos_diag[i_s][j_s],
            CountTypes::NegDiag => self.neg_diag[i_s][j_s]
        }
    }
}

fn init_counts(r: usize, c: usize) -> ConnectCounts {
    ConnectCounts {
        row: r,
        col: c,
        h: vec![vec![0; c]; r],
        v: vec![vec![0; c]; r],
        pos_diag: vec![vec![0; c]; r],
        neg_diag: vec![vec![0; c]; r],
    }
}

struct Board {
    r: usize,
    c: usize,
    n: u32,
    turn: u32,
    board: Vec<Vec<u32>>, // 0-empty and 1/2-tokens (assume 1 goes first), bottom row of board is row 0, left-most col is col 0
    column_tops: Vec<usize> // index of first free slot in column (starts at 0)
}

trait ConnectN {
    fn column_is_full(&self, col: usize) -> bool; // when the user is hovering over a column that's full, gray it out
    fn board_is_full(&self) -> bool; // used to determine tie
    fn place_token(&mut self, col: usize);
    fn detect_win(&self) -> u32; // returns winner's player num (1/2), else 0

    // debug functions
    fn debug_print(&self);
}

impl ConnectN for Board {
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
        self.board[self.column_tops[col]][col] = self.turn;
        self.column_tops[col] += 1;

        if self.turn == 1 {
            self.turn = 2;
        } else {
            self.turn = 1;
        }
    }

    fn detect_win(&self) -> u32 {
        let mut counts_1 = init_counts(self.r, self.c);
        let mut counts_2 = init_counts(self.r, self.c);

        for i in 0..self.r {
            for j in 0..self.c {
                let i_32 = i as i32;
                let j_32 = j as i32;

                if self.board[i][j] == 1 {
                    counts_1.h[i][j] = 1 + counts_1.get(CountTypes::H, i_32, j_32-1);
                    counts_1.v[i][j] = 1 + counts_1.get(CountTypes::V, i_32-1, j_32);
                    counts_1.pos_diag[i][j] = 1 + counts_1.get(CountTypes::PosDiag, i_32-1, j_32-1);
                    counts_1.neg_diag[i][j] = 1 + counts_1.get(CountTypes::NegDiag, i_32-1, j_32+1);

                    if counts_1.h[i][j] == self.n || counts_1.v[i][j] == self.n || counts_1.pos_diag[i][j] == self.n || counts_1.neg_diag[i][j] == self.n {
                        return 1;
                    }
                } else if self.board[i][j] == 2 {
                    counts_2.h[i][j] = 1 + counts_2.get(CountTypes::H, i_32, j_32-1);
                    counts_2.v[i][j] = 1 + counts_2.get(CountTypes::V, i_32-1, j_32);
                    counts_2.pos_diag[i][j] = 1 + counts_2.get(CountTypes::PosDiag, i_32-1, j_32-1);
                    counts_2.neg_diag[i][j] = 1 + counts_2.get(CountTypes::NegDiag, i_32-1, j_32+1);

                    if counts_2.h[i][j] == self.n || counts_2.v[i][j] == self.n || counts_2.pos_diag[i][j] == self.n || counts_2.neg_diag[i][j] == self.n {
                        return 2;
                    }
                }
            }
        }

        0
    }

    fn debug_print(&self) {
        for row in self.board.iter().rev() {
            for &element in row {
                print!("{} ", element)
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

fn init_board(row: usize, col: usize, connect_n: u32) -> Board {
    Board {
        r: row,
        c: col,
        n: connect_n,
        turn: 1,
        board: vec![vec![0; col]; row],
        column_tops: vec![0; col]
    }
}

fn test_game(row: usize, col: usize, n: u32) {
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
