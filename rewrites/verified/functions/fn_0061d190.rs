// original: 0x0061D190 net_deserialize_01

/// Deserialize message kind 1 (caller-sized frame), reporting bits.
///
/// Builds an eight-word read frame and runs it through the bit reader and
/// this message's deserializer. `this` is the endpoint object, `buf` a
/// caller buffer recorded in the frame, `count` is the element count (stored scaled by 8), `out` an optional
/// out-pointer for the consumed bit count as bytes (`(BITS+7)>>3`,
/// arithmetic shift), or null to skip it. Returns 1 when both helpers
/// accept, 0 otherwise, with the failing helper's accumulator in the
/// upper 24 bits of the return value.
///
/// Frame layout (words): `BUF`, 0, `SIZE`, 0, `BITS`, 0, flag word,
/// spare. The flag word comes from uninitialized stack in the original
/// and is never read back on any path, so the rewrite stores 0 there.
/// `SIZE` is `count*8`; `BITS` is the deserializer's out-word.
/// Original: 0x0061D190 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_0061D190(this: u32, buf: u32, count: u32, out: u32) -> u32 {
    unsafe {
        const GLOBAL: u32 = 0x019F09DC;
        const READ_CALLEE: u32 = 1;
        const DESER_CALLEE: u32 = 2;
        const BITS_SLOT: usize = 4;
        let mut frame = [0u32; 8];
        frame[0] = buf;
        frame[2] = count.wrapping_mul(8);
        let base = frame.as_mut_ptr() as u32;
        let kind = lf_checker_rt::global::<u32>(GLOBAL).read_unaligned();
        let r1 = lf_checker_rt::callee_fastcall!(READ_CALLEE, u32, kind, base);
        if r1 & 0xFF == 0 {
            if out != 0 {
                (out as *mut u32).write_unaligned(0);
                return 0;
            }
            return r1 & 0xFFFF_FF00;
        }
        let r2 = lf_checker_rt::callee_fastcall!(DESER_CALLEE, u32, base, this);
        if r2 & 0xFF == 0 {
            if out != 0 {
                (out as *mut u32).write_unaligned(0);
                return 0;
            }
            return r2 & 0xFFFF_FF00;
        }
        if out == 0 {
            return (r2 & 0xFFFF_FF00) | 1;
        }
        let bytes = ((frame[BITS_SLOT].wrapping_add(7)) as i32 >> 3) as u32;
        (out as *mut u32).write_unaligned(bytes);
        (bytes & 0xFFFF_FF00) | 1
    }
});
