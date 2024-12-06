//use chrono::NaiveDate;

mod db;
mod estructuras;
mod funciones;
use funciones::{crear_usuario, crear_libro, prestar_libro, devolver_libro, mostrar_reporte, menu_usuarios, menu_libros};
use crate::estructuras::{Usuario, Libro};

fn main() {
    loop {
        println!("Menú:");
        println!("1. Usuarios");
        println!("2. Libros");
        println!("3. Prestar libro");
        println!("4. Devolver libro");
        println!("5. Mostrar reporte");
        println!("6. Salir");
        let mut opcion = String::new();
        std::io::stdin().read_line(&mut opcion).unwrap();
        let opcion: i32 = opcion.trim().parse().unwrap();

        match opcion {
            1 => {
                menu_usuarios();
            }
            2 => {
                menu_libros();
            }
            3 => {
                prestar_libro();
            }
            4 => {
                devolver_libro();
            }
            5 => {
                mostrar_reporte();
            }
            6 => {
                println!("Saliendo...");
                break;
            }
            _ => {
                println!("Opción no válida, intente de nuevo.");
            }
        }
    }
}
