// original: 0x00602A90 rage::rmPtfxManager::find_effect_rule (proposed)

/// Find an effect rule by name reference, or by key when no reference
/// is given.
///
/// `this` in ECX with two stack arguments: a name reference (`key_ref`,
/// null to search by key) and the search key (`key`). When `key_ref` is
/// non-null the answer comes from the reference lookup (callee 1,
/// thiscall with the table at `[key_ref + TABLE]` (+0x18) and `key`)
/// and is returned as is. Otherwise the shared registry (file VA
/// `REGISTRY` 0x01BB67C0) is tried first when set (callee 2, thiscall
/// with the registry and `key`); a non-null answer is returned. When
/// that fails too, the manager's own sorted rule table (`RULES` at
/// +0x234, 16-bit `RULE_COUNT` at +0x238, 8 bytes per entry) is searched
/// with a binary search (callee 3, cdecl with the address of `key_ref`'s
/// stack slot, the table, the count, the entry size 8 and the fixed
/// comparator address `COMPAR`); the entry word past the hit is
/// returned, or 0 when the search misses or hits the rejected sentinel.
/// Thiscall.
lf_checker_rt::export!(thiscall, rw_00602A90(this: u32, key_ref: u32, key: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x18;
        const REGISTRY: u32 = 0x01BB67C0;
        const RULES: u32 = 0x234;
        const RULE_COUNT: u32 = 0x238;
        const ENTRY_SIZE: u32 = 8;
        const COMPAR: u32 = 0x00412950;
        const REF_CALLEE: u32 = 1;
        const REG_CALLEE: u32 = 2;
        const SEARCH_CALLEE: u32 = 3;

        if key_ref != 0 {
            let table = ((key_ref + TABLE) as *const u32).read();
            return lf_checker_rt::callee_thiscall!(REF_CALLEE, u32, table, key);
        }
        let reg = lf_checker_rt::global::<u32>(REGISTRY).read();
        if reg != 0 {
            let hit: u32 = lf_checker_rt::callee_thiscall!(REG_CALLEE, u32, reg, key);
            if hit != 0 {
                return hit;
            }
        }
        let count = ((this + RULE_COUNT) as *const u16).read();
        if count == 0 {
            return 0;
        }
        // The original passes the address of its own key_ref argument
        // slot; the checker compares the pointed-to word by snapshot.
        let slot = key_ref;
        let slot_addr = (&slot as *const u32) as u32;
        let found: u32 = lf_checker_rt::callee_cdecl!(
            SEARCH_CALLEE,
            u32,
            slot_addr,
            ((this + RULES) as *const u32).read(),
            count as u32,
            ENTRY_SIZE,
            lf_checker_rt::relocated(COMPAR)
        );
        core::hint::black_box(&slot);
        if found == 0 || found.wrapping_add(4) == 0 {
            return 0;
        }
        ((found + 4) as *const u32).read()
    }
});
