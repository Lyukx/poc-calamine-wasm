use calamine::Reader;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
pub async fn excel_to_str(file: web_sys::File) -> String {
    let buffer: web_sys::Blob = file.slice().unwrap();
    let buffer: JsValue = JsFuture::from(buffer.array_buffer()).await.unwrap();
    let buffer: Vec<u8> = js_sys::Uint8Array::new(&buffer).to_vec();
    let buffer: Cursor<Vec<u8>> = Cursor::new(buffer);

    let mut sheets = calamine::open_workbook_auto_from_rs(buffer).unwrap();
    let first_sheet = sheets.worksheet_range_at(0);

    let mut output = "".to_owned();

    match first_sheet {
        Some(sheet_result) => match sheet_result {
            Ok(range) => {
                let n = range.get_size().1 - 1;
                for r in range.rows() {
                    for (i, cell) in r.iter().enumerate() {
                        output.push_str(&format!("{}", cell));
                        if i != n {
                            output.push_str(",")
                        }
                    }
                    output.push_str(";")
                }
            }
            Err(_e) => panic!("Error!"),
        },
        None => panic!("None!"),
    }

    return format!("{}", output);
}
