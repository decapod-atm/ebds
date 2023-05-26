#![no_main]

use libfuzzer_sys::fuzz_target;

use ebds::{OmnibusReply, MessageOps, MessageType, len};

fuzz_target!(|data: &[u8]| {
    if data.len() < len::OMNIBUS_REPLY {return}

    let mut reply = OmnibusReply::new();
    if let Err(_err) = reply.from_buf(data) {return}

    assert_eq!(reply.buf().len(), len::OMNIBUS_REPLY);
});
