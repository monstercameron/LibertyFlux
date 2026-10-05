// original: 0x008FBA20 NativeImpl_CLEAR_SMALL_PRINTS
/// Clear one small-table print entry and detach both resources.
///
/// Like the big-table clear, on the small table: entry `a` is reset
/// while its two link words go to the detach routine (each read
/// before its own reset), then both links become -1, the marker word
/// 0x0101 is written and the flag byte cleared. Cdecl, one stack
/// argument; returns the second detach result.
export!(cdecl, rw_008fba20(a: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x118e5c0;
        const MGR: u32 = 0x11db280;
        const ENTRY: u32 = 0x40;
        let e = relocated(TABLE).wrapping_add(a.wrapping_mul(ENTRY));
        let v30 = ((e + 0x30) as *const u32).read_unaligned();
        ((e) as *mut u32).write_unaligned(0);
        ((e + 0x04) as *mut u32).write_unaligned(0xFFFFFFFF);
        ((e + 0x08) as *mut u32).write_unaligned(0);
        ((e + 0x0c) as *mut u32).write_unaligned(0xFFFFFFFF);
        ((e + 0x10) as *mut u32).write_unaligned(0);
        ((e + 0x14) as *mut u32).write_unaligned(0);
        for off in [0x18u32, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
            ((e + off) as *mut u32).write_unaligned(0xFFFFFFFF);
        }
        ((e + 0x38) as *mut u16).write_unaligned(0);
        let _: u32 = callee_thiscall!(1, u32, relocated(MGR), v30, 0);
        let v34 = ((e + 0x34) as *const u32).read_unaligned();
        let ans: u32 = callee_thiscall!(2, u32, relocated(MGR), v34, 0);
        ((e + 0x30) as *mut u32).write_unaligned(0xFFFFFFFF);
        ((e + 0x34) as *mut u32).write_unaligned(0xFFFFFFFF);
        ((e + 0x3a) as *mut u16).write_unaligned(0x0101);
        ((e + 0x3c) as *mut u8).write(0);
        ans
    }
});
