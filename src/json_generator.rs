use mysql::*;
use serde_json::Value;
use std::path::Path;

pub struct JsonGenerator {
    _mysql_version: String,
}

impl JsonGenerator {
    pub fn new(mysql_version: String) -> Self {
        JsonGenerator {
            _mysql_version: mysql_version,
        }
    }
    
    pub fn generate_json(&self, results: &[Vec<Row>], sql_file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        // 从SQL文件路径提取根节点名称（如kpi.sql -> kpi）
        let root_name = sql_file_path.file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("result");
        
        let mut output = serde_json::Map::new();
        let mut data_array = Vec::new();
        
        for rows in results {
            if rows.is_empty() {
                continue;
            }
            
            let columns = rows[0].columns_ref().iter()
                .map(|col| col.name_str().to_string())
                .collect::<Vec<String>>();
            
            for row in rows {
                let mut obj = serde_json::Map::new();
                for i in 0..columns.len() {
                    let column_name = &columns[i];
                    let json_value = if let Some(value) = row.as_ref(i) {
                        self.convert_value(value)
                    } else {
                        Value::Null
                    };
                    obj.insert(column_name.clone(), json_value);
                }
                data_array.push(Value::Object(obj));
            }
        }
        
        output.insert(root_name.to_string(), Value::Array(data_array));
        
        let json_str = serde_json::to_string_pretty(&Value::Object(output))?;
        Ok(json_str)
    }
    
    fn convert_value(&self, value: &mysql::Value) -> Value {
        match value {
            mysql::Value::NULL => Value::Null,
            mysql::Value::Bytes(b) => {
                let s = String::from_utf8(b.clone()).unwrap_or_default();
                // 尝试将字符串转换为数值
                if let Ok(int_val) = s.parse::<i64>() {
                    Value::Number(int_val.into())
                } else if let Ok(float_val) = s.parse::<f64>() {
                    // 处理精度问题，去除不必要的尾部零
                    let str_val = format!("{}", float_val);
                    if str_val.contains('.') {
                        let trimmed = str_val.trim_end_matches('0').trim_end_matches('.');
                        if let Ok(fixed_float) = trimmed.parse::<f64>() {
                            if let Some(num) = serde_json::Number::from_f64(fixed_float) {
                                return Value::Number(num);
                            }
                        }
                    }
                    // 如果处理失败，直接使用原始转换
                    if let Some(num) = serde_json::Number::from_f64(float_val) {
                        Value::Number(num)
                    } else {
                        Value::String(s)
                    }
                } else {
                    Value::String(s)
                }
            },
            mysql::Value::Int(i) => Value::Number((*i).into()),
            mysql::Value::UInt(u) => Value::Number((*u).into()),
            mysql::Value::Float(f) => {
                if let Some(num) = serde_json::Number::from_f64((*f).into()) {
                    Value::Number(num)
                } else {
                    Value::Null
                }
            },
            mysql::Value::Double(d) => {
                if let Some(num) = serde_json::Number::from_f64(*d) {
                    Value::Number(num)
                } else {
                    Value::Null
                }
            },
            mysql::Value::Date(year, month, day, _, _, _, _) => {
                Value::String(format!("{:04}-{:02}-{:02}", year, month, day))
            },
            mysql::Value::Time(_, _, hours, minutes, seconds, microseconds) => {
                Value::String(format!("{:02}:{:02}:{:02}.{:06}", 
                    hours, minutes, seconds, microseconds))
            },
        }
    }
}
