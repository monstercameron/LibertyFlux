// original: 0x00a96d10 filemem_repack_slot_chain

/// Repack every live slot in the chain, stamping idle ones zero.
///
/// `this` points to an object whose word at `+0x8ec50` heads a chain of
/// slot nodes linked through their first word. A node whose slot index at
/// `+0x64` tables to a null entry in the table at `0x01295cd8` is idle:
/// its mark at `+0x74` is cleared. Otherwise the probe callee (handed 8
/// then 0) must answer non-null, and the fetch callee (handed the probe
/// answer and the slot index) names the repack object; the measure callee
/// (its vtable slot `+0x8`, asked twice) yields samples combined with
/// SIGNED 16-based modular arithmetic — the first sample modulo 16 (signed
/// remainder), a rotation of 16 minus that modulo 16, the second sample
/// plus the rotation divided by 16 (signed, truncating) times 0x4000 —
/// and the result is merged into the object's word at `+0x4` keeping only
/// bits 0x1ffc000: `new = old ^ ((t ^ old) & mask)`.
///
/// Original: 0x00A96D10 (thiscall, no stack arguments; two direct callees,
/// one indirect callee asked twice).
lf_checker_rt::export!(thiscall, rw_00a96d10(this: u32) -> u32 {
    unsafe {
        /// Head of the slot-node chain, from the object base.
        const LIST_OFF: u32 = 0x8ec50;
        /// Next link / slot index / idle mark, from a node.
        const NODE_NEXT: u32 = 0x00;
        const NODE_SLOT: u32 = 0x64;
        const NODE_MARK: u32 = 0x74;
        /// Liveness table (file VA) and measure machinery.
        const TABLE: u32 = 0x01295cd8;
        const VT_MEASURE: u32 = 0x8;
        const ACCUM_OFF: u32 = 0x4;
        const MOD_MASK: u32 = 0x01ffc000;
        const PROBE: u32 = 1;
        const FETCH: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let mut node = rd32(this.wrapping_add(LIST_OFF));
        if node == 0 {
            return 0;
        }
        loop {
            let next = rd32(node.wrapping_add(NODE_NEXT));
            let slot = rd32(node.wrapping_add(NODE_SLOT));
            let ent = rd32(lf_checker_rt::relocated(TABLE).wrapping_add(slot.wrapping_mul(4)));
            if ent == 0 {
                wr32(node.wrapping_add(NODE_MARK), 0);
            } else {
                let p: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32, 8, 0);
                let mut obj: u32 = 0;
                if p != 0 {
                    obj = lf_checker_rt::callee_thiscall!(FETCH, u32, p, slot);
                }
                let mslot = rd32(rd32(obj).wrapping_add(VT_MEASURE));
                let measure: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(mslot as usize);
                let m1 = (measure(obj) as i32) % 16;
                let rot = (16 - m1) % 16;
                let m2 = (measure(obj) as i32).wrapping_add(rot);
                let t = (m2 / 16).wrapping_mul(0x4000) as u32;
                let f = rd32(obj.wrapping_add(ACCUM_OFF));
                wr32(obj.wrapping_add(ACCUM_OFF), f ^ ((t ^ f) & MOD_MASK));
            }
            node = next;
            if node == 0 {
                break;
            }
        }
        0
    }
});
