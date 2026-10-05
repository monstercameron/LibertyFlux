// original: 0x008748F0 crmt_lookup_and_bind

/// Bind a looked-up record: resolve `idx` through the 0x872b20 lookup into `[this]`, allocate a 0x14-byte block, and unless allocation failed combine the block with the record's key word at +0x14 and `arg1` through 0x605aa0 into `[this+4]` (zero when allocation failed). Returns `this`.
///
/// Original: 0x008748F0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_008748f0(this: u32, idx: u32, arg1: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const ALLOC_SLOT: u32 = 8;
    const LOOKUP: u32 = 1;
    const BIND: u32 = 3;
    unsafe {
        (this as *mut u32).write_unaligned(0);
        let t: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32, idx);
        (this as *mut u32).write_unaligned(t);
        let tls0 = lf_checker_rt::tls_slot(0);
        let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
        let vtable = (manager as *const u32).read_unaligned();
        let target = ((vtable as *const u8).add(ALLOC_SLOT as usize) as *const u32)
            .read_unaligned();
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let fresh = alloc(manager, 0x14, 0x10, 0);
        if fresh == 0 {
            ((this + 4) as *mut u32).write_unaligned(0);
        } else {
            let key = ((t + 0x14) as *const u32).read_unaligned();
            let r: u32 = lf_checker_rt::callee_thiscall!(BIND, u32, fresh, key, arg1);
            ((this + 4) as *mut u32).write_unaligned(r);
        }
        this
    }
});
