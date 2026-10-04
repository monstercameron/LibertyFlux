// original: 0x00AB82E0 build_player_diff_resource_name
// Build (or reuse) a player texture-difference resource name, intern it,
// resolve it through the file manager, and publish it into the caller's
// table slot.
//
// `slot` (`a7`) memoizes the interned name: when it already holds a
// non-null pointer the name-building step is skipped and the stored name is
// reused. Otherwise the name is assembled from the source record (`a3`: an
// 8-byte prefix plus a discriminator byte), the low decimal digits of `a4`,
// a separator, the character `a5`, another separator, and a 3-letter suffix
// looked up from a data table by `a6`; the result is interned and stored
// back into `slot`. The manager for key `a1` then resolves the name to a
// resource, which is returned and, when non-null and a destination table
// (`a0`) was given, stored at index `a2` of that table's resource row.
export!(cdecl, rw_00AB82E0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32) -> u32 {
    unsafe {
        // Scratch area the original zeroes through its fill call (60 bytes).
        let mut scratch = [0u8; 60];
        callee_cdecl!(1, u32, scratch.as_mut_ptr() as u32, 0, 0x3c);
        let slot = a7 as *mut u32;
        let name: u32 = if *slot != 0 {
            *slot
        } else {
            let mut buf = [0u8; 32];
            let src = a3 as *const u8;
            buf[0] = *src;
            buf[1] = *src.add(1);
            buf[2] = *src.add(2);
            buf[3] = *src.add(3);
            buf[4] = *src.add(4);
            buf[5] = *src.add(5);
            buf[6] = *src.add(6);
            buf[7] = *src.add(7);
            buf[8] = *src.add(8);
            buf[9] = b'_';
            // Three low decimal digits of the number, most significant first
            // (the loop counter is decremented before each store, so the
            // hundreds digit lands here, overwriting the template byte).
            buf[10] = (((a4 / 100) % 10) as u8) + b'0';
            buf[11] = (((a4 / 10) % 10) as u8) + b'0';
            buf[12] = ((a4 % 10) as u8) + b'0';
            buf[13] = b'_';
            buf[14] = a5 as u8;
            buf[15] = b'_';
            let tab = global::<u32>(0x0103EE34);
            let suf = *tab.add(a6 as usize) as *const u8;
            buf[16] = *suf;
            buf[17] = *suf.add(1);
            buf[18] = *suf.add(2);
            // Terminator from the read-only template (a zero byte).
            buf[19] = *global::<u8>(0x00EA50AB);
            // buf[20..] stay zero: the original's fill call zeroes the tail.
            let interned: u32 = callee_cdecl!(2, u32, buf.as_ptr() as u32, 0);
            *slot = interned;
            interned
        };
        let mgr: u32 = callee_cdecl!(3, u32, a1);
        let res: u32 = callee_thiscall!(4, u32, mgr, name);
        if res != 0 && a0 != 0 {
            let dest =
                a0.wrapping_add(a2.wrapping_mul(4)).wrapping_add(0x318) as *mut u32;
            *dest = res;
        }
        res
    }
});
