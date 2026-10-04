// original: 0x009f6430 NativeImpl_GET_STAT_FRONTEND_VISIBILITY
use lf_k2_rt::{export, global};

/// First valid tunable code for the pointer table.
const F1_FIRST: u32 = 0x289;

/// Number of valid codes minus one (codes 0x289..=0x2A9).
const F1_SPAN: u32 = 0x20;

/// Pointer-table lookup with validity check (cdecl/1 -> eax).
///
/// Returns the table entry for `code` when the code is in range, the entry is
/// non-null and the pointed-to byte is nonzero; otherwise null.
export!(cdecl, rw_s18f1(code: u32) -> u32 {
    unsafe {
        if code.wrapping_sub(F1_FIRST) > F1_SPAN {
            return 0;
        }
        // Note: indexed by the full code, not code - FIRST.
        let entry = *global::<u32>(0x12B5A74).add(code as usize);
        if entry == 0 {
            return 0;
        }
        if *((entry as *const u8)) == 0 {
            return 0;
        }
        entry
    }
});
