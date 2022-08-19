use std::io;

fn main() {
    println!("¡Bienvenid@! ¡Vamos a convertir cosas!");
    //Loop with tag
    'user_loop: loop {
        println!("¿Convertirmos de Fahrenheit a Celsius, o al contrario?");
        let transformation: String = retrieve_user_selection("Selecciona 1 para la primera opcion; 2 para la segunda.");
        //Shadowing to cast int
        let transformation: i32 = transform_string_to_int(transformation);
        //Retrieving data from user - Data
        let user_selection: String = retrieve_user_selection("¿Que dato quieres convertir?");
        //Shadowing to cast float
        let user_selection: f64 = transform_string_to_float(user_selection);
        //Auxiliar function to transform data
        let new_data: f64 = if transformation == 1 {
            transform_to_celsius(user_selection)
        } else if transformation == 2 {
            transform_to_fahrenheit(user_selection)
        } else {
            println!("No se que has querido decirme.");
            continue;
        };
        //Show result
        println!("{user_selection} °F = {new_data} °C");
        //Retrieving data from user - Loop state control
        let user_selection: String = retrieve_user_selection("Pulse 0 para continuar u otra tecla para salir");
        //Shadowing to cast int
        let user_selection: i32 = transform_string_to_int(user_selection);
        if user_selection != 0 {
            break 'user_loop;
        }
    }
    println!("¡¡Chaito!!");
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
//Auxiliar function to cast from String to Float
fn transform_string_to_float(string: String) -> f64 {
    return match string.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
}
//Auxiliar function to cast from String to Integer
fn transform_string_to_int(string: String) -> i32 {
    return match string.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
}
//Using explicit declaration as return
fn transform_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
//Using explicit declaration as return
fn transform_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
