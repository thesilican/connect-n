use wasm_bindgen::prelude::*;
use std::io;
use std::cmp::max;
use std::cmp::min;
use std::time::Instant;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: &str);
}

#[wasm_bindgen]
pub fn welcome() {
    log("Hello from wasm!");
}

#[wasm_bindgen]
pub fn bot(v: Vec<i32>, depth: u32) -> i32 {
    let mut turn = 1;
    let mut one_count = 0;
    let mut two_count = 0;
    for i in &v {
        if *i == 1 {
            one_count += 1;
        } else if *i == 2 {
            two_count += 1;
        }
    }

    if one_count == two_count {
        turn = 1;
    } else {
        turn = 2;
    }

    let mut cur_vec = vec![vec![0; 7]; 7];
    let mut nonempty_vec = vec![vec![0; 7]; 7];
    let mut i = 0;
    for r in 0..6 {
        for c in 0..7 {
            if v[i] == turn {
                cur_vec[r][c] = 1;
            }
            if v[i] != 0 {
                nonempty_vec[r][c] = 1;
            }
            i += 1;
        }
    }

    let mut cur : u64 = 0;
    let mut nonempty : u64 = 0;
    for c in 0..7 {
        for r in (0..7).rev() {
            cur <<= 1;
            nonempty <<= 1;
            if cur_vec[r][c] == 1 {
                cur += 1;
            }
            if nonempty_vec[r][c] != 0 {
                nonempty += 1;
            }
        }
    }

    let mut b = Board4 {
        cur_player: cur,
        non_empty: nonempty
    };
    let mut table = vec![(0 as u64, 0 as i32); 10000000];
    let mut revs : u64 = 0;
    b.minimax_driver(depth, -100, 100, &mut revs, &mut table) as i32
}

// fn main() {
//     test_bot42(17);
// }

fn sort_tuples(tuples: &mut Vec<(usize, usize, usize, usize)>) {
    tuples.sort_by(|a, b| {
        if a.0 != b.0 {
            b.0.cmp(&a.0)
        } else if a.1 != b.1 {
            b.1.cmp(&a.1)
        } else {
            b.2.cmp(&a.2)
        }
    });
}

struct Board {
    r: usize,
    c: usize,
    n: usize,
    turn: u32,
    total_placed: u32,
    revs: u64,
    board: Vec<u32>, // 0-empty and 1/2-tokens (assume 1 goes first), bottom row of board is row 0, left-most col is col 0
    column_tops: Vec<usize> // index of first free slot in column (starts at 0)
}

impl Board {
    fn column_is_full(&self, c: usize) -> bool {
        self.column_tops[c] >= self.r
    }

    fn board_is_full(&self) -> bool {
        self.total_placed == (self.r as u32) * (self.c as u32)
    }

    fn place_token(&mut self, c: usize) {
        self.board[self.column_tops[c]*self.c + c] = self.turn;
        self.column_tops[c] += 1;
        self.turn = 3-self.turn; // next player's turn
        self.total_placed += 1;
    }

    // undos last turn, which was on column c
    fn undo_turn(&mut self, c: usize) {
        // revert struct fields
        self.turn = 3-self.turn;
        self.total_placed -= 1;
        self.board[(self.column_tops[c]-1)*self.c + c] = 0;
        self.column_tops[c] -= 1;
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
        if ans >= self.n {
            return (ans,1);
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
        if ans >= self.n {
            return (ans,1);
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
        if ans >= self.n {
            return (ans,1);
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

    fn minimax_driver(&mut self, depth: u32, mut alpha: i32, beta: i32) -> usize {
        self.revs += 1;

        let mut bestVal = -9999;
        let mut col = 0;
        static MOVE_ORDERING : [usize; 7] = [3,2,4,1,5,0,6];

        let mut final_order = Vec::with_capacity(7);
        
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
            final_order.push((p.0, p.1, center_heuristic as usize, i));
            center_heuristic -= 1;

            self.undo_turn(i);
        }
        sort_tuples(&mut final_order);

        for i in final_order {
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

            let value = self.minimax(depth-1, -1, alpha, beta);
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

    fn minimax(&mut self, depth: u32, isMaximizer: i32, mut alpha: i32, mut beta: i32) -> i32 {
        self.revs += 1;
        
        if depth == 0 {           
            return 0;
        }

        static MOVE_ORDERING : [usize; 7] = [3,2,4,1,5,0,6];
        let mut final_order = Vec::with_capacity(7);
        
        let mut center_heuristic : i32 = 6;
        for i in MOVE_ORDERING {
            if self.column_is_full(i) {
                continue;
            }

            self.place_token(i);
            let p = self.detect_sequence(self.column_tops[i]-1, i, 3-self.turn);
            if p.0 >= self.n {
                self.undo_turn(i);
                return isMaximizer*100;
            }
            if self.board_is_full() {
                self.undo_turn(i);
                return 0;
            }
            final_order.push((p.0, p.1, center_heuristic as usize, i));
            center_heuristic -= 1;

            self.undo_turn(i);
        }
        sort_tuples(&mut final_order);

        let mut bestVal = -9999*isMaximizer;
        for i in final_order {
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

            if isMaximizer == 1 {
                bestVal = max(bestVal, self.minimax(depth-1, -1, alpha, beta));
                alpha = max(alpha, bestVal);
            } else {
                bestVal = min(bestVal, self.minimax(depth-1, 1, alpha, beta));
                beta = min(beta, bestVal);
            }
            
            self.undo_turn(i.3);
            
            if beta <= alpha {
                break;
            }
        }

        return bestVal;
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
        total_placed: 0,
        revs: 0,
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
        b.revs = 0;
        
        let start = Instant::now();
        let bot_move = b.minimax_driver(15, -9999, 9999);
        let end = Instant::now();
        let d = end - start;
        println!("Bot speed: {:?} @ {} states/s", d, (b.revs as f64 / (d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9)).floor());
        b.place_token(bot_move);
        b.debug_print();
    }
    
    if b.detect_win() != 0 {
        println!("Winner is player {}!", b.detect_win());
    } else {
        println!("Game ended in tie.");
    }  
}

fn test_bot2(r: usize, c: usize, n: usize) {
    let mut b = init_board(r, c, n);
    b.debug_print();
    while b.detect_win() == 0 && !b.board_is_full() {
        println!("Bot move {}:", b.turn);
        b.revs = 0;
        
        let start = Instant::now();
        let bot_move = b.minimax_driver(14, -9999, 9999);
        let end = Instant::now();
        let d = end-start;
        println!("Bot speed: {:?} @ {} states/s", d, (b.revs as f64 / (d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9)).floor());
        b.place_token(bot_move);
        b.debug_print();

        if b.detect_win() != 0 || b.board_is_full() {
            break;
        }

        println!("Player move: ");

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

struct Board4 {
    cur_player: u64,
    non_empty: u64
}

impl Board4 {
    fn copy(&self) -> Board4 {
        Board4 {
            cur_player: self.cur_player,
            non_empty: self.non_empty
        }
    }

    fn get_key(&self) -> u64 {
        static LOWER_ONES : u64 = 0b0000001000000100000010000001000000100000010000001;
        self.cur_player + self.non_empty + LOWER_ONES
    }

    fn get_pos(&self, r: usize, c: usize) -> u64 {
        self.get_lower_one(c) << r
    }

    fn get_lower_one(&self, c: usize) -> u64 {
        1 << (6-c)*7
    }

    fn column_is_full(&self, c: usize) -> bool {
        ((0b100000 << (6-c)*7) & self.non_empty) != 0
    }

    fn place_token(&mut self, c: usize) {
        self.cur_player ^= self.non_empty;
        self.non_empty |= self.non_empty + self.get_lower_one(c);
    }

    fn board_is_full(&self) -> bool {
        self.non_empty == 0b0111111011111101111110111111011111101111110111111
    }

    fn detect_win(&self) -> bool {
        let prev = self.cur_player ^ self.non_empty;
        //vert
        let mut temp : u64 = prev & (prev << 1);
        if temp & (temp << 2) != 0 {
            return true;
        }

        //horiz
        temp = prev & (prev << 7);
        if temp & (temp << 14) != 0 {
            return true;
        }

        //pos diag
        temp = prev & (prev << 6);
        if temp & (temp << 12) != 0 {
            return true;
        }

        //neg diag
        temp = prev & (prev << 8);
        if temp & (temp << 16) != 0 {
            return true;
        }

        false
    }

    fn detect_sequence(&self, c: usize) -> (usize, usize) {
        let prev = self.cur_player ^ self.non_empty;

        // calculate r
        let mut r : usize = 0;
        let mut col_bottom = self.get_lower_one(c) << 1;
        while (col_bottom & self.non_empty) != 0 {
            col_bottom <<= 1;
            r += 1;
        }

        let mut ans = 0;
        let mut ans_count = 0;

        let mut ans_cand = 0;
        let mut buffer = 0;

        // horiz
        let mut i = 1;
        let mut b = 0;
        loop {
            if c+i >= 7 || (self.get_pos(r, c+i) & self.cur_player != 0) { //3-player is the other player
                break;
            }

            if (self.get_pos(r, c+i) & prev != 0) && b == 0 {
                ans_cand += 1;
                if ans_cand >= 4 {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= 4 {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        i = 1;
        b = 0;
        loop {
            if (c as i32) - (i as i32) < 0 || (self.get_pos(r, c-i) & self.cur_player != 0) { //3-player is the other player
                break;
            }

            if (self.get_pos(r, c-i) & prev != 0) && b == 0 {
                ans_cand += 1;
                if ans_cand >= 4 {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= 4 {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        ans_cand += 1;
        if ans_cand + buffer >= 4 {
            ans = ans_cand;
            ans_count = 1;
        }
        if ans >= 4 {
            return (ans,1);
        }

        buffer = 0;
        ans_cand = 0;
        i = 1;
        b = 0;

        //vert
        loop {
            if r+i >= 6 || (self.get_pos(r+i, c) & self.cur_player != 0) { //3-player is the other player
                break;
            }

            if (self.get_pos(r+i, c) & prev != 0) && b == 0 {
                ans_cand += 1;
                if ans_cand >= 4 {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= 4 {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        i = 1;
        b = 0;
        loop {
            if (r as i32) - (i as i32) < 0 || (self.get_pos(r-i, c) & self.cur_player != 0) { //3-player is the other player
                break;
            }

            if (self.get_pos(r-i, c) & prev != 0) && b == 0 {
                ans_cand += 1;
                if ans_cand >= 4 {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= 4 {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        ans_cand += 1;
        if ans_cand + buffer >= 4 {
            if ans_cand == ans {
                ans_count += 1;
            } else if ans_cand > ans {
                ans = ans_cand;
                ans_count = 1;
            }
        }
        if ans >= 4 {
            return (ans,1);
        }

        buffer = 0;
        ans_cand = 0;
        i = 1;
        b = 0;

        //pos diag
        loop {
            if r+i >= 6 || c+i >= 7 || (self.get_pos(r+i, c+i) & self.cur_player != 0) { //3-player is the other player
                break;
            }

            if (self.get_pos(r+i, c+i) & prev != 0) && b == 0 {
                ans_cand += 1;
                if ans_cand >= 4 {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= 4 {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        i = 1;
        b = 0;
        loop {
            if (r as i32) - (i as i32) < 0 || (c as i32) - (i as i32) < 0 || (self.get_pos(r-i, c-i) & self.cur_player != 0) { //3-player is the other player
                break;
            }

            if (self.get_pos(r-i, c-i) & prev != 0) && b == 0 {
                ans_cand += 1;
                if ans_cand >= 4 {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= 4 {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        ans_cand += 1;
        if ans_cand + buffer >= 4 {
            if ans_cand == ans {
                ans_count += 1;
            } else if ans_cand > ans {
                ans = ans_cand;
                ans_count = 1;
            }
        }
        if ans >= 4 {
            return (ans,1);
        }

        buffer = 0;
        ans_cand = 0;
        i = 1;
        b = 0;

        //neg diag
        loop {
            if r+i >= 6 || (c as i32) - (i as i32) < 0 || (self.get_pos(r+i, c-i) & self.cur_player != 0) { //3-player is the other player
                break;
            }

            if (self.get_pos(r+i, c-i) & prev != 0) && b == 0 {
                ans_cand += 1;
                if ans_cand >= 4 {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= 4 {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        i = 1;
        b = 0;
        loop {
            if (r as i32) - (i as i32) < 0 || c+i >= 7 || (self.get_pos(r-i, c+i) & self.cur_player != 0) { //3-player is the other player
                break;
            }

            if (self.get_pos(r-i, c+i) & prev != 0) && b == 0 {
                ans_cand += 1;
                if ans_cand >= 4 {
                    break;
                }
            } else {
                b += 1;
                if ans_cand + buffer + b >= 4 {
                    break;
                }
            }
            i += 1;
        }
        buffer += b;
        ans_cand += 1;
        if ans_cand + buffer >= 4 {
            if ans_cand == ans {
                ans_count += 1;
            } else if ans_cand > ans {
                ans = ans_cand;
                ans_count = 1;
            }
        }

        (ans, ans_count)
    }

    fn debug_print(&self, cur_player: char, op: char) {
        let mut board = vec![vec![' '; 7]; 6];
        let mut temp_cur = self.cur_player;
        let mut temp_all = self.non_empty;

        for c in (0..7).rev() {
            for r in 0..6 {
                if (temp_all & 1) == 0 {
                    temp_cur >>= 1;
                    temp_all >>= 1;
                    continue;
                }

                if (temp_cur & 1) == 1 {
                    board[r][c] = cur_player;
                } else {
                    board[r][c] = op;
                }

                temp_cur >>= 1;
                temp_all >>= 1;
            }
            temp_cur >>= 1;
            temp_all >>= 1;
        }

        for r in (0..6).rev() {
            print!("{} ",r);
            for c in 0..7 {
                print!("{} ", board[r][c]);
            }
            println!("");
        }
        print!("  ");
        for i in 0..7 {
            print!("{} ", i);
        }
        println!("");
    }  

    fn minimax(&mut self, depth: u32, isMaximizer: i32, mut alpha: i32, mut beta: i32, revs: &mut u64, table: &mut Vec<(u64,i32)>) -> i32 {     
        *revs += 1;

        let table_val = table[(self.get_key() % 10000000) as usize];
        if table_val.0 == self.get_key() {
            return table_val.1;
        }

        if depth == 0 {           
            return 0;
        }

        static MOVE_ORDERING : [usize; 7] = [3,2,4,1,5,0,6];
        let mut final_order = Vec::with_capacity(7);
        let mut center_heuristic : i32 = 6;
        for i in MOVE_ORDERING {
            if self.column_is_full(i) {
                continue;
            }

            let mut b = self.copy();
            b.place_token(i);
            let p = b.detect_sequence(i);
            if p.0 >= 4 {
                table[(self.get_key() % 10000000) as usize] = (self.get_key(), 100*isMaximizer);
                return 100*isMaximizer;
            }
            if b.board_is_full() {
                return 0;
            }

            final_order.push((p.0, p.1, center_heuristic as usize, i));
            center_heuristic -= 1;
        }
        sort_tuples(&mut final_order);

        let mut bestVal = -100*isMaximizer;
        for i in final_order {
            let mut b = self.copy();

            b.place_token(i.3);

            let mut skip_move = false;
            for j in MOVE_ORDERING {
                if b.column_is_full(j) {
                    continue;
                }
                let mut b2 = b.copy();
                b2.place_token(j);
                if b2.detect_win() {
                    skip_move = true;
                    break;
                }
            }
            if skip_move {
                continue;
            }

            if isMaximizer == 1 {
                bestVal = max(bestVal, b.minimax(depth-1, -1, alpha, beta, revs, table));
                alpha = max(alpha, bestVal);
            } else {
                bestVal = min(bestVal, b.minimax(depth-1, 1, alpha, beta, revs, table));
                beta = min(beta, bestVal);
            }
                        
            if beta <= alpha {
                break;
            }
        }

        if bestVal != 0 {
            table[(self.get_key() % 10000000) as usize] = (self.get_key(), bestVal);
        }

        bestVal
    }

    fn minimax_driver(&mut self, depth: u32, mut alpha: i32, beta: i32, revs: &mut u64, table: &mut Vec<(u64,i32)>) -> usize {
        *revs += 1;
        let mut bestVal = -100;
        let mut col = 0;
        let mut free_col = 0;
        static MOVE_ORDERING : [usize; 7] = [3,2,4,1,5,0,6];

        let mut final_order = Vec::with_capacity(7);
        let mut center_heuristic : i32 = 6;
        for i in MOVE_ORDERING {
            if self.column_is_full(i) {
                continue;
            }

            let mut b = self.copy();
            b.place_token(i);
            let p = b.detect_sequence(i);
            if p.0 >= 4 {
                return i;
            }
            if b.board_is_full() {
                return i;
            }

            final_order.push((p.0, p.1, center_heuristic as usize, i));
            center_heuristic -= 1;
        }
        sort_tuples(&mut final_order);

        for i in final_order {
            free_col = i.3;
            let mut b = self.copy();
            b.place_token(i.3);

            let mut skip_move = false;
            for j in MOVE_ORDERING {
                if b.column_is_full(j) {
                    continue;
                }
                let mut b2 = b.copy();
                b2.place_token(j);
                if b2.detect_win() {
                    skip_move = true;
                    break;
                }
            }
            if skip_move {
                continue;
            }

            let value = b.minimax(depth-1, -1, alpha, beta, revs, table);
            if value > bestVal {
                col = i.3;
                bestVal = value;
            }
            alpha = max(alpha, bestVal);
            
            if beta <= alpha {
                break;
            }
        }

        if self.column_is_full(col) {
            return free_col;
        } else {
            return col;
        }
    }
}

fn test_bot4(depth: u32) {
    let mut table = vec![(0 as u64, 0 as i32); 10000000];

    let mut b = Board4 {
        cur_player: 0,
        non_empty: 0
    };
    b.debug_print('X', 'O');
    let mut winner = 'X';
    while !b.detect_win() && !b.board_is_full() {
        println!("Player turn: ");

        let mut valid_index: bool = false;
        let mut col : i32 = 0;

        while !valid_index {
            let mut inp_c = String::new();
            io::stdin()
                .read_line(&mut inp_c)
                .expect("Failed to read line");

            col = inp_c.trim().parse().expect("enter a number");

            if col < 0 || col >= 7 {
                println!("index out of range");
            } else {
                valid_index = true;
            }
        }

        if b.column_is_full(col as usize) {
            continue;
        }

        b.place_token(col as usize);
        b.debug_print('O', 'X');
        if b.detect_win() || b.board_is_full() {
            winner = 'X';
            break;
        }
        
        let mut revs : u64 = 0;
        let start = Instant::now();
        let bot_move = b.minimax_driver(depth, -100, 100, &mut revs, &mut table);
        let end = Instant::now();
        let d = end - start;
        println!("Bot speed: {:?} @ {} states/s", d, (revs as f64 / (d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9)).floor());
        println!("Bot move: {}", bot_move);
        b.place_token(bot_move);
        b.debug_print('X', 'O');
        winner = 'O';
    }
    
    if b.detect_win() {
        println!("Winner is player {}!", winner);
    } else {
        println!("Game ended in tie.");
    }  
}

fn test_bot42(depth: u32) {
    let mut table = vec![(0 as u64, 0 as i32); 10000000];

    let mut b = Board4 {
        cur_player: 0,
        non_empty: 0
    };
    b.debug_print('X', 'O');
    let mut winner = 'X';
    while !b.detect_win() && !b.board_is_full() {
        let mut revs : u64 = 0;
        let start = Instant::now();
        let bot_move = b.minimax_driver(depth, -100, 100, &mut revs, &mut table);
        let end = Instant::now();
        let d = end - start;
        println!("Bot speed: {:?} @ {} states/s", d, (revs as f64 / (d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9)).floor());
        println!("Bot move: {}", bot_move);
        b.place_token(bot_move);
        b.debug_print('O', 'X');
        if b.detect_win() || b.board_is_full() {
            winner = 'X';
            break;
        }

        println!("Player turn: ");

        let mut valid_index: bool = false;
        let mut col : i32 = 0;

        while !valid_index {
            let mut inp_c = String::new();
            io::stdin()
                .read_line(&mut inp_c)
                .expect("Failed to read line");

            col = inp_c.trim().parse().expect("enter a number");

            if col < 0 || col >= 7 {
                println!("index out of range");
            } else {
                valid_index = true;
            }
        }

        b.place_token(col as usize);
        b.debug_print('X', 'O');
        winner = 'O';
    }
    
    if b.detect_win() {
        println!("Winner is player {}!", winner);
    } else {
        println!("Game ended in tie.");
    }  
}

