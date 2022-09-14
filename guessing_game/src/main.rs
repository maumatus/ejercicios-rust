use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivina el numero ctm!!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
    println!("POr favor ingresa tu apuesta");

    let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("fallo al leer linea");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("tu adivinaste: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
              }
        }    
    }
}
