// original: 0x00891240 aud_sound_attach
/// Attaches parameter blocks to a sound object and finishes its setup.
///
/// Copies the identifying word and dword from block A into this object, runs
/// the pair callee and records its answer, then invokes virtual slot 4 once
/// per non-null handle in block C, storing each answer in the matching field.
/// Finally runs the finish callee; unless the flag argument's low byte is set,
/// derives the two trailing fields from block C. Returns the finish answer on
/// the flag path, else the last value written.
export!(thiscall, rw_00891240(this: *mut u8, a: *mut u8, flag: u32, c: *mut u8) -> u32 {
    unsafe {
        *(this.add(8) as *mut u16) = *(a.add(9) as *const u16);
        *(this.add(0x70) as *mut u32) = *(a.add(5) as *const u32);
        let v: u32 = callee_cdecl!(1, u32, a as u32, c as u32);
        *(this.add(0x94) as *mut u32) = v;
        let pairs = [
            (0x39usize, 0x0cusize),
            (0x3d, 0x10),
            (0x41, 0x14),
            (0x4d, 0x18),
            (0x45, 0x5c),
            (0x49, 0x64),
        ];
        for (src, dst) in pairs {
            let h = *(c.add(src) as *const u32);
            if h != 0 {
                let vtable = *(this as *const u32);
                let slot = *((vtable.wrapping_add(0x10)) as *const u32);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                *(this.add(dst) as *mut u32) = f(this as u32, h);
            }
        }
        let done: u32 = callee_thiscall!(3, u32, this as u32, c as u32, flag);
        if flag & 0xff != 0 {
            return done;
        }
        let w = *(c.add(0x29) as *const u16);
        let s = if (w as i16) < 0 { 0xffffffff } else { w as u32 };
        *(this.add(0x88) as *mut u32) = s;
        if *(this.add(0x44) as *const u32) == 0 {
            let w2 = *(c.add(0x27) as *const u16) as u32;
            *(this.add(0x44) as *mut u32) = w2;
            return w2;
        }
        s
    }
});
