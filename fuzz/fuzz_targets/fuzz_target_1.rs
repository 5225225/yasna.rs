#![no_main]
use libfuzzer_sys::fuzz_target;



fuzz_target!(|data: &[u8]| {
    let asn: Result<bool, _> = yasna::decode_ber(data);
    // fuzzed code goes here
});
