// original: 0x009b8730 CamInstr_E94638_Create
/// Factory creating the instruction object identified by class word 0xE94638.
///
/// Allocates a 28-byte block, stamps the class word and a zero flag byte,
/// then stores four float bit-patterns and one flag byte from the stack
/// arguments. Returns null when allocation fails.
export!(stdcall, rw_009b8730(f0: u32, f1: u32, f2: u32, f3: u32, flag: u32) -> u32 {
    const CLASS: u32 = 0x00E94638;
    unsafe {
        let p = callee_cdecl!(0, u32, 0x1cu32) as *mut u8;
        if p.is_null() {
            return 0;
        }
        *(p as *mut u32) = relocated(CLASS);
        *p.add(4) = 0;
        *(p.add(8) as *mut u32) = f0;
        *(p.add(0x0c) as *mut u32) = f1;
        *(p.add(0x10) as *mut u32) = f2;
        *(p.add(0x14) as *mut u32) = f3;
        *p.add(0x18) = (flag & 0xFF) as u8;
        p as u32
    }
});
