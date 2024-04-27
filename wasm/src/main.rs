use std::io;

fn main() {
    println!("Hi Om");
    first_func();
}

fn first_func() {
    loop {
        let mut guess = String::new();

        let res = io::stdin()
            .read_line(&mut guess);

        let _res = match res {
            Ok(res) => res,
            Err(_err) => {
                println!("read line failed");
                continue;
            }
        };

        let guess = guess.trim().parse::<u32>();
        let guess = match guess {
            Ok(guess) => guess,
            Err(_err) => {
                println!("please enter a positive int");
                continue;
            }
        };
    
        println!("You guessed: {}", guess);

        if guess < 32 {
            println!("small num: {}", guess);
        } else {
            println!("big num: {}", guess);
        }
        break;
    }
}
