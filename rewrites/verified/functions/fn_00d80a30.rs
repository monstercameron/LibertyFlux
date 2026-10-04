// original: 0x00d80a30 watch_event_gate

/// Gate and fire a proximity "watch" event between two tracked objects.
///
/// Both objects carry a matrix at +0x20, a state block at +0xf50 and a status
/// word at +0x2c. The event fires only when every gate passes: the status
/// byte is below 100, both state blocks exist with the watch flag set and the
/// mode word not equal to 2, the forward-axis dot product is below -0.93, the
/// lateral offset form below 1.0, and both objects' reported velocity vectors
/// (vtable slot 0xec) have squared length above 81. On the armed path
/// (+0x1304 == 1) it forwards a constant descriptor plus the slot base to the
/// event helper and returns its answer; otherwise it runs the disarm hook
/// (slot 0x1a8) and, for status below 30, re-arms the retry stamp from the
/// frame timer plus 1000. Only the low return byte is meaningful on the
/// early-exit paths (the rest is entry-register residue).
export!(cdecl, rw_00d80a30(this: u32, other: u32) -> u32 {
    unsafe {
        let status = ((this + 0x2c) as *const u16).read() as u32 & 0xff;
        if status >= 0x64 {
            return status;
        }
        let other_state = ((other + 0xf50) as *const u32).read();
        if other_state == 0 {
            return 0;
        }
        if ((other_state + 0x219) as *const u8).read() == 0 {
            return other_state;
        }
        let this_state = ((this + 0xf50) as *const u32).read();
        if this_state == 0 {
            return 0;
        }
        let mode_block = ((this_state + 0x21c) as *const u32).read();
        if ((mode_block + 0x12c) as *const u32).read() == 2 {
            return mode_block;
        }
        let a = ((other + 0x20) as *const u32).read();
        let b = ((this + 0x20) as *const u32).read();
        let rf = |p: u32| (p as *const f32).read();
        // Forward-axis alignment, in the original's accumulation order.
        let dot = rf(a + 0x14) * rf(b + 0x14) + rf(a + 0x10) * rf(b + 0x10)
            + rf(a + 0x18) * rf(b + 0x18);
        let align_limit = lf_checker_rt::global::<f32>(0x00ee_c92c).read();
        if !(align_limit > dot) {
            return a;
        }
        // Lateral offset form, in the original's accumulation order.
        let lateral = rf(a + 4) * (rf(b + 0x34) - rf(a + 0x34))
            + rf(a) * (rf(b + 0x30) - rf(a + 0x30))
            + rf(a + 8) * (rf(b + 0x38) - rf(a + 0x38));
        let lateral_limit = lf_checker_rt::global::<f32>(0x00fe_88e8).read();
        if !(lateral_limit > lateral) {
            return a;
        }
        // Velocity of the other object through its own vtable slot.
        let avt = (other as *const u32).read();
        let vtable_fn: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(((avt + 0xec) as *const u32).read() as usize);
        let mut slot = [0u32; 4];
        let v0 = vtable_fn(other, slot.as_mut_ptr() as u32);
        let speed0 = {
            let x = ((v0 + 4) as *const f32).read();
            let y = (v0 as *const f32).read();
            let z = ((v0 + 8) as *const f32).read();
            y * y + x * x + z * z
        };
        let speed_limit = lf_checker_rt::global::<f32>(0x00fe_8ba0).read();
        if !(speed0 > speed_limit) {
            return v0;
        }
        // Own velocity through our own vtable slot.
        let bvt = (this as *const u32).read();
        let vtable_fn2: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(((bvt + 0xec) as *const u32).read() as usize);
        let v1 = vtable_fn2(this, slot.as_mut_ptr() as u32);
        let speed1 = {
            let x = ((v1 + 4) as *const f32).read();
            let y = (v1 as *const f32).read();
            let z = ((v1 + 8) as *const f32).read();
            y * y + x * x + z * z
        };
        if !(speed1 > speed_limit) {
            return v1;
        }
        if ((this + 0x1304) as *const u32).read() == 1 {
            let descriptor = lf_checker_rt::relocated(0x00ee_c748);
            return lf_checker_rt::callee_thiscall!(
                4, u32, this_state.wrapping_add(0x570), descriptor,
                0, 0, 0, 0xffff_ffff, 0, 0, 0x3f80_0000, 0, 0
            );
        }
        let disarm: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((bvt + 0x1a8) as *const u32).read() as usize);
        let hook_answer = disarm(this);
        let restatus = ((this + 0x2c) as *const u16).read() as u32 & 0xff;
        if restatus >= 0x1e {
            return (hook_answer & 0xffff_0000) | restatus;
        }
        let now = lf_checker_rt::global::<u32>(0x0117_35b4).read();
        let stamp = now.wrapping_add(0x3e8);
        ((this + 0xf2c) as *mut u32).write(stamp);
        stamp
    }
});
