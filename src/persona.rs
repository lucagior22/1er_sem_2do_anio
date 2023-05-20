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
        Persona{
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