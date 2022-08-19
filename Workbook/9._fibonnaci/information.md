# Información del proyecto

Algo más sencillo que el de convertidor de temperaturas una vez establecido en el lenguaje, en este caso tratamos un generador de Fibonnaci, el cuál te dará el resultado del número *n* de la sucesión. 

Vamos a ir despedazando el código y explicando las situaciones encontradas y los recursos usados. 

Se inicia el código en la función *main*, damos la bienvenida al usuario y planteo la posibilidad de que el usuario pueda realizar 1 o *n* generaciones. 

Subsano esta necesidad con un bucle, en este caso, *while*, dado que sabemos el número de vueltas que vamos a dar, introducido por el usuario previamente. 

Apoyandome de varias funciones y lógicas del anterior ejercicio, seguimos una lógica principal muy sencilla: Recoger del usuario el número de vueltas que se quieren dar y en cada iteración, generar la formula de Fibonnaci y trasladar los números para una siguiente vuelta. 

```
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
```

A denotar, tenemos el contador de *while* con la variable insertada por el usuario. Usamos su propia indicación como contador hacia atrás, hasta llegar a 0. 

Como nota final, estoy seguro de que las tres variables de Fibonnaci se pueden generar de otra manera o se esperaba algun tipo de optimización diferente, pero sin querer mirar en Internet hasta no haber finalizado el *WorkBook*, dejo esto como un *quizás se pueda optimizar*.