// original: 0x00c24f30 cam_mode_apply (proposed)
/// Apply mode `index` (1-5) to `obj`, anything else is a no-op. Mode 1
/// delegates to the mode worker with a zero flag and returns its result;
/// modes 2-5 clear or set bits of the flag byte at +0x13c (clear 0x04,
/// clear 0x08, set 0x0c, clear 0x0c) and return `obj`. The default returns
/// `index - 1` (the decremented value left in eax).
///
/// Original: 0x00c24f30 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_00c24f30(obj: u32, index: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x13c;
        match index.wrapping_sub(1) {
            0 => lf_checker_rt::callee_thiscall!(1, u32, obj, 0),
            1 => {
                let p = obj.wrapping_add(FLAG_OFF) as *mut u8;
                p.write(p.read() & 0xfb);
                obj
            }
            2 => {
                let p = obj.wrapping_add(FLAG_OFF) as *mut u8;
                p.write(p.read() & 0xf7);
                obj
            }
            3 => {
                let p = obj.wrapping_add(FLAG_OFF) as *mut u8;
                p.write(p.read() | 0x0c);
                obj
            }
            4 => {
                let p = obj.wrapping_add(FLAG_OFF) as *mut u8;
                p.write(p.read() & 0xf3);
                obj
            }
            _ => index.wrapping_sub(1),
        }
    }
});
