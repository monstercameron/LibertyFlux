// original: 0x00B63F70 veh_classify_store_7a8
/// Classify `a1` through a lookup call and store the class at `[a0+0x7a8]`.
///
/// Returns at once when `a1` is null. Otherwise calls the classifier (stubbed,
/// cdecl/2) with `(a0, [a1+0x52])`. Codes 0x4B4/0x4B5/0x37A0 take threshold 1,
/// codes 0/0x1A1/0x4B2/0x4B3/0x36A0/0x36A1/0x4C7/0x4C0 threshold 5, anything
/// else returns without storing. On a classified code, resolves the handle
/// (stubbed, cdecl/1) from `[this+0x18]`; stores the code at `[a0+0x7a8]` unless
/// the dword at handle+0x10 is below the threshold (signed). Thiscall, two
/// stack words. No meaningful return value.
export!(thiscall, rw_00b63f70(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const WORD: u32 = 0x52;
        const HANDLE: u32 = 0x18;
        const LEVEL: u32 = 0x10;
        const SLOT: u32 = 0x7a8;
        if a1 == 0 {
            return 0;
        }
        let w = ((a1 + WORD) as *const u16).read_unaligned() as u32;
        let s: u32 = callee_cdecl!(1, u32, a0, w);
        let thresh: i32 = match s {
            0x4b4 | 0x4b5 | 0x37a0 => 1,
            0 | 0x1a1 | 0x4b2 | 0x4b3 | 0x36a0 | 0x36a1 | 0x4c7 | 0x4c0 => 5,
            _ => return 0,
        };
        let h: u32 = callee_cdecl!(2, u32, ((this + HANDLE) as *const u32).read_unaligned());
        if ((h + LEVEL) as *const i32).read_unaligned() < thresh {
            return 0;
        }
        ((a0 + SLOT) as *mut u32).write_unaligned(s);
        0
    }
});
