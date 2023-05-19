use std::collections::VecDeque;

// 6- Escribir un programa que defina una estructura Estudiante que tenga campos para el
// nombre, el número de identificación y las calificaciones de exámenes. De cada Examen se
// conoce el nombre de la materia y la nota. Para dichas estructuras implemente los siguientes
// métodos:
// ❖ Examen
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
    pub fn obtener_promedio(&self) -> Option<f64>{
        let mut suma = 0.0;
        if self.calificaciones.len() != 0{
            for i in self.calificaciones{
                suma += i.nota;
            }            
            Some(suma / self.calificaciones.len() as f64)
        } else{
            println!("No existen notas de examenes!");
            None
        }
    }

    pub fn obtener_calificacion_mas_alta(&self) -> Option<f64>{
        let mut max = -1.0;
        for i in self.calificaciones{
            if max < i.nota{
                max = i.nota
            }
        }
        if max == -1.0 {
            None
        } else {
            Some(max)
        }
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