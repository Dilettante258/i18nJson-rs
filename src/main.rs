use calamine::{Sheets, Xlsx, Range, DataType};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let languages = vec!["zh-CN", "en-US"];
    let mut data: HashMap<String, HashMap<String, String>> = HashMap::new();

    for language in &languages {
        let file_path = format!("./{}.json", language);
        let file = File::open(&file_path)?;
        let bundle_data: Value = serde_json::from_reader(file)?;
        if let Some(object) = bundle_data.as_object() {
            for (key, value) in object {
                if let Some(value) = value.as_str() {
                    data.entry(key.to_string())
                        .or_insert_with(HashMap::new)
                        .insert(language.to_string(), value.to_string());
                }
            }
        }
    }

    let mut workbook = Xlsx::new();
    let mut worksheet = workbook.new_sheet("test");

    let mut headers = vec!["ID"];
    headers.extend_from_slice(&languages);
    worksheet.add_row(headers.iter().map(|&h| DataType::String(h.to_string())).collect());

    for (id, translations) in &data {
        let mut row = vec![DataType::String(id.to_string())];
        for language in &languages {
            row.push(DataType::String(translations.get(language).cloned().unwrap_or_default()));
        }
        worksheet.add_row(row);
    }

    let path = Path::new("./bundle.xlsx");
    workbook.save(path)?;

    println!("Excel file created successfully at {:?}", path);

    Ok(())
}