use mysql::*;

pub fn get_connection() -> PooledConn {
    let url = "mysql://root:123456@localhost:3306/biblioteca";
    let pool = Pool::new(url).expect("Error creando el pool de conexión");
    pool.get_conn().expect("Error obteniendo la conexión")
}
