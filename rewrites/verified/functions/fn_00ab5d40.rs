// original: 0x00ab5d40 stream_buf_reset (proposed)

/// Reset a two-word allocation header and return it.
///
/// Clears both words, releases a null block through the heap-free callee
/// (which answers through the script), clears both words again and returns
/// the header pointer.
///
/// Callees: 1 = heap free (cdecl, one word).
///
/// Original: 0x00ab5d40 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ab5d40(hdr: u32) -> u32 {
    unsafe {
        const FREE_CALLEE: u32 = 1;
        (hdr as *mut u32).write_unaligned(0);
        ((hdr + 4) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, 0u32);
        ((hdr + 4) as *mut u32).write_unaligned(0);
        (hdr as *mut u32).write_unaligned(0);
        hdr
    }
});
