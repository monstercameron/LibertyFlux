// original: 0x00bf2d90 decode_forward_short_t2
/// Decode a packed word and hand the buffer to callee 2 (second table).
///
/// Same shape as `rw_bf1310` but the packed entry lives in the table at
/// `this + idx*4 + 0x370`.
export!(thiscall, rw_bf2d90(this_obj: u32, target: u32, idx_arg: u32) -> u32 {
    unsafe {
        let idx = (idx_arg & 0xff) as u32;
        let packed = *((this_obj + idx * 4 + 0x370) as *const u32);
        let mut buf = [0u32; 4];
        let base = buf.as_mut_ptr() as u32;
        callee_cdecl!(1, u32, base, packed);
        callee_thiscall!(2, u32, target, base)
    }
});
