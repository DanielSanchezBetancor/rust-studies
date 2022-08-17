# Rust

## Índice
- [Rust](#rust)
  - [Índice](#índice)
  - [Instalación](#instalación)
    - [Descarga](#descarga)
    - [Instalación](#instalación-de-rust)
    - [Coimprobación de gestor de paquetes](#comprobación-de-gestor-de-paquetes)
  - [Rust](#rust-1)
    - [Ejecución](#ejecución)
    - [Ficheros](#ficheros)
    - [Funciones](#funciones)
  - [Cargo](#cargo)
  - [Buenas prácticas](#buenas-prácticas)

## Instalación

### Descarga

Podemos descargar *Rust* en la página web oficial

> https://www.rust-lang.org/tools/install

Escoger nuestra plataforma (Ej: ```DOWNLOAD RUSTUP-INIT.EXE (64-BIT)``` )

### Instalación de Rust

1. Ejecutamos 'rustup-ini.exe' *(o el archivo descargado)* y aparecerá una ventana de selección en CMD. 
2. Seleccionar la primera opcion (1). *Si no se tiene Visual Studio instalado, este proceso lo instalará.*

*Nota*

> Cuando pida descargar Visual Studio, seleccionar: 
> - “Desktop Development with C++”
> - SDK Windows 10 u 11
> - Pack de componentes Inglés + Español

3. Tras la instalación de Visual Studio, veremos información acerca de Cargo y la localización de este gestor de paquetes. Esto modificará el PATH del usuario actual para añadir los scripts de Cargo.

>   C:\Users\Baifo\\.rustup

4. Continuamos la instalación con la configuración recomendada, en la primera opción (1).
5. Finalmente, tras haberse instalado, nos solicitará apretar Enter para salir. 

### Comprobación de gestor de paquetes

Para comprobar que se ha instalado correctamente el gestor de paquetes *Cargo*, se debe abrir una nueva instancia de terminal y ejecutar: 

> rustc --version

 - Ejemplo

```
C:\Users\Baifo>rustc --version

rustc 1.63.0 (4b91a6ea7 2022-08-08)
```

Si ves esta información o alguna de la misma estructura, se ha instalado correctamente.

## Rust

### Ejecución

La ejecución de un fichero Rust, buscará como primera función de inicio de la aplicación la siguiente:

```
 fn main() {}
```

*Ejemplo de ejecución de un fichero Rust, desde la terminal.*

 1. Generamos los archivos ejecutables con *Cargo*, apuntando al fichero *Rust* que queremos ejecutar.

> ~/proyectos/Estudios/Rust/projects/hello_world $ **rustc hello_world.rs**

2. Comprobamos que los archivos se han generado
> 
> ~/proyectos/Estudios/Rust/projects/hello_world $ **ls**
> 
> **hello_world.exe\*  hello_world.pdb  hello_world.rs**
> 
1. Lanzamos el ejecutable que se acaba de generar (Windows)
> 
> ~/proyectos/Estudios/Rust/projects/hello_world $ **./hello_world.exe**
> 
> **Hello, world!**

### Ficheros

Características
- Los ficheros Rust tienen la extension ***rs***
- Combinaciónes de palabras se forman con barras bajas, como por ejemplo: *hello_world.rs*

**Comentarios**

- Comentario simple

```
// Comentario simple
```

- Comentario multilínea: Solo llevan un asterisco de entrada y salida

```
/*
* Comentario multilínea
*/
```

**Variables**

- Palabra reservada: *let*
- Las variables generadas en *Rust* son inmutables, no cambian de valor.
- Para generar una variable mutable, añadimos la palabra clave *mut*.
  - El tipado de una variable mutable no puede cambiar. *Esto marca la principal diferencia con Shadowing*
- Se pueden generar variables tipadas.
- Constantes
  - Las constantes se generan con la palabra reservada: *const*
  - Las constantes no pueden ser mutables
  - Las constantes deben tener tipado estricto
  - Solo pueden tener valores constantes, no generados en tiempo de ejecución
- Shadowing
  - Las variables que aplican shadow, generan una nueva variable, de un nuevo tipo (igual o diferente al anterior tipo).
  - El valor de las variables con shadowing se mantienen a nivel de scope

*Ejemplos*

```
let manzana = "Manzana"; //Inmutable
let mut fruta = ""; //Mutable
let mut nombre = String::new(); //Variable tipada
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //Constante
let shadow = "    "; //Variable declarada para Shadowing
let shadow = shadow.len(); //Shadowing, generando un nuevo valor con un nuevo tipo
//Scope de Shadowing
let x = 5;
let x = x + 1; //6 = 5 + 1
{
  let x = x * 2; //12 = 6 * 2
  println!("{x}"); //12
}
println!("{x}"); //6
```

**Tipos de variables**

*Escalares*

- Integer

Números sin fracción. 

| Tamaño | Con signo *(+/-)* | Sin signo |
|-|-|-|
| 8-bit |	i8 |	u8 |
| 16-bit |	i16 |	u16 |
| 32-bit |	i32 *(Por defecto)* |	u32 |
| 64-bit |	i64 |	u64 |
| 128-bit |	i128 |	u128 |
| arch |	isize |	usize |

Adicionalmente, se pueden usar varios tipos de representación de números.

| Literal | Ejemplo 
|-|-|
| Decimal |	98_222
| Hex |	0xff
| Octal |	0o77
| Binario |	0b1111_0000
| Byte | (u8)	b'A'

- Float

Existen únicamente dos: *f32* y *f64*; Siendo *f64* el tipo por defecto, dado que en CPU modernas es la misma velocidad pero con mejor precisión.

- Boolean

Con 1 byte de tamaño, solo acepta dos valores: *true* y *false*.

- Char

Unidad primitiva de caracteres del teclado. Se envuelven con comillas simples.

*Compuestos*

- Tuplas

Contenedor de números de diferentes tipos. Tamaño determinado en la declaración. 

*Ejemplo*

> Generación de una tupla

```
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

> Desestructuración de una tupla a variables

```
let (x, y, z) = tup;
```

> Selección de un componente concreto de la tupla

```
let x = tup.1; //6.4
```
- Arrays

Contenedor de variables del mismo tipo. Tamaño determinado en la declaración.

> Declaración de un array. En el tipado, determinamos la cantidad de elementos

```
let array: [i32; 5] = [0, 1, 2, 3, 4];
```

> Selección de un componente concreto del array

```
let x = array[2]; //2
```

### Funciones

- Palabra reservada: fn

*Ejemplo*

```
fn HelloWorld() {

}
```

- Las macros o funciones nativas de Rust son llamadas con una exclamación.

*Macro de Rust*
> println!("Hola mundo")

*Función del usuario*

> println("Hola mundo")

- Las funciones con argumentos deben declarar el tipo.
- Multiples argumentos se separan con coma *(,)*

*Incorrecto*

```
fn sum(a, b) {
fn sum(a: u32, b) {
fn sum(a, b:i32) {
```

*Correcto*
```
fn sum(a: i32, b: i32) {
```

- Declaraciones que acaban en expresion sin punto y coma, se utilizan como valor de retorno
- Los valores de retorno deben especificar su tipo en la funcion a traves de una flecha *(->)*

*Ejemplos*

```
let function_return = function_with_return(5, 10); //15

fn function_with_return(x: u32, y: u32) -> u32 {
    let sum = x + y;
    sum 
}
```

```
let expression_in_statement = {
    let x = 4;
    x + 1
}; //5
```

**Librerías**

- STD
  - IO
> Inputs del sistema

*Ejemplos*

```
//Recogiendo un valor introducido por consola
io::stdin.read_line()
```

---

## Cargo

**Generar un nuevo proyecto**

Para generar un nuevo proyecto, usamos el comando: 

> cargo new nombre_de_proyecto

**Ejecución**

***1. Ejemplo de ejecución de un proyecto Cargo, desde la terminal.***

Generamos los archivos ejecutables con *Cargo*, dentro del proyecto *Rust* que queremos ejecutar.

Este comando también nos ayuda a actualizar las dependencias que hemos introducido nuevas en el archivo *Cargo.toml*

> ~/proyectos/Estudios/Rust/projects/hello_cargo $ **cargo build**

Comprobamos que los archivos se han generado

```
~/proyectos/Estudios/Rust/projects/hello_cargo/target/debug $ ls
build/  deps/  examples/  hello_cargo.d  hello_cargo.exe\*  hello_cargo.pdb  incremental/
```

Lanzamos el ejecutable que se acaba de generar (Windows)

``` 
~/proyectos/Estudios/Rust/projects/hello_cargo/target/debug $ ./hello_cargo.exe
Hello, world!
```

***2. Compilado y ejecución de un proyecto Rust, desde la terminal en un solo comando***

```
~/proyectos/Estudios/Rust/projects/hello_cargo (master) $ cargo run
Finished dev [unoptimized + debuginfo] target(s) in 0.00s
Running `target\debug\hello_cargo.exe`
Hello, world!
```

***3. Comprobación de posibilidad de compilación del fichero, sin crear el ejecutable***

```
~/proyectos/Estudios/Rust/projects/hello_cargo (master) $ cargo check
Checking hello_cargo v0.1.0 (C:\Users\Baifo\proyectos\Estudios\Rust\projects\hello_cargo)
Finished dev [unoptimized + debuginfo] target(s) in 0.12s
```

**Ficheros**

*Cargo.toml*

Archivo de configuración de Cargo. Controla paquetes y dependencias. 

## Buenas prácticas

- Las constantes deben ir en mayúsculas, con formato *camel_case*.

Correcto

> const HORAS_DE_TRABAJO: u32 = 8;

- Las llaves de apertura de una función, deben estar a la misma altura que la función, separadas con un espacio.

Correcto

> fn main() {
> 
> }

Incorrecto; Debe estar a la misma altura que la función.

> fn main()
> 
> {
> 
> }

Incorrecto; Debe separadarse con un espacio.

> fn main(){
> 
> }

- Rust se indenta con 4 espacios, no con tabulaciones.
- El código generado debe residir en *src*
- Los nombres de variables y de funciones deben ser *camel_case*