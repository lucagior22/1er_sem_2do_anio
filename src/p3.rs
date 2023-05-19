use std::collections::VecDeque;

//     1- Escribir un programa que defina una estructura Persona que tenga campos para el
// nombre, la edad y la dirección(que puede ser nulo al momento de la creación de una
    // persona). Para dicha estructura implemente los siguientes métodos:
    // ➢ new: que pasando los parámetros correspondientes, crea una Persona y la retorna.
    // ➢ imprimir: que imprime los datos de la persona sobre el mensaje ejecutado por ej:
    // person.imprimir() , donde person es una variable del tipo Persona.
    // ➢ obtener_edad: retorna la edad de la persona.
    // ➢ actualizar_direccion(nueva_direccion)
    
pub struct Persona{
    nombre : String,
    edad : u32,
    direccion : Option<String>,
}

impl Persona{
    pub fn new(nombre: String, edad: u32, direccion: Option<String>) -> Persona{
            return Persona{
                nombre,
                edad,
                direccion,
        }
    }
    pub fn imprimir(&self){
        if let Some(dir) = &self.direccion{
            println!("Nombre: {}, edad: {}, direccion: {}.", self.nombre, self.edad, dir)
        } else{
        println!("Nombre: {}, edad: {}, no se registra direccion.", self.nombre, self.edad)
        }
    }

    pub fn obtener_edad(&self) -> u32{
        self.edad
    }

    pub fn actualizar_direccion(&mut self, nueva_direccion : Option<String>){
        if let Some(dir) = nueva_direccion{
            self.direccion = Some(dir);
        } else {
            self.direccion = None;
        }
    }
}

//2- Escribir un programa que defina la estructura Rectángulo que tenga campos para la
// longitud y el ancho. Para dicha estructura implemente los siguientes métodos:
// ➢ new: que pasando los parámetros correspondientes, crea un Rectángulo y lo
// retorna.
// ➢ calcular_area: calcular el área y la retorna.
// ➢ calcular_perimetro: calcula el perímetro y lo retorna.
// ➢ es_cuadrado: retorna true si es cuadrado, false caso contrario

pub struct Rectangulo{
    longitud : u32,
    ancho : u32,
}
impl Rectangulo{
    pub fn new(longitud : u32, ancho : u32) -> Rectangulo {
        Rectangulo {longitud, ancho}
    }
    pub fn calcular_area(&self) -> u32{
        self.longitud * self.ancho
    }
    pub fn calcular_perimetro(&self) -> u32{
        (self.longitud * 2) + (self.ancho * 2)
    }
    pub fn es_cuadrado(&self) -> bool{
        self.longitud == self.ancho
    }

}

// 3- Escribir un programa que defina una estructura Fecha que tenga campos para el día, el
// mes y el año. Para dicha estructura implemente los siguientes métodos:
// ➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna.
// ➢ es_fecha_valida: retorna true si es una fecha valida, false caso contrario.//tenga en
// cuenta los años bisiestos también.
// ➢ es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto.
// ➢ sumar_dias(dias): suma la cantidad de días a la fecha, modificándose
// ➢ restar_dias(dias): resta la cantidad de días a la fecha, modificándose
// ➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a
// la fecha pasada por parámetro..

pub struct Fecha{
    dia: u32,
    mes: u32,
    año: u32,
}

impl Fecha{
    pub fn new(dia : u32, mes : u32, año : u32) -> Fecha{
        Fecha { dia: (dia), mes: (mes), año: (año) }
    }

    pub fn igual(&self, fecha : &Fecha) -> bool {
        self.dia == fecha.dia && self.mes == fecha.mes && self.año == fecha.año
    }

    pub fn dias_del_mes(f : &mut Fecha) -> u32 {
        match f.mes {
            1 => 31,
            2 => if f.año % 4 == 0 {
                29
            } else {
                28
            },
            3 => 31,
            4 => 30,
            5 => 31,
            6 => 30,
            7 => 31,
            8 => 31,
            9 => 30,
            10 => 31,
            11 => 30,
            12 => 31,
            _ => u32::max_value(),
        }
    }
    

    pub fn es_fecha_valida(&self) -> bool{
        match self.mes{
            1 => self.dia <= 31,
            2 => if self.es_bisiesto() {
                    self.dia <= 29
                } else {
                    self.dia <= 28
                 },
            3 => self.dia <= 31,
            4 => self.dia <= 30,
            5 => self.dia <= 31,
            6 => self.dia <= 30,
            7 => self.dia <= 31,
            8 => self.dia <= 31,
            9 => self.dia <= 30,
            10 => self.dia <= 31,
            11 => self.dia <= 30,
            12 => self.dia <= 31,
            _ => false,
        }
    } 
    pub fn es_bisiesto(&self) -> bool{
        self.año % 4 == 0
    }
    
    pub fn sumar_dias(&mut self, suma : u32){
        if self.es_fecha_valida(){
            self.dia += suma;
            if self.dia > Fecha::dias_del_mes(self){
                self.dia = self.dia - Fecha::dias_del_mes(self); 
                self.mes += 1;
                if self.mes > 12{
                    self.mes -= 12;
                    self.año += 1;
                } 
            }
        } else {
            println!("La fecha ingresada no es valida!")
        }
    }
    
    pub fn restar_dias(&mut self, resta : u32){
        if self.es_fecha_valida(){
            if resta >= self.dia {
                if self.mes == 1 {
                    if !(self.año == 0){
                        self.año -= 1;
                        self.mes = 12;
                        self.dia = Fecha::dias_del_mes(self) - (resta - self.dia );
                    } else {
                        println!("Se alcanzó el limite inferior en el año de la fecha!");
                    }
                } else {
                    self.mes -= 1;
                    self.dia = Fecha::dias_del_mes(self) - (resta - self.dia);
                }
 
            } else {
                self.dia -= resta;
            }
        } else {
            println!("La fecha ingresada no es valida!")
        }
    }

    pub fn es_mayor(&self, f : &Fecha) -> bool{
        if self.es_fecha_valida() && f.es_fecha_valida(){
            if self.año == f.año {
                if self.mes == f.mes{
                    if self.dia == f.dia || self.dia < f.dia {
                        false
                    } else {
                        true
                    }
                } else if self.mes > f.mes{
                    true
                } else {
                    false
                }
            } else if self.año > f.año{
                true
            } else {
                false
            }
        } else {
            println!("Alguna de las fechas es incorrecta!");
            false
        }
    }

    pub fn get_año(&self) -> u32{
        self.año
    }
    pub fn get_mes(&self) -> u32{
        self.mes
    }
    pub fn get_dia(&self) -> u32{
        self.dia
    }
}

// 4- Escribir un programa que defina la estructura Triángulo que tenga campos para las
// longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos:
// ➢ new: que pasando los parámetros correspondientes, crea un Triángulo y lo retorna.
// ➢ determinar_tipo: retorna el tipo del triángulo, los tipos pueden ser equilátero,
// isósceles o escaleno.
// ➢ calcular_area: calcular el área y la retorna.
// ➢ calcular_perimetro: calcula el perímetro y lo retorna.

pub struct Triangulo{
    base : u32,
    l2 : u32,
    l3 : u32,
}

impl Triangulo{
    pub fn new(base : u32, l2 : u32, l3 : u32) -> Triangulo{
        Triangulo { base: (base), l2: (l2), l3: (l3) }
    }
    pub fn determinar_tipo(&self) -> String{
        if self.base == self.l2 && self.base == self.l3 {
            "Equilatero".to_string()
        } else if self.base != self.l2 && self.base != self.l3 && self.l2 != self.l3{
            "Escaleno".to_string()
        } else {
            "Isosceles".to_string()
        }
    } 
    pub fn calcular_area(&self) -> f64 {
        let s = (self.base + self.l2 + self.l3) as f64 / 2.0;
        (s * (s - self.base as f64) * (s - self.l2 as f64) * (s - self.l3 as f64)).sqrt()
    }
    pub fn calcular_perimetro(&self) -> u32{
        self.base + self.l2 + self.l3
    }
}

// 5- Escribir un programa que defina una estructura Producto que tenga campos para el
// nombre, el precio bruto y un número identificatorio. Para dicha estructura implemente los
// siguientes métodos:
// ➢ new: que pasando los parámetros correspondientes, crea un Producto y lo retorna.
// ➢ calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre
// el precio bruto
// ➢ aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de
// descuento sobre el precio bruto
// ➢ calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el
// precio total a pagar aplicando impuesto y descuento. Tenga en cuenta que los
// parámetros son opcionales

pub struct Producto{
    nombre : String,
    precio_brut : f64,
    id : i32,
}
impl Producto{
    pub fn new(nombre : String, precio_brut : f64, id : i32) -> Producto {
        Producto{
            nombre,
            precio_brut,
            id,
        }
    }
    pub fn calcular_impuestos(&self, porcentaje_de_impuestos : f64) -> f64{
        self.precio_brut * (porcentaje_de_impuestos / 100.0)
    }
    pub fn aplicar_descuento(&self, porcentaje_de_descuento : f64) -> f64{
        self.precio_brut * (porcentaje_de_descuento / 100.0)
    }
    pub fn calcular_precio_total(&self, porcentaje_de_descuento : Option<f64>, porcentaje_de_impuestos : Option<f64>) -> f64 {
        if let Some(imp) = porcentaje_de_impuestos{
            if let Some(desc) = porcentaje_de_descuento{
                self.precio_brut - self.aplicar_descuento(desc) + self.calcular_impuestos(imp)
            } else{
                self.precio_brut + self.calcular_impuestos(imp)
            }
        } else {
            if let Some(desc) = porcentaje_de_descuento{
                self.precio_brut - self.aplicar_descuento(desc)
            } else {
                self.precio_brut
            }
        }
    }
}

// 6- Escribir un programa que defina una estructura Estudiante que tenga campos para el
// nombre, el número de identificación y las calificaciones de exámenes. De cada Examen se
// conoce el nombre de la materia y la nota. Para dichas estructuras implemente los siguientes
// métodos:
// ❖ Examen:
// ➢ new: que pasando los parámetros correspondientes, crea un Examen y lo
// retorna.
// ❖ Estudiante:
// ➢ new: que pasando los parámetros correspondientes, crea un Estudiante y lo
// retorna.
// ➢ obtener_promedio: retorna el promedio de las notas.
// ➢ obtener_calificacion_mas_alta: retorna la nota más alta.
// ➢ obtener_calificacion_mas_baja: retorna la nota más baja.
// Nota: Tenga en cuenta que el Estudiante puede tener entre 0 y n notas de examen.

pub struct Examen{
    nombre : String,
    nota : f64,
}
impl Examen{
    pub fn new(nombre: String, nota : f64) -> Examen{
        Examen{
            nombre,
            nota,
        }
    }
    pub fn get_nota(&self) -> f64 {
        self.nota
    }
}

pub struct Estudiante{
    nombre : String,
    id : i32,
    calificaciones : VecDeque<Examen>,
}
impl Estudiante{
    pub fn new(nombre : String, id : i32, calificaciones : VecDeque<Examen>) -> Estudiante {
        Estudiante{
            nombre,
            id,
            calificaciones,
        }

    }
    pub fn obtener_promedio(&self) -> f64{
        let mut suma = 0.0;
        if self.calificaciones.len() != 0{
            for i in 0..self.calificaciones.len(){
                suma += self.calificaciones.get(i).unwrap().get_nota();
            }            
            suma / self.calificaciones.len() as f64
        } else{
            println!("No existen notas de examenes!");
            -1.0
        }
    }

    pub fn obtener_calificacion_mas_alta(&self) -> f64{
        let mut max = -1.0;
        for i in 0..self.calificaciones.len(){
            if max < self.calificaciones[i].get_nota(){
                max = self.calificaciones[i].get_nota()
            }
        }
        max
    }
    pub fn obtener_calificacion_mas_baja(&self) -> f64{
        let mut min = 999999.0;
        for i in 0..self.calificaciones.len(){
            if min > self.calificaciones[i].get_nota(){
                min = self.calificaciones[i].get_nota()
            }
        }
        min
    }
}

// 7- Defina una estructura llamada ConcesionarioAuto donde se conoce el nombre, la
// dirección y tiene una capacidad máxima para albergar X cantidad de autos. De los autos se
// conocen los campos de la marca, modelo, año, precio bruto y color que pueden ser:rojo,
// verde, azul, amarillo, blanco o negro.
// Para dichas estructuras implemente los siguientes métodos:
// ❖ ConcesionarioAuto:
// ➢ new: que pasando los parámetros correspondientes, crea un
// ConcesionarioAuto y lo retorna.
// ➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene sin superar
// la máxima cantidad para albergarlos y retorna true, en caso de que lo supere
// no lo agrega y retorna false.
// ➢ eliminar_auto(auto): elimina un auto de la lista de autos.
// ➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
// ❖ Auto:
// ➢ new: que pasando los parámetros correspondientes, crea un Auto y lo
// retorna.
// ➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
// ■ si es de color primario le aplica un recargo del 25%, sino le aplica un
// descuento del 10%.
// ■ si la marca es BMW le aplica un recargo del 15%-
// ■ si el año es menor a 2000 le aplica un descuento del 5%.


pub enum Color{
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}


pub struct Auto{
    marca : String,
    modelo : String,
    año : u32,
    precio_bruto : f64,
    color : Color,
}
impl Auto{
    pub fn new(marca : String, modelo : String, año : u32, precio_bruto : f64, color : Color) -> Auto{
        Auto{
            marca,
            modelo,
            año,
            precio_bruto,
            color,
        }
    }
    pub fn calcular_precio(&self) -> f64 {
        let mut rec_desc = 0.0;
        match self.color{
            Color::Rojo | Color::Amarillo | Color::Azul => rec_desc += 25.0,
            _ => rec_desc -= 10.0,
        }
        if self.marca == "BMW" {
            rec_desc += 15.0;
        }
        if self.año < 2000 {
            rec_desc -= 5.0;
        }
        self.precio_bruto + self.precio_bruto * (rec_desc / 100.0)
    }
}

const X : usize = 10;

#[derive(Debug)]
pub struct ConcesionarioAuto{
    nombre : String,
    direccion : String,
    cant_autos : usize,
    autos : Vec<Auto>,
}
impl ConcesionarioAuto{
    pub fn new(nombre : String, direccion : String, x : usize) -> ConcesionarioAuto{
        ConcesionarioAuto{
            nombre,
            direccion,
            cant_autos : x,
            autos : Vec::new(),
        }
    }
    pub fn agregar_auto(&mut self, auto : Auto) -> bool{
        if self.autos.len() <= self.cant_autos{
            println!("Existe espacio en el arreglo!");
            self.autos.push(auto);
            true
        } else {
            println!("No existe espacio en el arreglo!");
            false
        }
    }
    pub fn eliminar_auto(&mut self, auto : Auto) -> bool{
        let mut i = 0;
        let mut index: usize = 0;
        let mut encontre = false;
        if self.autos.len() != 0 && self.autos.contains(&auto){
            while i < self.autos.len()-1 && !encontre{
                if auto == *self.autos.get(i).unwrap(){
                    encontre = true;
                    index = i;
                }
                i += 1;
            } 
            self.autos.swap_remove(index);
            true
        } else {
            println!("No quedan mas elementos en el vector o el elemento no existe!");
            false
        }
    }

    pub fn buscar_auto(&self, auto : Auto) -> &Auto{
        let mut i = 0;
        let mut index: usize = 0;
        let mut encontre = false;
        if self.autos.contains(&auto){
            while i < self.autos.len()-1 && !encontre{
                if auto == *self.autos.get(i).unwrap(){
                    encontre = true;
                    index = i;
                }
                i += 1;
            } 
            self.autos.get(index).unwrap()
        } else {
            println!("No se encontró el elemento buscado!");
            panic!()
        }
    }
}

// 8- Defina la estructura Cancion con campos para el título, el artista y el género. El género
// puede ser rock, pop, rap, jazz, otros. Luego modele una playlist. La playlist está compuesta
// por una lista de canciones y un nombre, y se permiten hacer las siguientes acciones sobre
// ella:
// ➔ agregar canción.
// ➔ eliminar canción.
// ➔ mover canción // mueve la canción a una determinada posición de la playlist.
// ➔ buscar canción por nombre.
// ➔ obtener las canciones de un determinado género.
// ➔ obtener las canciones de un determinado artista.
// ➔ modificar título de la playlist.
// ➔ eliminar todas las canciones

// pub enum Genero{
//     Rock,
//     Pop,
//     Rap,
//     Jazz,
//     Otros,
// }

pub struct Cancion{
    titulo : String,
    artista : String,
    genero : Genero,
}
impl Cancion{
    pub fn new(titulo : String, artista : String, genero : Genero) -> Cancion{
        Cancion{
            titulo,
            artista,
            genero,
        }
    }
    pub fn get_titulo(&self) -> String{
        self.titulo.clone()
    }
    pub fn get_artista(&self) -> String{
        self.artista.clone()
    }
    pub fn get_genero(&self) -> &Genero{
        &self.genero
    }
}

#[derive(Debug)]
pub struct Playlist{
    nombre : String,
    lista_canciones : Vec<Cancion>,
}

impl Playlist{
    pub fn new(nombre : String) -> Playlist{
        Playlist{
            lista_canciones : Vec::new(),
            nombre,
        }
    }
    pub fn agregar_cancion(&mut self, cancion : Cancion){
        self.lista_canciones.push(cancion);
    }
    pub fn eliminar_cancion(&mut self, titulo : String) -> bool{
        let mut i : usize = 0;
        let mut encontre : bool = false;
        while i <= self.lista_canciones.len() - 1 && !encontre{
            if *self.lista_canciones.get(i).unwrap().get_titulo() == titulo{
                self.lista_canciones.remove(i);
                return true
            }
            i += 1;
        }
        false
    }
    pub fn mover_cancion(&mut self, titulo : String, i: usize){
        if let Some(index) = self.buscar_cancion(titulo) {
            self.lista_canciones.swap(index, i)
        }
    }

    pub fn buscar_cancion(&self, titulo : String) -> Option<usize>{
        let mut i : usize = 0;
        let mut encontre : bool = false;
        while i <= self.lista_canciones.len() - 1 && !encontre{
            if self.lista_canciones.get(i).unwrap().get_titulo() == titulo{
                return Some(i)
            }
            i += 1;
        }
        None
    }

    pub fn canciones_genero(&self, genero : Genero) -> Vec<Cancion> {
        let mut v_aux : Vec<Cancion> = Vec::new();
        let mut c_aux : Cancion;
        for i in 0..self.lista_canciones.len() - 1 {
            c_aux = self.lista_canciones.get(i).unwrap().clone(); 
            if  *c_aux.get_genero() == genero{
                v_aux.push(c_aux);
            } 
        }
        v_aux
    }
    pub fn canciones_artista(&self, artista : String) -> Option<Vec<Cancion>> {
        let mut v_aux : Vec<Cancion> = Vec::new();
        let mut c_aux : Cancion;
        if !self.lista_canciones.is_empty(){
            for i in 0..self.lista_canciones.len() - 1 {
                c_aux = self.lista_canciones.get(i).unwrap().clone(); 
                if  *c_aux.get_artista() == artista{
                    v_aux.push(c_aux);
                } 
            }
            Some(v_aux);
        }
        None
    }
    pub fn actualizar_nombre(&mut self, nuevo_titulo : String){
        self.nombre = nuevo_titulo;
    }
    pub fn limpiar_playlist(&mut self){
        self.lista_canciones.clear();
    }

}


// 9.-Dada una cadena de veterinarias se desea implementar un sistema de atención de
// pacientes para cada veterinaria, de la veterinaria se conoce el nombre, la dirección y un id.
// Para la atención de mascotas se requiere administrar una cola de atención. De la mascota
// se conoce el nombre, la edad, el tipo de animal(perro, gato, caballo, otros) y su dueño. Del
// dueño se conoce el nombre, la dirección y un teléfono de contacto. Luego de la atención se
// desea tener un registro de las atenciones realizadas guardando los datos de la mascota, el
// diagnóstico final, tratamiento y fecha de la próxima visita si es que se requiere.
// Dado todo lo mencionado anteriormente implemente los métodos para realizar las
// siguientes acciones:
// Nota: para la fecha utilice lo implementado en el punto 3.


pub enum Animal{
    Perro,
    Gato,
    Caballo,
    Otros,
}


pub struct Dueño{
    nombre: String,
    direccion : String,
    telefono : String,
}
impl Dueño{
    pub fn new(nombre : String, direccion : String, telefono : String) -> Dueño{
     Dueño{ nombre, direccion, telefono}
    }
}


pub struct Mascota{
    nombre : String,
    edad : u32,
    tipo_animal : Animal,
    dueño : Dueño, 
}
impl Mascota{
    pub fn new(nombre : String, edad : u32, tipo_animal : Animal, dueño : Dueño) -> Mascota{
        Mascota { nombre, edad, tipo_animal, dueño}
    }
}


pub struct Atencion{
    mascota : Mascota,
    diagnostico : String,
    tratamiento : String,
    fecha_proxima : Option<Fecha>,
}
impl Atencion{
    pub fn set_diagnostico(&mut self, nuevo_diagnostico : String){
        self.diagnostico = nuevo_diagnostico;
    }
    pub fn set_fecha(&mut self, nueva_fecha : Option<Fecha>){
        self.fecha_proxima = nueva_fecha;
    }
}

#[derive(Debug)]
pub struct Veterinaria{
    nombre: String,
    direccion : String, 
    id : u32,
    cola_de_atencion : VecDeque<Mascota>,
    registro_atenciones : Vec<Atencion>,
}

impl Veterinaria{
    // ➔ crear una veterinaria.
    pub fn new(nombre : String, direccion : String, id : u32) -> Veterinaria{
        Veterinaria { nombre, direccion, id, cola_de_atencion: VecDeque::new(), registro_atenciones: Vec::new()}
    }
    pub fn buscar_mascota(&self, mascota : &Mascota) -> Option<usize>{
        let mut i : usize = 0;
        while i <= self.cola_de_atencion.len()- 1{
            if self.cola_de_atencion.get(i).unwrap() == mascota{
                return Some(i);
            }
            i += 1;
        }
        None
    }
    pub fn buscar_atencion(&self, atencion : &Atencion) ->Option<usize>{
        let mut i : usize = 0;
        let mut encontre : bool = false;
        while i <= self.registro_atenciones.len() - 1 && !encontre{
            if self.registro_atenciones.get(i).unwrap() == atencion{
                return Some(i)
            }
            i += 1;
        }
        None
    }
    // ES BUENA PRACTICA DEVOLVER BOOL PARA NOTIFICAR EL ESTADO DE LA TRANSACCION?
    // ➔ agregar una nueva mascota a la cola de atención de la veterinaria.
    pub fn agregar_mascota(&mut self, mascota : Mascota ){
        self.cola_de_atencion.push_back(mascota);
    }
    // ➔ agregar una nueva mascota a la cola de atención pero que sea la siguiente
    // en atender porque tiene la máxima prioridad.
    pub fn agregar_mascota_prioritario(&mut self, mascota : Mascota){
        self.cola_de_atencion.push_front(mascota);
    }
    // ➔ atender la próxima mascota de la cola.
    pub fn atender_mascota(&mut self) -> Option<Mascota> {
        self.cola_de_atencion.pop_front()
    }
    // ➔ eliminar una mascota específica de la cola de atención dado que se retira.
    pub fn retirar_de_cola(&mut self, mascota : &Mascota){
        if let Some(index) = self.buscar_mascota(&mascota){
            self.cola_de_atencion.remove(index);
            println!("La mascota {} se retiró de la cola!", mascota.nombre);
        } else {
            println!("La mascota {} no se encuentra en la cola!", mascota.nombre);
        }
    }
    // ➔ registrar una atención.
    pub fn registrar_atencion(&mut self, atencion : Atencion){
        self.registro_atenciones.push(atencion);
    }
    // ➔ buscar una atención dado el nombre de la mascota, el nombre del dueño y el
    // teléfono.
    pub fn obtener_atencion(&mut self, atencion : Atencion) -> Option<&mut Atencion>{
        if self.registro_atenciones.contains(&atencion){
            if let Some(index) = self.buscar_atencion(&atencion){
                Some(self.registro_atenciones.get_mut(index).unwrap())
            } else {None}
        } else {
            None
        }
    }
    // ➔ modificar el diagnóstico de una determinada atención.
    pub fn cambiar_diagnostico(&mut self, atencion: Atencion, nuevo_diagnostico : String){
        if let Some(aux_atencion) = self.obtener_atencion(atencion){
            aux_atencion.set_diagnostico(nuevo_diagnostico);
        } else {
            println!("Error en la busqueda!")
        }
    }
    // ➔ modificar la fecha de la próxima visita de una determinada atención.
    pub fn cambiar_fecha(&mut self, atencion: Atencion, nueva_fecha : Option<Fecha>){
        if let Some(aux_atencion) = self.obtener_atencion(atencion){
            aux_atencion.set_fecha(nueva_fecha);
        } else {
            println!("Error en la busqueda!")
        }
    }
    
    // ➔ eliminar una determinada atención.
    pub fn eliminar_atencion(&mut self, atencion: Atencion){
        if let Some(index) = self.buscar_atencion(&atencion){
            self.registro_atenciones.swap_remove(index);
        } else {
            println!("El elemento no existe en el registro!");
        }
    }
}

// 10-Para una biblioteca se desea implementar un sistema de préstamos de libros. De la
// biblioteca se conoce el nombre y la dirección, las copias de los libros a disposición para
// prestar y los préstamos efectuados. Los libros a disposición es un registro donde se indica
// la cantidad de ejemplares que tiene a disposición para prestar de determinado libro. De
// cada libro se conoce el título, autor, número de páginas, género(novela, infantil, técnico,
// otros). Para registrar un préstamo se requiere el libro, el cliente, la fecha de vencimiento del
// préstamo, la fecha de devolución y el estado que puede ser devuelto o en préstamo. Del
// cliente se conoce el nombre, teléfono y dirección de correo electrónico.

struct Cliente{
    nombre : String,
    telefono : String,
    mail : String
}

impl Cliente{
    fn igual(&self, cliente : &Cliente) -> bool{
        self.nombre == cliente.nombre && self.telefono == cliente.telefono && self.mail == cliente.mail
    }
}

enum Genero{
    Novela,
    Infantil,
    Tecnico,
    Otros
}
impl Genero{
    fn igual(&self, genero : &Genero) -> bool{
        match(*self, *genero){
            (Genero::Novela, Genero::Novela) => true,
            (Genero::Infantil, Genero::Infantil) => true,
            (Genero::Tecnico, Genero::Tecnico) => true,
            (Genero::Otros, Genero::Otros) => true,
            _ => false,
        }
    }
}

//Usar enum en vez de struct?
struct Copias{
    libro : Libro,
    cant : u32
}

enum Estado{
    EnPrestamo,
    Devuelto,
}
impl Estado{
    fn igual(&self, estado : &Estado) -> bool{
        match(*self, *estado){
            (Estado::EnPrestamo, Estado::EnPrestamo) => true,
            (Estado::Devuelto, Estado::Devuelto) => true,
            _ => false,
        }
    }
}

struct Prestamo<'a>{
    libro : &'a Libro,
    cliente : &'a Cliente,
    fecha_vencimiento : Fecha,
    fecha_devolucion : Option<Fecha>,
    devuelto : Estado
}

struct Libro{ 
    titulo : String,
    autor : String,
    paginas : u32,
    genero : Genero,
}
impl Libro{
    fn igual(self, libro : &Libro ) -> bool{
        self.titulo == *libro.titulo && self.autor == *libro.autor && self.paginas == libro.paginas && self.genero.igual(&libro.genero)
    }
}

struct Biblioteca<'a>{
    nombre : String,
    direccion : String,
    libros_disponibles : Vec<Copias>, // Como se podria implementar esto en un hashmap
    prestamos : Vec<Prestamo<'a>>
}
// Implemente los métodos necesarios para realizar las siguientes acciones:
impl <'a> Biblioteca <'a>{
    fn new(nombre : String, direccion : String) -> Biblioteca<'a> {
        Biblioteca{
            nombre,
            direccion,
            libros_disponibles : Vec::new(),
            prestamos : Vec::new()
        }
    }
    
// ➔ obtener cantidad de copias: dado un determinado libro retorna la
// cantidad de copias a disposición que hay para prestar de dicho libro.
    fn cantidad_copias(&self, libro : &Libro) -> Option<u32>{
        if !self.libros_disponibles.is_empty(){
            for i in self.libros_disponibles{
                if i.libro.igual(&libro) {
                    return Some(i.cant)
                }
            } 
            None
        } else {
            None
        }
    }
// ➔ decrementar cantidad de copias a disposición; dado un libro decrementa en 1
// la cantidad de copias de libros a disposición para prestar.
    fn decrementar_copias(&self, libro : &Libro) -> bool{
        if !self.libros_disponibles.is_empty(){
            for mut i in self.libros_disponibles{
                if i.libro.igual(libro){
                    i.cant -= 1;
                    return true;
                }
            }
        } 
        false
    }
// ➔ incrementar cantidad de copias a disposición: dado un libro incremente en 1
// la cantidad de copias del libro a disposición para ser prestado.
fn incrementar_copias(&self, libro : &Libro) -> bool{
    if !self.libros_disponibles.is_empty(){
        for mut i in self.libros_disponibles{
            if i.libro.igual(libro){
                i.cant += 1;
                return true;
            }
        }
        let mut c = Copias{libro : *libro, cant: 1};
        self.libros_disponibles.push(c);
    } 
    false

}
// ➔ contar préstamos de un cliente: devuelve la cantidad de préstamos en estado
// “en préstamo” de un determinado cliente.
    fn contar_prestamos(&self, cliente : &Cliente) -> Option<u32>{
        let mut suma : u32 = 0;
        if !self.prestamos.is_empty(){
            for i in self.prestamos{
                if i.cliente.igual(&cliente) && i.devuelto.igual(&Estado::EnPrestamo){
                    suma += 1;
                }
            }
            return Some(suma) 
        }
        None
    }
// ➔ realizar un préstamo de un libro para un cliente: crea un préstamo de un libro
// para un determinado cliente cumpliendo con lo siguiente
// ◆ el cliente no tenga más de 5 préstamos en el estado “en préstamo”
// ◆ haya al menos una copia disponible en el registro de copias a
// disposición.
// De ser así descuenta 1 en el registro de “copias a disposición” y
// retorna true, si no cumple con alguna de las condiciones retorna false.
    fn realizar_prestamo(&self, libro : &Libro, cliente : &Cliente, fecha_vencimiento : Fecha) -> bool{
        let mut bool : bool = false;
        if !self.libros_disponibles.is_empty() && self.contar_prestamos(&cliente).unwrap() <= 5 && self.cantidad_copias(&libro).unwrap() > 0{
               self.decrementar_copias(&libro);
               let mut p = Prestamo{
                    libro,
                    cliente,
                    fecha_vencimiento,
                    fecha_devolucion : None,
                    devuelto : Estado::EnPrestamo,
               };
               self.prestamos.push(p);
               true
        } else {
            false
        }
    }
// ➔ ver préstamos a vencer el los próximos días: retorna una lista de préstamos a
// vencer el los próximos días, el valor de días es pasado por parámetro.
    fn proximos_a_vencer(&self, fecha_actual : &Fecha) -> Option<Vec<&Prestamo>>{
        if !self.prestamos.is_empty(){
            let mut v_aux : Vec<&Prestamo> = Vec::new();
            let mut aux: Fecha = Fecha{
                dia : fecha_actual.dia, 
                mes : fecha_actual.mes, 
                año : fecha_actual.año};
            aux.sumar_dias(5);
            for i in self.prestamos{
                if i.fecha_vencimiento.es_mayor(&fecha_actual) && !i.fecha_vencimiento.es_mayor(&aux){
                    v_aux.push(&i);
                } 
            }
            return Some(v_aux)
        }
        None
    }
// ➔ ver los préstamos vencidos: retorna una lista de préstamos en el estado “en
// préstamos” donde la fecha de vencimiento es menor a la fecha actual.
    fn prestamos_vencidos(&self, fecha_actual : &Fecha) -> Option<Vec<&Prestamo>>{
        if !self.prestamos.is_empty(){
            let mut v_aux : Vec<&Prestamo> = Vec::new();
            for i in self.prestamos{
                if !i.fecha_vencimiento.es_mayor(&fecha_actual) && !i.fecha_vencimiento.igual(fecha_actual) && i.devuelto.igual(&Estado::EnPrestamo){
                    v_aux.push(&i);
                } 
            }
            return Some(v_aux)
        }
        None
}
// ➔ buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si
// existe.
    fn buscar_prestamo(&self, libro : &Libro, cliente : &Cliente) -> Option<&mut Prestamo>{ //Prestamo por valor o por referencia? 
        if !self.prestamos.is_empty(){
            for mut i in self.prestamos{
                if i.libro.igual(&libro) && i.cliente.igual(&cliente){
                    return Some(&mut i);
                }
            }
            return None
        }
        None
    }
// ➔ devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al
// estado “devuelto”, se registra la fecha de devolución y se incrementa la
// cantidad de libros en 1 del libro devuelto en el registro de copias a
// disposición.
    fn devolver_libro(&self, libro : &Libro, cliente : &Cliente, fecha_actual : &Fecha) -> bool{
        if let Some(prestamo) = self.buscar_prestamo(&libro, &cliente){
            let mut aux: Fecha = Fecha{
                dia : fecha_actual.dia, 
                mes : fecha_actual.mes, 
                año : fecha_actual.año};
            prestamo.devuelto = Estado::Devuelto;
            prestamo.fecha_devolucion = Some(aux);
            self.incrementar_copias(&libro);
            return true
        }
        false
    }

}

// Nota: para la fecha utilice lo implementado en el punto 3.