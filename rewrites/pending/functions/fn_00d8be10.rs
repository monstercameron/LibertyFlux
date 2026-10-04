// original: 0x00d8be10 Weapon_MatchesAnyCategory
/// Weapon category matcher: `id` selects a weapon, `kind` selects the test.
///
/// Returns 0 for the null id (-1). Otherwise asks the weapon-info helper
/// for the weapon's record and reads the category field at offset 0xC:
/// kind 0x38 tests membership in category 1, kind 0x39 tests membership in
/// categories 1..=4, any other kind returns 0.
export!(cdecl, rw_00d8be10(id: u32, kind: u32) -> u32 {
    unsafe {
        if id == 0xFFFF_FFFF {
            return 0;
        }
        let info = callee_cdecl!(1, u32, id);
        let category = *((info + 0xC) as *const u32);
        if kind == 0x38 {
            (category == 1) as u32
        } else if kind == 0x39 {
            (category >= 1 && category <= 4) as u32
        } else {
            0
        }
    }
});
