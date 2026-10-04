// original: 0x00dd9770 UIMontageClip::vf77
/// `UIMontageClip::vf77`: stamp one byte into the top byte of several words.
///
/// Takes a byte, keeps the low 24 bits of each target word, and replaces the
/// top byte: first the object's own words at `+0x1f0`/`+0x1f4`, then word
/// `+0x1e0` of each of six member objects that is non-null. Returns the last
/// word written, matching the value the original leaves in EAX.
export!(thiscall, rw_00dd9770(this_ptr: u32, arg: u32) -> u32 {
    unsafe {
        const KEEP: u32 = 0x00ff_ffff;
        const OWN: [usize; 2] = [0x1f0, 0x1f4];
        const MEMBERS: [usize; 6] = [0x1e0, 0x1e4, 0x300, 0x304, 0x308, 0x30c];
        let top = (arg & 0xff) << 24;
        let mut last = 0u32;
        for off in OWN {
            let p = (this_ptr as *mut u32).add(off / 4);
            last = p.read() & KEEP | top;
            p.write(last);
        }
        for moff in MEMBERS {
            let m = ((this_ptr as *const u32).add(moff / 4)).read();
            if m != 0 {
                let p = (m as *mut u32).add(0x1e0 / 4);
                last = p.read() & KEEP | top;
                p.write(last);
            }
        }
        last
    }
});
