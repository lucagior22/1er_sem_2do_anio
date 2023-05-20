//mod p3;
mod persona;
mod biblioteca;
mod concesionario;
mod playlist;
mod veterinaria;
mod triangulo;
mod fecha;
mod estudiante;
mod producto;
mod rectangulo;

fn main() {
    //biblioteca::main();  
    concesionario::main();
}


//     let c1 = Cancion::new("Loca".to_string(), "Khea".to_string(), Genero::Pop);
//     let c12 = Cancion::new("Hitboy".to_string(), "Duki".to_string(), Genero::Pop);
//     let c13 = Cancion::new("Chico Estrella".to_string(), "Duki".to_string(), Genero::Pop);
//     let c14 = Cancion::new("Loca Remix".to_string(), "Duki".to_string(), Genero::Pop);
//     let c15 = Cancion::new("Top 5".to_string(), "Duki".to_string(), Genero::Pop);
//     let c16 = Cancion::new("Como si no importara".to_string(), "Duki".to_string(), Genero::Pop);
//     let c17 = Cancion::new("She don't give a fo".to_string(), "Duki".to_string(), Genero::Pop);
//     let c18 = Cancion::new("Sol".to_string(), "Duki".to_string(), Genero::Pop);
//     let c19 = Cancion::new("Malbec".to_string(), "Duki".to_string(), Genero::Pop);
//     let c20 = Cancion::new("Bzrp Session".to_string(), "Duki".to_string(), Genero::Pop);
// let mut pl = Playlist::new("Mi Playlist 1".to_string());
//     pl.agregar_cancion(c1);
//     pl.agregar_cancion(c12);
//     pl.agregar_cancion(c13);
//     pl.agregar_cancion(c14);
//     pl.agregar_cancion(c15);
//     pl.agregar_cancion(c16);
//     pl.agregar_cancion(c17);
//     pl.agregar_cancion(c18);
//     pl.agregar_cancion(c19);
//     pl.agregar_cancion(c20);
//     pl.mover_cancion("Loca Remix".to_string(), 8);
//     println!("{:#?}", pl);
//     pl.actualizar_nombre("Alta Playlist Padre".to_string());
//     println!("{:#?}", pl);
//     pl.limpiar_playlist();
//     println!("{:#?}", pl);
//     let mut v_aux = pl.canciones_artista("Khea".to_string());
//     println!("{:#?}", v_aux);
//     let mut buf:String = String::new();
//     std::io::stdin().read_line(&mut buf).expect("Error");
    
    // let mut dueño1 = Dueño::new("Daniel".to_string(), "Calle 13".to_string(), "221 45663123".to_string());
    // let mut dueño2 = Dueño::new("Tomas".to_string(), "Calle 11".to_string(), "221 49863322".to_string());
    // let mut aux_dueño2 = Dueño::new("Tomas".to_string(), "Calle 11".to_string(), "221 49863322".to_string());
    // let mut dueño3 = Dueño::new("Gabriela".to_string(), "Calle 41".to_string(), "221 41367623".to_string());
    // let mut aux_dueño3 = Dueño::new("Gabriela".to_string(), "Calle 41".to_string(), "221 41367623".to_string());
    // let mut mascota1 = Mascota::new("Tobi".to_string(), 7, Animal::Perro, dueño1);
    // let mut mascota2 = Mascota::new("Gati".to_string(), 4, Animal::Gato, dueño2);
    // let mut mascota3 = Mascota::new("Piolin".to_string(), 11, Animal::Otros, dueño3);
    // // let mut atencion : Atencion = Atencion{};
    // let mut aux_mascota = Mascota::new("Gati".to_string(), 4, Animal::Gato, aux_dueño2);
    // let mut vet = Veterinaria::new("Mascotas".to_string(), "Calle 3".to_string(), 888);

    // vet.agregar_mascota(mascota1); 
    // vet.agregar_mascota(mascota2); 
    // vet.agregar_mascota_prioritario(mascota3); 
    // if let Some(a) = vet.atender_mascota() {
    //     println!("Se atendió a {:#?}", a);
    // } else {
    //     println!("No hay mas mascotas en la cola!")
    // }
    // vet.retirar_de_cola(&aux_mascota);
    // if let Some(a) = vet.atender_mascota() {
    //     println!("Se atendió a {:#?}", a);
    // } else {
    //     println!("No hay mas mascotas en la cola!")
    // }
    // if let Some(a) = vet.atender_mascota() {
    //     println!("Se atendió a {:#?}", a);
    // } else {
    //     println!("No hay mas mascotas en la cola!")
    // }
    