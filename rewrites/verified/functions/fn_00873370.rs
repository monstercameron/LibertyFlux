// original: 0x00873370 crmt_vector_grow_and_append

/// Growable vector of 20-byte records (`[this]` = data, `[this+4]` = count, `[this+6]` = capacity, all words): when count equals capacity, raise capacity by 16, allocate through 0x873410, copy the old records over, free the old block through the thread manager and install the new one; then hand out the slot at `data + count*20` and bump the count. Returns the new slot pointer.
///
/// Original: 0x00873370 (thiscall, one ignored stack word).
lf_checker_rt::export!(thiscall, rw_00873370(this: u32, _arg: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const GROW: u32 = 1;
    const REC_SIZE: u32 = 20;
    unsafe {
        let count = ((this + 4) as *const u16).read_unaligned();
        let cap = ((this + 6) as *const u16).read_unaligned();
        if count == cap {
            let newcap = cap.wrapping_add(0x10);
            ((this + 6) as *mut u16).write_unaligned(newcap);
            let newbuf: u32 = lf_checker_rt::callee_stdcall!(GROW, u32, newcap as u32);
            let old = (this as *const u32).read_unaligned();
            let mut i = 0u32;
            while i < count as u32 {
                let src = old + i * REC_SIZE;
                let dst = newbuf + i * REC_SIZE;
                (dst as *mut u64).write_unaligned((src as *const u64).read_unaligned());
                ((dst + 8) as *mut u64).write_unaligned(((src + 8) as *const u64).read_unaligned());
                ((dst + 16) as *mut u32).write_unaligned(((src + 16) as *const u32).read_unaligned());
                i += 1;
            }
            if old != 0 {
                let tls0 = lf_checker_rt::tls_slot(0);
            let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable as *const u8).add(FREE_SLOT as usize) as *const u32)
                .read_unaligned();
            let free_it: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            free_it(manager, old);
            }
            (this as *mut u32).write_unaligned(newbuf);
        }
        let n = ((this + 4) as *const u16).read_unaligned();
        let base = (this as *const u32).read_unaligned();
        ((this + 4) as *mut u16).write_unaligned(n.wrapping_add(1));
        base + (n as u32) * REC_SIZE
    }
});
