// original: 0x009b7870 rw_009b7870
/// Allocate a 16-byte script instruction node of the fourth kind: vtable,
/// zero flag byte, low byte of the first argument, second argument word.
/// Returns the node, or null when allocation fails.
export!(stdcall, rw_009b7870(a: u32, b: u32) -> u32 {
    unsafe {
        const SIZE: u32 = 16;
        const VTABLE: u32 = 0xE93FF8;
        let p = callee_cdecl!(1, u32, SIZE);
        if p == 0 {
            return 0;
        }
        *(p as *mut u32) = relocated(VTABLE);
        *((p.wrapping_add(4)) as *mut u8) = 0;
        *((p.wrapping_add(8)) as *mut u8) = a as u8;
        *((p.wrapping_add(0xC)) as *mut u32) = b;
        p
    }
});
