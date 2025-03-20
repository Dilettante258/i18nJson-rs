use std::collections::HashMap;
use std::fs;
use rust_xlsxwriter::*;
use serde_json::Value;

fn flatten_json(
    json: &Value,
    prefix: &str,
    separator: &str,
    mut result: &mut HashMap<String, String>,
) {
    match json {
        Value::Object(map) => {
            for (key, value) in map {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}{}{}", prefix, separator, key)
                };
                flatten_json(value, &new_prefix, separator, &mut result);
            }
        }
        Value::String(s) => {
            result.insert(prefix.to_string(), s.clone());
        }
        Value::Number(n) => {
            result.insert(prefix.to_string(), n.to_string());
        }
        Value::Bool(b) => {
            result.insert(prefix.to_string(), b.to_string());
        }
        _ => {
            println!("Unsupported value type for key: {}", prefix);
        }
    }
}

fn sort_vec_of_hashmaps(data: &mut Vec<HashMap<String, String>>) {
    data.sort_by(|a, b| {
        let id_a = a.get("id").unwrap();
        let id_b = b.get("id").unwrap();

        let cleaned_id_a: String = id_a.chars().collect();
        let cleaned_id_b: String = id_b.chars().collect();

        cleaned_id_a.cmp(&cleaned_id_b)
    });
}


fn main() -> Result<(), XlsxError> {
    let languages = vec!["zh-CN", "en-US"];
    let mut bundle_data: Vec<HashMap<String, String>> = Vec::new();

    // 读取每种语言的JSON文件
    for lang in &languages {
        let file_path = format!("./{}.json", lang);
        let file_content = fs::read_to_string(&file_path).expect("Unable to read file");

        let json_value: Value = serde_json::from_str(&file_content).unwrap();
        let mut flattened_data: HashMap<String, String> = HashMap::new();
        flatten_json(&json_value, "", ">", &mut flattened_data);
        bundle_data.push(flattened_data);
    }

    let mut data: Vec<HashMap<String, String>> = Vec::new();

    for (index, lang_data) in bundle_data.iter().enumerate() {
        for (key, value) in lang_data {
            if let Some(found_item) = data.iter_mut().find(|item| item.get("id") == Some(&key.to_string())) {
                found_item.insert(languages[index].to_string(), value.clone());
            } else {
                let mut new_item = HashMap::new();
                new_item.insert("id".to_string(), key.clone());
                new_item.insert(languages[index].to_string(), value.clone());
                data.push(new_item);
            }
        }
    }

    // 排序
    sort_vec_of_hashmaps(&mut data);

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.set_column_width(0, 40)?;
    worksheet.set_column_width(1, 50)?;
    worksheet.set_column_width(2, 50)?;

    worksheet.write(0, 0, "id")?;
    worksheet.write(0, 1, "en-US")?;
    worksheet.write(0, 2, "zh-CN")?;

    for (index, item) in data.iter().enumerate() {
        worksheet.write((index + 1) as u32, 0, item.get("id"))?;
        worksheet.write((index + 1) as u32, 1, item.get("en-US"))?;
        worksheet.write((index + 1) as u32, 2, item.get("zh-CN"))?;
    }
    workbook.save("demo.xlsx")?;
    Ok(())
}