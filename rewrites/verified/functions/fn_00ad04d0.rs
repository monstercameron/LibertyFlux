// original: 0x00ad04d0 accumulate_and_clear_140
/// Add the argument onto the field at 0x140 when flag bit 12 is set.
///
/// On the taken path flag bits 20 and 21 are cleared, 0x144 is zeroed and
/// the sum is stored back to 0x140. Returns the flag word (masked on the
/// taken path).
export!(thiscall, rw_00ad04d0(this: *mut u8, add: f32) -> u32 {
    unsafe {
        let flags = *(this.add(0x164) as *const u32);
        if flags & 0x1000 == 0 {
            return flags;
        }
        let cur = *(this.add(0x140) as *const f32);
        let f = flags & !0x300000;
        *(this.add(0x164) as *mut u32) = f;
        *(this.add(0x144) as *mut u32) = 0;
        *(this.add(0x140) as *mut f32) = cur + add;
        f
    }
});
