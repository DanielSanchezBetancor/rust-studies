fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_arguments(5);
    sum(5, 10);
    let expression_in_statement = {
        let x = 4;
        x + 1
    };
    println!("La declaración dentro de la expresion da como valor: {expression_in_statement}");
    let function_return = function_with_return(5, 10);
    println!("La declaración dentro de la funcion da como valor: {function_return}");
}

fn another_function() {
    println!("Llamada a otra funcion");
}

fn another_function_with_arguments(valor: u32) {
    println!("Llamada a otra funcion con el argumento: {valor}");
}

fn sum(x: u32, y: u32) {
    let sum = x + y;
    println!("{x} + {y} = {sum}");
}

fn function_with_return(x: u32, y: u32) -> u32 {
    let sum = x + y;
    sum
}