// original: 0x00875b20 crmt_channel_publish
/// Publish the channel cookie: table slot while small, fresh block once large.
///
/// cdecl/0. Reads the channel counter; while it is below 23 the cookie is
/// stamped into the counter-th table slot, otherwise a fresh block is
/// allocated for it. Returns the table base or the fresh block.
///
/// Note: the table base is read from a literal address (the original's
/// displacement has no relocation fixup), so the rewrite reads the same
/// literal address and faults exactly when the original does.
export!(cdecl, rw_00875b20() -> u32 {
    unsafe {
        const LIMIT: i32 = 0x17;
        let count = *global::<u32>(0x01111088);
        if (count as i32) >= LIMIT {
            let block: u32 = callee_stdcall!(1, u32, count);
            *(block as *mut u32) = relocated(0x01111088);
            block
        } else {
            let table = *(0x01b4af2c as *const u32);
            *((table as *mut u32).add(count as usize)) = relocated(0x01111088);
            table
        }
    }
});
