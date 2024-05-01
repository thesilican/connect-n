use std::io;
use std::cmp::max;
use std::cmp::min;
use std::time::Instant;

fn main() {
    test_bot(6,7,4);
}

struct TripletPrioQ {
    v: Vec<(usize,usize,usize,usize)>
}

impl TripletPrioQ {
    fn sortDecreasing(&mut self) {
        self.v.sort_unstable_by(|a, b| {
            if a.0 != b.0 {
                // If the first numbers are different, sort by them
                b.0.cmp(&a.0) // Sort in descending order
            } else if a.1 != b.1 {
                // If the first numbers are equal, but the second numbers are different
                b.1.cmp(&a.1) // Sort in descending order
            } else {
                // If the first and second numbers are equal, compare the third numbers
                b.2.cmp(&a.2) // Sort in descending order
            }
        });
    }
}

struct Board {
    r: usize,
    c: usize,
    n: usize,
    turn: u32,
    prev_move: usize, // column of last move
    total_placed: u32,
    bot_turn_no: u32,
    board: Vec<u32>, // 0-empty and 1/2-tokens (assume 1 goes first), bottom row of board is row 0, left-most col is col 0
    column_tops: Vec<usize> // index of first free slot in column (starts at 0)
}

impl Board {
    fn column_is_full(&self, c: usize) -> bool {
        // if c >= self.c {
        //     return false;
        // }
        self.column_tops[c] >= self.r
    }

    fn board_is_full(&self) -> bool {
        self.total_placed == (self.r as u32) * (self.c as u32)
    }

    fn place_token(&mut self, c: usize) {
        // if c >= self.c || self.column_is_full(c) {
        //     return;
        // }
        self.board[self.column_tops[c]*self.c + c] = self.turn;
        self.column_tops[c] += 1;

        self.turn = 3-self.turn; // next player's turn
        self.prev_move = c;
        self.total_placed += 1;
    }

    // returns length of largest sequence stemming from last placed token (<= n).
    // only considers sequences that in the future may extend to length n.
    // r,c is last placed token, by player.
    // make it return count of largest sequence e.g. 2 sequences of length 3 -> (3,2).
    // use length and count only when length >= 2 (otherwise ignore return pair).
    fn detect_sequence(&self, r: usize, c: usize, player: u32) -> (usize, usize) {
        let mut ans = 0;
        let mut ans_count = 0;

        let mut ans_cand = 0;
        let mut buffer = 0;

        // horiz
        let mut i = 1;
        let mut b = 0;
        loop {
            if c+i >= self.c || self.board[r*self.c + c+i] == 3-player { //3-player is the other player
                break;
            }

            if self.board[r*self.c + c+i] == player && b == 0 {
                ans_cand += 1;
                if ans_cand >= self.n {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= self.n {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        i = 1;
        b = 0;
        loop {
            if (c as i32) - (i as i32) < 0 || self.board[r*self.c + c-i] == 3-player { //3-player is the other player
                break;
            }

            if self.board[r*self.c + c-i] == player && b == 0 {
                ans_cand += 1;
                if ans_cand >= self.n {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= self.n {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        ans_cand += 1;
        if ans_cand + buffer >= self.n {
            ans = ans_cand;
            ans_count = 1;
        }

        buffer = 0;
        ans_cand = 0;
        i = 1;
        b = 0;

        //vert
        loop {
            if r+i >= self.r || self.board[(r+i)*self.c + c] == 3-player { //3-player is the other player
                break;
            }

            if self.board[(r+i)*self.c + c] == player && b == 0 {
                ans_cand += 1;
                if ans_cand >= self.n {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= self.n {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        i = 1;
        b = 0;
        loop {
            if (r as i32) - (i as i32) < 0 || self.board[(r-i)*self.c + c] == 3-player { //3-player is the other player
                break;
            }

            if self.board[(r-i)*self.c + c] == player && b == 0 {
                ans_cand += 1;
                if ans_cand >= self.n {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= self.n {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        ans_cand += 1;
        if ans_cand + buffer >= self.n {
            if ans_cand == ans {
                ans_count += 1;
            } else if ans_cand > ans {
                ans = ans_cand;
                ans_count = 1;
            }
        }

        buffer = 0;
        ans_cand = 0;
        i = 1;
        b = 0;

        //pos diag
        loop {
            if r+i >= self.r || c+i >= self.c || self.board[(r+i)*self.c + c+i] == 3-player { //3-player is the other player
                break;
            }

            if self.board[(r+i)*self.c + c+i] == player && b == 0 {
                ans_cand += 1;
                if ans_cand >= self.n {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= self.n {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        i = 1;
        b = 0;
        loop {
            if (r as i32) - (i as i32) < 0 || (c as i32) - (i as i32) < 0 || self.board[(r-i)*self.c + c-i] == 3-player { //3-player is the other player
                break;
            }

            if self.board[(r-i)*self.c + c-i] == player && b == 0 {
                ans_cand += 1;
                if ans_cand >= self.n {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= self.n {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        ans_cand += 1;
        if ans_cand + buffer >= self.n {
            if ans_cand == ans {
                ans_count += 1;
            } else if ans_cand > ans {
                ans = ans_cand;
                ans_count = 1;
            }
        }

        buffer = 0;
        ans_cand = 0;
        i = 1;
        b = 0;

        //neg diag
        loop {
            if r+i >= self.r || (c as i32) - (i as i32) < 0 || self.board[(r+i)*self.c + c-i] == 3-player { //3-player is the other player
                break;
            }

            if self.board[(r+i)*self.c + c-i] == player && b == 0 {
                ans_cand += 1;
                if ans_cand >= self.n {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= self.n {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        i = 1;
        b = 0;
        loop {
            if (r as i32) - (i as i32) < 0 || c+i >= self.c || self.board[(r-i)*self.c + c+i] == 3-player { //3-player is the other player
                break;
            }

            if self.board[(r-i)*self.c + c+i] == player && b == 0 {
                ans_cand += 1;
                if ans_cand >= self.n {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= self.n {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        ans_cand += 1;
        if ans_cand + buffer >= self.n {
            if ans_cand == ans {
                ans_count += 1;
            } else if ans_cand > ans {
                ans = ans_cand;
                ans_count = 1;
            }
        }

        (ans, ans_count)
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

    // if no win/tie/loss, check length/count of longest sequence with current move, then
    // use matrix of weights (each cell represents number of possible sequences containing that square)
    fn evaluate(&self, seq_pair: (usize, usize)) -> i32 {
        //TODO: generate connectivity table based on (r, c, n)
        static CONNECTIVITY_TABLE : [i32; 42] =    [3, 4, 5, 7, 5, 4, 3, 
                                                    4, 6, 8, 10, 8, 6, 4,
                                                    5, 8, 11, 13, 11, 8, 5, 
                                                    5, 8, 11, 13, 11, 8, 5,
                                                    4, 6, 8, 10, 8, 6, 4,
                                                    3, 4, 5, 7, 5, 4, 3];

        // 20 points for seq len 2, + 4 for each additional seq len 2.
        // this allows minimum 4 seqs len n to overtake 1 seq len n+1.

        // this should be seq length score bot - seq length score user. and seq length score = max of seq length
        // score of each column's topmost token, if it's the right player's.
        // same with con table, you can't just look at prev move, you have to do con of bot - con of player.
        if seq_pair.0 >= 2 {
            if 3-self.turn == self.bot_turn_no {
                return CONNECTIVITY_TABLE[(self.column_tops[self.prev_move]-1)*self.c + self.prev_move] + (seq_pair.0 as i32)*10 + 4*(seq_pair.1 as i32 - 1);
            } else {
                return -(CONNECTIVITY_TABLE[(self.column_tops[self.prev_move]-1)*self.c + self.prev_move] + (seq_pair.0 as i32)*10 + 4*(seq_pair.1 as i32 - 1));
            }
        }

        0
    }

    // undos last turn, which was on column c
    fn undo_turn(&mut self, c: usize) {
        // revert struct fields
        self.turn = 3-self.turn;
        self.total_placed -= 1;
        self.board[(self.column_tops[c]-1)*self.c + c] = 0;
        self.column_tops[c] -= 1;
    }

    fn minimax_driver(&mut self, depth: u32, mut alpha: i32, beta: i32) -> usize {
        let mut bestVal = -9999;
        let mut col = 0;
        static MOVE_ORDERING : [usize; 7] = [3,2,4,1,5,0,6];

        let mut final_order = TripletPrioQ {
            v: Vec::new()
        };
        
        let mut center_heuristic : i32 = 6;
        for i in MOVE_ORDERING {
            if self.column_is_full(i) {
                continue;
            }

            self.place_token(i);
            let p = self.detect_sequence(self.column_tops[i]-1, i, 3-self.turn);
            if p.0 >= self.n {
                self.undo_turn(i);
                return i;
            }
            if self.board_is_full() {
                self.undo_turn(i);
                return i;
            }
            final_order.v.push((p.0, p.1, center_heuristic as usize, i));
            center_heuristic -= 1;

            self.undo_turn(i);
        }
        final_order.sortDecreasing();

        for i in &final_order.v {
            self.place_token(i.3);

            let mut skip_move = false;
            for j in MOVE_ORDERING {
                if self.column_is_full(j) {
                    continue;
                }
                self.place_token(j);
                if self.detect_sequence(self.column_tops[j]-1, j, 3-self.turn).0 >= self.n {
                    skip_move = true;
                    self.undo_turn(j);
                    break;
                }
                self.undo_turn(j);
            }
            if skip_move {
                self.undo_turn(i.3);
                continue;
            }

            self.prev_move = i.3;
            let value = self.minimax(depth-1, false, alpha, beta);
            if value > bestVal {
                col = i.3;
                bestVal = value;
            }
            alpha = max(alpha, bestVal);

            self.undo_turn(i.3);
            
            if beta <= alpha {
                break;
            }
        }

        return col;
    }

    fn minimax(&mut self, depth: u32, isMaximizer: bool, mut alpha: i32, mut beta: i32) -> i32 {
        if depth == 0 {           
            return 0;
        }

        static MOVE_ORDERING : [usize; 7] = [3,2,4,1,5,0,6];
        if isMaximizer {
            let mut final_order = TripletPrioQ {
                v: Vec::new()
            };
            
            let mut center_heuristic : i32 = 6;
            for i in MOVE_ORDERING {
                if self.column_is_full(i) {
                    continue;
                }

                self.place_token(i);
                let p = self.detect_sequence(self.column_tops[i]-1, i, 3-self.turn);
                if p.0 >= self.n {
                    self.undo_turn(i);
                    return 100;
                }
                if self.board_is_full() {
                    self.undo_turn(i);
                    return 0;
                }
                final_order.v.push((p.0, p.1, center_heuristic as usize, i));
                center_heuristic -= 1;

                self.undo_turn(i);
            }
            final_order.sortDecreasing();

            let mut bestVal = -9999;
            for i in &final_order.v {
                self.place_token(i.3);

                let mut skip_move = false;
                for j in MOVE_ORDERING {
                    if self.column_is_full(j) {
                        continue;
                    }
                    self.place_token(j);
                    if self.detect_sequence(self.column_tops[j]-1, j, 3-self.turn).0 >= self.n {
                        skip_move = true;
                        self.undo_turn(j);
                        break;
                    }
                    self.undo_turn(j);
                }
                if skip_move {
                    self.undo_turn(i.3);
                    continue;
                }

                self.prev_move = i.3;
                bestVal = max(bestVal, self.minimax(depth-1, false, alpha, beta));
                alpha = max(alpha, bestVal);

                self.undo_turn(i.3);
                
                if beta <= alpha {
                    break;
                }
            }

            return bestVal;
        } else {
            let mut final_order = TripletPrioQ {
                v: Vec::new()
            };
            
            let mut center_heuristic : i32 = 6;
            for i in MOVE_ORDERING {
                if self.column_is_full(i) {
                    continue;
                }

                self.place_token(i);
                let p = self.detect_sequence(self.column_tops[i]-1, i, 3-self.turn);
                if p.0 >= self.n {
                    self.undo_turn(i);
                    return -100;
                }
                if self.board_is_full() {
                    self.undo_turn(i);
                    return 0;
                }
                final_order.v.push((p.0, p.1, center_heuristic as usize, i));
                center_heuristic -= 1;

                self.undo_turn(i);
            }
            final_order.sortDecreasing();

            let mut bestVal = 9999;
            for i in &final_order.v {
                self.place_token(i.3);

                let mut skip_move = false;
                for j in MOVE_ORDERING {
                    if self.column_is_full(j) {
                        continue;
                    }
                    self.place_token(j);
                    if self.detect_sequence(self.column_tops[j]-1, j, 3-self.turn).0 >= self.n {
                        skip_move = true;
                        self.undo_turn(j);
                        break;
                    }
                    self.undo_turn(j);
                }
                if skip_move {
                    self.undo_turn(i.3);
                    continue;
                }


                self.prev_move = i.3;
                bestVal = min(bestVal, self.minimax(depth-1, true, alpha, beta));
                beta = min(beta, bestVal);

                self.undo_turn(i.3);
                
                if beta <= alpha {
                    break;
                }
            }

            return bestVal;
        }
    }

    fn debug_print(&self) {
        for r in (0..self.r).rev() {
            print!("{} ",r);
            for c in 0..self.c {
                let mut ch = ' ';
                if self.board[r*self.c + c] == 0 {
                    ch = ' ';
                } else if self.board[r*self.c + c] == 1 {
                    ch = 'X';
                } else {
                    ch = 'O'
                }
                print!("{} ", ch);
            }
            println!("");
        }
        print!("  ");
        for i in 0..self.c {
            print!("{} ", i);
        }
        println!("");
    }
}

fn init_board(r: usize, c: usize, n: usize) -> Board {
    Board {
        r,
        c,
        n,
        turn: 1,
        prev_move: 0,
        total_placed: 0,
        bot_turn_no: 2,
        board: vec![0; c*r],
        column_tops: vec![0; c]
    }
}

fn test_game(r: usize, c: usize, n: usize) {
    let mut b = init_board(r, c, n);
    b.debug_print();
    while b.detect_win() == 0 && !b.board_is_full() {
        println!("Player {} turn: ", b.turn);

        let mut valid_index: bool = false;
        let mut col : i32 = 0;

        while !valid_index {
            let mut inp_c = String::new();
            io::stdin()
                .read_line(&mut inp_c)
                .expect("Failed to read line");

            col = inp_c.trim().parse().expect("enter a number");

            if col < 0 || col >= c as i32 {
                println!("index out of range");
            } else {
                valid_index = true;
            }
        }

        b.place_token(col as usize);
        b.debug_print();
    }
    
    if b.detect_win() != 0 {
        println!("Winner is player {}!", b.detect_win());
    } else {
        println!("Game ended in tie.");
    }  
}

fn test_bot(r: usize, c: usize, n: usize) {
    let mut b = init_board(r, c, n);
    b.debug_print();
    while b.detect_win() == 0 && !b.board_is_full() {
        println!("Player turn: ");

        let mut valid_index: bool = false;
        let mut col : i32 = 0;

        while !valid_index {
            let mut inp_c = String::new();
            io::stdin()
                .read_line(&mut inp_c)
                .expect("Failed to read line");

            col = inp_c.trim().parse().expect("enter a number");

            if col < 0 || col >= c as i32 {
                println!("index out of range");
            } else {
                valid_index = true;
            }
        }

        if b.column_is_full(col as usize) {
            continue;
        }

        b.place_token(col as usize);
        b.debug_print();
        if b.detect_win() != 0 || b.board_is_full() {
            break;
        }

        println!("Bot move {}:", b.turn);
        
        let start = Instant::now();
        let bot_move = b.minimax_driver(15, -9999, 9999);
        let end = Instant::now();
        println!("Bot took {:?} and chose {}", end-start, bot_move);
        b.place_token(bot_move);
        b.debug_print();
    }
    
    if b.detect_win() != 0 {
        println!("Winner is player {}!", b.detect_win());
    } else {
        println!("Game ended in tie.");
    }  
}