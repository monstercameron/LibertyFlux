// original: 0x009b8810 CamInstr_E945E8_Create
/// Factory creating the instruction object identified by class word 0xE945E8.
///
/// Allocates a 20-byte block, stamps the class word and a zero flag byte,
/// then stores three integer fields from the stack arguments. Returns null
/// when allocation fails.
export!(stdcall, rw_009b8810(a0: u32, a1: u32, a2: u32) -> u32 {
    const CLASS: u32 = 0x00E945E8;
    unsafe {
        let p = callee_cdecl!(0, u32, 0x14u32) as *mut u8;
        if p.is_null() {
            return 0;
        }
        *(p as *mut u32) = relocated(CLASS);
        *p.add(4) = 0;
        *(p.add(8) as *mut u32) = a0;
        *(p.add(0x0c) as *mut u32) = a1;
        *(p.add(0x10) as *mut u32) = a2;
        p as u32
    }
});
