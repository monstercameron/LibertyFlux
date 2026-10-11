// original: 0x00CA2320 GetScenarioProgressValue

/// Extract the 4-bit scenario state from the object's flags at +0x28. For state 3, use the dword at +0x2B0 as a base index, add three, multiply by three, and read the resulting table entry beginning at +0x2B0. For state 6, follow pointers at +0x118 and then +0x25C, returning the final dword at +0x18; a null at either link returns zero. Other states return zero. This stdcall helper removes its object argument.
lf_checker_rt::export!(stdcall, rw_00ca2320(object: u32) -> u32 {
    const SCENARIO_FLAGS: u32 = 0x28;
    const TABLE_BASE: u32 = 0x2B0;
    const SCENARIO_ENTITY: u32 = 0x118;
    const ENTITY_DATA: u32 = 0x25C;
    const FINAL_VALUE: u32 = 0x18;
    let flags = unsafe { (object.wrapping_add(SCENARIO_FLAGS) as *const u32).read_unaligned() };
    let state = (flags >> 6) & 0xF;
    match state {
        3 => {
            let base_index = unsafe { (object.wrapping_add(TABLE_BASE) as *const u32).read_unaligned() };
            let index = base_index.wrapping_add(3).wrapping_mul(3);
            let address = object.wrapping_add(TABLE_BASE).wrapping_add(index.wrapping_mul(4));
            unsafe { (address as *const u32).read_unaligned() }
        }
        6 => {
            let entity = unsafe { (object.wrapping_add(SCENARIO_ENTITY) as *const u32).read_unaligned() };
            if entity == 0 {
                0
            } else {
                let data = unsafe { (entity.wrapping_add(ENTITY_DATA) as *const u32).read_unaligned() };
                if data == 0 {
                    0
                } else {
                    unsafe { (data.wrapping_add(FINAL_VALUE) as *const u32).read_unaligned() }
                }
            }
        }
        _ => 0,
    }
});
