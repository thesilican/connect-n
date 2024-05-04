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
    let mut table = vec![(0 as u64, 0 as i32); 100000000];
    let mut revs : u64 = 0;
    b.minimax_driver(depth, -100, 100, &mut revs, &mut table) as i32
}

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

        let r = (self.non_empty >> (6-c)*7).trailing_ones() as usize - 1;

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

