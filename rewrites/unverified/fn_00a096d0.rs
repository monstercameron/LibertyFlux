// original: 0x00a096d0 mission_cleanup_add_entry
/// Add a mission-cleanup record to the manager's 256-entry primary table.
///
/// `this` owns 0x2c-byte entries beginning at +4. Each entry stores the low
/// byte of `kind` at +0, `key` at +4, the optional owner identifier at +8,
/// `stored_value` at +0x0c, and a cleared word at +0x10. An owner identifier
/// is read from `owner + 4`, or treated as zero when `owner` is null. The scan
/// returns without mutation on the first entry matching kind, key, and owner;
/// otherwise it remembers the last empty entry and initializes it. A nonzero
/// `intern_value` then calls the intern helper with the manager, key,
/// `intern_value`, and owner. The method uses thiscall with five stack words.
lf_checker_rt::export!(thiscall, rw_00a096d0(
    this: u32,
    key: u32,
    kind: u32,
    owner: u32,
    stored_value: u32,
    intern_value: u32,
) -> u32 {
    unsafe {
        const ENTRIES: u32 = 4;
        const ENTRY_COUNT: u32 = 256;
        const ENTRY_STRIDE: u32 = 0x2c;
        const KIND: u32 = 0;
        const KEY: u32 = 4;
        const OWNER: u32 = 8;
        const STORED_KEY: u32 = 0x0c;
        const CLEARED_WORD: u32 = 0x10;
        const INTERN: u32 = 0;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let kind_byte = kind as u8;
        let owner_id = if owner == 0 {
            0
        } else {
            read_u32(owner.wrapping_add(4))
        };
        let mut last_empty = None;

        for index in 0..ENTRY_COUNT {
            let entry = this
                .wrapping_add(ENTRIES)
                .wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            if read_u32(entry.wrapping_add(KEY)) == key
                && read_u8(entry.wrapping_add(KIND)) == kind_byte
                && read_u32(entry.wrapping_add(OWNER)) == owner_id
            {
                return 0;
            }
            if read_u8(entry.wrapping_add(KIND)) == 0 {
                last_empty = Some(entry);
            }
        }

        if let Some(entry) = last_empty {
            write_u32(entry.wrapping_add(STORED_KEY), stored_value);
            write_u32(entry.wrapping_add(KEY), key);
            write_u8(entry.wrapping_add(KIND), kind_byte);
            write_u32(entry.wrapping_add(OWNER), owner_id);
            write_u32(entry.wrapping_add(CLEARED_WORD), 0);

            if intern_value != 0 {
                let _ = lf_checker_rt::callee_thiscall!(
                    INTERN,
                    u32,
                    this,
                    key,
                    intern_value,
                    owner
                );
            }
        }

        0
    }
});
