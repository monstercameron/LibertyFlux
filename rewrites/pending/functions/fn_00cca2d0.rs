// original: 0x00cca2d0 euphoria_behavior_ctor_id194
/// Euphoria behaviour constructor, variant id 0x194: same shape as
/// `rw_00cca290` with this variant's tag words. Returns `this`.
export!(thiscall, rw_00cca2d0(this: *mut u8) -> u32 {
    unsafe {
        const FLAG: u32 = 0xC1;
        const ID: u32 = 0x194;
        const BASE_TAG: u32 = 0xED9CD0;
        const VTABLE: u32 = 0xED9C7C;
        let _: u32 = callee_thiscall!(1, u32, this as u32, 0x2D, FLAG,
            0x4080_0000, ID, relocated(BASE_TAG), 0, 0x3F80_0000, 0);
        *(this as *mut u32) = relocated(VTABLE);
        this as u32
    }
});
