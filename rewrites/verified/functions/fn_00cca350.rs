// original: 0x00cca350 euphoria_behavior_ctor_id196
/// Euphoria behaviour constructor, variant id 0x196: same shape as
/// `rw_00cca290` with this variant's tag words. Returns `this`.
export!(thiscall, rw_00cca350(this: *mut u8) -> u32 {
    unsafe {
        const FLAG: u32 = 0xC4;
        const ID: u32 = 0x196;
        const BASE_TAG: u32 = 0xED9E08;
        const VTABLE: u32 = 0xED9DB4;
        let _: u32 = callee_thiscall!(1, u32, this as u32, 0x2D, FLAG,
            0x4080_0000, ID, relocated(BASE_TAG), 0, 0x3F80_0000, 0);
        *(this as *mut u32) = relocated(VTABLE);
        this as u32
    }
});
