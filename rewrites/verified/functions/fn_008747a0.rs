// original: 0x008747A0 crmt_indexed_slot_store

/// Store one entry into the table at `[this]`: allocate an 8-byte scratch block; for a small index (below 0x80) build the entry through 0x8748f0, or zero when allocation failed, and store it at `table + idx*4`, returning the table; for a large index resolve the destination cell through 0x43e700 (whose pushed context word is callee-residue the checker cannot reproduce, so it is skipped) and store the built entry, or zero, there, returning the cell.
///
/// Original: 0x008747A0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_008747a0(this: u32, idx: u32, arg1: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const ALLOC_SLOT: u32 = 8;
    const BUILD: u32 = 2;
    const RESOLVE: u32 = 3;
    const BIG_INDEX: i32 = 0x80;
    unsafe {
        let table = (this as *const u32).read_unaligned();
        let tls0 = lf_checker_rt::tls_slot(0);
        let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
        let vtable = (manager as *const u32).read_unaligned();
        let target = ((vtable as *const u8).add(ALLOC_SLOT as usize) as *const u32)
            .read_unaligned();
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let fresh = alloc(manager, 8, 0x10, 0);
        if (idx as i32) < BIG_INDEX {
            if fresh != 0 {
                let r: u32 = lf_checker_rt::callee_thiscall!(BUILD, u32, fresh, idx, arg1);
                ((table + idx * 4) as *mut u32).write_unaligned(r);
            } else {
                ((table + idx * 4) as *mut u32).write_unaligned(0);
            }
            table
        } else if fresh != 0 {
            let r: u32 = lf_checker_rt::callee_thiscall!(BUILD, u32, fresh, idx, arg1);
            let slot: u32 = lf_checker_rt::callee_thiscall!(RESOLVE, u32, this, 0);
            (slot as *mut u32).write_unaligned(r);
            slot
        } else {
            let slot: u32 = lf_checker_rt::callee_thiscall!(RESOLVE, u32, this, 0);
            (slot as *mut u32).write_unaligned(0);
            slot
        }
    }
});
