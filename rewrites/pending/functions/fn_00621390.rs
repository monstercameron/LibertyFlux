// original: 0x00621390 net_session_set_flag_bit
/// Record a session bit and mirror the enable flag into the status byte.
///
/// Does nothing except clear status bit 1 unless the state at `this+0x50`
/// is 2 or 3 and the id pairs at `this+0xbf0`/`+0xbf4` and `this+0xc30`/
/// `+0xc34` match. On a match, when the low byte of `arg` is nonzero, sets
/// the bit indexed by `this+0x32f4` in the bitmap at `this+0x24` past
/// header 0x68c, then sets status (`this+0x32f8`) bit 1 to the low bit of
/// `arg`. Returns incidental residue like the original.
export!(thiscall, rw_00621390(this: u32, arg: u32) -> u32 {
    unsafe {
        let base = this as *mut u8;
        let r = |off: usize| (base.add(off) as *const u32).read_unaligned();
        let state = r(0x50);
        if state < 2 || state > 3 {
            let f = base.add(0x32f8);
            f.write(f.read() & 0xfd);
            return state;
        }
        let b0 = r(0xbf0);
        if b0 != r(0xc30) {
            let f = base.add(0x32f8);
            f.write(f.read() & 0xfd);
            return b0;
        }
        let b1 = r(0xbf4);
        if b1 != r(0xc34) {
            let f = base.add(0x32f8);
            f.write(f.read() & 0xfd);
            return b1;
        }
        let bl = arg as u8;
        let mut eax = b1;
        if bl != 0 {
            let x = r(0x32f4);
            let bit = x & 0x1f;
            let word_addr = (r(0x24) as *mut u8)
                .wrapping_add(0x68c)
                .wrapping_add((((x as i32 >> 5) * 4) as isize) as usize)
                as *mut u32;
            word_addr.write(word_addr.read() | (1u32 << bit));
            eax = 1u32 << bit;
        }
        let f = base.add(0x32f8);
        f.write((f.read() & !2) | ((bl & 1) << 1));
        eax
    }
});
