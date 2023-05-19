// 1- Escribir un programa que defina una variable de tipo flotante con algún valor, y luego 
// permita al usuario ingresar un número decimal por teclado para multiplicar, dividir, sumar y 
// restar su valor. Se deben imprimir los resultados. 
pub fn e1() {
    let mut buf:String = String::new();
    let mut flt:f32 = 100.0;
    let mut flt2:f32;
    println!("Ingrese el valor por el que dividir el float inicial {}: ", flt);
    std::io::stdin().read_line(&mut buf).expect("Error");
    flt2 = buf.trim().parse().expect("NO ES UN FLOTANTE.");
    flt = flt / flt2;
    buf.clear();
    println!("Ingrese el valor por el que multiplicar el float previo {}: ", flt);
    std::io::stdin().read_line(&mut buf).expect("Error");
    flt2 = buf.trim().parse().expect("Error no es un float");
    flt = flt * flt2;
    buf.clear();
    println!("Ingrese el valor a sumar al float previo {}: ", flt);
    std::io::stdin().read_line(&mut buf).expect("Error");
    flt2 = buf.trim().parse().expect("Error no es un float");
    flt = flt + flt2;
    buf.clear();
    println!("Ingrese el valor a restar del float previo {}: ", flt);
    std::io::stdin().read_line(&mut buf).expect("Error");
    flt2 = buf.trim().parse().expect("Error no es un float");
    flt = flt - flt2;
    buf.clear();
    println!("El valor del float alterado es {}", flt);
}

// 2- Escribir un programa que defina una variable de tipo entero sin signo, y luego imprima su 
// valor en hexadecimal. 
pub fn e2() {
    let mut buf : String = String::new();
    println!("Ingrese el numero a convertir en hexadecimal: ");
    std::io::stdin().read_line(&mut buf).expect("Error en la lectura.");
    let mut num : u32 = buf.trim().parse().expect("Error en parseo.");
    let mut hexa : String = String::new();
    buf.clear();
    while (num != 0){
        match num % 16 {
            0 => buf.push_str("0"),
            1 => buf.push_str("1"),
            2 => buf.push_str("2"),
            3 => buf.push_str("3"),
            4 => buf.push_str("4"),
            5 => buf.push_str("5"),
            6 => buf.push_str("6"),
            7 => buf.push_str("7"),
            8 => buf.push_str("8"),
            9 => buf.push_str("9"),
            10 =>buf.push_str("A"),
            11 =>buf.push_str("B"),
            12 =>buf.push_str("C"),
            13 =>buf.push_str("D"),
            14 =>buf.push_str("E"),
            15 =>buf.push_str("F"),
            _ => println!("Error.")
        }
        num /= 16;
    }
    while !buf.is_empty(){
        hexa.push(buf.pop().expect("Error"));
    }
   println!("Numero en hexadecimal: {}", hexa)
}

// 3- Escribir un programa que defina una variable de tipo booleano, y luego permita al usuario 
// ingresar un valor booleano por teclado para actualizar su valor haciendo las operaciones 
// and y or. Se deben imprimir ambos resultados. 
pub fn e3() {
    let mut b: bool = false;
    let mut b2: bool = false;
    let mut cadena :String = String::new();
    let mut buf:String = String::new();
    loop{
        buf.clear();
        println!();
        println!("El valor del boolean original es: {}.", b2);
        println!("Ingrese 'v' para 'True' o 'f' para 'false'.");
        std::io::stdin().read_line(&mut buf).expect("Error");
        if buf.trim().eq("v") {
            b = true;
        } else if buf.trim().eq("f"){
            b = false;
        }
        println!("Usted ingreso: {}",b);
        println!("Booleano 1 ({}) AND Booleano 2 ({}): {}.", b, b2, b & b2);
        println!("Booleano 1 ({}) OR Booleano 2 ({}): {}.", b, b2, b || b2);
    }
}

// 4- Escribir un programa que defina una tupla que contenga una cadena, un número entero 
// con signo y un valor booleano, y luego imprima cada valor de la tupla 
pub fn e4() {
    let mut tup : (String, bool, i32) = ("Hola Mundo!".to_string(), true, 32);
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
}

// 5- Escribir un programa que defina una variable de tipo cadena, y luego permita al usuario 
// ingresar una cadena por teclado para concatenar su valor. El programa debe imprimir la 
// cadena en mayúsculas.
use std::ops::Add;
pub fn e5() {
    let mut cadena: String = String::new();
    let mut buf : String = String::new();
    loop{
        println!("{} es el string actual!", cadena.to_uppercase());
        buf.clear();
        std::io::stdin().read_line(&mut buf).expect("Error en la lectura de buf!"); 
        if buf.trim().eq("zzz"){
            break;
        }
        cadena.push_str(buf.trim());
    }
}

// 6- Escribir un programa que defina una variable de tipo entero sin signo, y luego permita al 
// usuario ingresar un número entero por teclado para sumarse con la variable definida. El 
// programa debe imprimir el valor del número elevado al cuadrado. 
pub fn e6() {
    let mut uNum : u32 = 10;
    let mut buf : String = String::new();
    println!("Ingrese el numero que desea sumar a {}", uNum);
    std::io::stdin().read_line(&mut buf).expect("Error en la lectura.");
    uNum  += buf.trim().parse::<u32>().expect("Error en el parseo.");
    println!("El resultado final es {}", uNum.pow(2))
}

// 7- Escribir un programa que defina una variable de tipo arreglo que contenga seis números 
// enteros, y luego multiplique cada valor del arreglo por un valor constante definido, 
// modificando el contenido del arreglo.
pub fn e7() {
    let mut arr: [i32; 6] = [1, 2, 3, 4, 5, 6]; 
    let valor :i32 = 2; 
    for i in 0..5 {
        arr[i] *= valor;
        println!("{}", arr[i]);
    }
}

//  8- Escribir un programa que defina una constante de tipo cadena, y luego imprima el 
//  número de veces que un caracter específico ingresado por el usuario aparece en la cadena. 
//  Se debe imprimir el resultado.
pub fn e8() {
    const cadena : &str = "Hola Mundo!";
    let mut car : char;
    let mut buf : String = String::new();
    loop{
        buf.clear();
        println!("Ingrese el caracter: ");
        std::io::stdin().read_line(&mut buf).expect("Error en la lectura");
        if buf.trim() == "zzz" {
            break;
        }
        car = buf.trim().parse().expect("Error en el Parseo.");
        let cant = cadena.matches(car).count();
        println!("El caracter {} aparece {}", car, cant);
    }
    println!("Fin!");
}

// 9- Escribir un programa que defina un arreglo de 5 números enteros, y luego imprima la 
// suma de los valores del  arreglo.
pub fn e9() {
    const dimF : usize = 5;
    let mut arr : [i32; dimF] = [10, 2, 3, 4, 5];
    let mut suma : i32 = 0;
    for i in 0..dimF {
        suma += arr[i];
    }
    println!("Suma final -> {}", suma);
}

// 10- Escribir un programa que defina dos arreglos de 5 números enteros cada uno, y luego 
// cree un tercer arreglo que contenga la suma de los elementos de los dos arreglos 
// originales.
pub fn e10() {
    let v1 : [i32; 5] = [1, 2, 3, 4, 5];
    let v2 : [i32; 5] = [10, 20, 30, 40, 50];
    let v3 : [i32; 5] = [v1[0]+v2[0], v1[1]+v2[1], v1[2]+v2[2], v1[3]+v2[3], v1[4]+v2[4]];
    for i in 0..5 {
        println!("-> {}", v3[i])
    }
}

// 11- Escribir un programa que defina un arreglo de 5 cadenas, y luego permita al usuario 
// ingresar una cadena por teclado. El programa debe imprimir un mensaje si la cadena 
// ingresada por el usuario se encuentra en el arreglo.
use std::io::Read;
pub fn e11() {
    let mut vCadenas : [String; 5] = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new()
    ];
    let mut buf : String = String::new();
    for i in 0..5{
        println!("Ingrese la cadena de la pos {}", i);
        std::io::stdin().read_line(&mut buf).expect("Error en la lectura.");
        vCadenas[i] = buf.trim().to_string();
        buf.clear();
    }
    for i in 0..5{
        println!("Indice: {}, Cadena: {}.", i, vCadenas[i]);
    }
}

// 12- Escribir un programa que defina una tupla que contenga una cadena y un arreglo de 
//  enteros, y luego imprima la cadena y la suma de los valores en el arreglo
pub fn e12() {
    const dimf : usize = 20;
    let mut tup : (String, [i32;5]) = (String::from("Hola Mundo!"), [1, 3, 5, 7, 9] );
    let mut suma : i32 = 0;
    for i in 0..5{
        suma += tup.1[i];
    } 
    println!("{} -> {}", tup.0, suma);
    }
