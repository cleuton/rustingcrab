use crate::db::schema::pessoas;
use diesel::{Insertable, Queryable};

// Import necessário para a macro #[derive(Serialize)]
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Pessoa {
    pub id: i32,
    pub nome: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = pessoas)]
pub struct NovaPessoa<'a> {
    pub nome: &'a str,
}
