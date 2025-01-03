use crate::schema::pessoas; // Importa a table! gerada no schema
use diesel::prelude::*;

// Precisamos das macros derivadas de Diesel
use diesel::Queryable;
use diesel::Insertable;

/// Para SELECTs: mapeia colunas (id, nome) -> (i32, String)
#[derive(Debug, Queryable)]
pub struct Pessoa {
    pub id: i32,
    pub nome: String,
}

/// Para INSERTs: não incluímos 'id' (pois é SERIAL no banco)
#[derive(Debug, Insertable)]
#[diesel(table_name = pessoas)]
pub struct NovaPessoa<'a> {
    pub nome: &'a str,
}
