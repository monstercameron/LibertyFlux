// original: 0x0065AF00 shader_bind_pass_arrays (proposed)

/// Bind every pass in two arrays, then run the slot-4 hook and stamp ready.
///
/// Binds each entry of the array at `+4` (count is the unsigned 16-bit word
/// at `+8`, compared unsigned for the empty check and signed in the loop,
/// identical for the small positive counts used) through the first callee
/// (thiscall: `this`, entry) and each entry of the array at `+0x0c` (count
/// at `+0x10`) through the second callee. Then calls the hook at slot `+4`
/// of `this`'s table (thiscall, no arguments) whose answer is returned, and
/// stamps the byte `1` at `+0x24` (thiscall, no arguments).
lf_checker_rt::export!(thiscall, rw_0065af00(this: u32) -> u32 {
    unsafe {
        const CALLEE_FIRST: u32 = 1;
        const CALLEE_SECOND: u32 = 2;
        let count1 = ((this + 8) as *const u16).read_unaligned() as u32;
        if count1 != 0 {
            let arr = ((this + 4) as *const u32).read_unaligned();
            let mut i = 0u32;
            while i < count1 {
                let e = ((arr + i * 4) as *const u32).read_unaligned();
                let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_FIRST, u32, this, e);
                i += 1;
            }
        }
        let count2 = ((this + 0x10) as *const u16).read_unaligned() as u32;
        if count2 != 0 {
            let arr = ((this + 0x0c) as *const u32).read_unaligned();
            let mut i = 0u32;
            while i < count2 {
                let e = ((arr + i * 4) as *const u32).read_unaligned();
                let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_SECOND, u32, this, e);
                i += 1;
            }
        }
        let vt = (this as *const u32).read_unaligned();
        let tgt = ((vt + 4) as *const u32).read_unaligned();
        let hook: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(tgt as usize);
        let ans = hook(this);
        ((this + 0x24) as *mut u8).write_unaligned(1);
        ans
    }
});
