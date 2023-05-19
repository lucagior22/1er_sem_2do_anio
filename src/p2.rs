// 1-Definir la función llamada es_parque recibe como parámetro un número entero y retorna
// true si el número es par, false caso contrario.
fn es_par(entero:i32) -> bool{
    if entero % 2 == 0 {
        return true;
    }
    else{
        return false;
    }
}

// 2- Definir la función llamada es_primoque recibe un número entero positivo mayor a 1 y
// retorna true si es primo, false caso contrario
fn es_primo(n : u32) -> bool{
    let mut ok : bool = true;
    for i in 2..n/2{
        println!("{}",n);
        if n % i == 0{
            ok = false;
        } 
    }
    return ok;
}

// 3- Definir la función llamada suma_paresque recibe como parámetro un arreglo de
// números enteros y retorna la suma de los números pares.
fn suma_pares(arreglo : [i32; 10])-> i32{
    let mut suma:i32 = 0;
    for i in 0..arreglo.len(){
        if es_par(arreglo[i]){
            suma += arreglo[i];
        }  
    }
    return suma;
}

// 4- Definir la función llamada cantidad_imparesque recibe como parámetro un arreglo de
// números enteros y retorna la cantidad de números impares
fn cantidad_impares(arr : [i32; 10]) -> i32{
    let mut suma : i32 = 0;
    for i in 0..arr.len(){
        if !es_par(arr[i]){
            suma+= 1;
        }  
    }
  return suma;
}

// 5-Defina la función llamada duplicar_valoresque recibe un arreglo de números flotantes y
// retorna un arreglo nuevo con los valores duplicados del parámetro
fn duplicar_valores(arr : [i32; 10]) -> [i32;10]{
    let mut aux : [i32;10] = [0;10];
    for i in 0..arr.len(){
        aux[i] = arr[i]*2;
    }
    return aux;
}


// 6-Definir la función llamada longitud_de_cadenasque recibe un arreglo de String y retorna
// un arreglo con la longitud de las cadenas del parámetro, correspondiéndose en posición del
// arreglo.
fn longitud_de_cadenas(arr : [String; 10]) -> [usize; 10]{
    let mut aux : [usize; 10] = [0;10];
    for i in 0..arr.len(){
        aux[i] = arr[i].len();
    }
    return aux;
}

// 7-Definir la función llamada cantidad_de_mayoresque recibe como parámetro un arreglo
// de números enteros y un número entero llamado límite. Esta función retorna la cantidad de
// números mayores al límite que tiene el arreglo
fn cantidad_de_mayores(arr : [i32; 10], lim : i32) -> i32 {
    let mut suma : i32 = 0;
    for i in 0..arr.len(){
        if arr[i] > lim {
            suma += 1;
        }
    }
    return suma;
}

// 8- Definir la función llamada sumar_arreglosque recibe 2 arreglos del mismo tamaño de
// números flotantes y retorna un nuevo arreglo que contiene la suma de los elementos de los
// arreglos pasados por parámetro, correspondiendose el resultado con cada posición de los
// arreglos pasados por parámetro.
fn sumar_arreglos(arr1 : [f32;10], arr2 : [f32;10]) -> [f32;10] {
    let mut arr3 : [f32;10] = [0.0;10];
    for i in 0..arr1.len() {
        arr3[i] = arr1[i] + arr2[i]; 
    }
    return arr3;
}

// 9-Definir la función llamada cantidad_en_rangoque recibe 3 parámetros: 1 arreglo de
// enteros, un número entero llamado inferior y otro número entero llamado superior. Esta
// función retorna la cantidad de números del arreglo que están entre el rango de los
// parámetros inferior y superior inclusive.
fn cantidad_en_rango(arr : [i32; 10], inf : i32, sup : i32) -> i32 {
    let mut suma : i32 = 0;
    for i in 0..arr.len(){
        if (arr[i] > inf) & (arr[i] < sup) {
            suma += 1;
        }
    }
    return suma;
}

// 10-Definir la función llamada cantidad_de_cadenas_mayor_aque recibe como parámetros
// un arreglo de String y un entero llamado límite. Esta función retorna la cantidad de Strings
// del arreglo que son de longitud mayor al parámetro límite.
fn cantidad_de_cadenas_mayor_que(arr : [String; 10], lim : usize) -> i32 {
    let mut suma : i32 = 0;
    for i in 0..arr.len(){
        if arr[i].len() > lim {
            suma += 1;
        }
    }
    return suma;
}

// 11-Definir la función llamada multiplicar_valoresque recibe como parámetro un arreglo de
// enteros y otro número entero llamado factor. Esta función multiplica los valores del arreglo
// por el parámetro factor modificándolo.
fn multiplicar_valores(arr : &mut [i32;10], fac: i32){
    for i in 0..arr.len(){
        arr[i] *= fac;
    }

}
// 12-Definir una función llamada reemplazar_paresque recibe un arreglo de enteros y
// reemplaza todos los números pares por -1.
fn reemplazar_pares(arr : &mut [i32; 10]){
    let mut suma : i32 = 0;
    for i in 0..arr.len(){
        if es_par(arr[i]){
            arr[i] = -1;
        }  
    }
}

// 13-Definir una función llamada ordenar_nombresque recibe un arreglo de String y los
// ordena en orden alfabético.
fn ordenar_nombres(arr : &mut [String; 10]){
    let mut elem : String = String::new();
    let mut q : usize;
    for i in 0..arr.len()-1{
        q = i;
        for j in i..arr.len(){
            if arr[j] < arr[q] {
                q = j;
            }
        }
        elem = arr[q].clone();
        arr[q] = arr[i].clone();
        arr[i] = elem;
        
    }
}
// 14-Definir una función llamada incrementarque recibe como parámetro un número flotante
// e incrementa en 1 su valor
fn incrementar(num: &mut f32){
    *num+= 1.0;
}
