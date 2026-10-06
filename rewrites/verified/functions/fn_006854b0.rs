// original: 0x006854B0 track_set_update (proposed)

/// Refresh a track set, append up to two entries offered by a source object,
/// and finalise the set.
///
/// `this` points to the set (`+0x00` vtable, `+0x08` result slot,
/// `+0x0c` entry array, `+0x10` 16-bit entry count). `key` is a value whose
/// low word tags appended entries, the low byte of `mode` selects the
/// refresh path, and `source` (nullable) offers the entries.
///
/// Behaviour: when the mode byte is non-zero and the count is non-zero,
/// clear `+0x04`, call the reset hook (callee 1, thiscall/0), release the
/// old array through the context disposer (callee 2, thiscall/1 on the
/// object reached through the TLS slot) when it is non-null, then clear the
/// array and count and install a fresh two-slot array from the allocator
/// (callee 3, stdcall/1). Otherwise ask the grow hook (callee 4, thiscall/1
/// on the array field) for two slots and drop the last entry by moving it
/// onto itself after decrementing the count (wrapping 16-bit).
/// Then run two offer rounds (5 and 6): when the source is non-null, call
/// the source's offer hook (callee 5, thiscall/3, with a pointer to the
/// source-holding stack slot) and take the round only when its low byte
/// answers non-zero; when the source is null the offer is skipped but the
/// round is always taken. A taken round builds an entry through the set's
/// own factory (callee 6, thiscall/1, vtable slot 1), tags it (byte `+5`,
/// word `+6`), stores it through the grow hook's next slot and clears the
/// result slot. When the mode byte is zero, call the compact hook
/// (callee 7, thiscall/1; its argument is the stale register residue the
/// original happens to pass). Finally call the finalise hook (callee 8,
/// thiscall/0), store its answer in the result slot and return it.
///
/// All comparisons are equality-or-zero tests (mode byte, count word,
/// null checks, answer low bytes); the count decrement wraps mod 2^16.
/// Thiscall with three stack words.
lf_checker_rt::export!(thiscall, rw_006854B0(this: u32, key: u32, mode: u32, source: u32) -> u32 {
    unsafe {
        const SET_ARRAY: u32 = 0x0c;
        const SET_COUNT: u32 = 0x10;
        const FRESH_SLOTS: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u32) {
            unsafe { (a as *mut u16).write_unaligned(v as u16) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        unsafe fn take_round(this: u32, key: u32, tag: u8, factory_arg: u32) {
            unsafe {
                const SET_ARRAY: u32 = 0x0c;
                let make: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(this).wrapping_add(4)) as usize);
                let entry: u32 = make(this, factory_arg);
                wr8(entry.wrapping_add(5), tag);
                wr16(entry.wrapping_add(6), key & 0xffff);
                let slot: u32 = lf_checker_rt::callee_thiscall!(4, u32, this.wrapping_add(SET_ARRAY), 1);
                wr32(slot, entry);
                wr32(this.wrapping_add(8), 0);
            }
        }

        if (mode as u8) != 0 && rd16(this.wrapping_add(SET_COUNT)) != 0 {
            wr32(this.wrapping_add(4), 0);
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
            let array = rd32(this.wrapping_add(SET_ARRAY));
            if array != 0 {
                let ctx = lf_checker_rt::tls_slot(0);
                let disposer_obj = rd32(ctx.wrapping_add(8));
                let dispose: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(disposer_obj).wrapping_add(0x0c)) as usize);
                dispose(disposer_obj, array);
            }
            wr32(this.wrapping_add(SET_ARRAY), 0);
            wr32(this.wrapping_add(SET_COUNT), 0);
            let fresh: u32 = lf_checker_rt::callee_stdcall!(3, u32, FRESH_SLOTS);
            wr32(this.wrapping_add(SET_ARRAY), fresh);
            wr16(this.wrapping_add(SET_COUNT).wrapping_add(2), FRESH_SLOTS);
        } else {
            let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, this.wrapping_add(SET_ARRAY), FRESH_SLOTS);
            let count = rd16(this.wrapping_add(SET_COUNT));
            let array = rd32(this.wrapping_add(SET_ARRAY));
            let shrunken = count.wrapping_sub(1) & 0xffff;
            wr16(this.wrapping_add(SET_COUNT), shrunken);
            let last = rd32(array.wrapping_add(shrunken.wrapping_mul(4)));
            wr32(array.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4), last);
        }
        let frame_alias = source;
        let alias_ptr = &frame_alias as *const u32 as u32;
        let offer_addr = if source != 0 { rd32(rd32(source).wrapping_add(0x0c)) } else { 0 };
        let offer: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(offer_addr as usize);
        let take1 = if source != 0 { (offer(source, 5, key, alias_ptr) as u8) != 0 } else { true };
        if take1 {
            take_round(this, key, 5, 0);
        }
        let take2 = if source != 0 { (offer(source, 6, key, alias_ptr) as u8) != 0 } else { true };
        if take2 {
            take_round(this, key, 6, 1);
        }
        if (mode as u8) == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(7, u32, this, 0);
        }
        let answer: u32 = lf_checker_rt::callee_thiscall!(8, u32, this);
        wr32(this.wrapping_add(8), answer);
        answer
    }
});
