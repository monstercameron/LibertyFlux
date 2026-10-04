// original: 0x00b24ef0 Entity_IsTouchingEntity
// s08_b24ef0 (Entity_IsTouchingEntity): touching-list membership.
// thiscall/1 (other: u32) -> u8: when the touching-list enable bit
// (bit 4 of the dword at +0x118) is set, scans the entity list at +0x154
// (count byte at +0x150) for `other`. Returns 1/0 in AL.
export!(thiscall, rw_b24ef0(this: *mut u8, other: u32) -> u8 {
    unsafe {
        if (*(this.add(0x118) as *const u32) >> 4) & 1 == 0 {
            return 0;
        }
        let count = *this.add(0x150);
        if count == 0 {
            return 0;
        }
        let list = this.add(0x154) as *const u32;
        let mut i = 0u32;
        while i < count as u32 {
            if *list.add(i as usize) == other {
                return 1;
            }
            i += 1;
        }
        0
    }
});
