#![no_main]

use libfuzzer_sys::fuzz_target;

use ebds::{MessageVariant, len};

fuzz_target!(|data: &[u8]| {
    // Omnibus Reply's length is the minimum for any reply message
    if data.len() < len::OMNIBUS_REPLY {return}

    let msg = match MessageVariant::from_buf(data) {
        Ok(msg) => msg,
        Err(_err) => return,
    };

    assert!(msg.as_omnibus_reply().len() <= data.len());
});
