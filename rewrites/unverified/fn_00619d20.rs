// original: 0x00619d20 skydome_resource_init (proposed)

/// Fill six resource slots of an owner object, then hand each to its tail
/// routine.
///
/// `this` points at the owner. Each block `k` owns one slot,
/// `SLOTS[k]` (`+0x2d4`, step 4), and one name, `NAMES[k]` (file VAs in
/// read-only data; the inventory's source-file hints point at sky textures).
/// When the slot is empty the block resolves the name through the lookup
/// callee (id 1: name in EDX, out-pointer to a stack spill in ECX, answer
/// address in EAX), swaps a differing answer into the slot with the usual
/// release-the-old / acquire-the-new refcount dance (count: 16-bit word at
/// `+0x0a`), then releases the spill temp: decrement its count, and when
/// the count reaches zero with a destroyable kind (byte at `+0x08` equal
/// to 2 or 4) call virtual slot 0 with argument 1 (id 9, planted stub).
/// The release-old call (id 2) is dead: the slot held zero on entry and
/// nothing between can change it, so the re-read old value is always zero
/// and the branch is never taken; it is kept for fidelity and exempted
/// from the call-coverage rule in the contract.
///
/// The tail then re-acquires every slot object (incrementing a nonzero
/// count, pushing even a null) and calls its tail routine (ids 3-8) with
/// `this + 0x10` in ECX and the object as one stack word; each tail pops
/// its word, which the six pushes with no intermediate cleanup require.
///
/// Original: thiscall, no stack words, caller cleanup (plain `ret`),
/// returns the last tail call's value. No globals are read (names pass as
/// immediates); no floating point.
lf_checker_rt::export!(thiscall, rw_00619d20(this: u32) -> u32 {
    unsafe {
        const SLOTS: [u32; 6] = [0x2d4, 0x2d8, 0x2dc, 0x2e0, 0x2e4, 0x2e8];
        const NAMES: [u32; 6] = [0xf95790, 0xf95758, 0xf95720, 0xf956ec, 0xf95a80, 0xf95a48];
        const TAILS: [u32; 6] = [3, 4, 5, 6, 7, 8];
        const REFCOUNT: u32 = 0x0a;
        const KIND: u32 = 0x08;
        const LOOKUP: u32 = 1;
        const RELEASE_OLD: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        // The original's spill slot initially holds the pushed `this`; the
        // lookup overwrites it before every read, so the value is carried
        // for fidelity only.
        let mut spill: u32 = this;
        for k in 0..6usize {
            if rd32(this + SLOTS[k]) == 0 {
                let found: u32 = lf_checker_rt::callee_fastcall!(
                    LOOKUP,
                    u32,
                    &mut spill as *mut u32 as u32,
                    lf_checker_rt::relocated(NAMES[k])
                );
                let new_obj = rd32(found);
                let old = rd32(this + SLOTS[k]);
                if new_obj != old {
                    if old != 0 {
                        lf_checker_rt::callee_thiscall!(RELEASE_OLD, u32, old);
                    }
                    wr32(this + SLOTS[k], new_obj);
                    if new_obj != 0 {
                        let rc = rd16(new_obj + REFCOUNT);
                        wr16(new_obj + REFCOUNT, rc.wrapping_add(1));
                    }
                }
                let temp = spill;
                if temp != 0 {
                    let rc = rd16(temp + REFCOUNT);
                    if rc != 0 {
                        wr16(temp + REFCOUNT, rc - 1);
                        let kind = rd8(temp + KIND);
                        let destroy = kind == 2 || kind == 4;
                        if rd16(temp + REFCOUNT) == 0 && destroy {
                            let release: extern "stdcall" fn(u32) -> u32 =
                                core::mem::transmute(rd32(temp) as usize);
                            release(1);
                        }
                    }
                }
            }
        }
        let mut ret: u32 = 0;
        for k in 0..6usize {
            let obj = rd32(this + SLOTS[k]);
            if obj != 0 {
                let rc = rd16(obj + REFCOUNT);
                wr16(obj + REFCOUNT, rc.wrapping_add(1));
            }
            ret = lf_checker_rt::callee_thiscall!(TAILS[k], u32, this + 0x10, obj);
        }
        ret
    }
});
