#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f1xx_hal::{flash::{FlashExt, FlashSize, SectorSize}, pac};

mod domain;
use domain::kv_store::{Database, Key, Value};

#[entry]
fn main() -> ! {
        
    let mut db = Database::new();

    let key = Key::parse("new").unwrap();
    let value = Value::parse("word").unwrap();

    let _ = db.set(key.clone(), value);

    let answer = db.get(&key);
    hprintln!("value: {:?}", answer);


    loop{}
    
}