// original: 0x00684BA0 track_list_merge (proposed)

/// Merge a source element list into a track set, offering each element to a
/// sink object when one is given, and finalise the set.
///
/// `this` points to the set (`+0x00` vtable, `+0x08` result slot,
/// `+0x0c` entry array, `+0x10` 16-bit entry count). `source` points to the
/// element list (`+0x0c` pointer array, `+0x10` 16-bit length N); each
/// element carries a kind byte at `+4` (low nibble used), a tag byte at
/// `+5` and an index word at `+6`. The low byte of `mode` selects the
/// refresh path and the loop family; `sink` (nullable) selects the offer
/// loops.
///
/// Behaviour: when the mode byte is non-zero and the set count is non-zero,
/// clear `+0x04`, call the reset hook (callee 1, thiscall/0), release the
/// old array through the context disposer (callee 2, thiscall/1 on the
/// object reached through the TLS slot) when non-null, then clear the
/// array and count (one dword) and, when N is non-zero, install a fresh
/// N-slot array from the allocator (callee 3, stdcall/1). Otherwise, when
/// N is positive, ask the grow hook (callee 4, thiscall/1 on the array
/// field) for N slots and drop the last entry by moving it onto itself
/// after decrementing the count (wrapping 16-bit).
/// Then run one of four loops over the N elements. With a non-zero mode
/// byte and a sink, offer each element to the sink (callee 5, thiscall/3
/// with the mode word whose low byte carries the element tag, the index,
/// and a frame pointer) and, on a non-zero low-byte answer, build an entry
/// through the set factory (callee 6, thiscall/1, vtable slot 1) with N
/// whose low byte carries the kind, tag it, store it through the grow
/// hook's next slot and clear the result slot. With a non-zero mode byte
/// and no sink, do the same without the offer, passing the bare kind to
/// the factory. With a zero mode byte the loops are the same except the
/// factory always gets the bare kind. Finally, with a zero mode byte call
/// the compact hook (callee 7, thiscall/1, stale-argument residue), then
/// call the finalise hook (callee 8, thiscall/0), store its answer in the
/// result slot and return it.
///
/// All bounds are signed but never negative (lengths, indices, counts);
/// every other comparison tests equality or zero. The original keeps its
/// loop counter and the tag-scribbled mode word in its own incoming stack
/// slots, so the stack comparison is off; the counter is observed in the
/// factory/grow call counts and the tags in the offer arguments.
/// Thiscall with three stack words.
lf_checker_rt::export!(thiscall, rw_00684BA0(this: u32, source: u32, mode: u32, sink: u32) -> u32 {
    unsafe {
        const SET_ARRAY: u32 = 0x0c;
        const SET_COUNT: u32 = 0x10;
        const SRC_ARRAY: u32 = 0x0c;
        const SRC_COUNT: u32 = 0x10;
        const KIND_MASK: u8 = 0x0f;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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

        unsafe fn dispose_array(array: u32) {
            unsafe {
                let ctx = lf_checker_rt::tls_slot(0);
                let disposer_obj = rd32(ctx.wrapping_add(8));
                let dispose: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(disposer_obj).wrapping_add(0x0c)) as usize);
                dispose(disposer_obj, array);
            }
        }

        unsafe fn grow_entry(this: u32, tag: u8, idx: u32, factory_arg: u32) {
            unsafe {
                const SET_ARRAY: u32 = 0x0c;
                let make: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(this).wrapping_add(4)) as usize);
                let entry: u32 = make(this, factory_arg);
                wr8(entry.wrapping_add(5), tag);
                wr16(entry.wrapping_add(6), idx);
                let slot: u32 = lf_checker_rt::callee_thiscall!(4, u32, this.wrapping_add(SET_ARRAY), 1);
                wr32(slot, entry);
                wr32(this.wrapping_add(8), 0);
            }
        }

        let total = rd16(source.wrapping_add(SRC_COUNT));
        if (mode as u8) != 0 && rd16(this.wrapping_add(SET_COUNT)) != 0 {
            wr32(this.wrapping_add(4), 0);
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
            let array = rd32(this.wrapping_add(SET_ARRAY));
            if array != 0 {
                dispose_array(array);
            }
            wr32(this.wrapping_add(SET_ARRAY), 0);
            wr32(this.wrapping_add(SET_COUNT), 0);
            if total != 0 {
                let fresh: u32 = lf_checker_rt::callee_stdcall!(3, u32, total);
                wr32(this.wrapping_add(SET_ARRAY), fresh);
                wr16(this.wrapping_add(SET_COUNT).wrapping_add(2), total);
            } else {
                wr32(this.wrapping_add(SET_ARRAY), 0);
                wr16(this.wrapping_add(SET_COUNT).wrapping_add(2), 0);
            }
        } else if (total as i32) > 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, this.wrapping_add(SET_ARRAY), total);
            let count = rd16(this.wrapping_add(SET_COUNT));
            let array = rd32(this.wrapping_add(SET_ARRAY));
            let shrunken = count.wrapping_sub(1) & 0xffff;
            wr16(this.wrapping_add(SET_COUNT), shrunken);
            let last = rd32(array.wrapping_add(shrunken.wrapping_mul(4)));
            wr32(array.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4), last);
        }
        if (mode as u8) != 0 {
            if sink != 0 {
                let offer_addr = rd32(rd32(sink).wrapping_add(0x0c));
                let offer: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(offer_addr as usize);
                let mut k = 0u32;
                while (k as i32) < (total as i32) {
                    let elem = rd32(rd32(source.wrapping_add(SRC_ARRAY)).wrapping_add(k.wrapping_mul(4)));
                    let kind = (rd8(elem.wrapping_add(4)) & KIND_MASK) as u32;
                    let tag = rd8(elem.wrapping_add(5)) as u32;
                    let idx = rd16(elem.wrapping_add(6));
                    let frame_words = [idx, 0];
                    let answer: u32 = offer(sink, (mode & !0xff) | tag, idx, &frame_words[1] as *const u32 as u32);
                    if (answer as u8) != 0 {
                        grow_entry(this, tag as u8, idx, (total & !0xff) | kind);
                    }
                    k = k.wrapping_add(1);
                }
            } else {
                let mut k = 0u32;
                while (k as i32) < (total as i32) {
                    let elem = rd32(rd32(source.wrapping_add(SRC_ARRAY)).wrapping_add(k.wrapping_mul(4)));
                    let kind = (rd8(elem.wrapping_add(4)) & KIND_MASK) as u32;
                    let tag = rd8(elem.wrapping_add(5));
                    let idx = rd16(elem.wrapping_add(6));
                    grow_entry(this, tag, idx, kind);
                    k = k.wrapping_add(1);
                }
            }
        } else if sink != 0 {
            let offer_addr = rd32(rd32(sink).wrapping_add(0x0c));
            let offer: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(offer_addr as usize);
            let mut k = 0u32;
            while (k as i32) < (total as i32) {
                let elem = rd32(rd32(source.wrapping_add(SRC_ARRAY)).wrapping_add(k.wrapping_mul(4)));
                let kind = (rd8(elem.wrapping_add(4)) & KIND_MASK) as u32;
                let tag = rd8(elem.wrapping_add(5)) as u32;
                let idx = rd16(elem.wrapping_add(6));
                let frame_words = [kind, 0];
                let answer: u32 = offer(sink, (mode & !0xff) | tag, idx, &frame_words[1] as *const u32 as u32);
                if (answer as u8) != 0 {
                    grow_entry(this, tag as u8, idx, kind);
                }
                k = k.wrapping_add(1);
            }
        } else {
            let mut k = 0u32;
            while (k as i32) < (total as i32) {
                let elem = rd32(rd32(source.wrapping_add(SRC_ARRAY)).wrapping_add(k.wrapping_mul(4)));
                let kind = (rd8(elem.wrapping_add(4)) & KIND_MASK) as u32;
                let tag = rd8(elem.wrapping_add(5));
                let idx = rd16(elem.wrapping_add(6));
                grow_entry(this, tag, idx, kind);
                k = k.wrapping_add(1);
            }
        }
        if (mode as u8) == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(7, u32, this, 0);
        }
        let answer: u32 = lf_checker_rt::callee_thiscall!(8, u32, this);
        wr32(this.wrapping_add(8), answer);
        answer
    }
});
