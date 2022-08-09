#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (&[u8], &[u8])| {
    let (wad, meta) = data;
    let wad = memfd_path::InMemFilePath::new(wad).unwrap();
    let meta = memfd_path::InMemFilePath::new(meta).unwrap();
    if let Ok(archive) = wad::Archive::open(&wad, &meta) {
        for level in 0..archive.num_levels() {
            let _ = wad::Level::from_archive(&archive, level);
        }
    }
});
