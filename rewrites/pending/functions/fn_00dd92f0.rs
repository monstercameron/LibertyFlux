// original: 0x00dd92f0 UIMontageClip::vf81
/// `UIMontageClip::vf81`: all-checks-pass predicate over three members.
///
/// Calls the object's own table slot `0x140`, then requires the flag byte at
/// `+0xcb` to be clear and the `field_1e0`/`field_1e8` members' slot `0x144`
/// checks to pass. Returns 1 only if every check passes, 0 at the first
/// failure (later checks then never run).
export!(thiscall, rw_00dd92f0(this_ptr: u32) -> u8 {
    unsafe {
        let thisp = this_ptr as *const u32;
        let target = (thisp.read() + 0x140) as *const u32;
        let check0: extern "thiscall" fn(u32) -> u8 =
            core::mem::transmute(target.read() as usize);
        if check0(this_ptr) == 0 {
            return 0;
        }
        if ((this_ptr as *const u8).add(0xcb)).read() != 0 {
            return 0;
        }
        for moff in [0x1e0, 0x1e8] {
            let m = thisp.add(moff / 4).read();
            let t = (((m as *const u32).read() + 0x144) as *const u32).read();
            let check: extern "thiscall" fn(u32) -> u8 =
                core::mem::transmute(t as usize);
            if check(m) == 0 {
                return 0;
            }
        }
        1
    }
});
