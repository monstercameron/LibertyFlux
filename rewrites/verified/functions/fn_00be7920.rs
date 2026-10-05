// original: 0x00be7920 CTaskSimpleTriggerLookAt::vf17

/// Fire a scripted look-at trigger unless it already ran inert.
///
/// `this` points to the task, `ped` to the ped. When the ran flag at `+0x4c`
/// is set but the handle at `+0x30` is null, there is nothing to fire and it
/// returns 1 at once. Otherwise it forwards the trigger parameters (the
/// sign-extended byte at `+0x3c`, the dwords at `+0x40` twice, `+0x48`, a
/// pointer to the embedded vector at `+0x20`, and the dwords at `+0x38`,
/// `+0x34`, `+0x30`, plus two trailing words holding 0 and the constant
/// `0xeb8b14`) to the engine through callee 1 (thiscall on `ped+0xbb0`, ten
/// stack words) and returns 1.
///
/// Original: thiscall, one stack word, the callee pops 4 bytes, returns `al`.
lf_checker_rt::export!(thiscall, rw_00be7920(this: u32, ped: u32) -> u32 {
    unsafe {
        const OFF_VEC: u32 = 0x20;
        const OFF_HANDLE: u32 = 0x30;
        const OFF_A: u32 = 0x34;
        const OFF_B: u32 = 0x38;
        const OFF_C: u32 = 0x3c;
        const OFF_D: u32 = 0x40;
        const OFF_E: u32 = 0x48;
        const OFF_RAN: u32 = 0x4c;
        const PED_LOOKAT: u32 = 0xbb0;
        // File address of the trailing constant; it points into the image and
        // carries a relocation, so it must be relocated like any pointer.
        const TRAILING_CONST_FILE: u32 = 0xeb8b14;
        const FIRE: u32 = 1;

        let handle = ((this + OFF_HANDLE) as *const u32).read_unaligned();
        if ((this + OFF_RAN) as *const u8).read() != 0 && handle == 0 {
            return 1;
        }
        let c = ((this + OFF_C) as *const i8).read() as i32 as u32;
        let d = ((this + OFF_D) as *const u32).read_unaligned();
        let e = ((this + OFF_E) as *const u32).read_unaligned();
        let b = ((this + OFF_B) as *const u32).read_unaligned();
        let a = ((this + OFF_A) as *const u32).read_unaligned();
        // The stub logs arg0 as the last-pushed word, so the parameters go
        // in reverse push order: const, 0, handle, a, b, &vec, e, d, d, c.
        lf_checker_rt::callee_thiscall!(
            FIRE, u32, ped.wrapping_add(PED_LOOKAT),
            lf_checker_rt::relocated(TRAILING_CONST_FILE), 0, handle, a, b,
            this.wrapping_add(OFF_VEC), e, d, d, c
        );
        1
    }
});
