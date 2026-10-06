// original: 0x00b28d70 slot_row_allocate

/// Allocates and trims one row's record data, then smooths it.
///
/// Calls the opener with (`a0`, name, 0, 1), sizes the allocation through
/// two measure calls, allocates through the allocator callee and stores the
/// pointer in row `a1`; stores the sized count from the sizer callee; calls
/// the commit callee; then trims the count (compared signed) down past any
/// trailing zero word at a 32-byte boundary, stopping below zero; finally
/// calls the smoother with `a1` and returns its answer with the low byte set
/// to 1. Cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_00b28d70(a0: u32, a1: u32) -> u32 {
    unsafe {
        const ROW_TABLE: u32 = 0x01657A10;
        const ROW_STRIDE: u32 = 16;
        const ROW_PTR: u32 = 4;
        const ROW_COUNT: u32 = 8;
        const CTX: u32 = 0x0110C0A0;
        const NAME: u32 = 0x00EAC4CD;
        const OPEN: u32 = 0;
        const MEASURE1: u32 = 1;
        const ALLOC: u32 = 2;
        const MEASURE2: u32 = 3;
        const SIZE: u32 = 4;
        const COMMIT: u32 = 5;
        const SMOOTH: u32 = 6;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let s = lf_checker_rt::callee_thiscall!(
            OPEN, u32, lf_checker_rt::relocated(CTX),
            a0, lf_checker_rt::relocated(NAME), 0, 1
        );
        let row = lf_checker_rt::relocated(ROW_TABLE) + a1.wrapping_mul(ROW_STRIDE);
        let x = lf_checker_rt::callee_thiscall!(MEASURE1, u32, s);
        let p = lf_checker_rt::callee_cdecl!(ALLOC, u32, x);
        wr32(row + ROW_PTR, p);
        let y = lf_checker_rt::callee_thiscall!(MEASURE2, u32, s);
        let n = lf_checker_rt::callee_thiscall!(SIZE, u32, s, p, y);
        wr32(row + ROW_COUNT, n);
        lf_checker_rt::callee_thiscall!(COMMIT, u32, s);
        let mut bound = n as i32;
        if bound > 0 {
            let mut off = 0i32;
            loop {
                let masked = (off as u32) & 0xFFFF_FFE0;
                if rd32(p.wrapping_add(masked)) == 0 && off != 0 {
                    bound = off;
                    wr32(row + ROW_COUNT, off as u32);
                }
                off = off.wrapping_add(0x20);
                if !(off < bound) {
                    break;
                }
            }
        }
        let ans = lf_checker_rt::callee_cdecl!(SMOOTH, u32, a1);
        (ans & 0xFFFF_FF00) | 1
    }
});
