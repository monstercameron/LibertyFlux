// original: 0x00904220 input_fire_float_callback (proposed)
/// Fire the stored two-float callback with the object's pair.
///
/// `this` points to the callback holder: word `+8` is the callback address,
/// words `+0xc` and `+0x10` the two float arguments (copied as raw words).
/// The callback is invoked through the pointer (a cdecl of two words) and
/// its answer returned. Thiscall with no stack arguments.
export!(thiscall, rw_00904220(this: u32) -> u32 {
    unsafe {
        const CB_OFF: u32 = 0x08;
        const F1_OFF: u32 = 0x0C;
        const F2_OFF: u32 = 0x10;
        let f1 = ((this.wrapping_add(F1_OFF)) as *const u32).read_unaligned();
        let f2 = ((this.wrapping_add(F2_OFF)) as *const u32).read_unaligned();
        let addr = ((this.wrapping_add(CB_OFF)) as *const u32).read_unaligned();
        let f: extern "cdecl" fn(u32, u32) -> u32 = core::mem::transmute(addr as usize);
        f(f1, f2)
    }
});
