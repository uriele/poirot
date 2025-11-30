
use std::fmt::{self, Display, Formatter};
use std::path::Path;
use std::path::PathBuf;
use cozo::{ DbInstance, ScriptMutability}; // cozo for database
use log::{info,error}; // logging
use crate::database::schema::{SCHEMA, HNSW_INDEX};
pub enum Engine{
    Mem,
    SQLite,
    RocksDB,
}

pub struct AcademicResourceManager {
    engine: Engine,
    path: Option<PathBuf>,
    pub db: DbInstance,
}

impl AcademicResourceManager {
    pub fn new(engine: Engine, path: impl AsRef<Path>) -> Result<Self, cozo::Error> {
        info!("Starting AcademicResourceManager...");
        let opt="{}";
        let _engine = match engine {
            Engine::Mem => "mem",
            Engine::SQLite => "sqlite",
            Engine::RocksDB => "rocksdb",
        };
        info!("Using engine: {:?}", _engine);
        let p = match engine {
            Engine::Mem => None,
            _ => {
                let p = path.as_ref().to_path_buf();
                Some(p)
            }
        };
        info!("Initializing database...");
        let db= DbInstance::new(_engine,path,opt)?;

        
       

        info!("Applying schema...");
        // create migrate the schema
        db.run_script(
            SCHEMA,
            Default::default(),
            ScriptMutability::Mutable,
        )
        .map_err(|e| {
            error!("Failed to apply schema: {}", e);
            e
        })?;

        info!("Applying HNSW index...");
        // Create HNSW index for vector search
        db.run_script(
            HNSW_INDEX,
            Default::default(),
            ScriptMutability::Mutable,
        )
        .map_err(|e| {
            error!("Failed to create HNSW index: {}", e);
            e
        })?;
        info!("Database initialized successfully.");



        Ok(Self{ engine, path: p, db })
    }
    

    pub fn get_path(&self)  {
        match &self.path {
            Some(p) => info!("Database path: {:?}", p),
            None => info!("In-memory database has no path."),
        }      
    }

    pub fn get_engine(&self)  {
        info!("Database engine: {}", self.engine);
    }
}

impl Display for Engine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Engine::Mem => write!(f, "In-Memory"),
            Engine::SQLite => write!(f, "SQLite"),
            Engine::RocksDB => write!(f, "RocksDB"),
        }
    }
}

impl Display for AcademicResourceManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AcademicResourceManager using {} engine at {}",
            self.engine,
            self.path
                .as_ref()
                .map_or("in-memory".to_string(), |p| p.display().to_string())
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    
    fn remove_if_exists(path: &str) {
        if let Err(e) = fs::remove_file(path) {
            // Ignore "not found", rethrow others
            if e.kind() != std::io::ErrorKind::NotFound {
                panic!("failed to remove test db {}: {e}", path);
            }
        }
    }
    #[test]
    fn test_academic_resource_manager_init() {
        let arm = AcademicResourceManager::new(Engine::Mem, ":memory:");
        assert!(arm.is_ok());
        let arm = arm.unwrap();
        assert_eq!(format!("{}", arm.engine), "In-Memory");
        assert!(arm.path.is_none());

        // SQLite: ensure fresh file
        let sqlite_path = "test_db_init.sqlite";
        remove_if_exists(sqlite_path);
        let arm_sqlite = AcademicResourceManager::new(Engine::SQLite, sqlite_path);
        assert!(arm_sqlite.is_ok());
        let arm_sqlite = arm_sqlite.unwrap();
        assert_eq!(format!("{}", arm_sqlite.engine), "SQLite");
        assert!(arm_sqlite.path.is_some());
        assert_eq!(arm_sqlite.path.unwrap(), PathBuf::from(sqlite_path));

        // RocksDB: ensure fresh directory/file
        let rocks_path = "test_db_init.db";
        // RocksDB may create a directory; remove both file or dir if present
        let _ = fs::remove_dir_all(rocks_path);
        remove_if_exists(rocks_path);
        let arm_rocksdb = AcademicResourceManager::new(Engine::RocksDB, rocks_path);
        assert!(arm_rocksdb.is_ok());
        let arm_rocksdb = arm_rocksdb.unwrap();
        assert_eq!(format!("{}", arm_rocksdb.engine), "RocksDB");
        assert!(arm_rocksdb.path.is_some());
        assert_eq!(arm_rocksdb.path.unwrap(), PathBuf::from(rocks_path));
    }
 #[test]
    fn test_academic_resource_manager_path_engine() {
        let path = "test1_db_path_engine.db";
        let _ = std::fs::remove_dir_all(path);
        remove_if_exists(path);

        let arm = AcademicResourceManager::new(Engine::RocksDB, path).unwrap();
        assert_eq!(format!("{}", arm.engine), "RocksDB");
        assert!(arm.path.is_some());
        assert_eq!(arm.path.unwrap(), PathBuf::from(path));
    }

    #[test]
    fn test_academic_resource_manager_display() {
        let path = "test1_db_display.sqlite";
        remove_if_exists(path);

        let arm = AcademicResourceManager::new(Engine::SQLite, path).unwrap();
        let display_str = format!("{}", arm);
        assert!(display_str.contains("SQLite"));
        assert!(display_str.contains(path));
    }
}