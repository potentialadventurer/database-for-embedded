#![cfg_attr(not(feature = "std"), no_std)]

use defmt_test;

mod domain;

struct MyState {
    flag: bool,
}

#[cfg(test)]
#[defmt_test::test]
mod test {
    #[init]
    fn init() ->super::MyState {
        super::MyState {
            flag: true
        }
    }
    use crate::domain::kv_store::{Database, Key, Value};

    #[before_each]
    fn before_each(state: &mut super::MyState) {
        defmt::println!("State flag before is {}", state.flag);
    }

    #[test]
    fn insert_key() {
        use crate::domain::kv_store::{Database, Key, Value};

        let mut db = Database::new();
        let key = Key::parse("key").unwrap();
        let value = Value::parse("value").unwrap();

        let _ =  db.set(key.clone(), value.clone());
        assert_eq!(db.get(&key).unwrap(), value);

    }

    #[after_each]
    fn after_each(state: &mut super::MyState) {
        defmt::println!("State flag after is {}", state.flag);
    }
}