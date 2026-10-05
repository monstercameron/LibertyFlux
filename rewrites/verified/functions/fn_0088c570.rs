// original: 0x0088C570 rage::audVoiceDSound::vf2

/// Tear down this DirectSound voice's two devices, then tail into the base
/// teardown.
///
/// `this` points to the voice. Each of the two device slots at `+0x90` and
/// `+0x94` is released when non-null (slot `+8` of the device's table,
/// called with the device), and control passes to the base teardown entry
/// with the voice as `this`; its answer is the answer of this function.
///
/// Original: 0x0088C570 (thiscall, no stack arguments, tail call).
lf_checker_rt::export!(thiscall, rw_0088C570(this: u32) -> u32 {
    unsafe {
        const DEVICE_A: u32 = 0x90;
        const DEVICE_B: u32 = 0x94;
        const SLOT_RELEASE: u32 = 0x08;
        const BASE_TEARDOWN: u32 = 2;

        let a = ((this + DEVICE_A) as *const u32).read_unaligned();
        if a != 0 {
            let table = (a as *const u32).read_unaligned();
            let release: extern "stdcall" fn(u32) -> u32 =
                core::mem::transmute(
                    ((table + SLOT_RELEASE) as *const u32).read_unaligned()
                        as usize
                );
            release(a);
        }
        let b = ((this + DEVICE_B) as *const u32).read_unaligned();
        if b != 0 {
            let table = (b as *const u32).read_unaligned();
            let release: extern "stdcall" fn(u32) -> u32 =
                core::mem::transmute(
                    ((table + SLOT_RELEASE) as *const u32).read_unaligned()
                        as usize
                );
            release(b);
        }
        lf_checker_rt::callee_thiscall!(BASE_TEARDOWN, u32, this)
    }
});
