#![no_main]

use libfuzzer_sys::fuzz_target;

use ebds::Method;

fuzz_target!(|data: &[u8]| {
    let upper_data_str = std::str::from_utf8(data).unwrap_or("").to_uppercase();
    let method = Method::from(data);

    match upper_data_str.as_str() {
        "ACCEPT" => assert_eq!(method, Method::Accept), 
        "STOP" => assert_eq!(method, Method::Stop),
        "DISPENSE" => assert_eq!(method, Method::Dispense),
        "STACK" => assert_eq!(method, Method::Stack), 
        "REJECT" => assert_eq!(method, Method::Reject),
        "STATUS" => assert_eq!(method, Method::Status),
        "ESCROW_FULL" => assert_eq!(method, Method::EscrowFull), 
        "RESET" => assert_eq!(method, Method::Reset),
        "SHUTDOWN" => assert_eq!(method, Method::Shutdown),
        "UNKNOWN" => assert_eq!(method, Method::Unknown),
        _ => assert_eq!(method, Method::Unknown),
    }
});
