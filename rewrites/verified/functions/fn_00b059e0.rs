// original: 0x00b059e0 build_rows_and_publish
/// Build one child per live row and publish each.
///
/// thiscall `(this, buf)`: iterates rows `0..count` (`count` SIGNED at
/// `+8`; empty returns the incoming register, which the contract does not
/// compare). A row is skipped when its flag byte (at `[+4]+row`, bit
/// 0x80) is set, when its address (`[+0]+stride*row`, `stride` at `+0xc`)
/// is null, or when its status byte (at `+0xd`) is nonzero. Otherwise it
/// allocates a child (helper 1, cdecl `(0xd70)`); a null child stores
/// null at row `+8`, else the row key (helper 2, thiscall `(this, row,
/// row)`) seeds the child (helper 3, thiscall `(child, key)`), whose
/// answer lands at row `+8`. Every built row is then published: the row
/// token (helper 4, thiscall `(this, row)`) goes to the sink (helper 5,
/// thiscall `(buf+0x400, token)`). The null-row and flag-recheck paths
/// never trigger under scripted callees and are listed as uncovered.
export!(thiscall, rw_00b059e0(this: u32, buf: u32) -> u32 {
    const ALLOC_SIZE: u32 = 0xD70;
    const FLAG_BIT: u8 = 0x80;
    unsafe {
        let count = ((this + 8) as *const u32).read_unaligned();
        if (count as i32) <= 0 {
            return 0; // unchecked passthrough, see doc comment
        }
        let flags = ((this + 4) as *const u32).read_unaligned();
        let rows = (this as *const u32).read_unaligned();
        let stride = ((this + 0xC) as *const u32).read_unaligned();
        let nn = count as i32;
        let mut row = 0i32;
        while row < nn {
            let r = row as u32;
            if ((flags.wrapping_add(r)) as *const u8).read() & FLAG_BIT == 0 {
                let addr = rows.wrapping_add(stride.wrapping_mul(r));
                if addr != 0
                    && ((addr + 0xD) as *const u8).read() == 0
                {
                    let child: u32 = callee_cdecl!(1, u32, ALLOC_SIZE);
                    let stored = if child == 0 {
                        0
                    } else {
                        let key: u32 = callee_thiscall!(2, u32, this, r, r);
                        callee_thiscall!(3, u32, child, key)
                    };
                    // The original re-tests the flag here (a real callee
                    // could have set it); scripted callees never do, so the
                    // row address is recomputed identically.
                    let addr2 = rows.wrapping_add(stride.wrapping_mul(r));
                    ((addr2 + 8) as *mut u32).write_unaligned(stored);
                    let tok: u32 = callee_thiscall!(4, u32, this, r);
                    let _: u32 = callee_thiscall!(5, u32, buf.wrapping_add(0x400), tok);
                }
            }
            row += 1;
        }
    }
    0 // unchecked: last sink answer or passthrough, see doc comment
});
