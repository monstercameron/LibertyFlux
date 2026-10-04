// original: 0x006f4e50 load_be32_to_out
/// Load a big-endian `u32` from bytes 6..10 and store it through `out`.
///
/// Returns `out`.
rt::export!(thiscall, rw_006f4e50(this: *const u8, out: *mut u32) -> u32 {
    unsafe {
        *out = ((*this.add(6) as u32) << 24)
            | ((*this.add(7) as u32) << 16)
            | ((*this.add(8) as u32) << 8)
            | (*this.add(9) as u32);
        out as u32
    }
});
