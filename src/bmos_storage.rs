// struct Person {
//     id: i32,
//     name: String,
//     data: Option<Vec<u8>>,
// }

pub struct BmosStorageError {
    pub detail: String,
}

//trait BmosStorageConnection;
pub type BmosStorageResult<T> = Result<T, BmosStorageError>;


pub trait BmosStorage {
    //fn get_connection(&self) -> BmosStorageConnection;
    //fn new(&self) -> BmosStorage;
    fn create_sensors_table(&self) -> BmosStorageResult<()>;
}
