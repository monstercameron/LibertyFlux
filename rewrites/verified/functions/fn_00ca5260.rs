// original: 0x00ca5260 CEventHandler::vf63
/// Float event slot: picks a vector from the event (+0x10 or +0x20 by the
/// flag at +0x30), evaluates a scalar through a float-returning helper,
/// then converts through the factory with the scalar and vector. The
/// scalar travels as an x87 register return; the rewrite holds it in a
/// local. (The original spills it and the +0x34 float through dead stack
/// slots; the values are verified downstream through the call arguments.)
lf_rs75_rt::export!(thiscall, rw_00ca5260(this: u32, ev: u32, _b: u32, _c: u32) -> u32 {
    unsafe {
        let flag = *((ev + 0x30) as *const u32);
        let vec = if flag == 1 {
            ev.wrapping_add(0x10)
        } else {
            ev.wrapping_add(0x20)
        };
        let f34 = *((ev + 0x34) as *const u32);
        let w38 = *((ev + 0x38) as *const u32);
        let scalar: f32 = lf_rs75_rt::callee_cdecl!(1, f32, w38);
        let mgr = *lf_rs75_rt::global::<u32>(0x0167E2A0);
        let h: u32 = lf_rs75_rt::callee_thiscall!(2, u32, mgr);
        if h == 0 {
            *((this + 0xC) as *mut u32) = 0;
            return 0;
        }
        let ans: u32 =
            lf_rs75_rt::callee_thiscall!(3, u32, h, scalar.to_bits(), vec, f34, 0);
        *((this + 0xC) as *mut u32) = ans;
        ans
    }
});
