// original: 0x00dda430 UIBasicClip::vf137
/// `UIBasicClip::vf137`: set a float-driven triple through slot `0x228`.
///
/// Same shape as `UIBasicClip::vf132` with a different table address and
/// middle slot: splits the float argument into three converted bytes, hands
/// them to an engine helper, runs the object's own slot `0x228`, stores the
/// untouched input bits into `field_2FC_f`, and finishes through slot `0x204`
/// and the frame-cookie check, whose answer is left in EAX.
export!(thiscall, rw_00dda430(this_ptr: u32, arg: u32) -> u32 {
    unsafe {
        // Raw cvttss2si semantics (Rust's `as` saturates instead).
        let cv = |v: f32| -> i32 {
            if v.is_nan() || v >= 2147483648.0f32 || v < -2147483648.0f32 {
                i32::MIN
            } else {
                v as i32
            }
        };
        let x = f32::from_bits(arg);
        let c0 = global::<f32>(0xfe8724).read();
        let c1 = global::<f32>(0xfe8b80).read();
        let c2 = global::<f32>(0xfe8bb0).read();
        let b0 = (cv(x * c0) & 0xff) as u32;
        let x1 = x - (b0 as f32) * c1;
        let b1 = (cv(x1) & 0xff) as u32;
        let x2 = (x1 - (b1 as f32)) * c2;
        let b2 = (cv(x2) & 0xff) as u32;
        let scratch = 0u32;
        let fp = &scratch as *const u32 as u32;
        callee_cdecl!(0, u32, fp, relocated(0xefb9b4), b0, b1, b2);
        let slot = ((((this_ptr as *const u32).read() + 0x228)) as *const u32).read();
        let mid: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        mid(this_ptr, fp, 0);
        ((this_ptr as *mut u32).add(0x2fc / 4)).write(arg);
        let slot = ((((this_ptr as *const u32).read() + 0x204)) as *const u32).read();
        let fin: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        fin(this_ptr, 1);
        callee_cdecl!(3, u32,)
    }
});
