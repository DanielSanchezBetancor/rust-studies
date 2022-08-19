# Información de este proyecto

Es una propuesta sencilla, un convertidor de Fahrenheit a Celsius y al contrario. 

Vamos a ir despedazando el código y explicando las situaciones encontradas y los recursos usados. 

Se inicia el código en la función *main*, damos la bienvenida al usuario y planteo la posibilidad de que el usuario pueda realizar 1 o *n* conversiones. 

Subsano esta necesidad con un bucle. De los bucles *Rust* presentados, creo que tengo dos posibilidades claras ante un bucle infinito:

- *loop*
- *while*

Por facilidad, uso *loop*. También es verdad, entre tu y yo, que intente hacer una condición *while*, pero creo que algo de *shadowing* no le gustaba y no queria usar dos variables, quiero que la selección del usuario se pueda transformar en la misma condición que escape el bucle. 

A este *loop*, le añado un identificador que referenciar en el *break*, para una sensación de mejor precisión de código. 

```
fn main() {
    println!("¡Bienvenid@! ¡Vamos a convertir cosas!");
    //Loop with tag
    'user_loop: loop {
```

Tras entrar en el bucle, solicitamos al usuario, por medio de un mensaje, el tipo de transformación que desea realizar. 

```
println!("¿Convertirmos de Fahrenheit a Celsius, o al contrario?");
let transformation: String = retrieve_user_selection("Selecciona 1 para la primera opcion; 2 para la segunda.");
```
Con uso de la libreria *STD:io*, creo una función reutilizable que recoja la información del usuario, además de una función de transformación de tipos de variables para poder trabajar con números. Podría trabajar tambien con *match* y la función *cmp()*, pero he visto más sencillo para este segundo ejercicio propuesto, usar condicionales numerales y plantear un programa que sea mas realista, a buscar otro tipo de condicionadores *(quizás para el siguiente)*. 

```
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

```

Guardo la selección del usuario como una variable diferente, dado que si usaramos la misma *user_selection* quedaría sobreescrita antes de poder decidir que función de transformación de temperaturas usar (salvo que dividamos todo el código en 2 bloques previamente, con lo cual quedaría un trozo de código mucho mas grande y lioso, pero podriamos usar *shadowing* para la misma variable *user_selection*). 

Solicitamos al usuario que dato quiere convertir, y con una expresion en una declaración, llamamos a la función necesaria de temperaturas, seleccionada previamente. Con esto, podemos mostrar al usuario ya la comparación de temperaturas. 

```
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
```

Por último, solicitamos al usuario si desea seguir o irse. Si desea continuar, presionando *0*, el bucle *loop* empezará de nuevo, repitiendo el proceso. En el caso contrario, muestro un mensaje de despedida y salgo del programa. 

```
//Retrieving data from user - Loop state control
let user_selection: String = retrieve_user_selection("Pulse 0 para continuar u otra tecla para salir");
//Shadowing to cast int
let user_selection: i32 = transform_string_to_int(user_selection);
if user_selection != 0 {
    break 'user_loop;
}
```

Adicionalmente, he intentado que todas las variables usadas esten tipadas, para mejor control del código a cualquier nivel. 

Un ejemplo de interacción con el usuario: 

```
Bienvenido! Vamos a convertir cosas!
¿Convertirmos de Fahrenheit a Celsius, o al contrario?
Selecciona 1 para la primera opcion; 2 para la segunda.
1
¿Que dato quieres convertir?
25
25 °F = -3.888888888888889 °C
Pulse 0 para continuar u otra tecla para salir
0
¿Convertirmos de Fahrenheit a Celsius, o al contrario?
Selecciona 1 para la primera opcion; 2 para la segunda.
2
¿Que dato quieres convertir?
-3.8
-3.8 °F = 25.16 °C
Pulse 0 para continuar u otra tecla para salir
0
¿Convertirmos de Fahrenheit a Celsius, o al contrario?
Selecciona 1 para la primera opcion; 2 para la segunda.
3
¿Que dato quieres convertir?
2
No se que has querido decirme.
¿Convertirmos de Fahrenheit a Celsius, o al contrario?
Selecciona 1 para la primera opcion; 2 para la segunda.
2
¿Que dato quieres convertir?
2525
2525 °F = 4577 °C
Pulse 0 para continuar u otra tecla para salir

¿Convertirmos de Fahrenheit a Celsius, o al contrario?
Selecciona 1 para la primera opcion; 2 para la segunda.
2
¿Que dato quieres convertir?
141
141 °F = 285.8 °C
Pulse 0 para continuar u otra tecla para salir
0
¿Convertirmos de Fahrenheit a Celsius, o al contrario?
Selecciona 1 para la primera opcion; 2 para la segunda.
1
¿Que dato quieres convertir?
24
24 °F = -4.444444444444445 °C
Pulse 0 para continuar u otra tecla para salir
1
¡¡Chaito!!

```