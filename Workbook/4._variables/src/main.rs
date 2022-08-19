
fn main() {
    //1. - Suelta error (cannot assign twice to immutable variable)
    /*
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    */
    //2. - 'x' es ahora mutable, por lo que no genera errores
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
