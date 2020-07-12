use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut rng = rand::thread_rng();
    let r_number = rng.gen_range(1, 101);
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("couldn't read line");
        let guess: u32 = guess.trim().parse().expect("that's not a number");
    
        match guess.cmp(&r_number) {
            Ordering::Less => println!("bigger"),
            Ordering::Greater => println!("smaller"),
            Ordering::Equal => {println!("win");
                                break;}
        }
    }   
}
