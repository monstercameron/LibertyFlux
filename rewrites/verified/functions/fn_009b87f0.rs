// original: 0x009b87f0 CamInstr_SetHintMoveInDistDefault::Create
/// Factory creating a `SetHintMoveInDistDefault` camera instruction.
///
/// Allocates an 8-byte block and stamps the class word and a zero flag byte.
/// Takes no arguments. Returns null when allocation fails.
export!(stdcall, rw_009b87f0() -> u32 {
    const CLASS: u32 = 0x00E94628;
    unsafe {
        let p = callee_cdecl!(0, u32, 8u32) as *mut u8;
        if p.is_null() {
            return 0;
        }
        *(p as *mut u32) = relocated(CLASS);
        *p.add(4) = 0;
        p as u32
    }
});
