// original: 0x0062DAA0 streaming_multi_lookup (proposed)

/// Resolve four named handles, query extended state, publish the selection.
///
/// Looks up three handles by name through the lookup callee (thiscall on the
/// resolver at `+4` of the holder at `+0x1c`: name, tag 1), storing the
/// answers at `+0x34`, `+0x3c` and `+0x38`. Then queries an 8-byte state
/// block through the query callee (thiscall with a scratch out-block: name,
/// tag), repeating with a second name when the flag byte at `+0x30` is set.
/// When the low word of the block's second half is non-zero the block's first
/// word, else the default constant, is looked up the same way into `+0x40`;
/// a non-zero first word is finally freed through the thread-local
/// allocator's free slot (`+0x0c`). Returns the last callee answer
/// (thiscall, no arguments).
lf_checker_rt::export!(thiscall, rw_0062daa0(this: u32) -> u32 {
    unsafe {
        const HOLDER: u32 = 0x1c;
        const RESOLVER: u32 = 0x04;
        const NAME_A: u32 = 0xF96C4C;
        const NAME_B: u32 = 0xF96C9C;
        const NAME_C: u32 = 0xF968F4;
        const NAME_Q1: u32 = 0xF96904;
        const NAME_Q2: u32 = 0xF9690C;
        const DEFAULT_NAME: u32 = 0xFC9C85;
        unsafe fn rd(base: u32, off: u32) -> u32 {
            unsafe { ((base + off) as *const u32).read_unaligned() }
        }
        unsafe fn wr(base: u32, off: u32, v: u32) {
            unsafe { ((base + off) as *mut u32).write_unaligned(v) }
        }
        let holder = rd(this, HOLDER);
        let resolver = rd(holder, RESOLVER);
        let a: u32 = lf_checker_rt::callee_thiscall!(
            1, u32, resolver, lf_checker_rt::relocated(NAME_A), 1);
        wr(this, 0x34, a);
        let b: u32 = lf_checker_rt::callee_thiscall!(
            2, u32, resolver, lf_checker_rt::relocated(NAME_B), 1);
        wr(this, 0x3c, b);
        let c: u32 = lf_checker_rt::callee_thiscall!(
            3, u32, resolver, lf_checker_rt::relocated(NAME_C), 1);
        wr(this, 0x38, c);
        let mut q = [0u32; 2];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            4, u32, q.as_mut_ptr() as u32, lf_checker_rt::relocated(NAME_Q1), 4);
        if ((this + 0x30) as *const u8).read() != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                5, u32, q.as_mut_ptr() as u32, lf_checker_rt::relocated(NAME_Q2), 9);
        }
        let edi = q[0];
        let name = if q[1] & 0xFFFF != 0 {
            edi
        } else {
            lf_checker_rt::relocated(DEFAULT_NAME)
        };
        let d: u32 = lf_checker_rt::callee_thiscall!(
            6, u32, rd(holder, 0), name, 1);
        wr(this, 0x40, d);
        if edi == 0 {
            return d;
        }
        let holder0 = lf_checker_rt::tls_slot(0);
        let frobj = rd(holder0, 8);
        let fvt = rd(frobj, 0);
        let ftgt = rd(fvt, 0x0c);
        let free: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(ftgt as usize);
        free(frobj, edi)
    }
});
