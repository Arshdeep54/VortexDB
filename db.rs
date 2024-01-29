//For rocks-db
use super::types::Data;
use crate::kd_tree::KDTree;
use crate::keygen::*;
use hex::{decode, FromHexError as hexerr};
use rocksdb::{
    DBWithThreadMode,
    Error as err,
    Options,
    // WriteBatch,
    // DBPinnableSlice,
    SingleThreaded,
    DB,
};

pub struct Database {
    pub db: DBWithThreadMode<SingleThreaded>,
    pub path: String,
    pub tree: Option<KDTree>,
}

impl Database {
    pub fn create_switch_database(addr: String) -> Result<Database, err> {
        let mut options = Options::default();

        //Optimize RocksDB
        options.increase_parallelism(12);
        options.optimize_level_style_compaction(512 * 1024 * 1024);

        //Create the database if not already present
        options.create_if_missing(true);

        //Open the database
        let database = Database {
            db: DB::open(&options, &addr).unwrap(),
            path: addr,
            tree: None,
        };
        return Ok(database);
    }

    pub fn get_current_path(&self) -> String {
        return self.path.clone();
    }

    pub fn insert_in_database(&self, data: Data) -> Result<String, err> {
        let value = serialize(data);
        let key_string = hash(value.clone());

        match self.db.put(&key_string, value.as_ref() as &[u8]) {
            Ok(_) => {
                return Ok(key_string);
            }
            Err(e) => {
                return Err(e);
            }
        };

        // also add a new node to the database
    }

    pub fn delete_database(&self) -> Result<(), err> {
        let options = Options::default();
        match DB::destroy(&options, &self.path) {
            Ok(()) => {
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }

        // Also remove the kd-tree database
    }

    pub fn delete_from_database_with_value(&self, data: Data) -> Result<Option<()>, err> {
        let value = serialize(data);
        let key = hash(value);

        match self.db.get(&key) {
            Ok(Some(_)) => match self.db.delete(key) {
                Ok(_) => Ok(Some(())),
                Err(e) => Err(e),
            },
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }

        // Find the corresponding point and remove it from the database
    }

    pub fn delete_from_database_with_key(
        &self,
        input: &str,
    ) -> Result<Result<Option<()>, err>, hexerr> {
        match decode(input) {
            Ok(bytes) => {
                let key = bytes.into_boxed_slice();
                match self.db.get(&key) {
                    Ok(Some(_)) => match self.db.delete(key) {
                        Ok(_) => Ok(Ok(Some(()))),
                        Err(e) => Ok(Err(e)),
                    },
                    Ok(None) => Ok(Ok(None)),
                    Err(e) => Ok(Err(e)),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_data_from_key(&self, input: &str) -> Result<Result<Option<Data>, err>, hexerr> {
        match decode(input) {
            Ok(bytes) => {
                let key = bytes.into_boxed_slice();
                match self.db.get(&key) {
                    Ok(Some(value)) => {
                        let vec = deserialize(&value);
                        return Ok(Ok(Some(vec)));
                    }
                    Ok(None) => {
                        return Ok(Ok(None));
                    }
                    Err(e) => {
                        return Ok(Err(e));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
