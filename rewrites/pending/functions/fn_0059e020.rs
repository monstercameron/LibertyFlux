// original: 0x0059e020 poll_mode_gate
// True when the polled mode byte is outside the inactive set and the
// secondary mode check reports inactive (returns 0 in AL).
//
// Inactive modes are {0, 2, 4, 5, 7, 8, 9, 10}; any other mode delegates to
// the callee, which receives the state base in ECX like the original.
export!(cdecl, rw_0059E020() -> u32 {
    let base = unsafe { *global::<u32>(0x18B6E84) };
    let mode = unsafe { *((base + 0x44D) as *const u8) };
    match mode {
        0 | 2 | 4 | 5 | 7 | 8 | 9 | 10 => 0,
        _ => {
            let r = callee_thiscall!(1, u32, base);
            u32::from((r & 0xFF) == 0)
        }
    }
});
