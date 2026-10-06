
// original: 0x00953BC0 txd_slot_setup (proposed)
/// Open a texture slot, hash two keys for it and publish the handles.
///
/// Calls the reset callee (id 1) on `this`, then resolves a slot id
/// through the find callee (id 2, passed the relocated name pointer
/// `TXD_NAME`); a slot of -1 returns -1 at once. Otherwise allocates a
/// 4-byte block (id 3, zeroed unless null) into `this + BLK_OFF` (0x14),
/// runs three notification callees (ids 4-6) on the slot, and takes a
/// context from id 7. It then hashes two keys (id 8, cdecl/2): the first
/// key is the caller's entry ESI, the second a misaligned word
/// overlapping the return address and `arg1` — both unknowable to this
/// rewrite, passed as 0 and skipped in the contract; the answers are
/// scripted and everything downstream is compared. Each hash is
/// resolved through id 9 (thiscall, this = context) into `this + H1_OFF`
/// (8) and `this + H2_OFF` (0xC); a non-null handle gets its reference
/// word at `REF_OFF` (0xA) incremented (wrapping). Returns the id 10
/// answer. Original is thiscall/2, returns EAX.
lf_checker_rt::export!(thiscall, rw_00953BC0(this: u32, _arg1: u32, _arg2: u32) -> u32 {
    const TXD_NAME: u32 = 0x00E8AC38;
    const SLOT_OFF: u32 = 0x18;
    const BLK_OFF: u32 = 0x14;
    const H1_OFF: u32 = 0x08;
    const H2_OFF: u32 = 0x0C;
    const REF_OFF: u32 = 0x0A;
    const NO_SLOT: u32 = 0xFFFFFFFF;
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        lf_checker_rt::callee_thiscall!(1, u32, this);
        let slot = lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(TXD_NAME));
        wr32(this.wrapping_add(SLOT_OFF), slot);
        if slot == NO_SLOT {
            return NO_SLOT;
        }
        let p = lf_checker_rt::callee_cdecl!(3, u32, 4);
        let blk = if p == 0 {
            0
        } else {
            (p as *mut u32).write_unaligned(0);
            p
        };
        wr32(this.wrapping_add(BLK_OFF), blk);
        let slot2 = rd32(this.wrapping_add(SLOT_OFF));
        lf_checker_rt::callee_cdecl!(4, u32, slot2);
        lf_checker_rt::callee_cdecl!(5, u32,);
        lf_checker_rt::callee_cdecl!(6, u32, slot2);
        let ctx = lf_checker_rt::callee_cdecl!(7, u32, slot2);
        let h1 = lf_checker_rt::callee_cdecl!(8, u32, 0, 0);
        let o1 = lf_checker_rt::callee_thiscall!(9, u32, ctx, h1);
        wr32(this.wrapping_add(H1_OFF), o1);
        let h2 = lf_checker_rt::callee_cdecl!(8, u32, 0, 0);
        let o2 = lf_checker_rt::callee_thiscall!(9, u32, ctx, h2);
        wr32(this.wrapping_add(H2_OFF), o2);
        let c1 = rd32(this.wrapping_add(H1_OFF));
        if c1 != 0 {
            let w = (c1.wrapping_add(REF_OFF) as *const u16).read_unaligned();
            (c1.wrapping_add(REF_OFF) as *mut u16).write_unaligned(w.wrapping_add(1));
        }
        let c2 = rd32(this.wrapping_add(H2_OFF));
        if c2 != 0 {
            let w = (c2.wrapping_add(REF_OFF) as *const u16).read_unaligned();
            (c2.wrapping_add(REF_OFF) as *mut u16).write_unaligned(w.wrapping_add(1));
        }
        lf_checker_rt::callee_cdecl!(10, u32,)
    }
});
