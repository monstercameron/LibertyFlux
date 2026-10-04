// original: 0x009b8900 CamInstr_E94238_Create
/// Factory creating the instruction object identified by class word 0xE94238.
///
/// Allocates a 16-byte block, stamps the class word and a zero flag byte,
/// then stores two integer fields from the stack arguments. Returns null
/// when allocation fails.
export!(stdcall, rw_009b8900(a0: u32, a1: u32) -> u32 {
    const CLASS: u32 = 0x00E94238;
    unsafe {
        let p = callee_cdecl!(0, u32, 0x10u32) as *mut u8;
        if p.is_null() {
            return 0;
        }
        *(p as *mut u32) = relocated(CLASS);
        *p.add(4) = 0;
        *(p.add(8) as *mut u32) = a0;
        *(p.add(0x0c) as *mut u32) = a1;
        p as u32
    }
});
