fn main() {
    //1. - Shadowing y su scope
    let x = 5_000;
    let x = x + 1; //6 = 5 + 1
    {
        let x = x * 2; //12 = 6 * 2
        println!("Valor de 'x' dentro del Scope de llaves: {x}"); //12
    }
    println!("Valor de 'x' fuera del Scope de llaves: {x}"); //6
    //2. - Diferencia de tipado con shadowing y mut
    let shadow = "    ";
    let shadow = shadow.len();
    println!("Valor por shadow: {shadow}");
    let mut shadow_mut = "Texto por defecto";
    //Descomentando esta linea, soltara un error por tipado (expected `&str`, found `usize`)
    // shadow_mut = shadow_mut.len();
    println!("Valor por shadow_mut: {shadow_mut}");
}
