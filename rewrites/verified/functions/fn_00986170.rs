// original: 0x00986170 HOSPITALS_POLICE_SCANNER
/// Rebuild the manager's record table from the list provider.
///
/// On the first call (flag byte clear) the flag is set, the record count is
/// cleared, and up to 99 scan passes run: each pass asks the list provider
/// for the current item list and copies every returned item's header plus a
/// few derived fields into the next free record slot. Items carrying a tag
/// bit also append their slot number to a small global index. A one-time
/// lookup result is cached in globals and compared against each item's mark
/// word; a finalizer call runs once the scan ends. Later calls return
/// immediately since the flag is set.
///
/// The original keeps a stack-canary cookie; that is a build artifact with
/// no observable behaviour and is intentionally not reproduced.
export!(thiscall, rw_00986170(this: u32) -> () {
    unsafe {
        const FLAG_OFF: u32 = 0x4_A551;
        const COUNT_OFF: u32 = 0x8230;
        const REC_BASE: u32 = 0x90;
        const REC_STRIDE: u32 = 0xD0;
        const REC_MAX: u32 = 0xA0;
        const OUTER_ITERS: u8 = 0x63;
        const LIST_MGR: u32 = 0x115D_9A0;
        const SETUP_TAG: u32 = 0xE8_E0C4;
        const INIT_TAG: u32 = 0xE8_E0CC;
        const G_SETUP_ARG: u32 = 0x103_8A88;
        const G_INDEX_COUNT: u32 = 0x123_8950;
        const G_INDEX_BASE: u32 = 0x123_8958;
        const G_INDEX_MAX: u32 = 0x1F;
        const G_INIT_ID: u32 = 0x128_2F44;
        const G_INIT_DONE: u32 = 0x128_2F48;
        let mgr = this as *mut u8;
        if *mgr.add(FLAG_OFF as usize) != 0 {
            return;
        }
        *mgr.add(FLAG_OFF as usize) = 1;
        let count_ptr = (this + COUNT_OFF) as *mut u32;
        *count_ptr = 0;
        let mut outer: u8 = 0;
        loop {
            // The setup call's output word is never read back (the inner
            // loop's entry slot overlaps it on the original's frame), so any
            // stack slot serves as the buffer.
            let mut setup_out: u32 = 0;
            let setup_arg = *(relocated(G_SETUP_ARG) as *const u32);
            let _ = callee_cdecl!(
                1,
                u32,
                &mut setup_out as *mut u32 as u32,
                relocated(SETUP_TAG),
                setup_arg,
                outer as u32 + 1
            );
            let mut scratch = [0u32; 4];
            let list_mgr = relocated(LIST_MGR);
            let list = callee_thiscall!(2, u32, list_mgr, scratch.as_mut_ptr() as u32);
            if list == 0 {
                break;
            }
            let mut inner: u8 = 0;
            loop {
                let bound = ((list + 0xA) as *const u16).read_unaligned();
                if (inner as u16) >= bound {
                    break;
                }
                let entry = ((list + (inner as u32) * 4 + 0xC) as *const u32).read();
                let item = callee_thiscall!(3, u32, list_mgr, entry);
                if item != 0 {
                    let count = *count_ptr;
                    if count < REC_MAX {
                        let base = this.wrapping_add(count.wrapping_mul(REC_STRIDE));
                        let dst = (base + REC_BASE) as *mut u32;
                        let src = item as *const u32;
                        for i in 0..14usize {
                            *dst.add(i) = *src.add(i);
                        }
                        let flags = ((item + 5) as *const u8).read_unaligned();
                        *((base + 0xF5) as *mut u8) = ((flags & 3) != 0) as u8;
                        // The setup-output slot overlaps the pushed entry on
                        // the original's frame, so this stores the entry.
                        *((base + 0xD8) as *mut u32) = entry;
                        *((base + 0x70) as *mut u32) = item;
                        *((base + 0x30) as *mut u32) =
                            ((item + 0x12) as *const u32).read_unaligned();
                        *((base + 0x34) as *mut u32) =
                            ((item + 0x16) as *const u32).read_unaligned();
                        *((base + 0x38) as *mut u32) =
                            ((item + 0x1A) as *const u32).read_unaligned();
                        *((base + 0xD4) as *mut u32) = 0;
                        let tag = ((item + 5) as *const u8).read_unaligned();
                        if (tag & 0x30) == 0x10 {
                            let g = *(relocated(G_INDEX_COUNT) as *const u32);
                            if g < G_INDEX_MAX {
                                *((relocated(G_INDEX_BASE) + g * 4) as *mut u32) = count;
                                *(relocated(G_INDEX_COUNT) as *mut u32) = g + 1;
                                *((base + 0xF2) as *mut u8) = 1;
                                *((base + 0xE4) as *mut u32) = 0;
                            }
                        }
                        let done = *(relocated(G_INIT_DONE) as *const u32);
                        let init_id = if (done & 1) == 0 {
                            let r = callee_cdecl!(4, u32, relocated(INIT_TAG), 0);
                            *(relocated(G_INIT_DONE) as *mut u32) = done | 1;
                            *(relocated(G_INIT_ID) as *mut u32) = r;
                            r
                        } else {
                            *(relocated(G_INIT_ID) as *const u32)
                        };
                        let mark = ((item + 0xA) as *const u32).read_unaligned();
                        *((base + 0xF4) as *mut u8) = (mark == init_id) as u8;
                        *((base + 0xE0) as *mut u32) = 0;
                        let r5 =
                            callee_thiscall!(5, u32, list_mgr, *((item + 0x30) as *const u32));
                        *((base + 0xE8) as *mut u32) = r5;
                        *((base + 0xEC) as *mut u32) = *((item + 0x34) as *const u32);
                        *count_ptr = count + 1;
                    }
                }
                inner = inner.wrapping_add(1);
            }
            outer = outer.wrapping_add(1);
            if outer >= OUTER_ITERS {
                break;
            }
        }
        let _ = callee_thiscall!(6, u32, this);
    }
});
