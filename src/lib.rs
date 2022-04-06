use calamine::Reader;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub async fn excel_to_json(file: web_sys::File) -> String {
    let buffer: web_sys::Blob = file.slice().unwrap();
    let buffer: JsValue = JsFuture::from(buffer.array_buffer()).await.unwrap();
    let buffer: Vec<u8> = js_sys::Uint8Array::new(&buffer).to_vec();
    let buffer: Cursor<Vec<u8>> = Cursor::new(buffer);

    let mut sheets = calamine::open_workbook_auto_from_rs(buffer).unwrap();
    let first_sheet = sheets.worksheet_range_at(0);

    match first_sheet {
        Some(sheet_result) => match sheet_result {
            Ok(range) => {
                let mut rows_vec = vec![];
                for r in range.rows() {
                    let mut r_vec = vec![];
                    for (_, cell) in r.iter().enumerate() {
                        r_vec.push(format!("{}", cell).to_string());
                    }
                    rows_vec.push(r_vec);
                }
                return serde_json::to_string(&rows_vec).unwrap();
            }
            Err(_e) => panic!("Error!"),
        },
        None => panic!("None!"),
    }
}
