#![no_main]

use libfuzzer_sys::fuzz_target;

use ebds::{OmnibusReply, MessageOps, len::OMNIBUS_REPLY};

fuzz_target!(|reply: OmnibusReply| {
    assert_eq!(reply.buf().len(), OMNIBUS_REPLY);
});
