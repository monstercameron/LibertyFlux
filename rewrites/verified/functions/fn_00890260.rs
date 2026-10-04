// original: 0x00890260 audio_masked_field_unpack
/// Copy the selected fields of a packed audio record to fixed offsets.
///
/// `flags`, a 24-bit mask at `src+0xb`, says which of the 24 fields are
/// present; present fields are stored back to back starting at `src+0xf` and
/// are copied to their fixed slots in `dst` (words at +0xf..+0x1d and
/// +0x27..+0x35, dwords at +0x1f..+0x23 and +0x2d..+0x4d, bytes at +0x37 and
/// +0x38). Returns the first unconsumed packed byte, matching the cursor the
/// original leaves in EAX.
export!(cdecl, rw_00890260(src: u32, dst: u32) -> u32 {
    const FIELDS: [(u32, u8, u32); 24] = [
        (0x000001, 2, 0x0f), (0x000002, 2, 0x11),
        (0x000004, 2, 0x13), (0x000008, 2, 0x15),
        (0x000010, 2, 0x17), (0x000020, 2, 0x19),
        (0x000040, 2, 0x1b), (0x000080, 2, 0x1d),
        (0x000100, 4, 0x1f), (0x000200, 4, 0x23),
        (0x000400, 2, 0x27), (0x000800, 2, 0x29),
        (0x001000, 2, 0x2b), (0x002000, 4, 0x2d),
        (0x004000, 4, 0x31), (0x008000, 2, 0x35),
        (0x010000, 1, 0x37), (0x020000, 1, 0x38),
        (0x040000, 4, 0x39), (0x080000, 4, 0x3d),
        (0x100000, 4, 0x41), (0x200000, 4, 0x45),
        (0x400000, 4, 0x49), (0x800000, 4, 0x4d),
    ];
    unsafe {
        let flags = ((src + 0x0b) as *const u32).read_unaligned();
        let mut cur = src + 0x0f;
        for &(bit, width, off) in FIELDS.iter() {
            if (flags & bit) != 0 {
                match width {
                    1 => {
                        let v = (cur as *const u8).read();
                        ((dst + off) as *mut u8).write(v);
                        cur += 1;
                    }
                    2 => {
                        let v = (cur as *const u16).read_unaligned();
                        ((dst + off) as *mut u16).write_unaligned(v);
                        cur += 2;
                    }
                    _ => {
                        let v = (cur as *const u32).read_unaligned();
                        ((dst + off) as *mut u32).write_unaligned(v);
                        cur += 4;
                    }
                }
            }
        }
        cur
    }
});
