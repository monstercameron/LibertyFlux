// original: 0x00bed070 copy_vec3_2c
/// Copy three dwords from offsets 0x2c-0x34 of `this` to an output buffer.
///
/// Reads `[this+0x2c]`, `[this+0x30]`, `[this+0x34]` and stores them at
/// `[dst]`, `[dst+4]`, `[dst+8]`. Returns the last word copied (the
/// original's EAX leftover). Thiscall, one stack argument (the destination).
export!(thiscall, rw_00bed070(this: u32, dst: u32) -> u32 {
    unsafe {
        let mut last = 0u32;
        for i in 0..3u32 {
            let v = ((this + 0x2c + i * 4) as *const u32).read_unaligned();
            ((dst + i * 4) as *mut u32).write_unaligned(v);
            last = v;
        }
        last
    }
});
