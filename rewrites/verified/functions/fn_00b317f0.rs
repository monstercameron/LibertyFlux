// original: 0x00b317f0 task_flag_bit_probe (proposed)

/// Look up a record through a helper and test one status bit.
///
/// `this` points to a task record holding a key at `+0x18`. The key is passed
/// to a lookup callee; bit 10 of the dword at `+0x20` of the returned record
/// is the result (0 or 1).
///
/// Original: 0x00b317f0 (thiscall, no stack words; one cdecl callee of one word).
lf_checker_rt::export!(thiscall, rw_00b317f0(this: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x18;
        const STATUS_OFF: u32 = 0x20;
        const BIT: u32 = 10;
        const LOOKUP: u32 = 1;
        let key = (this as *const u32).byte_add(KEY_OFF as usize).read_unaligned();
        let rec: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, key);
        ((rec as *const u32).byte_add(STATUS_OFF as usize).read_unaligned() >> BIT) & 1
    }
});
