// original: 0x0092D0A0 current_frame_slot_ptr (proposed)

/// Return the render slot selected by the current thread's frame flags.
///
/// Reads the thread-local frame word: the TLS index comes from the global
/// `TLS_INDEX`, the slot value is read through `tls_slot` (the original
/// reads `FS:[0x2c]`), and the word at `+0x8d0` holds the flags. When bit 1
/// is set the primary frame index (`FRAME_A`) is used; otherwise bit 3
/// selects between the primary index (set) and the secondary index
/// (`FRAME_B`, clear). The result is `SLOT_BASE + index * SLOT_STRIDE`.
///
/// Original: 0x0092D0A0 (cdecl, no arguments). Leaf: no calls, reads TLS
/// and globals.
lf_checker_rt::export!(cdecl, rw_0092D0A0() -> u32 {
    unsafe {
        const TLS_INDEX: u32 = 0x017A_BA14;
        const FLAGS_OFF: u32 = 0x8d0;
        const FRAME_A: u32 = 0x0117_4790;
        const FRAME_B: u32 = 0x0117_4794;
        const SLOT_BASE: u32 = 0x011A_1DF0;
        const SLOT_STRIDE: u32 = 0x270;
        let idx = (lf_checker_rt::relocated(TLS_INDEX) as *const u32).read_unaligned();
        let tls = lf_checker_rt::tls_slot(idx as usize);
        let flags = (tls.wrapping_add(FLAGS_OFF) as *const u32).read_unaligned();
        let sel = if (flags >> 1) & 1 != 0 {
            (lf_checker_rt::relocated(FRAME_A) as *const u32).read_unaligned()
        } else if (flags >> 3) & 1 != 0 {
            (lf_checker_rt::relocated(FRAME_A) as *const u32).read_unaligned()
        } else {
            (lf_checker_rt::relocated(FRAME_B) as *const u32).read_unaligned()
        };
        sel.wrapping_mul(SLOT_STRIDE).wrapping_add(lf_checker_rt::relocated(SLOT_BASE))
    }
});
