// original: 0x008FB980 stream_request_clear_big
/// Clear one big-table request entry and detach both resources.
///
/// Entry `b + a * 4` of the big table is reset to the idle pattern
/// while its two link words are handed to the detach routine (each
/// read before its own reset), then both links are set to -1 and the
/// valid flag is set. Cdecl, two stack arguments; returns the second
/// detach result.
export!(cdecl, rw_008fb980(a: u32, b: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x118dec0;
        const MGR: u32 = 0x11db280;
        const ENTRY: u32 = 0x40;
        let e = relocated(TABLE).wrapping_add(
            b.wrapping_add(a.wrapping_mul(4)).wrapping_mul(ENTRY));
        let v30 = ((e + 0x30) as *const u32).read_unaligned();
        ((e) as *mut u32).write_unaligned(0);
        ((e + 0x04) as *mut u32).write_unaligned(0xFFFFFFFF);
        ((e + 0x08) as *mut u32).write_unaligned(0);
        ((e + 0x0c) as *mut u32).write_unaligned(0xFFFFFFFF);
        ((e + 0x38) as *mut u16).write_unaligned(0);
        ((e + 0x10) as *mut u32).write_unaligned(0);
        ((e + 0x14) as *mut u32).write_unaligned(0);
        for off in [0x18u32, 0x1c, 0x20, 0x24, 0x28, 0x2c] {
            ((e + off) as *mut u32).write_unaligned(0xFFFFFFFF);
        }
        let _: u32 = callee_thiscall!(1, u32, relocated(MGR), v30, 0);
        let v34 = ((e + 0x34) as *const u32).read_unaligned();
        ((e + 0x30) as *mut u32).write_unaligned(0xFFFFFFFF);
        let ans: u32 = callee_thiscall!(2, u32, relocated(MGR), v34, 0);
        ((e + 0x34) as *mut u32).write_unaligned(0xFFFFFFFF);
        ((e + 0x3b) as *mut u8).write(1);
        ans
    }
});
