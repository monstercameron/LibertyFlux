// original: 0x00bf1310 decode_forward_short
/// Decode a packed word and hand the buffer to callee 2.
///
/// Short form of `rw_bf1240` without the reference vector: decodes
/// `this + idx*4 + 0x1f0` (`idx` is the low byte of the last argument)
/// through callee 1 and forwards the same buffer to callee 2
/// (`this` = `target`). Returns callee 2's answer.
export!(thiscall, rw_bf1310(this_obj: u32, target: u32, idx_arg: u32) -> u32 {
    unsafe {
        let idx = (idx_arg & 0xff) as u32;
        let packed = *((this_obj + idx * 4 + 0x1f0) as *const u32);
        let mut buf = [0u32; 4];
        let base = buf.as_mut_ptr() as u32;
        callee_cdecl!(1, u32, base, packed);
        callee_thiscall!(2, u32, target, base)
    }
});
