// original: 0x00d426a0 ambient_record_add (proposed)

/// Allocate a 36-byte ambient record, fill it, append it, mark its kind.
///
/// `this` points to the owner: table object at `+0x04`, 16-bit entry count at
/// `+0x08`, kind mask at `+0x28`. Allocates 36 bytes and fills the record:
/// `kind` at `+0x00`, `data` at `+0x04`, -1 at `+0x08`, 0 at `+0x0c`, -1 at
/// `+0x10`, byte 0 at `+0x14`, and the float -1.0 at `+0x18`. Appends the
/// record through the table helper (object `this+0x04`, size word 16). When
/// `kind` is below 27 (signed compare) sets bit `kind` in the mask at
/// `+0x28` (x86 shift-count masking applies). Returns the entry count minus
/// one, wrapping.
///
/// Original: thiscall, two stack words (kind, data), callee pops 8.
/// Allocator is cdecl/1, table helper thiscall/1 returning the slot.
lf_checker_rt::export!(thiscall, rw_00d426a0(this: u32, kind: u32, data: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const INSERT: u32 = 2;
        const REC_SIZE: u32 = 0x24;
        const TABLE_OFF: u32 = 4;
        const INSERT_ARG: u32 = 0x10;
        const COUNT_OFF: u32 = 8;
        const MASK_OFF: u32 = 0x28;
        const KIND_LIMIT: u32 = 0x1b;
        const NEG_ONE_F: u32 = 0xbf800000;
        let rec: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, REC_SIZE);
        (rec as *mut u32).write_unaligned(kind);
        ((rec.wrapping_add(4)) as *mut u32).write_unaligned(data);
        ((rec.wrapping_add(8)) as *mut u32).write_unaligned(0xffffffff);
        ((rec.wrapping_add(0x0c)) as *mut u32).write_unaligned(0);
        ((rec.wrapping_add(0x10)) as *mut u32).write_unaligned(0xffffffff);
        ((rec.wrapping_add(0x14)) as *mut u8).write(0);
        ((rec.wrapping_add(0x18)) as *mut u32).write_unaligned(NEG_ONE_F);
        let slot: u32 = lf_checker_rt::callee_thiscall!(
            INSERT,
            u32,
            this.wrapping_add(TABLE_OFF),
            INSERT_ARG
        );
        (slot as *mut u32).write_unaligned(rec);
        if (kind as i32) < (KIND_LIMIT as i32) {
            let bit = 1u32.wrapping_shl(kind);
            let m = ((this + MASK_OFF) as *const u32).read_unaligned();
            ((this + MASK_OFF) as *mut u32).write_unaligned(m | bit);
        }
        let c = ((this + COUNT_OFF) as *const u16).read_unaligned() as u32;
        c.wrapping_sub(1)
    }
});
