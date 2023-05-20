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