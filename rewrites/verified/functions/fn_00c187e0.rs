// original: 0x00c187e0 camera_refresh_via_global_channel

/// Refresh the camera target through the shared global channel.
///
/// Builds two scratch vectors from the floats at `+0x40/+0x44/+0x48` (the
/// third lane minus 50.0) and runs them through the transform helper
/// (stdcall, two scratch pointers; its six output words are scripted), then
/// passes the first vector to virtual slot `+8` of the global channel object
/// (with two zero words), clears the channel byte at `+0xbc`, and notifies
/// virtual slot `+8` of the channel's sub-object at `+0x80` (with words 0
/// and -2). Records the channel at `+0x2a4` and sets the flag at `+0x140`
/// exactly when the channel words at `+0xb0` (nonzero) and `+0xb8` (SIGNED
/// greater than zero) both say active. Returns the notify call's answer.
/// Both helpers take pointers to the caller's own scratch area; the contract
/// skips those arguments and snapshots the contents, and observes six words
/// of the transform's output (any output past six words is unobservable).
///
/// Original: 0x00C187E0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00c187e0(this: u32) -> u32 {
    unsafe {
        const TRANSFORM: u32 = 1;
        const CHANNEL: u32 = 0x011f_694c;
        let g = lf_checker_rt::global::<u32>(CHANNEL).read();
        let x = ((this + 0x40) as *const f32).read_unaligned();
        let y = ((this + 0x44) as *const f32).read_unaligned();
        let z = ((this + 0x48) as *const f32).read_unaligned();
        let k50 = lf_checker_rt::global::<f32>(0x0104_7fe8).read();
        let mut scratch = [0u32; 8];
        scratch[0] = x.to_bits();
        scratch[1] = y.to_bits();
        scratch[2] = (z - k50).to_bits();
        scratch[3] = 0;
        scratch[4] = x.to_bits();
        scratch[5] = y.to_bits();
        scratch[6] = z.to_bits();
        scratch[7] = 0;
        let buf = scratch.as_mut_ptr() as u32;
        lf_checker_rt::callee_stdcall!(TRANSFORM, u32, buf + 16, buf);
        let vtable = (g as *const u32).read_unaligned();
        let slot = ((vtable + 8) as *const u32).read_unaligned() as usize;
        let send: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(slot);
        send(g, buf, 0, 0);
        let g2 = lf_checker_rt::global::<u32>(CHANNEL).read();
        let sub = g2 + 0x80;
        let vtable2 = (sub as *const u32).read_unaligned();
        let slot2 = ((vtable2 + 8) as *const u32).read_unaligned() as usize;
        let notify: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(slot2);
        ((g2 + 0xbc) as *mut u8).write(0);
        let ans: u32 = notify(sub, 0, 0xffff_fffe);
        let active = ((g2 + 0xb0) as *const u32).read_unaligned() != 0
            && ((g2 + 0xb8) as *const i32).read_unaligned() > 0;
        ((this + 0x2a4) as *mut u32).write_unaligned(g);
        ((this + 0x140) as *mut u8).write(u8::from(active));
        ans
    }
});
