// original: 0x00AF62F0 lookup_weapon_category_name

/// Search a fixed name table with the intercepted string comparator.
/// Empty input and a complete miss return the table-specific sentinel.
/// The string is a cdecl stack argument; the table holds 32-bit pointers.
lf_checker_rt::export!(cdecl, rw_00AF62F0(value_name: u32) -> u32 {
    const TABLE_VA: u32 = 0x0103FE08;
    const ENTRY_COUNT: u32 = 18;
    const NOT_FOUND: u32 = 0x00000000;

    let empty = unsafe { (value_name as *const u8).read_volatile() == 0 };
    if empty {
        return NOT_FOUND;
    }

    let table = lf_checker_rt::global::<u32>(TABLE_VA);
    let mut index = 0u32;
    while index < ENTRY_COUNT {
        let candidate = unsafe { table.add(index as usize).read_volatile() };
        let comparison = lf_checker_rt::callee_cdecl!(1, u32, value_name, candidate);
        if comparison == 0 {
            return index;
        }
        index += 1;
    }
    NOT_FOUND
});
