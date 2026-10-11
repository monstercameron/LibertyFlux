// original: 0x00CC7FA0 phInstGta::vf5

/// Produces a four-word physics result from the receiver's nested record.
/// The first three output words are cleared up front. Record kind 3 returns
/// that cleared result. Otherwise, a valid candidate index selects an entry
/// from the relocated manager table; an enabled entry copies four words from
/// offsets `+0x110` through `+0x11c`. A missing, aliased, or sentinel-index
/// candidate falls back to the record's virtual method at `+0xec` for kinds 2
/// through 4 and copies its first three result words. The fallback method's
/// stack pointer argument is compared by a four-word snapshot; the contract
/// gives this uninitialized scratch area a zero fill.
lf_checker_rt::export!(thiscall, rw_00cc7fa0(this: u32, output: u32) -> () {
    const RECORD_POINTER: u32 = 0x0c;
    const RECORD_KIND: u32 = 0x28;
    const RECORD_CANDIDATE: u32 = 0x38;
    const RECORD_VTABLE: u32 = 0;
    const RECORD_TYPE_MASK: u32 = 0x0f;
    const SKIPPED_KIND: u32 = 3;
    const INVALID_INDEX: u16 = 0xffff;
    const TABLE_POINTER: u32 = 0x012b_9c7c;
    const ROOT_MANAGER: u32 = 4;
    const MANAGER_COUNT: u32 = 0x26;
    const MANAGER_FLAGS: u32 = 0x70;
    const MANAGER_ENTRIES: u32 = 0x80;
    const ENTRY_STRIDE: u32 = 8;
    const ENTRY_FLAGS: u32 = 4;
    const VTABLE_METHOD: u32 = 0xec;
    const DATA_WORDS: [u32; 4] = [0x110, 0x114, 0x118, 0x11c];

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        write_u32(output, 0);
        write_u32(output.wrapping_add(4), 0);
        write_u32(output.wrapping_add(8), 0);

        let record = read_u32(this.wrapping_add(RECORD_POINTER));
        if record == 0 {
            return;
        }
        let kind = (read_u32(record.wrapping_add(RECORD_KIND)) >> 6) & RECORD_TYPE_MASK;
        if kind == SKIPPED_KIND {
            return;
        }

        let candidate = read_u32(record.wrapping_add(RECORD_CANDIDATE));
        if candidate == 0 {
            if kind <= 1 || kind >= 5 {
                return;
            }
            let vtable = read_u32(record.wrapping_add(RECORD_VTABLE));
            let method_address = read_u32(vtable.wrapping_add(VTABLE_METHOD));
            let method: unsafe extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(method_address);
            let mut scratch = [0u32; 4];
            let result = method(record, scratch.as_mut_ptr() as u32);
            write_u32(output, read_u32(result));
            write_u32(output.wrapping_add(4), read_u32(result.wrapping_add(4)));
            write_u32(output.wrapping_add(8), read_u32(result.wrapping_add(8)));
            return;
        }

        let index = read_u16(candidate.wrapping_add(8));
        if index == INVALID_INDEX || candidate == this {
            if kind <= 1 || kind >= 5 {
                return;
            }
            let vtable = read_u32(record.wrapping_add(RECORD_VTABLE));
            let method_address = read_u32(vtable.wrapping_add(VTABLE_METHOD));
            let method: unsafe extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(method_address);
            let mut scratch = [0u32; 4];
            let result = method(record, scratch.as_mut_ptr() as u32);
            write_u32(output, read_u32(result));
            write_u32(output.wrapping_add(4), read_u32(result.wrapping_add(4)));
            write_u32(output.wrapping_add(8), read_u32(result.wrapping_add(8)));
            return;
        }

        let table_root = lf_checker_rt::global::<u32>(TABLE_POINTER).read_unaligned();
        let manager = read_u32(table_root.wrapping_add(ROOT_MANAGER));
        let count = read_u16(manager.wrapping_add(MANAGER_COUNT));
        if index >= count {
            return;
        }
        let flags = read_u32(manager.wrapping_add(MANAGER_FLAGS));
        let entry_flags = read_u8(
            flags
                .wrapping_add((index as u32).wrapping_mul(ENTRY_STRIDE))
                .wrapping_add(ENTRY_FLAGS),
        );
        if entry_flags & 3 != 0 {
            return;
        }
        let entries = read_u32(manager.wrapping_add(MANAGER_ENTRIES));
        let data = read_u32(entries.wrapping_add((index as u32).wrapping_mul(4)));
        if data == 0 {
            return;
        }
        for (word, offset) in DATA_WORDS.into_iter().enumerate() {
            write_u32(output.wrapping_add((word as u32) * 4), read_u32(data.wrapping_add(offset)));
        }
    }
});
