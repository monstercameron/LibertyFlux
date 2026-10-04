// original: 0x009e2090 CPortalVisTracker::vf1
/// 0x009E2090 (CPortalVisTracker::vf1): clear the changed flag, refresh via
/// the helper, drop the stale cached cell (if any differs from the current
/// one), and recache the current cell. Returns the current cell. (thiscall/2)
export!(thiscall, rw_009e2090(this: *mut u8, cell: u32, _unused: u32) -> u32 {
    unsafe {
        *this.add(0x74) = 0;
        callee_thiscall!(1, u32, this as u32, cell, 0);
        let cached = *((this.add(0xc0)) as *const u32);
        let current = *((this.add(0x30)) as *const u32);
        if cached != 0 && cached != current {
            *((cached.wrapping_add(0x72)) as *mut u8) = 0;
        }
        *(this.add(0xc0) as *mut u32) = current;
        current
    }
});
