use std::{
    fs::File,
    io::{self, BufReader, Read},
};

use chrono::{DateTime, Local};
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};

#[derive(Debug, Default, sqlx::FromRow, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    id: i64,
    pub file_path: String,
    pub file_hash: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    deleted_at: Option<DateTime<Local>>,
}

impl Song {
    pub fn new(
        id: Option<i64>,
        file_path: String,
        created_at: Option<DateTime<Local>>,
        updated_at: Option<DateTime<Local>>,
        deleted_at: Option<DateTime<Local>>,
    ) -> Self {
        let input = File::open(file_path.as_str()).expect("Could not open file path!");
        let reader = BufReader::new(input);
        let file_hash = HEXUPPER.encode(
            Self::sha256_digest(reader)
                .expect("Could not hash file!")
                .as_ref(),
        );

        Self {
            id: id.unwrap_or_default(),
            file_path,
            file_hash,
            created_at: created_at.unwrap_or_default(),
            updated_at: updated_at.unwrap_or_default(),
            deleted_at,
        }
    }

    fn sha256_digest<R: Read>(mut reader: R) -> io::Result<Digest> {
        let mut context = Context::new(&SHA256);
        let mut buffer = [0; 1024];

        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }

            context.update(&buffer[..count]);
        }

        Ok(context.finish())
    }
}
