use std::io;

fn main() {
    println!("¡Bienvenido! Toca calcular Fibonnaci :)");
    loop {
        //Retrieving data - Numbers to calculate
        let user_selection = retrieve_user_selection("¿Hasta que número deseas calcular?");
        //Shadowing to cast int
        let mut user_selection = transform_string_to_int(user_selection);
        //Variable declaration for the formula
        let mut a = 1;
        let mut b = 1;
        let mut c = 0;
        //loop n times
        while user_selection > 0 {
            c = a + b;
            a = b;
            b = c;
            user_selection -= 1;
        }
        println!("El resultado es: {c}");
        //Retrieving data - Exiting program
        let user_selection = retrieve_user_selection("¿Continuamos? Pulsa 0 para continuar o cualquier otra tecla para salir");
        //Shadowing to cast int
        let user_selection = transform_string_to_int(user_selection);
        if user_selection != 0 {
            //Exiting program
            break;
        }
    }
}
/**
 * Base function to ask the user for inputs. Gets a 'message' by params to show the user what will be entering
 */
fn retrieve_user_selection(message: &str) -> String {
    println!("{message}");
    let mut user_selection = String::new();
    io::stdin()
        .read_line(&mut user_selection)
        .expect("Algo ha pasado con la terminal");
    return user_selection;
}
//Auxiliar function to cast from String to Integer
fn transform_string_to_int(string: String) -> i32 {
    return match string.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
}