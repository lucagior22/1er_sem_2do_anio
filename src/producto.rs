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