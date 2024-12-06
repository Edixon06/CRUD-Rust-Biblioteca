use chrono::NaiveDate;

#[derive(Debug)]
pub struct Usuario {
    pub cedula: String,
    pub nombre: String,
    pub apellido: String,
    pub direccion: String,
    pub telefono: String,
}

#[derive(Debug)]
pub struct Libro {
    pub id: i32,
    pub titulo: String,
    pub autor: String,
    pub paginas: i32,
    pub idioma: String,
    pub categoria: String,
    pub edicion: String,
    pub stock: i32,
}

#[derive(Debug)]
pub struct Prestamo {
    pub id: i32,
    pub id_libro: i32,
    pub cedula_usuario: String,
    pub fecha_prestamo: NaiveDate,
    pub fecha_devolucion: Option<NaiveDate>,
}
