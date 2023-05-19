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
            c_aux = *self.lista_canciones.get(i).unwrap(); 
            if  c_aux.get_genero().igual(&genero){
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
                c_aux = *self.lista_canciones.get(i).unwrap(); 
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