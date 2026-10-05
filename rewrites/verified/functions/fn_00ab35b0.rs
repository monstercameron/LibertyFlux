// original: 0x00ab35b0 stream_free_bucket (proposed)

/// Release every entry of a bucket list, then free the bucket array.
///
/// Walks the `next` chain at `+0xB8`, releasing each node through the
/// release callee on the loader singleton, then, when the word at `+8` is
/// nonzero, frees the array at `+8` through the free callee and clears the
/// word. Returns the free callee's answer, or 0 when there was nothing to
/// free.
///
/// Callees: 1 = entry release (thiscall, two words),
/// 2 = array free (stdcall, one word).
///
/// Original: 0x00ab35b0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ab35b0(this: u32) -> u32 {
    unsafe {
        const RELEASE: u32 = 1;
        const FREE: u32 = 2;
        const SINGLETON: u32 = 0x013B_ABA0;
        const CHAIN_OFF: u32 = 0xB8;
        const ARRAY_OFF: u32 = 8;
        let loader = lf_checker_rt::relocated(SINGLETON);
        let mut node = ((this + CHAIN_OFF) as *const u32).read_unaligned();
        while node != 0 {
            let next = (node as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(RELEASE, u32, loader, this, node);
            node = next;
        }
        if ((this + ARRAY_OFF) as *const u32).read_unaligned() != 0 {
            let ans =
                lf_checker_rt::callee_stdcall!(FREE, u32, this.wrapping_add(ARRAY_OFF));
            ((this + ARRAY_OFF) as *mut u32).write_unaligned(0);
            ans
        } else {
            0
        }
    }
});
