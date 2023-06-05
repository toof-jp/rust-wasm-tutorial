use wasm_bindgen::prelude::*;
use time::OffsetDateTime;
use js_sys::Date;

#[wasm_bindgen]
pub fn add(date: Date) -> u32 {
    let t = OffsetDateTime::from(date);
    t.unix_timestamp() as u32
}
