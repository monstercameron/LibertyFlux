// original: 0x0088D570 rage::audVoiceDSound::vf4

/// Resume this DirectSound voice on its device, unless it is stopped or idle.
///
/// `this` points to the voice. When bit 3 of the flag byte at `+0x8c` is
/// clear, or bit 0 is set, there is nothing to do. Otherwise the device at
/// `+0x90` is told to resume (slot `+0x30` of its table, called with the
/// device, two zero words and a loop flag that is 1 when bits 1 or 4 of the
/// voice flags are set), the voice's restart position at `+0x98` is handed
/// to the device (slot `+0x24`, called with the device and the position),
/// and bit 3 of the voice flags is cleared.
///
/// Original: 0x0088D570 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0088D570(this: u32) -> ()) {
    unsafe {
        const FLAGS: u32 = 0x8c;
        const NEEDS_RESUME: u8 = 0x08;
        const STOPPED: u8 = 0x01;
        const LOOP_BITS: u8 = 0x12;
        const VOICE_DEVICE: u32 = 0x90;
        const RESTART_POS: u32 = 0x98;
        const SLOT_RESUME: u32 = 0x30;
        const SLOT_SEEK: u32 = 0x24;

        let flags = ((this + FLAGS) as *const u8).read();
        if flags & NEEDS_RESUME == 0 || flags & STOPPED != 0 {
            return;
        }
        let looping = ((flags & LOOP_BITS) != 0) as u32;
        let obj = ((this + VOICE_DEVICE) as *const u32).read_unaligned();
        let table = (obj as *const u32).read_unaligned();
        let resume: extern "stdcall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(
                ((table + SLOT_RESUME) as *const u32).read_unaligned() as usize
            );
        resume(obj, 0, 0, looping);
        let table = (obj as *const u32).read_unaligned();
        let seek: extern "stdcall" fn(u32, u32) -> u32 = core::mem::transmute(
            ((table + SLOT_SEEK) as *const u32).read_unaligned() as usize
        );
        seek(obj, this + RESTART_POS);
        let flags = (this + FLAGS) as *mut u8;
        flags.write(flags.read() & !NEEDS_RESUME);
    }
});
