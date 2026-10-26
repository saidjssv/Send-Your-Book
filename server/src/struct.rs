use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewBook {
    pub titulo: String,
    pub autor_id: String,
    pub n_paginas: String,
    pub registrado: NaiveDate,
    pub genero_id: String,
    pub n_paginas: u32
}