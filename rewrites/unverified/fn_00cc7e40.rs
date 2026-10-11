// original: 0x00CC7E40 phInstGta::vf6

/// Writes a physics record to the caller's 16-byte output. The first three
/// output words are cleared before lookup. A candidate at input-record offset
/// `+0x38` supplies a 16-bit table index at `+0x08`; the relocated manager
/// pointer selects a count, per-index flags (stride 8), and a pointer table
/// (stride 4). A valid entry copies its word at `+0x120`, two raw float words
/// at `+0x124` and `+0x128`, and its final word at `+0x12c`. The runtime table
/// is supplied by the contract as a synthetic heap graph; the fallback helper
/// path for record kinds 2 through 4 is outside this proof's input domain.
lf_checker_rt::export!(thiscall, rw_00cc7e40(this: u32, output: u32) -> () {
    const RECORD_POINTER: u32 = 0x0c;
    const RECORD_CANDIDATE: u32 = 0x38;
    const CANDIDATE_INDEX: u32 = 0x08;
    const TABLE_POINTER: u32 = 0x012b_9c7c;
    const ROOT_MANAGER: u32 = 0x04;
    const MANAGER_COUNT: u32 = 0x26;
    const MANAGER_FLAGS: u32 = 0x70;
    const MANAGER_ENTRIES: u32 = 0x80;
    const ENTRY_STRIDE: u32 = 8;
    const ENTRY_FLAGS: u32 = 4;
    const DATA_WORD: u32 = 0x120;
    const DATA_FLOAT0: u32 = 0x124;
    const DATA_FLOAT1: u32 = 0x128;
    const DATA_FINAL: u32 = 0x12c;
    const FLAG_MASK: u8 = 3;
    const INVALID_INDEX: u16 = 0xffff;

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
        let candidate = read_u32(record.wrapping_add(RECORD_CANDIDATE));
        if candidate == 0 {
            return;
        }
        let index = read_u16(candidate.wrapping_add(CANDIDATE_INDEX));
        if index == INVALID_INDEX || candidate == this {
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
        if entry_flags & FLAG_MASK != 0 {
            return;
        }

        let entries = read_u32(manager.wrapping_add(MANAGER_ENTRIES));
        let data = read_u32(entries.wrapping_add((index as u32).wrapping_mul(4)));
        if data == 0 {
            return;
        }
        write_u32(output, read_u32(data.wrapping_add(DATA_WORD)));
        write_u32(output.wrapping_add(4), read_u32(data.wrapping_add(DATA_FLOAT0)));
        write_u32(output.wrapping_add(8), read_u32(data.wrapping_add(DATA_FLOAT1)));
        write_u32(output.wrapping_add(12), read_u32(data.wrapping_add(DATA_FINAL)));
    }
});
