// original: 0x00dd9860 UIBasicClip::vf131
/// `UIBasicClip::vf131`: conditional forward with a rewritten argument.
///
/// When the low byte of the argument is 1, runs the `field_1e8` member's slot
/// `0x124` predicate; the forwarded argument becomes 1 unless the predicate
/// passed while `flag_2F8` holds any value other than 1 or 2. Otherwise the
/// forwarded argument is 0. Tail-calls the `field_1e4` member's slot `0x120`
/// with that argument and returns its answer.
export!(thiscall, rw_00dd9860(this_ptr: u32, arg0: u32) -> u32 {
    unsafe {
        let thisp = this_ptr as *const u32;
        let mut fwd = 0u32;
        if arg0 & 0xff == 1 {
            let m = thisp.add(0x1e8 / 4).read();
            let t = (((m as *const u32).read() + 0x124) as *const u32).read();
            let pred: extern "thiscall" fn(u32) -> u8 =
                core::mem::transmute(t as usize);
            let flag = ((this_ptr as *const u8).add(0x2f8)).read();
            if pred(m) == 0 || flag == 1 || flag == 2 {
                fwd = 1;
            }
        }
        let m = thisp.add(0x1e4 / 4).read();
        let t = (((m as *const u32).read() + 0x120) as *const u32).read();
        let target: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(t as usize);
        target(m, fwd)
    }
});
