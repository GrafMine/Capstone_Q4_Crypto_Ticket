use diesel::result::Error;

pub trait Repository<T, ID> {
    fn create(&mut self, entity: T) -> Result<T, Error>;
    fn find_by_id(&mut self, id: ID) -> Result<Option<T>, Error>;
    fn find_all(&mut self) -> Result<Vec<T>, Error>;
    fn update(&mut self, entity: T) -> Result<T, Error>;
    fn delete(&mut self, id: ID) -> Result<(), Error>;
}