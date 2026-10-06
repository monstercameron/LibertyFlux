// original: 0x005ef2a0 tls_batch_release

/// Release every entry of a thread-local batch list, then clear its header.
///
/// `list` points at a header holding a base index word at `+0x00`, a remaining
/// count word at `+0x02`, and entry words from `+0x08`. While the count is
/// nonzero it is decremented and entry `(base + count) * 3` is released: callee 1
/// (virtual slot 4 of the thread-local allocator reached through TLS slot 0, called
/// with the constant 3) yields a worker object, and callee 2 (the worker's virtual
/// slot 3) is called on it with the entry word. Afterwards the header's first
/// dword is cleared. Returns 0.
///
/// Original: cdecl, one stack argument, two indirect callees, caller cleans up.
lf_checker_rt::export!(cdecl, rw_005ef2a0(list: u32) -> u32 {
    unsafe {
        const BASE: u32 = 0x00;
        const COUNT: u32 = 0x02;
        const ENTRIES: u32 = 0x08;
        const ALLOC_OFF: u32 = 0x08;
        const ALLOC_SLOT: u32 = 0x10;
        const WORKER_SLOT: u32 = 0x0C;
        const KIND_ARG: u32 = 3;
        const STRIDE: u32 = 3;
        let count = ((list + COUNT) as *const u16).read_unaligned();
        if count != 0 {
            let tls_block = lf_checker_rt::tls_slot(0);
            let alloc = ((tls_block + ALLOC_OFF) as *const u32).read_unaligned();
            let mut remaining = count;
            loop {
                remaining = remaining.wrapping_sub(1);
                ((list + COUNT) as *mut u16).write_unaligned(remaining);
                let base = ((list + BASE) as *const u16).read_unaligned() as u32;
                let index = (base + remaining as u32) * STRIDE;
                let entry = ((list + ENTRIES + index * 4) as *const u32).read_unaligned();
                let avt = ((alloc) as *const u32).read_unaligned();
                let aslot = ((avt + ALLOC_SLOT) as *const u32).read_unaligned();
                let acquire: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(aslot as usize);
                let worker = acquire(alloc, KIND_ARG);
                let wvt = ((worker) as *const u32).read_unaligned();
                let wslot = ((wvt + WORKER_SLOT) as *const u32).read_unaligned();
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(wslot as usize);
                release(worker, entry);
                if remaining == 0 {
                    break;
                }
            }
        }
        ((list) as *mut u32).write_unaligned(0);
        0
    }
});
