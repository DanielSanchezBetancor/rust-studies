fn main() {
    let edad: i32 = 28;
    
    //Condicional verdadero
    if edad < 100 {
        println!("¡Que joven eres!");
    } else {
        println!("Podrias considerarte un adulto");
    }

    //Condicional falso
    if edad < 1 {
        println!("¡Que joven eres!");
    } else {
        println!("Podrias considerarte un adulto");
    }

    //Condiciones anidadas
    if edad == 0 {
        println!("¡Ese número no vale!");
    } else if edad % 2 == 0 {
        println!("Tu edad es divisible entre 2");
    } else {
        println!("Tu edad no es divisible entre 2");
    }

    //Declaracion de una variable con una expresion de condicion
    let es_par = if edad % 2 == 0 { "Si" } else { "No" };
    println!("¿Es par? => {es_par}");

    //Variable de salida para poder ver los demas resultados
    let mut exit = 0;
    //Sin condiciones de salida, este bucle se ejecutaria para siempre
    loop {
        //Condición de salida
        if exit == 10 {
            break;
        }
        println!("No paro de dar vueltas!!");
        exit += 1;
    }

    //Loops
    let mut times = 0;
    let how_many_times = loop {
        if times == 10 {
            break times;
        }
        times += 1;
    };
    println!("Eso fueron {how_many_times} veces!!");
    
    //Loops anidados con identificadores
    let mut enemigos = 5;
    let mut conseguido: bool = 'primer_nivel: loop {
        let mut pistoletazos = 0;
        'ataque: loop {
            if enemigos == 0 {
                break 'primer_nivel true;
            }
            pistoletazos += 1;
            if pistoletazos == 5 {
                println!("¡¡Conseguido!! Has matado a 1 enemigo");
                enemigos -= 1;
                break 'ataque;
            } else {
                println!("Has fallado el {pistoletazos}º disparo, apunta mejor!!");
            } 
        }
        println!("Quedan {enemigos} enemigos");
    };
    println!("¿Primer nivel superado? {conseguido}");

    //Misma version que el loop anidado y con identificadores, pero usando 'while'
    println!("Empieza el segundo nivel.");
    enemigos = 10;
    while enemigos != 0 {
        let mut pistoletazos = 0;
        while pistoletazos != 8 {
            pistoletazos += 1;
            if pistoletazos == 8 {
                println!("¡¡Conseguido!! Has matado a 1 enemigo");
                enemigos -= 1;
            } else {
                println!("Has fallado el {pistoletazos}º disparo, apunta mejor!!");
            }
        }
        println!("Quedan {enemigos} enemigos");
    }
    let nivel_conseguido = if enemigos == 0 { "Si!" } else { "Que va" }; 
    println!("Segundo nivel superado? {nivel_conseguido}");

    //Bucles for
    let armas_desbloqueadas = ["katana", "Machete", "Cuchillo de pescador", "Bazooka"];
    println!("Este es tu listado de armas disponibles:");
    for arma in armas_desbloqueadas {
        println!(" - {arma}");
    }

}
