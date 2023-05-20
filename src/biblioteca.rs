#[derive(Debug)]
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

// 10-Para una biblioteca se desea implementar un sistema de préstamos de libros. De la
// biblioteca se conoce el nombre y la dirección, las copias de los libros a disposición para
// prestar y los préstamos efectuados. Los libros a disposición es un registro donde se indica
// la cantidad de ejemplares que tiene a disposición para prestar de determinado libro. De
// cada libro se conoce el título, autor, número de páginas, género(novela, infantil, técnico,
// otros). Para registrar un préstamo se requiere el libro, el cliente, la fecha de vencimiento del
// préstamo, la fecha de devolución y el estado que puede ser devuelto o en préstamo. Del
// cliente se conoce el nombre, teléfono y dirección de correo electrónico.

#[derive(Debug)]
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

#[derive(Debug)]
enum Genero{
    Novela,
    Infantil,
    Tecnico,
    Otros
}
impl Genero{
    fn igual(&self, genero : &Genero) -> bool{
        match(self, genero){
            (Genero::Novela, Genero::Novela) => true,
            (Genero::Infantil, Genero::Infantil) => true,
            (Genero::Tecnico, Genero::Tecnico) => true,
            (Genero::Otros, Genero::Otros) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Copias<'a>{
    libro : &'a Libro,
    cant : u32
}

#[derive(Debug)]
enum Estado{
    EnPrestamo,
    Devuelto,
}
impl Estado{
    fn igual(&self, estado : &Estado) -> bool{
        match(self, estado){
            (Estado::EnPrestamo, Estado::EnPrestamo) => true,
            (Estado::Devuelto, Estado::Devuelto) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Prestamo<'a>{
    libro : &'a Libro,
    cliente : &'a Cliente,
    fecha_vencimiento : Fecha,
    fecha_devolucion : Option<Fecha>,
    devuelto : Estado
}

#[derive(Debug)]
struct Libro{ 
    titulo : String,
    autor : String,
    paginas : u32,
    genero : Genero,
}
impl Libro{
    fn igual(&self, libro : &Libro ) -> bool{
        self.titulo == *libro.titulo && self.autor == *libro.autor && self.paginas == libro.paginas && self.genero.igual(&libro.genero)
    }
}

#[derive(Debug)]
struct Biblioteca<'a>{
    nombre : String,
    direccion : String,
    libros_disponibles : Vec<Copias<'a>>, // Como se podria implementar esto en un hashmap
    prestamos : Vec<Prestamo<'a>>
}
// Implemente los métodos necesarios para realizar las siguientes acciones:
impl <'a> Biblioteca <'a>{
    fn new(nombre : String, direccion : String, libros_disponibles : Vec<Copias<'a>>, prestamos : Vec<Prestamo<'a>>) -> Biblioteca<'a> {
        Biblioteca{
            nombre,
            direccion,
            libros_disponibles,
            prestamos : prestamos,
        }
    }   
// ➔ obtener cantidad de copias: dado un determinado libro retorna la
// cantidad de copias a disposición que hay para prestar de dicho libro.
    fn cantidad_copias(&self, libro : &Libro) -> Option<u32>{
        if !self.libros_disponibles.is_empty(){
            for i in &self.libros_disponibles{
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
    fn decrementar_copias(&mut self, libro : &Libro) -> bool{
        if !self.libros_disponibles.is_empty(){
            for i in &mut self.libros_disponibles{
                if i.cant != 0{
                    if i.libro.igual(libro){
                        i.cant -= 1;
                        return true;
                    }
                } else {
                    return false
                }
            }
        } 
        false
    }
// ➔ incrementar cantidad de copias a disposición: dado un libro incremente en 1
// la cantidad de copias del libro a disposición para ser prestado.
// DUDA : Como trato las fechas invalidas?
    fn incrementar_copias(&mut self, libro : &Libro) -> bool{
        if !self.libros_disponibles.is_empty(){
            for mut i in &mut self.libros_disponibles{
                if i.libro.igual(libro){
                    i.cant += 1;
                    return true;
                }
            }
        } 
        false
    }
// ➔ contar préstamos de un cliente: devuelve la cantidad de préstamos en estado
// “en préstamo” de un determinado cliente.
    fn contar_prestamos(&self, cliente : &Cliente) -> Option<u32>{
        let mut suma : u32 = 0;
        if !self.prestamos.is_empty(){
            for i in &self.prestamos{
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
    fn realizar_prestamo(&mut self, libro : &'a Libro, cliente : &'a Cliente, fecha_vencimiento : Fecha) -> bool{
        if !self.libros_disponibles.is_empty() && self.contar_prestamos(&cliente).unwrap() <= 5 && self.cantidad_copias(&libro).unwrap() > 0 && fecha_vencimiento.es_fecha_valida(){
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
    fn proximos_a_vencer(&self, fecha_actual : &Fecha, dias : u32) -> Option<Vec<&Prestamo>>{
        if !self.prestamos.is_empty(){
            let mut v_aux : Vec<&Prestamo> = Vec::new();
            let mut aux: Fecha = Fecha{
                dia : fecha_actual.dia, 
                mes : fecha_actual.mes, 
                año : fecha_actual.año};
            aux.sumar_dias(dias);
            for i in &self.prestamos{
                if i.fecha_vencimiento.es_fecha_valida() && (i.fecha_vencimiento.es_mayor(&fecha_actual) || i.fecha_vencimiento.igual(fecha_actual)) && !i.fecha_vencimiento.es_mayor(&aux){
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
            for i in &self.prestamos{
                if i.fecha_vencimiento.es_fecha_valida() && !i.fecha_vencimiento.es_mayor(&fecha_actual) && !i.fecha_vencimiento.igual(fecha_actual) && i.devuelto.igual(&Estado::EnPrestamo){
                    v_aux.push(&i);
                } 
            }
            return Some(v_aux)
        }
        None
}
// ➔ buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si
// existe.
    fn buscar_prestamo(&mut self, libro : &Libro, cliente : &Cliente) -> Option<&mut Prestamo<'a>>{ //Prestamo por valor o por referencia? 
        if !self.prestamos.is_empty(){
            for mut i in &mut self.prestamos{
                if i.libro.igual(&libro) && i.cliente.igual(&cliente){
                    return Some(i);
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
    fn devolver_libro(&mut self, libro : &Libro, cliente : &Cliente, fecha_actual : &Fecha) -> bool{
        if let Some(prestamo) = self.buscar_prestamo(&libro, &cliente){
            let aux: Fecha = Fecha{
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

pub fn main(){
    
    let lib1: Libro = Libro{ titulo : "Encuentros".to_string(), autor : "Gabriel Rolon".to_string(), paginas : 341, genero : Genero::Tecnico};
    let lib2: Libro = Libro{ titulo : "Star Wars I".to_string(), autor : "George Lucas".to_string(), paginas : 312, genero : Genero::Novela};
    let lib3: Libro = Libro{ titulo : "Star Wars II".to_string(), autor : "George Lucas".to_string(), paginas : 544, genero : Genero::Novela};
    let lib4: Libro = Libro{ titulo : "Star Wars III".to_string(), autor : "George Lucas".to_string(), paginas : 349, genero : Genero::Novela};
    let lib5: Libro = Libro{ titulo : "Star Wars IV".to_string(), autor : "George Lucas".to_string(), paginas : 423, genero : Genero::Novela};
    let lib6: Libro = Libro{ titulo : "Star Wars V".to_string(), autor : "George Lucas".to_string(), paginas : 322, genero : Genero::Novela};
    let lib7: Libro = Libro{ titulo : "Star Wars VI".to_string(), autor : "George Lucas".to_string(), paginas : 390, genero : Genero::Novela};
    let lib8: Libro = Libro{ titulo : "Manuelita la tortuga".to_string(), autor : "Maria Elena Walsh".to_string(), paginas : 40, genero : Genero::Infantil};
    let cop1: Copias = Copias{libro : &lib1, cant : 5}; 
    let cop2: Copias = Copias{libro : &lib2, cant : 2}; 
    let cop3: Copias = Copias{libro : &lib3, cant : 1}; 
    let cop4: Copias = Copias{libro : &lib4, cant : 0}; 
    let cop5: Copias = Copias{libro : &lib5, cant : 10}; 
    let cop6: Copias = Copias{libro : &lib6, cant : 3}; 
    let cop7: Copias = Copias{libro : &lib7, cant : 2}; 
    let cop8: Copias = Copias{libro : &lib8, cant : 14}; 
    
    let mut aux_vec: Vec<Copias> = Vec::new();
    aux_vec.push(cop1);
    aux_vec.push(cop2);
    aux_vec.push(cop3);
    aux_vec.push(cop4);
    aux_vec.push(cop5);
    aux_vec.push(cop6);
    aux_vec.push(cop7);
    aux_vec.push(cop8);
    
    let cl1: Cliente = Cliente{ nombre : "Juan".to_string(), telefono : "123423".to_string(), mail : "yo.com".to_string()};
    let cl2: Cliente = Cliente{ nombre : "Tomas".to_string(), telefono : "254563".to_string(), mail : "vos.com".to_string()};
    let cl3: Cliente = Cliente{ nombre : "Pancho".to_string(), telefono : "225541".to_string(), mail : "el.com".to_string()};
    let cl4: Cliente = Cliente{ nombre : "Carlos".to_string(), telefono : "540223".to_string(), mail : "vosotros.com".to_string()};
    let cl5: Cliente = Cliente{ nombre : "Martin".to_string(), telefono : "443512".to_string(), mail : "vosotros.com".to_string()};
    let fec1: Fecha = Fecha::new(23, 5, 2023);
    let fec2: Fecha = Fecha::new(0, 5, 2023);
    let fec3: Fecha = Fecha::new(18, 5, 2023);
    let fec4: Fecha = Fecha::new(26, 5, 2023);
    let fec5: Fecha = Fecha::new(5, 4, 2023);
    let prest1: Prestamo = Prestamo{libro : &lib1, cliente : &cl1, fecha_vencimiento : fec1, fecha_devolucion : None, devuelto : Estado::EnPrestamo};
    let prest2: Prestamo = Prestamo{libro : &lib1, cliente : &cl3, fecha_vencimiento : fec2, fecha_devolucion : None, devuelto : Estado::EnPrestamo};
    let prest3: Prestamo = Prestamo{libro : &lib4, cliente : &cl4, fecha_vencimiento : fec3, fecha_devolucion : None, devuelto : Estado::EnPrestamo};
    let prest4: Prestamo = Prestamo{libro : &lib6, cliente : &cl4, fecha_vencimiento : fec4, fecha_devolucion : None, devuelto : Estado::EnPrestamo};
    let prest5: Prestamo = Prestamo{libro : &lib8, cliente : &cl2, fecha_vencimiento : fec5, fecha_devolucion : None, devuelto : Estado::EnPrestamo};
    
    let mut aux_vec_pres: Vec<Prestamo> = Vec::new();
    
    aux_vec_pres.push(prest1);
    aux_vec_pres.push(prest2);
    aux_vec_pres.push(prest3);
    aux_vec_pres.push(prest4);
    aux_vec_pres.push(prest5);
    
    let mut biblioteca: Biblioteca = Biblioteca::new("Biblioteca Manuel Belgrano".to_string(), "Calle 123".to_string(), aux_vec, aux_vec_pres);

    println!("{:#?}", biblioteca);
    
    println!("El libro {:?} tiene {:?} copias", lib1 ,biblioteca.cantidad_copias(&lib1));
    
    println!("{:?} tiene {:?} prestamos", cl4 ,biblioteca.contar_prestamos(&cl5));
    
    biblioteca.decrementar_copias(&lib1);
    biblioteca.decrementar_copias(&lib1);
    biblioteca.decrementar_copias(&lib1);
    biblioteca.decrementar_copias(&lib1);
    biblioteca.decrementar_copias(&lib1);
    biblioteca.decrementar_copias(&lib1);
    biblioteca.decrementar_copias(&lib1);
    biblioteca.decrementar_copias(&lib1);
    println!("El libro {:?} tiene {:?} copias", lib1 ,biblioteca.cantidad_copias(&lib1));
    
    biblioteca.incrementar_copias(&lib1);
    biblioteca.incrementar_copias(&lib1);
    biblioteca.incrementar_copias(&lib1);
    biblioteca.incrementar_copias(&lib1);
    println!("El libro {:?} tiene {:?} copias", lib1 ,biblioteca.cantidad_copias(&lib1));
    
    let fecha_actual = Fecha::new(18, 5, 2023);
    
    println!("Proximos a vencer: {:#?}", biblioteca.proximos_a_vencer(&fecha_actual, 365));
    println!("Prestamos vencidos: {:#?}", biblioteca.prestamos_vencidos(&fecha_actual));
    
    println!("{:#?}", biblioteca.buscar_prestamo(&lib1, &cl1));
    println!("{:#?}", biblioteca.devolver_libro(&lib1, &cl1, &fecha_actual));
    println!("Proximos a vencer: {:#?}", biblioteca.proximos_a_vencer(&fecha_actual, 365));

}