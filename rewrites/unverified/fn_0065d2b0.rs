// original: 0x0065D2B0 rage::IntervalShadows::IntervalShadowVarCache::vf0

/// Resolve 11 shader-variable indices by hashed name and cache them.
///
/// `var_cache` is the cache object (ECX), `effect` the effect holding the
/// shader-variable table. Each block hashes one variable name through the
/// string-hash callee, scans the table's entries (0x30 bytes each, two hash
/// words at +0 and +4) for either word equal to the hash, and stores the
/// one-based index of the first matching entry, or zero when nothing matches
/// (including an empty table). The table base is at `effect+0x18`, its entry
/// array at `+8` (entries start 0xc bytes in) and its 16-bit entry count at
/// `+0xc`. Stores land at 0x04, 0x08, 0x0C, 0x10, 0x14, 0x18, 0x1C, 0x20, 0x24, 0x28, 0x2C. The return value is the last block's
/// stored value, or the table count when the last block matched nothing.
///
/// Original: 0x0065D2B0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0065d2b0(var_cache: u32, effect: u32) -> u32 {
    unsafe {
        const HASH_CALLEE: u32 = 1;
        const EFFECT_VAR_TABLE: u32 = 0x18;
        const TABLE_ENTRIES: u32 = 0x08;
        const TABLE_COUNT: u32 = 0x0c;
        const ENTRIES_SKIP: u32 = 0x0c;
        const ENTRY_STRIDE: u32 = 0x30;
        const ENTRY_HASH_FIRST: u32 = 0x00;
        const ENTRY_HASH_SECOND: u32 = 0x04;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn table_count(table: u32) -> u32 {
            unsafe { (table.wrapping_add(TABLE_COUNT) as *const u16).read_unaligned() as u32 }
        }

        /// One-based index of the first entry whose either hash word equals
        /// `hash`, or zero. The +4 word is tested before the +0 word.
        unsafe fn lookup(table: u32, hash: u32) -> u32 {
            unsafe {
                let count = table_count(table);
                let mut entry = rd32(table.wrapping_add(TABLE_ENTRIES)).wrapping_add(ENTRIES_SKIP);
                let mut idx = 0u32;
                while idx < count {
                    if rd32(entry.wrapping_add(ENTRY_HASH_SECOND)) == hash
                        || rd32(entry.wrapping_add(ENTRY_HASH_FIRST)) == hash
                    {
                        return idx.wrapping_add(1);
                    }
                    entry = entry.wrapping_add(ENTRY_STRIDE);
                    idx = idx.wrapping_add(1);
                }
                0
            }
        }

        #[inline(always)]
        unsafe fn hash_name(name: u32) -> u32 {
            lf_checker_rt::callee_cdecl!(HASH_CALLEE, u32, name, 0)
        }

        let table = rd32(effect.wrapping_add(EFFECT_VAR_TABLE));
        wr32(var_cache.wrapping_add(0x04), lookup(table, hash_name(lf_checker_rt::relocated(0xF9B284))));
        wr32(var_cache.wrapping_add(0x08), lookup(table, hash_name(lf_checker_rt::relocated(0xF9B2A0))));
        wr32(var_cache.wrapping_add(0x0C), lookup(table, hash_name(lf_checker_rt::relocated(0xF9B204))));
        wr32(var_cache.wrapping_add(0x10), lookup(table, hash_name(lf_checker_rt::relocated(0xF9B228))));
        wr32(var_cache.wrapping_add(0x14), lookup(table, hash_name(lf_checker_rt::relocated(0xF9B24C))));
        wr32(var_cache.wrapping_add(0x18), lookup(table, hash_name(lf_checker_rt::relocated(0xF9B25C))));
        wr32(var_cache.wrapping_add(0x1C), lookup(table, hash_name(lf_checker_rt::relocated(0xF9B304))));
        wr32(var_cache.wrapping_add(0x20), lookup(table, hash_name(lf_checker_rt::relocated(0xF9B318))));
        wr32(var_cache.wrapping_add(0x24), lookup(table, hash_name(lf_checker_rt::relocated(0xF9B324))));
        wr32(var_cache.wrapping_add(0x28), lookup(table, hash_name(lf_checker_rt::relocated(0xF9B32C))));
        let last = lookup(table, hash_name(lf_checker_rt::relocated(0xF9B2C0)));
        wr32(var_cache.wrapping_add(0x2C), last);
        if last != 0 {
            last
        } else {
            table_count(table)
        }
    }
});
