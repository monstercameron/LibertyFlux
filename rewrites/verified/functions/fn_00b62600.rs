// original: 0x00B62600 resolve_weapon_part_entry

/// Resolve one weapon part for this object. `model_info` is the optional
/// model record (its signed model index is at `+0x2e`), and `part_index`
/// selects a cached entry in the object list at `+0x64`. A cached value other
/// than `u32::MAX` is returned directly. Otherwise the model-indexed global
/// descriptor chain is followed; a missing link also returns `u32::MAX`.
/// Selectors 1 and 2 pass the resolved pointer with the corresponding part
/// tag to the shared cdecl helper; other selectors return `u32::MAX`.
/// The original is thiscall with two stack words, only the model pointer and
/// selector are read, and all game pointers use the executable's 32-bit layout.
lf_checker_rt::export!(thiscall, rw_00B62600(this: u32, model_info: u32, part_index: u32) -> u32 {
    unsafe {
        const MODEL_PART_TABLE_VA: u32 = 0x0129_5cd8;
        const MODEL_INDEX: u32 = 0x2e;
        const CACHE_VALUES: u32 = 0x64;
        const NO_PART: u32 = u32::MAX;
        const PART_HELPER: u32 = 2;
        const FIND_LIST: u32 = 1;
        const PART_TAG_ONE: u32 = 0xa3c2;
        const PART_TAG_TWO: u32 = 0x4724;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_i16(address: u32) -> i16 {
            unsafe { (address as *const i16).read_unaligned() }
        }

        let cached_list = lf_checker_rt::callee_thiscall!(FIND_LIST, u32, this);
        if cached_list != 0 {
            let cache_address = cached_list
                .wrapping_add(CACHE_VALUES)
                .wrapping_add(part_index.wrapping_mul(4));
            let cached_part = read_u32(cache_address);
            if cached_part != NO_PART {
                return cached_part;
            }
        }

        if model_info == 0 {
            return NO_PART;
        }
        let model_index = read_i16(model_info.wrapping_add(MODEL_INDEX)) as i32;
        let table = lf_checker_rt::relocated(MODEL_PART_TABLE_VA);
        let model_entry = read_u32(table.wrapping_add((model_index as u32).wrapping_mul(4)));

        let model_data = read_u32(model_entry.wrapping_add(8));
        let part_source = if model_data != 0 {
            read_u32(model_data.wrapping_add(0xb4))
        } else {
            let chain = read_u32(model_entry.wrapping_add(0x0c));
            if chain == 0 {
                return NO_PART;
            }
            read_u32(chain)
        };
        if part_source == 0 {
            return NO_PART;
        }
        let part_pointer = read_u32(part_source.wrapping_add(0x0c));
        if part_pointer == 0 {
            return NO_PART;
        }

        match part_index {
            1 => lf_checker_rt::callee_cdecl!(PART_HELPER, u32, part_pointer, PART_TAG_ONE),
            2 => lf_checker_rt::callee_cdecl!(PART_HELPER, u32, part_pointer, PART_TAG_TWO),
            _ => NO_PART,
        }
    }
});
