use heapless::{String, FnvIndexMap as IndexMap};


#[derive(PartialEq, Clone, Debug)]
pub struct Key(String<16>);

impl Key {
    pub fn parse(s: &str) -> Result<Key, KVStoreError> {
        if s.len() > 16 {
            return Err(KVStoreError::TooLongError);
        }
        let mut key: String<16> = String::new();

        let result = key.push_str(s);
        if result.is_err() {
            return Err(KVStoreError::UnknownError);
        }
        Ok(Key(key))
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Value(String<32>);

impl Value {
    pub fn parse(s: &str) -> Result<Value, KVStoreError> {
        if s.len() >32 {
            return Err(KVStoreError::TooLongError);
        }
        let mut value: String<32> = String::new();

        let result = value.push_str(s);
        if result.is_err() {
            return Err(KVStoreError::UnknownError);
        }
        Ok(Value(value))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum KVStoreError {
    NotFound,
    WriteError,
    EraseError,
    TooLongError,
    UnknownError,
}



pub struct Database {
    data: IndexMap<Key, Value, 64>,
}

impl Database {
    pub fn new() -> Self {
        Database {data: IndexMap::new()}
    }

    pub fn get(&self, key: &Key) -> Result<Value, KVStoreError> {
        let result = self.data.get(key);
        if let Some(value) = result {
            return Ok(value.clone()); 
        }
        Err(KVStoreError::NotFound)
    }

    pub fn set(&mut self, key: Key, value: Value) -> Result<(), KVStoreError> {
        let result = self.data.insert(key, value);
        if result.is_err() {
            return Err(KVStoreError::WriteError);
        }
        Ok(())
    }

    pub fn delete(&mut self, key: &Key) -> Result<(), KVStoreError> {
        self.data.swap_remove(key);
        Ok(())
    }
}

