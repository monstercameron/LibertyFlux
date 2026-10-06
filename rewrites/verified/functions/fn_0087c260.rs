// original: 0x0087c260 rage::crmtNodeBlend::vf2
/// Weighted accumulate-and-notify with saturation-gated listener calls.
///
/// Computes r = src[index]*weight + base (index from `src+0x24`, weight from
/// `this+0x24`, base from `this+0x20`; multiply before add, in the
/// original's order, pinned against reassociation), clamps r into [0, MAX]
/// with ordered comparisons (MAX is the global 1.0; NaN passes through) and
/// stores it at `+0x20`. With positive weight, the listener list at `+0x14`
/// is walked only when r equals MAX exactly (the original's
/// ucomiss+lahf/test/jp falls through on equality alone, skipping ordered
/// non-equal and NaN alike), notifying slot 0x0c of each node with a
/// two-word {0x50005, 0} record built on the stack. With negative weight,
/// direct callee 4 is notified once only when r equals either signed zero.
/// When the flag byte at `+0x30` is then set, slot 0x30 of this node's table
/// is polled (argument 1 on the positive path, 0 on the negative) and a
/// non-null answer is linked through direct callee 3; that call's second
/// stack argument is the callee-clobbered ecx residue (the real slot-0x30
/// target overwrites ecx first, verified from its code), so the contract
/// skips it. Computes no return value. All float comparisons are ordered.
///
/// Original: thiscall/1, two indirect plus two direct calls.
export!(thiscall, rw_0087c260(this: u32, src: u32) -> u32 {
    /// Global clamp upper bound, 1.0 (file VA; read relocated).
    const MAX_OFF: u32 = 0x00FE88E8;
    /// Index word in the source block; sample stride is 4 bytes.
    const IDX_OFF: u32 = 0x24;
    /// Weight in this node; its sign picks the notify path.
    const WEIGHT_OFF: u32 = 0x24;
    /// Accumulator in this node, read and rewritten.
    const ACCUM_OFF: u32 = 0x20;
    /// Listener-list head in this node.
    const HEAD_OFF: u32 = 0x14;
    /// Next link in a listener node, read before the notify call.
    const NEXT_OFF: u32 = 0x0C;
    /// Notify slot in a listener node's table.
    const NOTIFY_SLOT: u32 = 0x0C;
    /// Flag byte gating the poll.
    const FLAG_OFF: u32 = 0x30;
    /// Poll slot in this node's table.
    const POLL_SLOT: u32 = 0x30;
    /// First word of the stack-built notify record.
    const REC_A: u32 = 0x00050005;
    #[inline(always)]
    fn mul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    #[inline(always)]
    fn add(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
    unsafe {
        let max = *global::<f32>(MAX_OFF);
        let idx = ((src + IDX_OFF) as *const u32).read_unaligned();
        let sample = ((src + idx.wrapping_mul(4)) as *const f32).read_unaligned();
        let weight = ((this + WEIGHT_OFF) as *const f32).read_unaligned();
        let base = ((this + ACCUM_OFF) as *const f32).read_unaligned();
        let mut r = add(mul(sample, weight), base);
        ((this + ACCUM_OFF) as *mut f32).write_unaligned(r);
        if r < 0.0 {
            r = 0.0;
        } else if r > max {
            r = max;
        }
        ((this + ACCUM_OFF) as *mut f32).write_unaligned(r);
        let positive = weight > 0.0;
        let negative = !positive && weight < 0.0;
        if positive {
            // ucomiss(r, max) + lahf/test/jp falls through only on equality.
            if r != max {
                return 0;
            }
            let mut node = ((this + HEAD_OFF) as *const u32).read_unaligned();
            while node != 0 {
                let vn = (node as *const u32).read_unaligned();
                let next = ((node + NEXT_OFF) as *const u32).read_unaligned();
                let tgt = ((vn + NOTIFY_SLOT) as *const u32).read_unaligned();
                let rec = [REC_A, 0u32];
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                f(node, rec.as_ptr() as u32);
                node = next;
            }
        } else if negative {
            // ucomiss(r, 0) + lahf/test/jp falls through only on equality
            // (either signed zero; NaN takes the jump).
            if r != 0.0 {
                return 0;
            }
            let rec = [REC_A, 0u32];
            callee_thiscall!(4, u32, this, rec.as_ptr() as u32);
        } else {
            return 0;
        }
        if ((this + FLAG_OFF) as *const u8).read() == 0 {
            return 0;
        }
        let vt = (this as *const u32).read_unaligned();
        let tgt = ((vt + POLL_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let r2 = f(this, if positive { 1 } else { 0 });
        if r2 != 0 {
            // arg1 is the callee-clobbered ecx residue on both sides; pass a
            // placeholder (the contract skips it, it cannot be observed).
            callee_thiscall!(3, u32, this, r2, 0);
        }
        0
    }
});
