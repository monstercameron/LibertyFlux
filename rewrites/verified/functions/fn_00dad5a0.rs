// original: 0x00dad5a0 CTaskComplexTrackEntity::vf19
/// When the progress float at +0x50 is negative, re-seed it from the kind
/// word at argument+0xB80 (1->0.0, 2->1.0, 3->2.0, anything else->3.0),
/// then tail-call the handler in this object's vtable slot 0x48 with the
/// same argument and return its answer.
export!(thiscall, rw_00dad5a0(this: u32, arg: u32) -> u32 {
    unsafe {
        let slot = (this as *mut u32).byte_add(0x50);
        if f32::from_bits(*slot) < 0.0 {
            let kind = *((arg as *const u32).byte_add(0xB80));
            *slot = match kind {
                1 => 0x00000000,
                2 => 0x3F800000,
                3 => 0x40000000,
                _ => 0x40400000,
            };
        }
        let vtable = *(this as *const u32);
        let target = *((vtable as *const u32).byte_add(0x48));
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, arg)
    }
});
