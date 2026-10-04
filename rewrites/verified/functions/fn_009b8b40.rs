// original: 0x009b8b40 CamInstr_E940A8_Create
/// Factory creating the instruction object identified by class word 0xE940A8.
///
/// Allocates a 32-byte block, stamps the class word and a zero flag byte,
/// stores one integer field, then copies a four-word vector from the
/// pointed-to source. The word at offset 0x0c is never written, matching
/// the original. Returns null when allocation fails.
export!(stdcall, rw_009b8b40(a0: u32, src: *const u32) -> u32 {
    const CLASS: u32 = 0x00E940A8;
    unsafe {
        let p = callee_cdecl!(0, u32, 0x20u32) as *mut u8;
        if p.is_null() {
            return 0;
        }
        *(p as *mut u32) = relocated(CLASS);
        *p.add(4) = 0;
        *(p.add(8) as *mut u32) = a0;
        *(p.add(0x10) as *mut u32) = *src;
        *(p.add(0x14) as *mut u32) = *src.add(1);
        *(p.add(0x18) as *mut u32) = *src.add(2);
        *(p.add(0x1c) as *mut u32) = *src.add(3);
        p as u32
    }
});
