#[derive(Debug)]
pub struct BmosStorageError {
    pub detail: String,
}

//trait BmosStorageConnection;
pub type BmosStorageResult<T> = Result<T, BmosStorageError>;


pub trait BmosStorage {
    //fn get_connection(&self) -> BmosStorageConnection;
    //fn new(&self) -> BmosStorage;
    fn create_tables(&self) -> BmosStorageResult<()>;
}
