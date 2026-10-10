// original: 0x00897AA0 input_slots_visit_vf8

/// Visits the 24 object pointers held in the array at the start of `this`. Nothing happens unless the
/// global flag byte at 0x115F800 is nonzero. For each non-null entry, the method in virtual slot 2 of
/// the object is called with the object as `this` and no arguments. Thiscall with one stack argument
/// which is not read, callee cleans up.
lf_checker_rt::export!(thiscall, rw_00897aa0(this: u32, _unused: u32) -> () {
    unsafe {
        const FLAG: u32 = 0x0115_F800;
        const SLOT_COUNT: u32 = 0x18;
        const VTABLE_SLOT: u32 = 8;
        if lf_checker_rt::global::<u8>(FLAG).read() == 0 {
            return;
        }
        for i in 0..SLOT_COUNT {
            let object = (this.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            if object != 0 {
                let vtable = (object as *const u32).read_unaligned();
                let method_bits = (vtable.wrapping_add(VTABLE_SLOT) as *const u32).read_unaligned();
                let method: extern "thiscall" fn(u32) = core::mem::transmute(method_bits);
                method(object);
            }
        }
    }
});
