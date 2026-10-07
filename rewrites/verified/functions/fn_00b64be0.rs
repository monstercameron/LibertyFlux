// original: 0x00b64be0 PedWeapon_GetTotalAmmo
// thiscall/1 (PedWeapon_GetTotalAmmo). Looks up the current entry; when it is
// present and its tag matches the requested index, applies the entry's limit
// to this object's indexed slot. Returns the halfword at the indexed slot.
///
/// Proven scope: one lookup entry and one info record match indices 0
/// through 3; the entry limit is 77. Lookup and translation return fixed
/// pointers and the apply stub returns 0. The nonmatching/no-apply path is
/// not covered.
export!(thiscall, rw_rs11f5(this: *mut u8, index: u32) -> u32 {
    unsafe {
        let lookup: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let entry = lookup(this as u32);
        if entry != 0 {
            let info = callee_cdecl!(2, u32, *((entry + 0x18) as *const u32));
            if index == *((info + 4) as *const u32) {
                let limit = *((entry + 0x60) as *const u16);
                let apply: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(callee_addr(3) as usize);
                apply(
                    (this as u32).wrapping_add(index.wrapping_add(3).wrapping_mul(12)),
                    limit as u32,
                );
            }
        }
        *((this as u32).wrapping_add(index.wrapping_mul(12)).wrapping_add(0x28) as *const u16)
            as u32
    }
});
