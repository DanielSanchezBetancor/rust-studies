# Información del proyecto

Christmas carol - The Twelve Days of Christmas. Letra.

Vamos a ir despedazando el código y explicando las situaciones encontradas y los recursos usados. 

Se inicia el código en la función *main*, generamos:

- Array de dias. Del 1 al 12, tal y como se muestran en la canción. 
- Array de regalos. El array esta ordenado de la forma en que se vería en el último día. 
- Contador de regalos. Contamos en que día estamos y cuantos regalos mostraremos. Esto podia evitarse con un *length* y otro array, pero no hemos visto nada de esto hasta ahora. 

Recorremos cada día y vamos escribiendo el primer verso con cada día.

```
let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twefth" ];
for day in days {
    println!("On the {day} day of Christmas");
```

El segundo verso siempre se repite, lo mostramos en una nueva línea.

```
println!("My good friends brought to me");
```

Tras esto, mostramos los regalos disponibles según el dia. Sabemos que tenemos 12 días *[0..11]* junto con 12 regalos, por lo que en cada iteración iremos descontando uno a uno este contador de regalos máximos posible, dado que necesitamos empezar desde el final del array. *Recuerda: El array se muestra como en el último parrafo. Esto significa que el primer regalo del primer parrafo, se mostrará en el último lugar del último parrafo*. Recorremos desde el valor del contador al valor final, mostrando todos los regalos en orden.

```
let presents_list = ["All their good wishes", "Gifts for one and all", "Some mistletoe", "A guardian angel", "Gold and silver tinsel", "Candles a glowing", "Little silver bells", "A shining star", "Four colored lights", "Three boughs of holly", "Two candy canes"];
let mut present_counter = 11;

for pos in present_counter..11 {
    let present = presents_list[pos];
    println!("({present})");
}

present_counter = if present_counter > 0 { present_counter - 1 } else { 0 };
```

El último verso tiene la condición de que la primera vez se muestra de forma diferente. Subsanable teniendo el contador declarado.

```
let final_verse = if present_counter == 11 { "A song and a Christmas tree" } else { "And a song for the Christmas tree" };
println!("{final_verse}");
```

