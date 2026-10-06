// original: 0x0087c940 crmt_cert_name_resolve (proposed)
/// Resolve-and-cache a name through the imported name service.
///
/// Queries the imported name service (intercepted callee 1, six stack
/// arguments: this, 2, 0, out-local, 0, 0) and returns 0 when it answers
/// null. Otherwise allocates a record through the TLS heap (slot 0,
/// allocator at `[slot]+8`, slot 8 of its table with the query answer, 0x10
/// and 0), re-queries with (this, 2, 0, out-local, record, answer), and
/// returns the record. The out-local is the function's own saved-ecx stack
/// slot, reused after the save (the epilogue restores garbage, discarded);
/// it is pre-filled with 3 and its contents are never read back, only
/// re-passed as an out-pointer. The second query carries the allocator
/// stub's ecx residue (0) since the original reloads no register between
/// the calls. All comparisons are null checks.
///
/// Original: thiscall/0, one import (two sites) plus one indirect call.
export!(thiscall, rw_0087c940(this: u32) -> u32 {
    /// Fabricated TLS slot holding the heap anchor.
    const TLS_SLOT: usize = 0;
    /// Allocator slot in the heap object's table.
    const ALLOC_SLOT: u32 = 8;
    unsafe {
        // The original reuses its saved-ecx slot as the out-local and
        // pre-fills it with 3 (clobbering the save, harmlessly); the sixth
        // query argument is the untouched push-0 slot, i.e. 0.
        let mut local = 3u32;
        let r1 = callee_thiscall!(1, u32, this, this, 2, 0,
            &mut local as *mut u32 as u32, 0, 0);
        if r1 == 0 {
            return 0;
        }
        let tls = tls_slot(TLS_SLOT);
        let alloc = ((tls + 8) as *const u32).read_unaligned();
        let vt = (alloc as *const u32).read_unaligned();
        let tgt = ((vt + ALLOC_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let r2 = f(alloc, r1, 0x10, 0);
        // The original reloads no register after the allocator call, so its
        // second query carries the stub's ecx residue (0, the sequence step);
        // the rewrite forwards the same value.
        callee_thiscall!(1, u32, 0, this, 2, 0,
            &mut local as *mut u32 as u32, r2, r1);
        r2
    }
});
