#![no_main]

// Parse arbitrary org-mode input, then drive both exporters over the parse
// tree (same code path as the original `parser` target, broadened to cover
// the export half of the library as well).
libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let org = orgize::Org::parse(s);
        let _ = org.to_org();
        let _ = org.to_html();
    }
});
