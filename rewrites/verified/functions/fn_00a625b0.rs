// original: 0x00a625b0 ped_task_cleanup (proposed)

/// Run a ped task object's cleanup pass: refresh the cached state, then
/// sweep the six task slots.
///
/// The low byte of `a0` gates the refresh block, the low byte of `a1`
/// the sweep; either may be skipped. Refresh: callee 0 (thiscall on
/// `this`) runs first; then, by the flag bit at master `+0x26c` and the
/// counter at master `+0xb30` (master = `this+0x40`), callee 1 (thiscall
/// on `this+0x84`) is asked with code `0x2c5` or `0xcb`. A non-zero
/// answer skips to the tail chain, otherwise callee 2 (thiscall on the
/// shared global `G_SHARED`) resolves a worker: null feeds 0 into the
/// frame object, else callee 3 (A path: thiscall on the worker with the
/// counter and 0) or callee 4 (B path: thiscall on the worker with three
/// zeros and `K_F32`) supplies the value. Callee 5 (thiscall with the
/// frame object in ECX) builds it, callee 6 (thiscall on `this+0x84`,
/// frame pointer, 0, 1) consumes it, callee 7 (thiscall on the frame
/// object) tears it down. The tail chain calls the object's own virtual
/// slot `+0x20` (callee 8), then callee 9 on that answer, callee 10 and
/// callee 11 on `this+0x44` (callee 11 inherits ECX from callee 10's
/// scratch; uncompared, see below).
///
/// The sweep runs slots 0..6 except 3: callee 12 (thiscall on
/// `this+0x44`, the slot index) fetches each entry; a null entry moves
/// on. An entry whose flag byte already has bit 0 goes straight to
/// callee 14 (thiscall on `this+0x44`, 0, index), otherwise virtual slot
/// `+0x14` (callee 13, thiscall on the entry: master, 1, 0) is tried: a
/// non-zero answer sets flag bit 1 and still runs callee 14, a zero
/// answer retries the slot with (master, 0, 0) and sets bit 1 on
/// success, skipping callee 14 either way on this path.
///
/// The function returns leftover EAX (the last callee scratch, or entry
/// EAX when both blocks are skipped), which no caller can rely on; the
/// return channel is off and the proof covers the outgoing calls and the
/// flag writes. Callee 11's ECX is the previous callee's scratch (the
/// real callee was read and does not preserve ECX), so it is uncompared.
/// The second-chance slot call's non-zero path (both calls in one visit
/// returning different answers) is not exercised: the stub answers once
/// per trial.
///
/// Original: 0x00a625b0 (thiscall, two stack words: the two gate bytes).
lf_checker_rt::export!(thiscall, rw_00a625b0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const MASTER: u32 = 0x40;
        const CTX: u32 = 0x44;
        const AUX: u32 = 0x84;
        const FLAG_OFF: u32 = 0x26c;
        const COUNT_OFF: u32 = 0xb30;
        const FLAG_BIT: u8 = 0x04;
        const CODE_A: u32 = 0x2c5;
        const CODE_B: u32 = 0xcb;
        const K_F32: u32 = 0x41000000;
        const G_SHARED: u32 = 0x0167e2a0;
        const ENTRY_FLAGS: u32 = 0x0c;
        const CALLEE_INIT: u32 = 0;
        const CALLEE_ASK: u32 = 1;
        const CALLEE_WORKER: u32 = 2;
        const CALLEE_VAL_A: u32 = 3;
        const CALLEE_VAL_B: u32 = 4;
        const CALLEE_BUILD: u32 = 5;
        const CALLEE_USE: u32 = 6;
        const CALLEE_TEARDOWN: u32 = 7;
        const CALLEE_VIRT: u32 = 8;
        const CALLEE_POST: u32 = 9;
        const CALLEE_C10: u32 = 10;
        const CALLEE_C11: u32 = 11;
        const CALLEE_SLOT: u32 = 12;
        const CALLEE_ENTRY: u32 = 13;
        const CALLEE_SWEEP: u32 = 14;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if (a0 & 0xff) != 0 {
            lf_checker_rt::callee_thiscall!(CALLEE_INIT, u32, this);
            let master = rd32(this + MASTER);
            let code = if rd8(master + FLAG_OFF) & FLAG_BIT != 0 && rd32(master + COUNT_OFF) != 0
            {
                CODE_A
            } else {
                CODE_B
            };
            let r1: u32 = lf_checker_rt::callee_thiscall!(CALLEE_ASK, u32, this + AUX, code);
            if (r1 & 0xff) == 0 {
                let g = lf_checker_rt::global::<u32>(G_SHARED).read_unaligned();
                let r2: u32 = lf_checker_rt::callee_thiscall!(CALLEE_WORKER, u32, g);
                let e = if r2 == 0 {
                    0
                } else if code == CODE_A {
                    lf_checker_rt::callee_thiscall!(CALLEE_VAL_A, u32, r2, rd32(master + COUNT_OFF), 0)
                } else {
                    lf_checker_rt::callee_thiscall!(CALLEE_VAL_B, u32, r2, 0, 0, 0, K_F32)
                };
                let mut s = [0u32; 6];
                lf_checker_rt::callee_thiscall!(CALLEE_BUILD, u32, s.as_mut_ptr() as u32, 3, e, 0);
                lf_checker_rt::callee_thiscall!(CALLEE_USE, u32, this + AUX, s.as_mut_ptr() as u32, 0, 1);
                lf_checker_rt::callee_thiscall!(CALLEE_TEARDOWN, u32, s.as_mut_ptr() as u32);
            }
            let r8: u32 = lf_checker_rt::callee_thiscall!(CALLEE_VIRT, u32, this);
            lf_checker_rt::callee_thiscall!(CALLEE_POST, u32, r8);
            lf_checker_rt::callee_thiscall!(CALLEE_C10, u32, this + CTX);
            lf_checker_rt::callee_thiscall!(CALLEE_C11, u32, this + CTX);
        }
        if (a1 & 0xff) != 0 {
            let master = rd32(this + MASTER);
            let mut edi = 0u32;
            while edi < 6 {
                if edi != 3 {
                    let esi: u32 =
                        lf_checker_rt::callee_thiscall!(CALLEE_SLOT, u32, this + CTX, edi);
                    if esi != 0 {
                        if rd32(esi + ENTRY_FLAGS) & 1 != 0 {
                            lf_checker_rt::callee_thiscall!(CALLEE_SWEEP, u32, this + CTX, 0, edi);
                        } else {
                            let q1: u32 = lf_checker_rt::callee_thiscall!(
                                CALLEE_ENTRY, u32, esi, master, 1, 0
                            );
                            if (q1 & 0xff) != 0 {
                                wr32(esi + ENTRY_FLAGS, rd32(esi + ENTRY_FLAGS) | 2);
                                lf_checker_rt::callee_thiscall!(CALLEE_SWEEP, u32, this + CTX, 0, edi);
                            } else if rd32(esi + ENTRY_FLAGS) & 1 == 0 {
                                let q2: u32 = lf_checker_rt::callee_thiscall!(
                                    CALLEE_ENTRY, u32, esi, master, 0, 0
                                );
                                if (q2 & 0xff) != 0 {
                                    wr32(esi + ENTRY_FLAGS, rd32(esi + ENTRY_FLAGS) | 2);
                                }
                            }
                        }
                    }
                }
                edi += 1;
            }
        }
        0
    }
});
