// original: 0x008C9580 stream_variant_select
/// Map the streaming variant global to its small integer code.
///
/// Reads the variant selector: 1 maps to 13, 2 maps to 14, anything else
/// maps to 12. Reads one global, makes no calls. Original: cdecl, no words.
lf_checker_rt::export!(cdecl, rw_008c9580() -> u32 {
    unsafe {
        match *lf_checker_rt::global::<u32>(0x11d6fd4) {
            1 => 13,
            2 => 14,
            _ => 12,
        }
    }
});
