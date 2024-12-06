use mysql::prelude::{Queryable, FromRow};
use mysql::PooledConn;
use crate::db::get_connection;
use crate::estructuras::{Usuario, Libro};
use chrono::NaiveDate;
use std::io::{self, Write};

pub fn menu_usuarios() {
    loop {
        println!("Menú Usuarios:");
        println!("1. Añadir usuario");
        println!("2. Editar usuario");
        println!("3. Eliminar usuario");
        println!("4. Buscar usuario");
        println!("5. Volver al menú principal");
        let mut opcion = String::new();
        std::io::stdin().read_line(&mut opcion).unwrap();
        let opcion: i32 = opcion.trim().parse().unwrap();

        match opcion {
            1 => {
                crear_usuario();
            }
            2 => {
                editar_usuario();
            }
            3 => {
                eliminar_usuario();
            }
            4 => {
                buscar_usuario();
            }
            5 => {
                break;
            }
            _ => {
                println!("Opción no válida, intente de nuevo.");
            }
        }
    }
}

pub fn menu_libros() {
    loop {
        println!("Menú Libros:");
        println!("1. Añadir libro");
        println!("2. Editar libro");
        println!("3. Eliminar libro");
        println!("4. Buscar libro");
        println!("5. Volver al menú principal");
        let mut opcion = String::new();
        std::io::stdin().read_line(&mut opcion).unwrap();
        let opcion: i32 = opcion.trim().parse().unwrap();

        match opcion {
            1 => {
                crear_libro();
            }
            2 => {
                editar_libro();
            }
            3 => {
                eliminar_libro();
            }
            4 => {
                buscar_libro();
            }
            5 => {
                break;
            }
            _ => {
                println!("Opción no válida, intente de nuevo.");
            }
        }
    }
}

pub fn crear_usuario() {
    let mut conn = get_connection();
    let usuario = obtener_datos_usuario();
    conn.exec_drop(
        "INSERT INTO usuarios (cedula, nombre, apellido, direccion, telefono) VALUES (?, ?, ?, ?, ?)",
        (usuario.cedula, usuario.nombre, usuario.apellido, usuario.direccion, usuario.telefono),
    ).expect("Error creando usuario");
    println!("Usuario creado exitosamente.");
}

pub fn editar_usuario(){
    let mut conn = get_connection();

    loop {
        let cedula = obtener_cedula();

        let usuario: Option<Usuario> = conn.exec_first(
            "SELECT * FROM usuarios WHERE cedula = ?",
            (cedula.clone(),),
        ).expect("Error buscando usuario");

        match usuario {
            Some(_) => {
                loop {
                    println!("Menú de Edición de Usuario:");
                    println!("1. Editar nombre");
                    println!("2. Editar apellido");
                    println!("3. Editar dirección");
                    println!("4. Editar teléfono");
                    println!("5. Volver al menú principal");
                    let mut opcion = String::new();
                    std::io::stdin().read_line(&mut opcion).unwrap();
                    let opcion: i32 = opcion.trim().parse().unwrap();

                    match opcion {
                        1 => {
                            let nombre = obtener_string("Ingrese el nuevo nombre: ");
                            conn.exec_drop(
                                "UPDATE usuarios SET nombre = ? WHERE cedula = ?",
                                (nombre, cedula.clone()),
                            ).expect("Error actualizando nombre del usuario");
                            println!("Nombre actualizado exitosamente.");
                        }
                        2 => {
                            let apellido = obtener_string("Ingrese el nuevo apellido: ");
                            conn.exec_drop(
                                "UPDATE usuarios SET apellido = ? WHERE cedula = ?",
                                (apellido, cedula.clone()),
                            ).expect("Error actualizando apellido del usuario");
                            println!("Apellido actualizado exitosamente.");
                        }
                        3 => {
                            let direccion = obtener_string("Ingrese la nueva dirección: ");
                            conn.exec_drop(
                                "UPDATE usuarios SET direccion = ? WHERE cedula = ?",
                                (direccion, cedula.clone()),
                            ).expect("Error actualizando dirección del usuario");
                            println!("Dirección actualizada exitosamente.");
                        }
                        4 => {
                            let telefono = obtener_string("Ingrese el nuevo teléfono: ");
                            conn.exec_drop(
                                "UPDATE usuarios SET telefono = ? WHERE cedula = ?",
                                (telefono, cedula.clone()),
                            ).expect("Error actualizando teléfono del usuario");
                            println!("Teléfono actualizado exitosamente.");
                        }
                        5 => {
                            break;
                        }
                        _ => {
                            println!("Opción no válida, intente de nuevo.");
                        }
                    }
                }
                break;
            }
            None => {
                println!("Usuario no encontrado. Intente de nuevo o presione 0 para salir.");
                let mut opcion = String::new();
                std::io::stdin().read_line(&mut opcion).unwrap();
                let opcion: i32 = opcion.trim().parse().unwrap();
                if opcion == 0 {
                    break;
                }
            }
        }
    }
}


pub fn eliminar_usuario() {
    let mut conn = get_connection();
    let cedula = obtener_cedula();
    conn.exec_drop(
        "DELETE FROM usuarios WHERE cedula = ?",
        (cedula,),
    ).expect("Error eliminando usuario");
    println!("Usuario eliminado exitosamente.");
}

pub fn buscar_usuario() {
    let mut conn = get_connection();
    let cedula = obtener_cedula();
    let usuario: Option<Usuario> = conn.exec_first(
        "SELECT * FROM usuarios WHERE cedula = ?",
        (cedula,),
    ).expect("Error buscando usuario");

    match usuario {
        Some(u) => {
            println!("Usuario encontrado: {:?}", u);
        }
        None => {
            println!("Usuario no encontrado.");
        }
    }
}

pub fn crear_libro() {
    let mut conn = get_connection();
    let libro = obtener_datos_libro();
    conn.exec_drop(
        "INSERT INTO libros (titulo, autor, paginas, idioma, categoria, edicion, stock) VALUES (?, ?, ?, ?, ?, ?, ?)",
        (libro.titulo, libro.autor, libro.paginas, libro.idioma, libro.categoria, libro.edicion, libro.stock),
    ).expect("Error creando libro");
    println!("Libro creado exitosamente.");
}

pub fn editar_libro() {
    let mut conn = get_connection();

    loop {
        let id = obtener_id_libro();

        let libro: Option<Libro> = conn.exec_first(
            "SELECT * FROM libros WHERE id = ?",
            (id,),
        ).expect("Error buscando libro");

        match libro {
            Some(_) => {
                loop {
                    println!("Menú de Edición de Libro:");
                    println!("1. Editar título");
                    println!("2. Editar autor");
                    println!("3. Editar número de páginas");
                    println!("4. Editar idioma");
                    println!("5. Editar categoría");
                    println!("6. Editar edición");
                    println!("7. Editar stock");
                    println!("8. Volver al menú principal");
                    let mut opcion = String::new();
                    std::io::stdin().read_line(&mut opcion).unwrap();
                    let opcion: i32 = opcion.trim().parse().unwrap();

                    match opcion {
                        1 => {
                            let titulo = obtener_string("Ingrese el nuevo título: ");
                            conn.exec_drop(
                                "UPDATE libros SET titulo = ? WHERE id = ?",
                                (titulo, id),
                            ).expect("Error actualizando título del libro");
                            println!("Título actualizado exitosamente.");
                        }
                        2 => {
                            let autor = obtener_string("Ingrese el nuevo autor: ");
                            conn.exec_drop(
                                "UPDATE libros SET autor = ? WHERE id = ?",
                                (autor, id),
                            ).expect("Error actualizando autor del libro");
                            println!("Autor actualizado exitosamente.");
                        }
                        3 => {
                            let paginas = obtener_entero("Ingrese el nuevo número de páginas: ");
                            conn.exec_drop(
                                "UPDATE libros SET paginas = ? WHERE id = ?",
                                (paginas, id),
                            ).expect("Error actualizando número de páginas del libro");
                            println!("Número de páginas actualizado exitosamente.");
                        }
                        4 => {
                            let idioma = obtener_string("Ingrese el nuevo idioma: ");
                            conn.exec_drop(
                                "UPDATE libros SET idioma = ? WHERE id = ?",
                                (idioma, id),
                            ).expect("Error actualizando idioma del libro");
                            println!("Idioma actualizado exitosamente.");
                        }
                        5 => {
                            let categoria = obtener_string("Ingrese la nueva categoría: ");
                            conn.exec_drop(
                                "UPDATE libros SET categoria = ? WHERE id = ?",
                                (categoria, id),
                            ).expect("Error actualizando categoría del libro");
                            println!("Categoría actualizada exitosamente.");
                        }
                        6 => {
                            let edicion = obtener_string("Ingrese la nueva edición: ");
                            conn.exec_drop(
                                "UPDATE libros SET edicion = ? WHERE id = ?",
                                (edicion, id),
                            ).expect("Error actualizando edición del libro");
                            println!("Edición actualizada exitosamente.");
                        }
                        7 => {
                            let stock = obtener_entero("Ingrese el nuevo stock: ");
                            conn.exec_drop(
                                "UPDATE libros SET stock = ? WHERE id = ?",
                                (stock, id),
                            ).expect("Error actualizando stock del libro");
                            println!("Stock actualizado exitosamente.");
                        }
                        8 => {
                            break;
                        }
                        _ => {
                            println!("Opción no válida, intente de nuevo.");
                        }
                    }
                }
                break;
            }
            None => {
                println!("Libro no encontrado. Intente de nuevo o presione 0 para salir.");
                let mut opcion = String::new();
                std::io::stdin().read_line(&mut opcion).unwrap();
                let opcion: i32 = opcion.trim().parse().unwrap();
                if opcion == 0 {
                    break;
                }
            }
        }
    }
}


pub fn eliminar_libro() {
    let mut conn = get_connection();
    let id = obtener_id_libro();
    conn.exec_drop(
        "DELETE FROM libros WHERE id = ?",
        (id,),
    ).expect("Error eliminando libro");
    println!("Libro eliminado exitosamente.");
}

pub fn buscar_libro() {
    let mut conn = get_connection();
    let id = obtener_id_libro();
    let libro: Option<Libro> = conn.exec_first(
        "SELECT * FROM libros WHERE id = ?",
        (id,),
    ).expect("Error buscando libro");

    match libro {
        Some(l) => {
            println!("Libro encontrado: {:?}", l);
        }
        None => {
            println!("Libro no encontrado.");
        }
    }
}

pub fn prestar_libro() {
    let mut conn = get_connection();
    let cedula = obtener_cedula();
    let id_libro = obtener_id_libro();

    let stock: Option<i32> = conn.exec_first(
        "SELECT stock FROM libros WHERE id = ?",
        (id_libro,),
    ).expect("Error obteniendo stock del libro");

    if let Some(stock) = stock {
        if stock > 0 {
            let fecha_prestamo = chrono::Local::now().naive_local().date().to_string();
            conn.exec_drop(
                "INSERT INTO prestamos (id_libro, cedula_usuario, fecha_prestamo) VALUES (?, ?, ?)",
                (id_libro, cedula, fecha_prestamo),
            ).expect("Error registrando préstamo");
            conn.exec_drop(
                "UPDATE libros SET stock = stock - 1 WHERE id = ?",
                (id_libro,),
            ).expect("Error actualizando stock del libro");
            println!("Préstamo registrado exitosamente.");
        } else {
            println!("No hay libros disponibles para prestar.");
        }
    } else {
        println!("Libro no encontrado.");
    }
}

pub fn devolver_libro() {
    let mut conn = get_connection();
    let id_prestamo = obtener_id_prestamo();

    let fecha_devolucion = chrono::Local::now().naive_local().date().to_string();
    conn.exec_drop(
        "UPDATE prestamos SET fecha_devolucion = ? WHERE id = ?",
        (fecha_devolucion, id_prestamo),
    ).expect("Error actualizando devolución");
    let id_libro: i32 = conn.exec_first(
        "SELECT id_libro FROM prestamos WHERE id = ?",
        (id_prestamo,),
    ).expect("Error obteniendo id del libro")
    .unwrap();
    conn.exec_drop(
        "UPDATE libros SET stock = stock + 1 WHERE id = ?",
        (id_libro,),
    ).expect("Error actualizando stock del libro");
    println!("Devolución registrada exitosamente.");
}

pub fn mostrar_reporte() { // Cambiar a referencia mutable
    let mut conn = get_connection();
    println!(
        "{:<10} | {:<8} | {:<30} | {:<15} | {:<20} | {:<15} | {:<15}",
        "ID Préstamo", "ID Libro", "Título Libro", "Cédula Usuario",
        "Nombre Usuario", "Fecha Préstamo", "Fecha Devolución"
    );

    let query = r#"
        SELECT 
            prestamos.id AS id_prestamo, 
            libros.id AS id_libro, 
            libros.titulo AS titulo_libro, 
            usuarios.cedula AS cedula_usuario, 
            CONCAT(usuarios.nombre, ' ', usuarios.apellido) AS nombre_usuario, 
            prestamos.fecha_prestamo, 
            prestamos.fecha_devolucion 
        FROM 
            prestamos
        JOIN 
            libros ON prestamos.id_libro = libros.id
        JOIN 
            usuarios ON prestamos.cedula_usuario = usuarios.cedula
        ORDER BY 
            prestamos.id;
    "#;

    let resultados = conn.query_map(
        query,
        |(
            id_prestamo,
            id_libro,
            titulo_libro,
            cedula_usuario,
            nombre_usuario,
            fecha_prestamo,
            fecha_devolucion,
        ): (
            u32,
            u32,
            String,
            String,
            String,
            String,
            Option<String>,
        )| {
            (
                id_prestamo,
                id_libro,
                titulo_libro,
                cedula_usuario,
                nombre_usuario,
                fecha_prestamo,
                fecha_devolucion.unwrap_or_else(|| "Sin devolver".to_string()),
            )
        },
    );

    match resultados {
        Ok(registros) => {
            for registro in registros {
                let (
                    id_prestamo,
                    id_libro,
                    titulo_libro,
                    cedula_usuario,
                    nombre_usuario,
                    fecha_prestamo,
                    fecha_devolucion,
                ) = registro;

                println!(
                    "{:<10} | {:<8} | {:<30} | {:<15} | {:<20} | {:<15} | {:<15}",
                    id_prestamo,
                    id_libro,
                    titulo_libro,
                    cedula_usuario,
                    nombre_usuario,
                    fecha_prestamo,
                    fecha_devolucion,
                );
            }
        }
        Err(e) => {
            println!("Error al obtener el reporte: {}", e);
        }
    }
}


fn obtener_datos_usuario() -> Usuario {
    print!("Ingrese la cédula: ");
    io::stdout().flush().unwrap();
    let mut cedula = String::new();
    std::io::stdin().read_line(&mut cedula).unwrap();
    let cedula = cedula.trim().to_string();

    print!("Ingrese el nombre: ");
    io::stdout().flush().unwrap();
    let mut nombre = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    let nombre = nombre.trim().to_string();

    print!("Ingrese el apellido: ");
    io::stdout().flush().unwrap();
    let mut apellido = String::new();
    std::io::stdin().read_line(&mut apellido).unwrap();
    let apellido = apellido.trim().to_string();

    print!("Ingrese la dirección: ");
    io::stdout().flush().unwrap();
    let mut direccion = String::new();
    std::io::stdin().read_line(&mut direccion).unwrap();
    let direccion = direccion.trim().to_string();

    print!("Ingrese el teléfono: ");
    io::stdout().flush().unwrap();
    let mut telefono = String::new();
    std::io::stdin().read_line(&mut telefono).unwrap();
    let telefono = telefono.trim().to_string();

    Usuario {
        cedula,
        nombre,
        apellido,
        direccion,
        telefono,
    }
}

fn obtener_datos_libro() -> Libro {
    print!("Ingrese el título: ");
    io::stdout().flush().unwrap();
    let mut titulo = String::new();
    std::io::stdin().read_line(&mut titulo).unwrap();
    let titulo = titulo.trim().to_string();

    print!("Ingrese el autor: ");
    io::stdout().flush().unwrap();
    let mut autor = String::new();
    std::io::stdin().read_line(&mut autor).unwrap();
    let autor = autor.trim().to_string();

    let paginas = obtener_entero("Ingrese el número de páginas: ");
    let stock = obtener_entero("Ingrese el stock: ");

    print!("Ingrese el idioma: ");
    io::stdout().flush().unwrap();
    let mut idioma = String::new();
    std::io::stdin().read_line(&mut idioma).unwrap();
    let idioma = idioma.trim().to_string();

    print!("Ingrese la categoría: ");
    io::stdout().flush().unwrap();
    let mut categoria = String::new();
    std::io::stdin().read_line(&mut categoria).unwrap();
    let categoria = categoria.trim().to_string();

    print!("Ingrese la edición: ");
    io::stdout().flush().unwrap();
    let mut edicion = String::new();
    std::io::stdin().read_line(&mut edicion).unwrap();
    let edicion = edicion.trim().to_string();

    Libro {
        id: 0, // El ID se genera automáticamente en la base de datos
        titulo,
        autor,
        paginas,
        idioma,
        categoria,
        edicion,
        stock,
    }
}

fn obtener_entero(prompt: &str) -> i32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<i32>() {
            Ok(value) => return value,
            Err(_) => println!("Por favor, ingrese un número entero válido."),
        }
    }
}

fn obtener_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn obtener_cedula() -> String {
    print!("Ingrese la cédula: ");
    io::stdout().flush().unwrap();
    let mut cedula = String::new();
    std::io::stdin().read_line(&mut cedula).unwrap();
    cedula.trim().to_string()
}

fn obtener_id_libro() -> i32 {
    obtener_entero("Ingrese el ID del libro: ")
}

fn obtener_id_prestamo() -> i32 {
    obtener_entero("Ingrese el ID del préstamo: ")
}

impl FromRow for Usuario {
    fn from_row(row: mysql::Row) -> Self {
        let cedula: String = row.get("cedula").unwrap();
        let nombre: String = row.get("nombre").unwrap();
        let apellido: String = row.get("apellido").unwrap();
        let direccion: String = row.get("direccion").unwrap();
        let telefono: String = row.get("telefono").unwrap();

        Usuario {
            cedula,
            nombre,
            apellido,
            direccion,
            telefono,
        }
    }
    
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized {
        todo!()
    }
}

impl FromRow for Libro {
    fn from_row(row: mysql::Row) -> Self {
        let id: i32 = row.get("id").unwrap();
        let titulo: String = row.get("titulo").unwrap();
        let autor: String = row.get("autor").unwrap();
        let paginas: i32 = row.get("paginas").unwrap();
        let idioma: String = row.get("idioma").unwrap();
        let categoria: String = row.get("categoria").unwrap();
        let edicion: String = row.get("edicion").unwrap();
        let stock: i32 = row.get("stock").unwrap();

        Libro {
            id,
            titulo,
            autor,
            paginas,
            idioma,
            categoria,
            edicion,
            stock,
        }
    }
    
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized {
        todo!()
    }
}
