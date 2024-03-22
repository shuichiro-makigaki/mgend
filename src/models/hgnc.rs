use crate::models::regex;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Definition {
    pub hgnc_id: String,
    pub symbol: String,
    pub alias_symbol: String,
}

pub struct HGNC {
    definitions: HashMap<String, String>,
}

impl HGNC {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let mut definitions = HashMap::new();

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b'\t')
            .from_path(path.as_ref())
            .map_err(|e| format!("{}: {}", path.as_ref().to_string_lossy(), e))?;

        for result in reader.deserialize::<Definition>() {
            let row = result.map_err(|e| format!("{}: {}", path.as_ref().to_string_lossy(), e))?;

            if let Some(caps) = regex!(r"HGNC:(\d+)").captures(row.hgnc_id.as_str()) {
                let id = &caps[1];
                definitions.insert(row.symbol.clone(), id.to_string());

                if let Some(caps) = regex!(r#"^"?(.*)"?$"#).captures(row.alias_symbol.as_str()) {
                    for x in caps[1].split("|") {
                        definitions.insert(x.to_string(), id.to_string());
                    }
                };
            }
        }

        Ok(HGNC { definitions })
    }

    pub fn find<T: AsRef<str>>(&self, symbol: T) -> Option<&String> {
        self.definitions.get(symbol.as_ref())
    }
}
