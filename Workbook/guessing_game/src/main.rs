use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Prueba tu suerte!");
    //Generate random number by RNG crate
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        //Generate new String or override existing one
        let mut user_value = String::new();
        println!("Introduce un número");
        //Retrieve input from user
        io::stdin()
            .read_line(&mut user_value)
            .expect("Algo ha ocurrido con la terminal");
        //Convert String to unassigned 32 integer. Keep looping on any different type. 
        let user_value: u32 = match user_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        /* Compare user input value to secret number reference
        *   user_value <> secret_number => Shows a hint and loops
        *   user_value = secret_number => Shows a winning text and exits
        */
        match user_value.cmp(&secret_number) {
            Ordering::Less => println!("El número es mayor."),
            Ordering::Equal => {
                println!("¡¡Has ganado!!: {secret_number}");
                break;
            },
            Ordering::Greater => println!("El número es menor."),
        }
        println!("¡Probemos de nuevo!");
    }
}
