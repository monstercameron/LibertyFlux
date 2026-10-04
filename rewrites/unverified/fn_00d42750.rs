// original: 0x00d42750 timed_clip_append (proposed)

/// Append an entry to the timed-clip arrays with a shared count.
///
/// `this` points to the owner: dword at `+0x280` is the count, dwords at
/// `+0x200..+0x23c` are flag slots, dwords at `+0x240..+0x27c` are weight
/// slots. If the count has reached 16 the function only returns the count.
/// Otherwise it calls the fill helper with the entry address (`this + count *
/// 32`) and `key`, re-reads the count twice (mirroring the original), stores
/// -1 in the flag slot and the `key` bits in the weight slot at the count
/// index, increments the count, and returns the pre-increment count.
///
/// Note the second stack word is never read by the original: the value stored
/// into the weight slot is the first word (`key`), moved through a vector
/// register as a pure bit copy. The rewrite keeps the unread parameter so the
/// signature and stack cleanup match. The helper is cdecl/2 taking
/// (entry, key) and is intercepted.
///
/// Original: thiscall, two stack words, callee pops 8.
lf_checker_rt::export!(thiscall, rw_00d42750(this: u32, key: u32, _unused: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x280;
        const CAP: u32 = 0x10;
        const ENTRY_STRIDE: u32 = 32;
        const FLAG_TABLE: u32 = 0x200;
        const WEIGHT_TABLE: u32 = 0x240;
        const FLAG_SET: u32 = 0xffffffff;
        const FILL: u32 = 1;
        let n = ((this + COUNT) as *const u32).read_unaligned();
        if n >= CAP {
            return n;
        }
        let entry = this.wrapping_add(n.wrapping_mul(ENTRY_STRIDE));
        let _: u32 = lf_checker_rt::callee_cdecl!(FILL, u32, entry, key);
        let m = ((this + COUNT) as *const u32).read_unaligned();
        ((this + FLAG_TABLE + m.wrapping_mul(4)) as *mut u32).write_unaligned(FLAG_SET);
        let k = ((this + COUNT) as *const u32).read_unaligned();
        ((this + WEIGHT_TABLE + k.wrapping_mul(4)) as *mut u32).write_unaligned(key);
        let c = ((this + COUNT) as *const u32).read_unaligned();
        ((this + COUNT) as *mut u32).write_unaligned(c.wrapping_add(1));
        k
    }
});
