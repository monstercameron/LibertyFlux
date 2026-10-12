// original: 0x00B0FF20 pool_entry_value_lookup
/// Resolve a pool handle to its value word. The active handle returns 48,
/// either reserved handle returns 47, and the all-ones handle returns zero.
/// Other handles index the global object table; virtual slot 3 returns a
/// status in AL, and status 4 selects the object's word at byte offset 96.
/// This cdecl routine scripts the indirect thiscall while preserving the
/// object pointer as ECX.
lf_checker_rt::export!(cdecl, rw_00b0ff20(handle: u32) -> u32 {
    const ACTIVE_HANDLE: u32 = 0x01632BF0;
    const RESERVED_HANDLE_A: u32 = 0x012FA3E0;
    const RESERVED_HANDLE_B: u32 = 0x012F9E94;
    const HANDLE_TABLE: u32 = 0x01295CD8;
    const TABLE_STRIDE: u32 = 4;
    const METHOD_SLOT: u32 = 12;
    const VALUE_WORD: u32 = 0x60;
    const VALUE_STATUS: u8 = 4;

    unsafe {
        let active = lf_checker_rt::global::<u32>(ACTIVE_HANDLE).read_volatile();
        if handle == active {
            return 0x30;
        }
        let reserved_a = lf_checker_rt::global::<u32>(RESERVED_HANDLE_A).read_volatile();
        if handle == reserved_a {
            return 0x2F;
        }
        let reserved_b = lf_checker_rt::global::<u32>(RESERVED_HANDLE_B).read_volatile();
        if handle == reserved_b {
            return 0x2F;
        }
        if handle == u32::MAX {
            return 0;
        }

        let table = lf_checker_rt::global::<u32>(HANDLE_TABLE) as u32;
        let entry = table.wrapping_add(handle.wrapping_mul(TABLE_STRIDE)) as *const u32;
        let object = entry.read_volatile();
        let object_table = (object as *const u32).read_volatile();
        let method = object_table.wrapping_add(METHOD_SLOT) as *const u32;
        let target = method.read_volatile();
        let is_value_ready: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        if is_value_ready(object) as u8 == VALUE_STATUS {
            (object.wrapping_add(VALUE_WORD) as *const u32).read_volatile()
        } else {
            0
        }
    }
});
