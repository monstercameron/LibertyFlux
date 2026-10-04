// original: 0x0094f930 record_tables_init
/// Initialise the subsystem's per-record tables and publish the result.
///
/// Unless table slot zero is already set (in which case it only raises the
/// ready flag), checks a scripted 64-bit version gate, optionally runs a
/// scripted pre-pass, then initialises one record per requested index:
/// each unset slot gets a scripted allocation, a scripted fixed-size
/// block, and a scripted three-argument construction whose result is
/// stored in the slot; the new object is then activated through its
/// table's second entry with a constant key. Indices below 11 publish
/// the activation result to the main table, higher indices to the
/// overflow area. Finally the two slot counters are decremented, the
/// published pointers and status bytes are written out, and the ready
/// flag is raised. Returns 1 on success, 0 when the version gate fails
/// or an allocation fails.
export!(cdecl, rw_0094f930(arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x11F6FA8;
        const BLOCKS: u32 = 0x11F6F38;
        const MAIN: u32 = 0x11F6F7C;
        const OVERFLOW: u32 = 0x11F6FD4;
        const COUNT_A: u32 = 0x11F6FFB;
        const COUNT_B: u32 = 0x11F6FFD;
        const MODE: u32 = 0x11F6FF0;
        const READY: u32 = 0x1160EB8;
        const PRECOND: u32 = 0x12088B4;
        const PRECOND_REF: u32 = 0xF1C040;
        const FIRST: u32 = 0x11F7000;
        const PUB_PTR: u32 = 0x11FA00C;
        const PUB_ZERO_B: u32 = 0x120F298;
        const PUB_MAIN0: u32 = 0x120F294;
        const PUB_ZERO_C: u32 = 0x11FA010;
        const OFF_A: u32 = 0x120F290;
        const BASE_B: u32 = 0x11FA008;
        const MARK: u32 = 0x11F7018;
        const VER_ID: u32 = 1;
        const PRE_ID: u32 = 2;
        const ALLOC_ID: u32 = 3;
        const BLOCK_ID: u32 = 4;
        const CTOR_ID: u32 = 5;

        let fail = | | unsafe {
            *(relocated(READY) as *mut u32) = 0;
        };
        let a0 = (arg0 & 0xFF) as u8;
        let a1 = (arg1 & 0xFF) as u8;
        // Size selector and activation key are raw file-VA constants: the
        // original's immediates carry no relocation entries.
        let ebx = if a0 == 0 { 0x80u32 } else { 0x896440u32 };
        let key = if a0 == 0 { 0x40u32 } else { 0x895440u32 };
        if *(relocated(SLOTS) as *const u32) != 0 {
            *(relocated(READY) as *mut u32) = 1;
            return 1;
        }
        let ver: u64 = callee_cdecl!(VER_ID, u64,);
        if (ver >> 32) as u32 == 0 && (ver as u32) <= 0x59999980u32 {
            fail();
            return 0;
        }
        if *(relocated(PRECOND) as *const u32) == *(relocated(PRECOND_REF) as *const u32)
            && a1 == 0
        {
            let _: u32 = callee_cdecl!(PRE_ID, u32, 0u32, 0x20000000u32);
        }
        let n = a0 as u32;
        let mut esi = 0u32;
        while esi < n {
            if *(relocated(SLOTS).wrapping_add(esi * 4) as *const u32) == 0 {
                let m: u32 = callee_cdecl!(ALLOC_ID, u32, ebx);
                *(relocated(BLOCKS).wrapping_add(esi * 4) as *mut u32) = m;
                if m == 0 {
                    fail();
                    return 0;
                }
                let q: u32 = callee_cdecl!(BLOCK_ID, u32, 0x20F8u32);
                let o: u32 = if q != 0 {
                    callee_thiscall!(CTOR_ID, u32, q, m, ebx, 1u32)
                } else {
                    0
                };
                *(relocated(SLOTS).wrapping_add(esi * 4) as *mut u32) = o;
                // Volatile: a null object faults here on the original, and
                // the fault must be reproduced exactly, not optimised away.
                let vt = core::ptr::read_volatile(o as *const u32);
                let act: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(*((vt.wrapping_add(8)) as *const u32));
                let r = act(o, key, 0x10u32, 0u32);
                if esi < 11 {
                    let c = relocated(COUNT_A) as *mut u8;
                    *c = (*c).wrapping_add(1);
                    *(relocated(MAIN).wrapping_add(esi * 4) as *mut u32) = r;
                } else {
                    let c = relocated(COUNT_B) as *mut u8;
                    *c = (*c).wrapping_add(1);
                    *(relocated(OVERFLOW).wrapping_add(esi * 4) as *mut u32) = r;
                }
            }
            esi += 1;
        }
        let first = *(relocated(FIRST) as *const u32);
        let ca = relocated(COUNT_A) as *mut u8;
        *ca = (*ca).wrapping_sub(1);
        let cb = relocated(COUNT_B) as *mut u8;
        *cb = (*cb).wrapping_sub(1);
        let slot0 = *(relocated(SLOTS) as *const u32);
        let main0 = *(relocated(MAIN) as *const u32);
        *(relocated(PUB_PTR) as *mut u32) = first;
        *(relocated(PUB_ZERO_B) as *mut u8) = 0;
        *(relocated(PUB_MAIN0) as *mut u32) = main0;
        *(relocated(PUB_ZERO_C) as *mut u8) = 0;
        if slot0 != 0 {
            let off = *(relocated(OFF_A) as *const u32);
            *(relocated(MODE) as *mut u8) = 2;
            *((main0.wrapping_add(off)) as *mut u8) = 0;
            *(relocated(MARK) as *mut u8) = 2;
            let base = *(relocated(BASE_B) as *const u32);
            let pubv = *(relocated(PUB_PTR) as *const u32);
            *((base.wrapping_add(pubv)) as *mut u8) = 0;
        }
        *(relocated(READY) as *mut u32) = 1;
        1
    }
});
