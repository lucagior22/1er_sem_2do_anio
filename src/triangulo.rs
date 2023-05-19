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