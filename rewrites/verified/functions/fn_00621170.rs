// original: 0x00621170 net_session_copy_addr_blocks
/// Copy the address block of each session entry into an output array.
///
/// Copies 16 bytes at +0x48 of each entry in the pointer table at
/// `this+0x1d94` into consecutive 16-byte slots at `out`, for up to
/// `count` (`this+0x1e14`, unsigned) entries but never more than 32.
/// Copies nothing when the count is zero. The second argument is unused.
/// Returns the number of blocks copied.
export!(thiscall, rw_00621170(this: u32, out: u32, _unused: u32) -> u32 {
    unsafe {
        let base = this as *const u8;
        let count = (base.add(0x1e14) as *const u32).read_unaligned();
        if count == 0 {
            return 0;
        }
        let n = core::cmp::min(count, 32);
        let table = base.add(0x1d94) as *const u32;
        let mut i: u32 = 0;
        while i < n {
            let e = table.add(i as usize).read_unaligned() as *const u8;
            let dst = (out as *mut u8).add((i as usize) * 16) as *mut u64;
            dst.write_unaligned((e.add(0x48) as *const u64).read_unaligned());
            dst.add(1).write_unaligned((e.add(0x50) as *const u64).read_unaligned());
            i += 1;
        }
        n
    }
});
