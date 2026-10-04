// original: 0x008D2EC0 budget_poll_or_reseed
/// Polls an active budget handle, or spends down and reseeds the budget.
///
/// `this` owns a handle word, an alive flag, an owner link and a budget
/// counter. When the handle is active it is polled: status 3 clears the
/// handle (done), status 2 keeps it (busy), anything else refreshes the
/// alive flag (set when the status is exactly 0) and clears the handle.
/// When no handle is active and the owner reports ready, the budget is
/// refilled and the object marked alive. Otherwise, once a global rate
/// gate passes, the budget is spent down by the truncated global rate
/// product; when it runs out it is refilled and a fresh handle is seeded
/// from the owner's position plus a global lift. Returns the poll status,
/// the spent amount, or the fresh handle, and 0 on the quiet paths.
lf_checker_rt::export!(thiscall, rb109_fn1(this: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_POLL: u32 = 1; // status poll (cdecl/2: handle, 0)
    const CAL_SEED: u32 = 2; // handle reseed (cdecl/7: pos, frame, 1, 9, 0, 6, 0)

    // Object layout (word offsets from `this` / linked objects).
    const OFF_HANDLE: u32 = 0x5C0; // active handle, 0 when idle
    const OFF_ALIVE: u32 = 0x5B8; // alive flag byte
    const OFF_OWNER: u32 = 0x598; // owner object link
    const OFF_BUDGET: u32 = 0x5BC; // remaining budget counter
    const OWNER_STATUS: u32 = 0x24; // owner status word
    const OWNER_POS: u32 = 0x20; // owner position record link
    const READY_BIT: u32 = 0x0800_0000; // owner ready flag
    const BUDGET_FULL: u32 = 0x2710; // refilled budget (10000)
    const POLL_DONE: u32 = 3;
    const POLL_BUSY: u32 = 2;

    // Globals (file VAs; resolved through the worker's image base).
    const G_GATE_LO: u32 = 0x012DDEAC; // rate-gate sample (f32)
    const G_GATE_HI: u32 = 0x00FE87D0; // rate-gate threshold (f32)
    const G_RATE_A: u32 = 0x011735BC; // rate factor A (f32)
    const G_RATE_B: u32 = 0x00FE8C58; // rate factor B (f32)
    const G_LIFT: u32 = 0x00FE8B48; // seed lift added to z (f32)

    #[inline(always)]
    unsafe fn load(base: u32, off: u32) -> u32 {
        *((base.wrapping_add(off)) as *const u32)
    }
    #[inline(always)]
    unsafe fn store(base: u32, off: u32, v: u32) {
        *((base.wrapping_add(off)) as *mut u32) = v;
    }

    /// Truncate `p` toward zero to 32 bits, matching x87 `fistp qword`
    /// with chop rounding followed by a low-dword load: out-of-range and
    /// NaN inputs yield the indefinite value whose low word is 0.
    #[inline(always)]
    fn trunc_fistp(p: f32) -> u32 {
        const LIM: f32 = 9223372036854775808.0; // 2^63, exact in f32
        if !p.is_finite() || p < -LIM || p >= LIM {
            0
        } else {
            (p as i64) as u32
        }
    }

    unsafe {
        let handle = load(this, OFF_HANDLE);
        if handle != 0 {
            let r = lf_checker_rt::callee_cdecl!(CAL_POLL, u32, handle, 0);
            if r == POLL_DONE {
                store(this, OFF_HANDLE, 0);
                return POLL_DONE;
            }
            if r == POLL_BUSY {
                return POLL_BUSY;
            }
            // `sete al`: only the low byte is set, the upper bytes of the
            // status survive into both the flag store and the return value.
            let alive = if r == 0 { 1u32 } else { 0u32 };
            *((this.wrapping_add(OFF_ALIVE)) as *mut u8) = alive as u8;
            store(this, OFF_HANDLE, 0);
            return (r & 0xFFFF_FF00) | alive;
        }
        let owner = load(this, OFF_OWNER);
        let status = load(owner, OWNER_STATUS);
        if (status & READY_BIT) != 0 {
            store(this, OFF_BUDGET, BUDGET_FULL);
            *((this.wrapping_add(OFF_ALIVE)) as *mut u8) = 1;
            return 0;
        }
        // `comiss` + `jb`: below or unordered both take the quiet path.
        let glo = *(lf_checker_rt::global::<f32>(G_GATE_LO));
        let ghi = *(lf_checker_rt::global::<f32>(G_GATE_HI));
        if !(glo >= ghi) {
            return 0;
        }
        let rate_a = *(lf_checker_rt::global::<f32>(G_RATE_A));
        let rate_b = *(lf_checker_rt::global::<f32>(G_RATE_B));
        let spent = trunc_fistp(rate_a * rate_b);
        let left = load(this, OFF_BUDGET).wrapping_sub(spent);
        store(this, OFF_BUDGET, left);
        if (left as i32) > 0 {
            return spent;
        }
        store(this, OFF_BUDGET, BUDGET_FULL);
        let pos = load(owner, OWNER_POS);
        let fx = *((pos.wrapping_add(0x30)) as *const f32);
        let fy = *((pos.wrapping_add(0x34)) as *const f32);
        let fz = *((pos.wrapping_add(0x38)) as *const f32);
        let lift = *(lf_checker_rt::global::<f32>(G_LIFT));
        let frame = [fx, fy, fz + lift];
        let fresh = lf_checker_rt::callee_cdecl!(
            CAL_SEED, u32,
            pos.wrapping_add(0x30), frame.as_ptr() as u32, 1, 9, 0, 6, 0
        );
        store(this, OFF_HANDLE, fresh);
        fresh
    }
});
