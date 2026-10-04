// original: 0x00985d70 audio_routing_table_find_or_insert
/// Look up `key` in the emitter routing table, inserting it when new.
///
/// Both out-parameters default to 0xFF. When the tag byte at `key+0x60` is
/// already assigned, its bucket is scanned for `key`: on a hit the slot
/// index is written to `out2`. When the tag is 0xFF and the table holds
/// fewer than 0x1FE0 entries, the allocator helper (cdecl/2, stubbed) runs,
/// `key` is appended to the current bucket with a 0xFFFF marker, and the
/// counters advance (moving to the next bucket once the current one holds
/// 0x20 entries). Returns an incidental register value the contract does
/// not compare (on the capacity-guard path it carries entry-register
/// garbage); all memory effects and the outgoing call are compared.
export!(thiscall, rw_00985d70(this: u32, key: u32, out1: u32, out2: u32) -> u32 {
    unsafe {
        const BUCKET: u32 = 0x104;
        const ENTRIES: u32 = 0x3a24c;
        const COUNT: u32 = 0x3a34c;
        const MARKS: u32 = 0x394;
        const TOTAL: u32 = 0x4a548;
        const CUR: u32 = 0x4a54c;
        const CAP: u32 = 0x1fe0;
        const FULL: u16 = 0x20;
        *(out1 as *mut u32) = 0xFF;
        *(out2 as *mut u32) = 0xFF;
        let tag = *((key.wrapping_add(0x60)) as *const u8);
        if tag != 0xFF {
            let idx = tag as u32;
            *(out1 as *mut u32) = idx;
            let base = this.wrapping_add(idx.wrapping_mul(BUCKET));
            let n = *((base.wrapping_add(COUNT)) as *const u16) as u32;
            let mut j = 0u32;
            while j < n {
                let e = *((base.wrapping_add(ENTRIES).wrapping_add(j * 8))
                    as *const u32);
                if e == key {
                    *(out2 as *mut u32) = j;
                    return j;
                }
                j += 1;
            }
            return j;
        }
        if *((this.wrapping_add(TOTAL)) as *const u32) >= CAP {
            return 0xFF;
        }
        let slot = *((this.wrapping_add(CUR)) as *const u32);
        callee_cdecl!(1, u32, key.wrapping_add(0x60), slot);
        *(out1 as *mut u32) = slot;
        let base = this.wrapping_add(slot.wrapping_mul(BUCKET));
        let n = *((base.wrapping_add(COUNT)) as *const u16);
        *(out2 as *mut u32) = n as u32;
        *((base.wrapping_add(COUNT)) as *mut u16) = n.wrapping_add(1);
        *((base.wrapping_add(ENTRIES).wrapping_add((n as u32) * 8))
            as *mut u32) = key;
        let mbase = this.wrapping_add(slot.wrapping_add(MARKS).wrapping_mul(BUCKET));
        *((mbase.wrapping_add((n as u32) * 8)) as *mut u16) = 0xFFFF;
        let t = (this.wrapping_add(TOTAL)) as *mut u32;
        *t = (*t).wrapping_add(1);
        if *((base.wrapping_add(COUNT)) as *const u16) == FULL {
            let c = (this.wrapping_add(CUR)) as *mut u32;
            *c = (*c).wrapping_add(1);
        }
        slot.wrapping_mul(BUCKET)
    }
});
