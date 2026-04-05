#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use mod domain;

#[entry]
fn main() -> ! {
        
    let mut db = Database::new();

    let key = Key::parse("new").unwrap();
    let value = Value::parse("word").unwrap();

    let _ = db.set(key.clone(), value);

    let answer = db.get(&key);
    hprintln!("value: {:?}, answer);


    loop{}
    
}