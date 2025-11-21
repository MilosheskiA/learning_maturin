use pyo3::prelude::*;

#[derive(Debug, FromPyObject)]
#[pyo3(from_item_all)]
pub struct CarDataFromPython {
    pub year: u32,
    pub brand: String,
    pub num_doors: u8,
    pub model: String,
    pub hp: f32,
    pub milage: f64,
}
