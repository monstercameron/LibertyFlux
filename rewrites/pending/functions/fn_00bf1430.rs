// original: 0x00bf1430 decode_pair_forward_short
/// Decode a packed pair and hand the buffer to callee 2.
///
/// Short form of `rw_bf1350` without the reference vector: decodes the pair
/// at `this + idx*8 + 0x2f0/0x2f4` (`idx` is the low word of the last
/// argument) through callee 1 and forwards the same buffer to callee 2
/// (`this` = `target`). Returns callee 2's answer.
export!(thiscall, rw_bf1430(this_obj: u32, target: u32, idx_arg: u32) -> u32 {
    unsafe {
        let idx = (idx_arg & 0xffff) as u32;
        let vlo = *((this_obj + idx * 8 + 0x2f0) as *const u32);
        let vhi = *((this_obj + idx * 8 + 0x2f4) as *const u32);
        let mut buf = [0u32; 4];
        let base = buf.as_mut_ptr() as u32;
        callee_cdecl!(1, u32, base, vlo, vhi);
        callee_thiscall!(2, u32, target, base)
    }
});
