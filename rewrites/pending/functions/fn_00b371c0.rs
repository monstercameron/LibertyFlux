// original: 0x00b371c0 eligibility_gate_chain
/// Eligibility test over an object: a flag byte, a virtual check, a mode
/// nibble with a subtype gate, a container gate, then a lookup whose hit
/// needs a final confirmation. Returns 1 when every gate passes.
lf_checker_rt::export!(cdecl, rw_b371c0(obj: u32) -> u32 {
    unsafe {
        let o = obj as *const u8;
        if o.add(0x219).read() != 0 {
            return 0;
        }
        let vtbl = (obj as *const u32).read();
        let check: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((((vtbl.wrapping_add(0x34)) as *const u32).read()) as usize);
        if check(obj) & 0xff == 0 {
            return 0;
        }
        if o.add(0x26c).read() & 4 != 0 {
            return 0;
        }
        if o.add(0x1e2).read() & 0x0f >= 2 {
            let p = (obj as *const u32).add(0x1bc / 4).read();
            if p != 0 {
                let v = ((p.wrapping_add(0x28)) as *const u32).read() & 0x3c0;
                if v == 0x80 {
                    return 0;
                }
            }
        }
        let q = (obj as *const u32).add(0x6c / 4).read();
        if q != 0 && (q as *const u8).add(0x0e).read() != 0 {
            return 0;
        }
        let r = lf_checker_rt::callee_thiscall!(3, u32, obj);
        if r == 0 {
            return 1;
        }
        if lf_checker_rt::callee_thiscall!(4, u32, r.wrapping_add(8), obj) & 0xff == 0 {
            0
        } else {
            1
        }
    }
});
