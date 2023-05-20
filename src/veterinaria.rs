use std::collections::VecDeque;

pub struct Fecha{
    dia: u32,
    mes: u32,
    año: u32,
}

impl Fecha{
    pub fn new(dia : u32, mes : u32, año : u32) -> Fecha{
        Fecha {dia, mes, año}
    }

    pub fn igual(&self, fecha : &Fecha) -> bool {
        self.dia == fecha.dia && self.mes == fecha.mes && self.año == fecha.año
    }

    pub fn dias_del_mes(&self) -> Option<u32> {
        match self.mes {
            1 => Some(31),
            2 => if self.año % 4 == 0 {
                Some(29)
            } else {
                Some(28)
            },
            3 => Some(31),
            4 => Some(30),
            5 => Some(31),
            6 => Some(30),
            7 => Some(31),
            8 => Some(31),
            9 => Some(30),
            10 => Some(31),
            11 => Some(30),
            12 => Some(31),
            _ => None,
        }
    }

    pub fn es_fecha_valida(&self) -> bool{
        if self.dia > 0{
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
        } else {
            false
        }
    }
     
    pub fn es_bisiesto(&self) -> bool{
        (self.año % 4 == 0) && (self.año % 100 != 0 || self.año % 400 == 0)
    }
    
    pub fn sumar_dias(&mut self, suma : u32) -> bool{
        if self.es_fecha_valida(){
            self.dia += suma;
            let dias_del_mes = self.dias_del_mes().unwrap(); 
            if self.dia > dias_del_mes{
                self.dia = self.dia - dias_del_mes; 
                self.mes += 1;
                if self.mes > 12{
                    self.mes -= 12;
                    self.año += 1;
                }
                return true 
            }
            true
        } else {
            println!("La fecha ingresada no es valida!");
            false
        }
    }
    
    pub fn restar_dias(&mut self, resta : u32) -> bool{
        if self.es_fecha_valida(){
            let dias_del_mes = self.dias_del_mes().unwrap();
            if resta >= self.dia {
                if self.mes == 1 {
                    if !self.año == 0{
                        self.año -= 1;
                        self.mes = 12;
                        self.dia = dias_del_mes - (resta - self.dia );
                        true
                    } else {
                        println!("Se alcanzó el limite inferior en el año de la fecha!");
                        false
                    }
                } else {
                    self.mes -= 1;
                    self.dia = dias_del_mes - (resta - self.dia);
                    true
                }
            } else {
                self.dia -= resta;
                true
            }
        } else {
            println!("La fecha ingresada no es valida!");
            false
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
impl Animal{
    pub fn igual(&self, animal : &Animal) -> bool{
        match (self, animal){
            (Animal::Perro, Animal::Perro) => true,
            (Animal::Gato, Animal::Gato) => true,
            (Animal::Caballo, Animal::Caballo) => true,
            (Animal::Otros, Animal::Otros) => true,
            _ => false,
        }
    }
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

    pub fn igual(&self, dueño : &Dueño) -> bool{
        self.nombre == dueño.nombre && self.direccion == dueño.direccion && self.telefono == dueño.telefono
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
    pub fn igual(&self, mascota : &Mascota) -> bool{
        self.nombre == mascota.nombre && self.edad == mascota.edad && self.tipo_animal.igual(&mascota.tipo_animal) && self.dueño.igual(&mascota.dueño)
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
    pub fn igual(&self, atencion : &Atencion) -> bool{
        let mut ok = self.mascota.igual(&atencion.mascota) && self.diagnostico == atencion.diagnostico && self.tratamiento == atencion.tratamiento;
        if let Some(self_fecha) = &self.fecha_proxima{
            if let Some(atencion_fecha) = &atencion.fecha_proxima{
                ok = ok && self_fecha.igual(&atencion_fecha);
            }
        } else{
            ok = false;
        }
        ok
    }
}

pub struct Veterinaria<'a>{
    nombre: String,
    direccion : String, 
    id : u32,
    cola_de_atencion : VecDeque<&'a Mascota>,
    registro_atenciones : Vec<Atencion>,
}

impl <'a>Veterinaria<'a>{
    // ➔ crear una veterinaria.
    pub fn new(nombre : String, direccion : String, id : u32) -> Veterinaria<'a>{
        Veterinaria { nombre, direccion, id, cola_de_atencion: VecDeque::new(), registro_atenciones: Vec::new()}
    }
    fn buscar_mascota(&self, mascota : &Mascota) -> Option<usize>{
        for i in 0..self.cola_de_atencion.len() - 1{
            if mascota.igual(self.cola_de_atencion[i]){
                return Some(i)
            }
        }
        None
    }
    fn buscar_atencion(&self, atencion : &Atencion) ->Option<usize>{
        for i in 0..self.cola_de_atencion.len() - 1{
            if atencion.igual(&self.registro_atenciones[i]){
                return Some(i)
            }
        }
        None
    }
    // ES BUENA PRACTICA DEVOLVER BOOL PARA NOTIFICAR EL ESTADO DE LA TRANSACCION?
    // ➔ agregar una nueva mascota a la cola de atención de la veterinaria.
    pub fn agregar_mascota(&mut self, mascota : &'a Mascota ){
        self.cola_de_atencion.push_back(&mascota);
    }
    // ➔ agregar una nueva mascota a la cola de atención pero que sea la siguiente
    // en atender porque tiene la máxima prioridad.
    pub fn agregar_mascota_prioritario(&mut self, mascota : &'a Mascota){
        self.cola_de_atencion.push_front(&mascota);
    }
    // ➔ atender la próxima mascota de la cola.
    pub fn atender_mascota(&mut self) -> Option<&Mascota> {
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
    pub fn obtener_atencion(&self, atencion : Atencion) -> Option<&Atencion>{
        for i in &self.registro_atenciones{
            if i.igual(&atencion){
                return Some(i)
            }
        }
        return None
    }
    // ➔ modificar el diagnóstico de una determinada atención.
    pub fn cambiar_diagnostico(&mut self, atencion: Atencion, nuevo_diagnostico : String) -> bool{
        for i in &mut self.registro_atenciones{
            if i.igual(&atencion){
                i.diagnostico = nuevo_diagnostico;
                return true
            }
        }
        return false
    }
    // ➔ modificar la fecha de la próxima visita de una determinada atención.
    pub fn cambiar_fecha(&mut self, atencion: Atencion, nueva_fecha : Option<Fecha>) -> bool{
        for i in &mut self.registro_atenciones{
            if i.igual(&atencion){
                i.fecha_proxima = nueva_fecha;
                return true
            }
        }
        return false
    }
    // ➔ eliminar una determinada atención.
    pub fn eliminar_atencion(&mut self, atencion: &Atencion){
        if let Some(index) = self.buscar_atencion(&atencion){
            self.registro_atenciones.swap_remove(index);
        } else {
            println!("El elemento no existe en el registro!");
        }
    }
}