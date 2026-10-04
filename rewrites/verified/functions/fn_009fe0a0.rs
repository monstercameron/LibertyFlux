// original: 0x009FE0A0 frag_batch_refresh (proposed)

/// Refresh every flagged slot of a frag batch object.
///
/// Bails out unless both link dwords (`+0x34`, `+0x38`) are non-null and
/// neither points at a word of all-bits-set at `+8`. Otherwise runs the
/// setup callee on the object, then scans `count` (`+0x4a0`) slots of
/// 0xB0 bytes starting at `+0x125`: a slot whose flag byte has bit 1 set
/// has that bit cleared, and when bit 0 is then also set the slot's item
/// (0xA5 bytes below the flag) goes through the item callee and the touch
/// callee. The setup/item callees take uninitialised frame blocks, which
/// are passed as zeroed stand-ins and left uncompared. Returns nothing.
///
/// Original: 0x009FE0A0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_009FE0A0(arg: u32) -> u32 {
    unsafe {
        const LINK_A: u32 = 0x34;
        const LINK_B: u32 = 0x38;
        const TAG_OFF: u32 = 8;
        const ABSENT: u16 = 0xFFFF;
        const COUNT_OFF: u32 = 0x4A0;
        const SLOTS_OFF: u32 = 0x125;
        const SLOT_STRIDE: u32 = 0xB0;
        const ITEM_BACK: u32 = 0xA5;
        const DIRTY_BIT: u8 = 2;
        const TOUCH_BIT: u8 = 1;
        let a = ((arg + LINK_A) as *const u32).read_unaligned();
        if a == 0 || ((a + TAG_OFF) as *const u16).read_unaligned() == ABSENT {
            return 0;
        }
        let b = ((arg + LINK_B) as *const u32).read_unaligned();
        if b == 0 || ((b + TAG_OFF) as *const u16).read_unaligned() == ABSENT {
            return 0;
        }
        let blk_a = [0u32; 8];
        let blk_b = [0u32; 8];
        lf_checker_rt::callee_thiscall!(1, u32, arg, blk_a.as_ptr() as u32, blk_b.as_ptr() as u32);
        let count = ((arg + COUNT_OFF) as *const i32).read_unaligned();
        if count <= 0 {
            return 0;
        }
        let blk_c = [0u32; 8];
        let blk_d = [0u32; 8];
        let mut i = 0;
        while i < count {
            let flag_at = arg + SLOTS_OFF + (i as u32) * SLOT_STRIDE;
            let mut f = (flag_at as *const u8).read();
            if f & DIRTY_BIT != 0 {
                f &= !DIRTY_BIT;
                (flag_at as *mut u8).write(f);
                if f & TOUCH_BIT != 0 {
                    let item = flag_at - ITEM_BACK;
                    lf_checker_rt::callee_thiscall!(2, u32, item,
                        blk_c.as_ptr() as u32, blk_d.as_ptr() as u32);
                    lf_checker_rt::callee_cdecl!(3, u32, item);
                }
            }
            i += 1;
        }
        0
    }
});
