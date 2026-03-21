// Rewrite needed

use crate::error::{self, StorageError};
use crate::{StorageEngine, VectorPage};
use bincode::{deserialize, serialize};
use defs::{DenseVector, Payload, Point, PointId};
use rocksdb::{DB, Options};
use snafu::ResultExt;
use std::path::PathBuf;

//TODO: Implement RocksDbStorage with necessary fields and implementations
//TODO: Optimize the basic design
pub struct RocksDbStorage {
    pub path: PathBuf,
    pub db: DB,
}

impl RocksDbStorage {
    // Creates new db or switches to existing db
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, StorageError> {
        // Initialize a db at the given location
        let mut options = Options::default();

        // Optimize RocksDB
        options.increase_parallelism(12);
        options.optimize_level_style_compaction(512 * 1024 * 1024);

        options.create_if_missing(true);

        let converted_path = path.into();
        let path_str = converted_path.display().to_string();

        let db = DB::open(&options, converted_path.clone())
            .context(error::RocksDbOpenSnafu { path: path_str })?;

        Ok(RocksDbStorage {
            path: converted_path,
            db,
        })
    }

    pub fn get_current_path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl StorageEngine for RocksDbStorage {
    fn insert_point(
        &self,
        id: PointId,
        vector: Option<DenseVector>,
        payload: Option<Payload>,
    ) -> Result<(), StorageError> {
        let key = id.to_string();
        let point = Point {
            id,
            vector,
            payload,
        };
        let value = serialize(&point).context(error::SerializationSnafu { id })?;

        self.db
            .put(key.as_bytes(), value.as_slice())
            .context(error::RocksDbWriteSnafu { id })?;

        Ok(())
    }

    fn contains_point(&self, id: PointId) -> Result<bool, StorageError> {
        // Efficient lookup inspired from https://github.com/facebook/rocksdb/issues/11586#issuecomment-1890429488
        let key = id.to_string();
        if self.db.key_may_exist(key.clone()) {
            let key_exist = self
                .db
                .get(key)
                .context(error::RocksDbReadSnafu { id })?
                .is_some();
            Ok(key_exist)
        } else {
            Ok(false)
        }
    }

    fn delete_point(&self, id: PointId) -> Result<(), StorageError> {
        let key = id.to_string();
        self.db
            .delete(key)
            .context(error::RocksDbDeleteSnafu { id })?;

        Ok(())
    }

    fn get_payload(&self, id: PointId) -> Result<Option<Payload>, StorageError> {
        let key = id.to_string();
        let Some(value_serialized) = self.db.get(key).context(error::RocksDbReadSnafu { id })?
        else {
            return Ok(None); // This should not return error but rather give None
        };

        let value =
            deserialize::<Point>(&value_serialized).context(error::DeserializationSnafu { id })?;

        Ok(value.payload)
    }

    fn get_vector(&self, id: PointId) -> Result<Option<DenseVector>, StorageError> {
        let key = id.to_string();
        let Some(value_serialized) = self.db.get(key).context(error::RocksDbReadSnafu { id })?
        else {
            return Ok(None); // This should not return error but rather give None
        };

        let value =
            deserialize::<Point>(&value_serialized).context(error::DeserializationSnafu { id })?;

        Ok(value.vector)
    }

    fn list_vectors(
        &self,
        offset: PointId,
        limit: usize,
    ) -> Result<Option<VectorPage>, StorageError> {
        if limit < 1 {
            return Ok(None);
        }

        let mut result = Vec::with_capacity(limit);
        let iter = self.db.iterator(rocksdb::IteratorMode::From(
            offset.to_string().as_bytes(),
            rocksdb::Direction::Forward,
        ));
        let mut last_id = offset;

        for item in iter {
            let (_, v) = item.context(error::RocksDbIterationSnafu)?;
            let point: Point =
                deserialize(&v).context(error::DeserializationSnafu { id: offset })?;

            if point.id <= offset {
                continue;
            }

            if let Some(vec) = point.vector {
                last_id = point.id;
                result.push((point.id, vec));
                if result.len() == limit {
                    break;
                }
            }
        }
        Ok(Some((result, last_id)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use defs::ContentType;
    use uuid::Uuid;

    use tempfile::{TempDir, tempdir};

    fn create_test_db() -> (RocksDbStorage, TempDir) {
        let temp_dir = tempdir().unwrap();

        let db = RocksDbStorage::new(temp_dir.path()).expect("Failed to create RocksDB");
        (db, temp_dir)
    }

    #[test]
    fn test_new_rocksdb_storage() {
        let (db, temp_dir) = create_test_db();
        assert_eq!(db.get_current_path(), PathBuf::from(temp_dir.path()));
    }

    #[test]
    fn test_insert_and_get_vector() {
        let (db, _temp_dir) = create_test_db();
        let id = Uuid::new_v4();
        let vector = Some(vec![0.1, 0.2, 0.3]);
        let payload = Some(Payload {
            content_type: ContentType::Text,
            content: "Test".to_string(),
        });

        assert!(db.insert_point(id, vector.clone(), payload).is_ok());
        let result = db.get_vector(id).unwrap();
        assert_eq!(result, vector);
    }

    #[test]
    fn test_insert_and_get_payload() {
        let (db, _temp_dir) = create_test_db();
        let id = Uuid::new_v4();
        let payload = Some(Payload {
            content_type: ContentType::Text,
            content: "Test".to_string(),
        });
        let vector = None;

        // Move payload into insert_point and recreate expected for comparison
        assert!(db.insert_point(id, vector, payload).is_ok());
        let result = db.get_payload(id).unwrap();
        let expected = Some(Payload {
            content_type: ContentType::Text,
            content: "Test".to_string(),
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_contains_point() {
        let (db, _temp_dir) = create_test_db();
        let id = Uuid::new_v4();
        let payload = Some(Payload {
            content_type: ContentType::Text,
            content: "Test".to_string(),
        });

        assert!(!db.contains_point(id).unwrap());

        let vector = Some(vec![0.4, 0.5, 0.6]);
        db.insert_point(id, vector, payload).unwrap();

        assert!(db.contains_point(id).unwrap());
    }

    #[test]
    fn test_delete_point() {
        let (db, _temp_dir) = create_test_db();
        let id = Uuid::new_v4();
        let payload = Some(Payload {
            content_type: ContentType::Text,
            content: "Test".to_string(),
        });

        let vector = vec![0.7, 0.8, 0.9];

        db.insert_point(id, Some(vector), payload).unwrap();

        assert!(db.contains_point(id).unwrap());

        db.delete_point(id).unwrap();

        assert!(!db.contains_point(id).unwrap());
        assert_eq!(db.get_vector(id).unwrap(), None);
        assert_eq!(db.get_payload(id).unwrap(), None);
    }

    #[test]
    fn test_get_nonexistent_vector() {
        let (db, _temp_dir) = create_test_db();
        let id = Uuid::new_v4();

        assert_eq!(db.get_vector(id).unwrap(), None);
    }

    #[test]
    fn test_get_nonexistent_payload() {
        let (db, _temp_dir) = create_test_db();
        let id = Uuid::new_v4();

        assert_eq!(db.get_payload(id).unwrap(), None);
    }
    #[test]
    fn test_error_context_preservation() {
        // Test that the error chain is preserved
        let result = RocksDbStorage::new("/proc/invalid-path");

        if let Err(err) = result {
            // The Display implementation should show both the context and source
            let err_string = format!("{}", err);
            println!("Full error message: {}", err_string);

            // Should contain our custom context
            assert!(err_string.contains("Failed to open RocksDB"));
            assert!(err_string.contains("/proc/invalid-path"));

            // The error should also be debuggable
            let debug_string = format!("{:?}", err);
            println!("Debug format: {}", debug_string);
        }
    }
}
