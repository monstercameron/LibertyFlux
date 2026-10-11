// original: 0x00990930 audVehicleAudioEntity::stop_heli_damage_loop

/// Forwards the entity and eight stored sound parameters to the intercepted
/// thiscall helper, followed by four zero words. The helper's EAX result is
/// the wrapper's result; all field arguments are passed in the original
/// stack order.
lf_checker_rt::export!(thiscall, rw_audvehicleaudioentity_stop_heli_damage_loop(this: u32) -> u32 {
    const PARAMETER_OFFSETS: [u32; 8] = [0xA88, 0xA8C, 0xA90, 0xA94, 0xA98, 0xA9C, 0xAA0, 0xAA8];
    const ZERO: u32 = 0;

    unsafe {
        let p0 = (this.wrapping_add(PARAMETER_OFFSETS[0]) as *const u32).read_unaligned();
        let p1 = (this.wrapping_add(PARAMETER_OFFSETS[1]) as *const u32).read_unaligned();
        let p2 = (this.wrapping_add(PARAMETER_OFFSETS[2]) as *const u32).read_unaligned();
        let p3 = (this.wrapping_add(PARAMETER_OFFSETS[3]) as *const u32).read_unaligned();
        let p4 = (this.wrapping_add(PARAMETER_OFFSETS[4]) as *const u32).read_unaligned();
        let p5 = (this.wrapping_add(PARAMETER_OFFSETS[5]) as *const u32).read_unaligned();
        let p6 = (this.wrapping_add(PARAMETER_OFFSETS[6]) as *const u32).read_unaligned();
        let p7 = (this.wrapping_add(PARAMETER_OFFSETS[7]) as *const u32).read_unaligned();
        let helper: unsafe extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(lf_checker_rt::callee_addr(1));
        helper(this, p0, p1, p2, p3, p4, p5, p6, p7, ZERO, ZERO, ZERO, ZERO)
    }
});
