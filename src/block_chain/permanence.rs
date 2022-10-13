use crate::block_chain::Block;

use hex::{encode, decode};
use rusqlite::{params, Connection};

pub trait Permanence {
    fn write(&self, block: &Block);
    fn read(&self, offset: usize) -> Result<Block, Box<dyn std::error::Error>>;
    fn read_range(&self, range: std::ops::Range<usize>) -> Result<Vec<Block>, Box<dyn std::error::Error>>;
}

pub struct SqliteDatabase {
    connection: Connection
}

impl SqliteDatabase {
    pub fn init() -> Self {
        let database = SqliteDatabase {
            connection: Connection::open("blockchain.db").unwrap()
        };

        database.connection.execute("
            CREATE TABLE IF NOT EXISTS block (
                idx             INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp       TEXT,
                prev_hash       TEXT,
                curr_hash       TEXT,
                nonce           INTEGER,
                data            TEXT
            );
        ", []).unwrap();

        database
    }
}

impl Permanence for SqliteDatabase {
    fn write(&self, block: &Block) {
        self.connection.execute("
            INSERT INTO block (
                timestamp, prev_hash, curr_hash, nonce, data
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5
            );
        ", params!(
            block.timestamp,
            encode(&block.prev_hash),
            encode(&block.curr_hash),
            block.nonce,
            encode(&block.data)
        )).unwrap();
    }

    fn read(&self, offset: usize) -> Result<Block, Box<dyn std::error::Error>> {
        let result = self.read_range(offset..offset + 1)?;
        if result.is_empty() {
            Err("Failed to read.")?
        }
        Ok(result[0].clone())
    }

    fn read_range(&self, range: std::ops::Range<usize>) -> Result<Vec<Block>, Box<dyn std::error::Error>> {
        let mut statement = self.connection.prepare("SELECT * FROM block ORDER BY idx DESC LIMIT ?, ?;")?;

        let rows: Vec<_> = statement.query_map([range.start, range.end], |row| {
            let prev_hash: String = row.get(2)?;
            let curr_hash: String = row.get(3)?;
            let data: String = row.get(5)?;

            Ok(Block {
                index: row.get(0)?,
                timestamp: row.get(1)?,
                prev_hash: decode(prev_hash.as_str()).unwrap(),
                curr_hash: decode(curr_hash.as_str()).unwrap(),
                nonce: row.get(4)?,
                data: decode(data.as_str()).unwrap(),
            })
        })?.collect();

        let mut blocks: Vec<Block> = Vec::new();

        for row in rows {
            blocks.push(row?);
        }

        Ok(blocks)
    }
}