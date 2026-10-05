// original: 0x00c0a9c0 stream_drain_all (proposed)

/// Drain every bucket chain and both queues, then release the queues.
///
/// `this` points to the set (the one stack word is ignored). Each bucket
/// below the global bound at `BOUND` whose head slot (at `HEADS` plus bucket
/// times `STRIDE`, link at `LINK`) is set has its node chain walked: every
/// node goes to the detacher (callee 1) with the global owner and its head
/// slot address. The overflow chain at `OVF` past `this` is walked the same
/// way with `this` and the slot at `OVFS`. The concluded (callee 2) runs with
/// the owner bias in `ecx`, then both queue buffers (at `Q0` and `Q1`) go to
/// the release helper (callee 3) and the four queue fields are cleared.
/// Returns 0.
///
/// Original: 0x00c0a9c0 (thiscall, one ignored stack word; 1-2 thiscall).
lf_checker_rt::export!(thiscall, rw_00c0a9c0(this: u32, _ignored: u32) -> u32 {
    unsafe {
        const BOUND: u32 = 0x16c8fb4;
        const HEADS: u32 = 0x16c8ff0;
        const STRIDE: u32 = 0x44;
        const LINK: u32 = 0x04;
        const OWNER: u32 = 0x1683290;
        const OVF: u32 = 0x3018;
        const OVFS: u32 = 0x3014;
        const OWNER_BIAS: u32 = 0x3008;
        const Q0: u32 = 0x5224;
        const Q0N: u32 = 0x5228;
        const Q1: u32 = 0x522c;
        const Q1N: u32 = 0x5230;
        const DETACH: u32 = 1;
        const CONCLUDE: u32 = 2;
        const FREE: u32 = 3;
        let bound = (lf_checker_rt::relocated(BOUND) as *const u32).read_unaligned();
        if (bound as i32) > 0 {
            let mut b = 0u32;
            while (b as i32) < (bound as i32) {
                let slot = lf_checker_rt::relocated(HEADS).wrapping_add(b.wrapping_mul(STRIDE));
                let mut node =
                    ((slot.wrapping_add(LINK)) as *const u32).read_unaligned();
                if node != 0 {
                    loop {
                        let next = (node as *const u32).read_unaligned();
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            DETACH, u32, lf_checker_rt::relocated(OWNER), node, slot);
                        node = next;
                        if next == 0 {
                            break;
                        }
                    }
                }
                b = b.wrapping_add(1);
            }
        }
        let mut node = (this.wrapping_add(OVF) as *const u32).read_unaligned();
        if node != 0 {
            loop {
                let next = (node as *const u32).read_unaligned();
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    DETACH, u32, this, node, this.wrapping_add(OVFS));
                node = next;
                if next == 0 {
                    break;
                }
            }
        }
        let _: u32 =
            lf_checker_rt::callee_thiscall!(CONCLUDE, u32, this.wrapping_add(OWNER_BIAS));
        let q0 = (this.wrapping_add(Q0) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_cdecl!(FREE, u32, q0);
        (this.wrapping_add(Q0) as *mut u32).write_unaligned(0);
        (this.wrapping_add(Q0N) as *mut u32).write_unaligned(0);
        let q1 = (this.wrapping_add(Q1) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_cdecl!(FREE, u32, q1);
        (this.wrapping_add(Q1) as *mut u32).write_unaligned(0);
        (this.wrapping_add(Q1N) as *mut u32).write_unaligned(0);
        0
    }
});
