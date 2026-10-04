// original: 0x006f5260 guarded_dispatch
/// Guarded dispatch under a critical section.
///
/// Enters the section at `this+0x14` when its slot is set, runs a check
/// call with the four arguments when the object at `this+4` exists and
/// flag bit 0 at `this+0x5C` is set, then leaves the section. Only the
/// low byte of the check answer matters. Returns 1 when the check
/// passed, else 0.
rt::export!(thiscall, rw_006f5260(this: *mut u8, a1: u32, a2: u32, a3: u32, a4: u32) -> u8 {
    unsafe {
        let section = this.add(0x14) as u32;
        if *(section as *const u32) != 0 {
            rt::callee_stdcall!(1, u32, section);
        }
        let obj = *this.add(4).cast::<u32>();
        let mut ok = 0u8;
        if obj != 0 && (*this.add(0x5C) & 1) != 0 {
            let ans: u32 = rt::callee_thiscall!(2, u32, obj, a1, a2, a3, a4);
            if (ans & 0xFF) != 0 {
                ok = 1;
            }
        }
        if *(section as *const u32) != 0 {
            rt::callee_stdcall!(3, u32, section);
        }
        ok
    }
});
