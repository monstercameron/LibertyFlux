// original: 0x008efaa0 resolve_control_binding
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

/// Resolve one control binding to its output value (original 0x008EFAA0).
///
/// A mismatched tag pair takes the fast path, stamping the row and routing
/// it through the row evaluator. Otherwise a four-stage dispatcher narrows
/// the binding down: two gated float stages, a three-query select, and a
/// final fallback query. Each float stage scales one channel, keeps the
/// result when the scaled value is non-negative and switches channels
/// otherwise, and the shared tail evaluates the surviving channel. The
/// value returned is the last evaluator answer in every path.
export!(thiscall, rw_008efaa0(this: u32) -> f32 {
    // The worker rebases this immediate (checker-verified); derive it.
    const ROW_BASE_FILE: u32 = 0x0118D470;
    const ROW_STRIDE: u32 = 0xBC;
    const OUT_SCALE: f32 = 127.5;
    #[inline(always)]
    unsafe fn gate(addr: u32) -> bool {
        (callee_thiscall!(2, u32, addr) as u8) != 0
    }
    #[inline(always)]
    unsafe fn query(addr: u32) -> u32 {
        callee_cdecl!(3, u32, addr)
    }
    /// One gated float stage: evaluate the channel, scale the answer, and
    /// pick the follow-up channel by the sign of the scaled value.
    #[inline(always)]
    unsafe fn float_block(this: u32, at: usize, neg: usize, pos: usize) -> u32 {
        const OUT_SCALE: f32 = 127.5;
        let v: f32 = callee_thiscall!(4, f32, this.wrapping_add(at as u32));
        let r: f32 = callee_cdecl!(5, f32, v.to_bits(), 0);
        let scaled = (r * OUT_SCALE) as i32;
        this.wrapping_add(if scaled < 0 { neg } else { pos } as u32)
    }
    unsafe {
        let base = this as *mut u8;
        let tag = base.add(0x2C5E).read() ^ base.add(0x2C5C).read();
        if tag != 0x80 {
            let table = (base.add(0x2D94) as *const u32).read();
            let row = base.add(0x2D88);
            let val = 0xFFu8.wrapping_sub(tag);
            row.add(6).write(val);
            if table != 0 {
                let slot = row.add(8).read() as u32;
                (table.wrapping_add(slot.wrapping_mul(8)) as *mut u8).write(val);
            }
            return callee_cdecl!(1, f32, row as u32);
        }
        let row_base = relocated(ROW_BASE_FILE);
        let row_a = row_base.wrapping_add(
            (base.add(0x32A4) as *const u32).read().wrapping_mul(ROW_STRIDE));
        let mut tail: u32 = 0;
        let mut have_tail = false;
        if gate(row_a)
            && query(base.add(0x3228) as u32) == 0
            && query(base.add(0x3238) as u32) == 0
        {
            tail = float_block(this, 0x2878, 0x2878, 0x2888);
            have_tail = true;
        }
        if !have_tail {
            let idx_b = (base.add(0x32A8) as *const u32).read();
            let row_b = row_base.wrapping_add(idx_b.wrapping_mul(ROW_STRIDE));
            if (idx_b as i32) >= 0
                && gate(row_b)
                && query(base.add(0x2878) as u32) == 0
                && query(base.add(0x2888) as u32) == 0
            {
                tail = float_block(this, 0x3228, 0x3228, 0x3238);
                have_tail = true;
            }
        }
        if have_tail {
            let v: f32 = callee_thiscall!(4, f32, tail);
            return callee_cdecl!(5, f32, v.to_bits(), 0);
        }
        if (base.add(0x32A8) as *const u32).read() as i32 >= 0
            && query(base.add(0x2878) as u32) == 0
            && query(base.add(0x2888) as u32) == 0
        {
            let t = query(base.add(0x3228) as u32);
            if (t as i32) < 0 {
                return callee_cdecl!(1, f32, base.add(0x3228) as u32);
            }
            return callee_cdecl!(1, f32, base.add(0x3238) as u32);
        }
        let t = query(base.add(0x2878) as u32);
        if (t as i32) < 0 {
            callee_cdecl!(1, f32, base.add(0x2878) as u32)
        } else {
            callee_cdecl!(1, f32, base.add(0x2888) as u32)
        }
    }
});
