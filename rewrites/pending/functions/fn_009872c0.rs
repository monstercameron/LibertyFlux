// original: 0x009872c0 bucket_table_remove
/// Remove the entry `arg` from the table it lives in.
///
/// The entry's slot tag selects a bucket; a linear search finds the entry,
/// which is cleared and unlinked from a small side chain (releasing each
/// chained node: two notify calls when flagged, a bitset clear, a peer
/// release, and counter updates). The bucket is compacted by moving its last
/// entry into the freed slot, the counts are decremented, and when the slot
/// sits below the bucket high-water mark the last bucket page is swapped
/// into place and both affected buckets' entries are re-announced.
export!(thiscall, rw_009872C0(this: u32, arg: u32) -> () {
    unsafe {
        const SLOT_TAG: u32 = 0x60;
        const SLOT_EMPTY: u8 = 0xFF;
        const BUCKET_STRIDE: u32 = 0x104;
        const BUCKET_ENTRIES: u32 = 0x3A24C;
        const BUCKET_COUNT: u32 = 0x3A34C;
        const TABLE2_ROW: u32 = 0x394;
        const NODE_STRIDE: u32 = 0x60;
        const NODE_NEXT: u32 = 0x8292;
        const NODE_FLAGS: u32 = 0x8294;
        const NODE_FLAGS_CALL: u8 = 0x10;
        const NODE_PEER: u32 = 0x8268;
        const NODE_OWNER: u32 = 0x8264;
        const NODE_END: u16 = 0xFFFF;
        const BITSET_PTR: u32 = 0x38240;
        const LIVE_COUNT: u32 = 0x3A248;
        const HIGH_MARK: u32 = 0x4A548;
        const BUCKET_TOP: u32 = 0x4A54C;
        const NOTIFY_MGR: u32 = 0x1165880;
        const PAGE_WORDS: usize = 0x41;
        let slot = *((arg + SLOT_TAG) as *const u8);
        if slot == SLOT_EMPTY {
            return;
        }
        let page = (slot as u32).wrapping_mul(BUCKET_STRIDE);
        let bucket = this.wrapping_add(page);
        let count = ((bucket + BUCKET_COUNT) as *const u16).read_unaligned();
        let mut idx: u32 = 0;
        if count != 0 {
            while idx < count as u32
                && ((bucket + BUCKET_ENTRIES + idx * 8) as *const u32).read() != arg
            {
                idx += 1;
            }
        }
        if idx >= count as u32 {
            *((arg + SLOT_TAG) as *mut u8) = SLOT_EMPTY;
            return;
        }
        let entry = (bucket + BUCKET_ENTRIES + idx * 8) as *mut u32;
        *entry = 0;
        let cell =
            (this + (slot as u32 + TABLE2_ROW) * BUCKET_STRIDE + idx * 8) as *mut u16;
        let mut cur = cell.read_unaligned();
        cell.write_unaligned(NODE_END);
        if cur != NODE_END {
            loop {
                let sub = this.wrapping_add((cur as u32).wrapping_mul(NODE_STRIDE));
                if ((sub + NODE_FLAGS) as *const u8).read_unaligned() & NODE_FLAGS_CALL != 0
                {
                    let mut buf = [0u32; 4];
                    let _ = callee_thiscall!(1, u32, this, cur as u32, buf.as_mut_ptr() as u32);
                    let _ = callee_thiscall!(
                        2,
                        u32,
                        relocated(NOTIFY_MGR),
                        buf.as_mut_ptr() as u32
                    );
                }
                let bits = ((this + BITSET_PTR) as *const u32).read();
                let word = (bits + ((cur as u32) >> 5) * 4) as *mut u32;
                *word &= !(1u32 << ((cur as u32) & 31));
                let peer = ((sub + NODE_PEER) as *const u32).read();
                if peer != 0 {
                    let _ = callee_thiscall!(3, u32, peer, 0);
                }
                *((sub + NODE_OWNER) as *mut u32) = 0;
                let live = (this + LIVE_COUNT) as *mut u32;
                *live = live.read().wrapping_sub(1);
                let next = ((sub + NODE_NEXT) as *const u16).read_unaligned();
                ((sub + NODE_NEXT) as *mut u16).write_unaligned(NODE_END);
                cur = next;
                if cur == NODE_END {
                    break;
                }
            }
        }
        let count = ((bucket + BUCKET_COUNT) as *const u16).read_unaligned();
        if idx != (count as u32).wrapping_sub(1) {
            let last = (bucket
                + BUCKET_ENTRIES
                + (count as u32).wrapping_mul(8).wrapping_sub(8))
                as *const u32;
            *entry = last.read();
            *((entry as *mut u8).add(4) as *mut u32) =
                *((last as *const u8).add(4) as *const u32);
            *(last as *mut u32) = 0;
            *((last as *mut u8).add(4) as *mut u16) = NODE_END;
        }
        ((bucket + BUCKET_COUNT) as *mut u16).write_unaligned(count.wrapping_sub(1));
        let high = (this + HIGH_MARK) as *mut u32;
        *high = high.read().wrapping_sub(1);
        *((arg + SLOT_TAG) as *mut u8) = SLOT_EMPTY;
        let top = ((this + BUCKET_TOP) as *const u32).read();
        if (slot as u32) >= top {
            return;
        }
        let top = top.wrapping_sub(1);
        *((this + BUCKET_TOP) as *mut u32) = top;
        let last_page =
            (this + top.wrapping_mul(BUCKET_STRIDE) + BUCKET_ENTRIES) as *mut u32;
        let cur_page = (bucket + BUCKET_ENTRIES) as *mut u32;
        let mut temp = [0u32; PAGE_WORDS];
        for i in 0..PAGE_WORDS {
            temp[i] = *last_page.add(i);
        }
        for i in 0..PAGE_WORDS {
            *last_page.add(i) = *cur_page.add(i);
        }
        for i in 0..PAGE_WORDS {
            *cur_page.add(i) = temp[i];
        }
        let top = ((this + BUCKET_TOP) as *const u32).read();
        let n1 = ((this + top.wrapping_mul(BUCKET_STRIDE) + BUCKET_COUNT) as *const u16)
            .read_unaligned();
        let mut i: u32 = 0;
        while i < n1 as u32 {
            let top = ((this + BUCKET_TOP) as *const u32).read();
            let e = ((this + top.wrapping_mul(BUCKET_STRIDE) + i * 8 + BUCKET_ENTRIES)
                as *const u32)
                .read()
                .wrapping_add(SLOT_TAG);
            let _ = callee_cdecl!(4, u32, e, top);
            i += 1;
        }
        let curb = (bucket + BUCKET_ENTRIES) as *mut u32;
        let n2 = ((bucket + BUCKET_COUNT) as *const u16).read_unaligned();
        let mut p = curb as u32;
        let mut j: u32 = 0;
        while j < n2 as u32 {
            let e = ((p as *const u32).read()).wrapping_add(SLOT_TAG);
            let _ = callee_cdecl!(5, u32, e, slot as u32);
            p = p.wrapping_add(8);
            j += 1;
        }
    }
});
