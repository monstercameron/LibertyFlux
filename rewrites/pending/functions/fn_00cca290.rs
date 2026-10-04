// original: 0x00cca290 euphoria_behavior_ctor_id193
/// Euphoria behaviour constructor, variant id 0x193: run the shared
/// 8-argument base setup (thiscall/8, id 1) with the variant's tag words,
/// stamp this variant's vtable. Returns `this`. (No merged symbol.)
export!(thiscall, rw_00cca290(this: *mut u8) -> u32 {
    unsafe {
        const FLAG: u32 = 0xC2;
        const ID: u32 = 0x193;
        const BASE_TAG: u32 = 0xED9D38;
        const VTABLE: u32 = 0xED9CE4;
        let _: u32 = callee_thiscall!(1, u32, this as u32, 0x2D, FLAG,
            0x4080_0000, ID, relocated(BASE_TAG), 0, 0x3F80_0000, 0);
        *(this as *mut u32) = relocated(VTABLE);
        this as u32
    }
});
