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
pub enum Genero{
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}
impl Genero{
    fn igual(&self, genero : &Genero) -> bool{
        match (self, genero) {
           (Genero::Rock, Genero::Rock) => true, 
           (Genero::Pop, Genero::Pop) => true, 
           (Genero::Rap, Genero::Rap) => true, 
           (Genero::Jazz, Genero::Jazz) => true, 
           (Genero::Otros, Genero::Otros) => true, 
           _ => false, 
        }
    }
}

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
}

pub struct Playlist<'a>{
    nombre : String,
    lista_canciones : Vec<&'a Cancion>,
}

impl <'a> Playlist<'a>{
    pub fn new(nombre : String) -> Playlist<'a>{
        Playlist{
            lista_canciones : Vec::new(),
            nombre,
        }
    }
    pub fn agregar_cancion(&mut self, cancion : &'a Cancion){
        self.lista_canciones.push(&cancion);
    }
    pub fn eliminar_cancion(&mut self, titulo : String) -> bool{
        for i in 0..self.lista_canciones.len() - 1{
            if self.lista_canciones[i].titulo == titulo{
                self.lista_canciones.remove(i);
                return true
            }
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
        while i <= self.lista_canciones.len() - 1{
            if self.lista_canciones.get(i).unwrap().titulo == titulo{
                return Some(i)
            }
            i += 1;
        }
        None
    }

    pub fn canciones_genero(&self, genero : Genero) -> Option<Vec<&Cancion>> {
        let mut v_aux : Vec<&Cancion> = Vec::new();
        if !self.lista_canciones.is_empty(){
            for i in &self.lista_canciones{ 
                if  i.genero.igual(&genero){
                    v_aux.push(i);
                } 
            }
            Some(v_aux);
        }
        None
    }
    pub fn canciones_artista(&self, artista : String) -> Option<Vec<&Cancion>> {
        let mut v_aux : Vec<&Cancion> = Vec::new();
        if !self.lista_canciones.is_empty(){
            for i in &self.lista_canciones{ 
                if  i.artista == artista{
                    v_aux.push(i);
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