// original: 0x009e17d0 CPortalTracker::vf3
/// 0x009E17D0 (CPortalTracker::vf3): attach a link record, then snapshot the
/// 16-byte position block it selects (either the/indirect block at
/// link+0x20 plus 0x30, or the inline block at link+0x10) into the primary
/// slot and mirror it into the secondary slot. (thiscall/1)
export!(thiscall, rw_009e17d0(this: *mut u8, link: u32) -> u32 {
    unsafe {
        *(this.add(0x34) as *mut u32) = link;
        if link == 0 {
            return 0;
        }
        let sel = *((link.wrapping_add(0x20)) as *const u32);
        let src = if sel != 0 {
            sel.wrapping_add(0x30)
        } else {
            link.wrapping_add(0x10)
        };
        let w0 = *(src as *const u32);
        let w1 = *((src.wrapping_add(4)) as *const u32);
        let w2 = *((src.wrapping_add(8)) as *const u32);
        let w3 = *((src.wrapping_add(12)) as *const u32);
        *(this.add(0x10) as *mut u32) = w0;
        *(this.add(0x14) as *mut u32) = w1;
        *(this.add(0x18) as *mut u32) = w2;
        *(this.add(0x1c) as *mut u32) = w3;
        // Mirror: the original re-reads the primary slot for the second copy.
        let m0 = *(this.add(0x10) as *const u32);
        let m1 = *(this.add(0x14) as *const u32);
        let m2 = *(this.add(0x18) as *const u32);
        let m3 = *(this.add(0x1c) as *const u32);
        *(this.add(0x20) as *mut u32) = m0;
        *(this.add(0x24) as *mut u32) = m1;
        *(this.add(0x28) as *mut u32) = m2;
        *(this.add(0x2c) as *mut u32) = m3;
        m3
    }
});
