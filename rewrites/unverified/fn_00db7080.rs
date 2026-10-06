// original: 0x00DB7080 UIFontString::vf86

/// Snapshot the live string settings into the selected row descriptor and copy the text after it.
///
/// Reads the global slot selector, copies the live fields (corners at
/// `+0x1F8`/`+0x1FC`, position at `+0x200`/`+0x204`, style at `+0x1DC`, tag
/// at `+0x1F4`, flag byte at `+0x208`) into the `slot * 17` row, then either
/// adds the width (`[0x1E4] + [0x1F8]`, original operand order) or stores
/// the 0/1.0 defaults depending on the byte at `+0x20B`, copies the
/// NUL-terminated text at `+0x20E` to `+0x30E + (slot << 8)` byte by byte,
/// and sets the row-ready flag. Leaf: no calls. Void.
lf_checker_rt::export!(thiscall, rw_00db7080(this_ptr: u32) -> u32 {
    const SLOT: u32 = 0x017A65A8;
    const ROW_WORDS: u32 = 17;
    unsafe {
        let slot = lf_checker_rt::global::<u32>(SLOT).read();
        let row = this_ptr
            .wrapping_add(slot.wrapping_mul(ROW_WORDS).wrapping_mul(4));
        let rd = |off: u32| unsafe { ((this_ptr + off) as *const u32).read_unaligned() };
        let wr = |off: u32, v: u32| unsafe {
            ((row + off) as *mut u32).write_unaligned(v)
        };
        wr(0xF0, rd(0x1F8));
        wr(0xF4, rd(0x1FC));
        wr(0x100, rd(0x200));
        wr(0x104, rd(0x204));
        wr(0x120, rd(0x1DC));
        wr(0x124, rd(0x1F4));
        ((row + 0x12C) as *mut u8).write(((this_ptr + 0x208) as *const u8).read());
        if ((this_ptr + 0x20B) as *const u8).read() != 0 {
            wr(0x108, rd(0x1F8));
            let a = f32::from_bits(rd(0x1E4));
            let b = f32::from_bits(rd(0x1F8));
            let s = core::hint::black_box(a) + core::hint::black_box(b);
            wr(0x10C, s.to_bits());
        } else {
            wr(0x108, 0);
            wr(0x10C, 0x3F800000);
        }
        let mut src = this_ptr + 0x20E;
        let mut dst = this_ptr + 0x30E + (slot << 8);
        loop {
            let byte = (src as *const u8).read();
            (dst as *mut u8).write(byte);
            src += 1;
            dst += 1;
            if byte == 0 {
                break;
            }
        }
        ((row + 0x130) as *mut u8).write(1);
    }
    0
});
