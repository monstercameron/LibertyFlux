// original: 0x00bf1350 decode_pair_align_forward
/// Decode a packed pair, align its sign, forward both.
///
/// Same shape as `rw_bf1240` but the packed entry is the word pair at
/// `this + idx*8 + 0x2f0/0x2f4` (`idx` is the low word of the last argument),
/// decoded through the three-argument callee 1.
export!(thiscall, rw_bf1350(this_obj: u32, target: u32, avec: u32, t_bits: u32, idx_arg: u32) -> u32 {
    unsafe {
        let idx = (idx_arg & 0xffff) as u32;
        let vlo = *((this_obj + idx * 8 + 0x2f0) as *const u32);
        let vhi = *((this_obj + idx * 8 + 0x2f4) as *const u32);
        let mut buf = [0u32; 8];
        let base = buf.as_mut_ptr() as u32;
        callee_cdecl!(1, u32, base, vlo, vhi);
        let a0 = f32::from_bits(*(avec as *const u32));
        let a1 = f32::from_bits(*((avec + 4) as *const u32));
        let a2 = f32::from_bits(*((avec + 8) as *const u32));
        let a3 = f32::from_bits(*((avec + 12) as *const u32));
        let q0 = f32::from_bits(buf[0]);
        let q1 = f32::from_bits(buf[1]);
        let q2 = f32::from_bits(buf[2]);
        let q3 = f32::from_bits(buf[3]);
        let mut d = a1 * q1 + a0 * q0;
        d = d + a2 * q2;
        d = d + a3 * q3;
        if d < 0.0 {
            let mask = *global::<u32>(0x00fe8fa0);
            buf[0] ^= mask;
            buf[1] ^= mask;
            buf[2] ^= mask;
            buf[3] ^= mask;
        }
        callee_thiscall!(2, u32, base + 16, t_bits, base, avec);
        callee_thiscall!(3, u32, target, base + 16)
    }
});
