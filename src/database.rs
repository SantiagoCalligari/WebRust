use gluesql::{Connection, Sqlite};

// Crea una base de datos SQLite si no existe y devuelve una conexión
fn create_or_connect_to_db() -> Connection<Sqlite> {
    Connection::new::<Sqlite>("mydb.sqlite3").unwrap()
}

// Crea una tabla en la base de datos si no existe
fn create_table(conn: &Connection<Sqlite>) {
    conn.execute("CREATE TABLE IF NOT EXISTS mytable (id INTEGER PRIMARY KEY, value BOOLEAN)").unwrap();
}

// Inserta 9 filas con valor falso en la tabla
fn insert_rows(conn: &Connection<Sqlite>) {
    for i in 1..10 {
        conn.execute(&format!("INSERT INTO mytable (value) VALUES (0)")).unwrap();
    }
}

// Obtiene los valores de la tabla como un array
fn get_values(conn: &Connection<Sqlite>) -> Vec<bool> {
    let rows = conn.query("SELECT value FROM mytable").unwrap();
    rows.map(|row| row.get(0)).collect()
}

// Modifica el valor de la fila con id = 5
fn update_value(conn: &Connection<Sqlite>) {
    conn.execute("UPDATE mytable SET value = 1 WHERE id = 5").unwrap();
}
