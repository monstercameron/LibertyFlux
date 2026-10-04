// original: 0x009b87c0 CamInstr_E94618_Create
/// Factory creating the instruction object identified by class word 0xE94618.
///
/// Allocates a 12-byte block, stamps the class word and a zero flag byte,
/// then stores one float bit-pattern from the stack argument. Returns null
/// when allocation fails.
export!(stdcall, rw_009b87c0(f0: u32) -> u32 {
    const CLASS: u32 = 0x00E94618;
    unsafe {
        let p = callee_cdecl!(0, u32, 0x0cu32) as *mut u8;
        if p.is_null() {
            return 0;
        }
        *(p as *mut u32) = relocated(CLASS);
        *p.add(4) = 0;
        *(p.add(8) as *mut u32) = f0;
        p as u32
    }
});
