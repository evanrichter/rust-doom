#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let wad = std::io::Cursor::new(data);
    let _ = wad::Archive::open_wad_fuzz(wad);
});
