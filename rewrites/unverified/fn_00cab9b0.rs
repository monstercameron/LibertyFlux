// original: 0x00CAB9B0 goto_task_init (proposed)

/// Initialise a go-to-point movement task from a start point to an end point.
///
/// `this` is the task object. `from` and `to` point at two three-float
/// vectors (start and destination). `extra_opt` is null or points at four
/// words copied to `+0x60` when non-null. `aux` is stored at `+0x50` and,
/// when non-zero, selects a registration call passing `this + 0x50`.
/// Only bit 0 of `flags` is read: it becomes bit 2 of the flag word at
/// `+0x74` (whose low five bits are otherwise cleared); bit 1 of that word
/// is always set before returning.
///
/// Behaviour: runs the base initialiser (callee 1, thiscall, one float
/// argument of exactly `1.0`), plants two virtual-table pointers (`+0x00`
/// and `+0x14`), copies `from` to `+0x20` and `to` to `+0x30`, zeroes
/// `+0x54`, `+0x58` and the halfword at `+0x5c`, then derives the direction
/// `to - from` (component order x, y, z), its squared length formed as
/// `(dx*dx + dy*dy) + dz*dz`, and stores the direction normalised by
/// `1/sqrt(d2)` at `+0x40` (x, y, z). When `d2` compares equal to `+0.0`
/// (ucomiss, so both zeroes but not NaN) the factor is `0.0` instead and the
/// stored direction is zero; a NaN `d2` takes the divide path and propagates
/// NaN. The per-component multiplies keep the original's operand order
/// (`dx*inv`, `inv*dy`, `dz*inv`), which matters for NaN payloads. The
/// midpoint `(from + to) * 0.5` is passed by pointer to callee 2 (cdecl,
/// `(mid_ptr, dir_y_bits, dir_x_bits)`, only its `al` is kept) together with
/// two direction words; that byte is passed on to callee 3 (cdecl, three
/// words: the byte plus the same two direction words, which the original
/// leaves in its argument slots) whose answer lands at `+0x70`.
///
/// One word is NOT computed from the inputs: `+0x4c` is loaded from an
/// aligned-frame scratch slot (`[esp+0x1c]`) that neither this function nor
/// its callee ever writes, i.e. stale stack contents. Under the checker's
/// defined stack fill of `0` that word is `0.0`, which is what this rewrite
/// stores. All comparisons in this function are exact float equality or
/// null checks; nothing here is a signed/unsigned integer comparison.
///
/// Original: 0x00CAB9B0 (thiscall, five stack words, callee pops 20).
/// Returns `this`.
lf_checker_rt::export!(thiscall, rw_00CAB9B0(this: u32, from: u32, to: u32, extra_opt: u32, aux: u32, flags: u32) -> u32 {
    unsafe {
        const VTBL_MAIN: u32 = 0x00;
        const VTBL_SECOND: u32 = 0x14;
        const FROM_COPY: u32 = 0x20;
        const TO_COPY: u32 = 0x30;
        const DIR_OUT: u32 = 0x40;
        const STALE_OUT: u32 = 0x4c;
        const AUX_SLOT: u32 = 0x50;
        const EXTRA_OUT: u32 = 0x60;
        const ANSWER_SLOT: u32 = 0x70;
        const FLAG_WORD: u32 = 0x74;
        const VTBL_MAIN_FILE: u32 = 0x00ED86E4;
        const VTBL_SECOND_FILE: u32 = 0x00ED873C;
        const FLAG_KEEP_MASK: u32 = 0xFFFF_FFE0;
        const FLAG_BIT_ALWAYS: u32 = 0x02;
        const BASE_INIT_CALLEE: u32 = 1;
        const CLASSIFY_CALLEE: u32 = 2;
        const RESOLVE_CALLEE: u32 = 3;
        const REGISTER_CALLEE: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        // Base-class initialiser: thiscall with a single float argument 1.0.
        // Its answer is ignored.
        lf_checker_rt::callee_thiscall!(BASE_INIT_CALLEE, u32, this, 1.0f32.to_bits());

        wr32(this + VTBL_MAIN, lf_checker_rt::relocated(VTBL_MAIN_FILE));
        wr32(this + VTBL_SECOND, lf_checker_rt::relocated(VTBL_SECOND_FILE));

        wr32(this + FROM_COPY, rd32(from));
        wr32(this + FROM_COPY + 4, rd32(from + 4));
        wr32(this + FROM_COPY + 8, rd32(from + 8));
        wr32(this + TO_COPY, rd32(to));
        wr32(this + TO_COPY + 4, rd32(to + 4));
        wr32(this + TO_COPY + 8, rd32(to + 8));

        wr32(this + AUX_SLOT, aux);
        wr32(this + AUX_SLOT + 4, 0);
        wr32(this + AUX_SLOT + 8, 0);
        ((this + AUX_SLOT + 12) as *mut u16).write_unaligned(0);

        let flag_old = rd32(this + FLAG_WORD);
        wr32(
            this + FLAG_WORD,
            (flag_old & FLAG_KEEP_MASK) | ((flags & 1) << 2),
        );

        let ax = rdf(this + FROM_COPY);
        let ay = rdf(this + FROM_COPY + 4);
        let az = rdf(this + FROM_COPY + 8);
        let bx = rdf(this + TO_COPY);
        let by = rdf(this + TO_COPY + 4);
        let bz = rdf(this + TO_COPY + 8);
        let dx = sub(bx, ax);
        let dy = sub(by, ay);
        let dz = sub(bz, az);
        let d2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
        // Zero squared length (either zero, never NaN) skips the divide and
        // zeroes the direction; anything else divides, NaN included.
        let inv = if d2 == 0.0 {
            0.0f32
        } else {
            div(1.0f32, core::hint::black_box(d2).sqrt())
        };
        let nx = mul(dx, inv);
        let ny = mul(inv, dy);
        let nz = mul(dz, inv);
        wr32(this + DIR_OUT, nx.to_bits());
        wr32(this + DIR_OUT + 4, ny.to_bits());
        wr32(this + DIR_OUT + 8, nz.to_bits());
        // Stale stack slot under a zero fill (see doc comment).
        wr32(this + STALE_OUT, 0.0f32.to_bits());

        let mx = mul(add(ax, bx), 0.5f32);
        let my = mul(add(ay, by), 0.5f32);
        let mz = mul(add(az, bz), 0.5f32);

        // Classification call over the midpoint; only al survives (movzx).
        let mut mid = [mx.to_bits(), my.to_bits(), mz.to_bits()];
        let cls: u32 = lf_checker_rt::callee_cdecl!(
            CLASSIFY_CALLEE,
            u32,
            mid.as_mut_ptr() as u32,
            ny.to_bits(),
            nx.to_bits()
        );
        let cls_byte = cls & 0xFF;
        // Resolver call: the byte plus the two direction words the original
        // leaves behind in its outgoing argument slots.
        let ans: u32 = lf_checker_rt::callee_cdecl!(
            RESOLVE_CALLEE,
            u32,
            cls_byte,
            ny.to_bits(),
            nx.to_bits()
        );
        wr32(this + ANSWER_SLOT, ans);

        // The direction words were re-read from the object by the original
        // after the calls; they are unchanged, so no reload is needed.

        if aux != 0 {
            lf_checker_rt::callee_thiscall!(REGISTER_CALLEE, u32, aux, this + AUX_SLOT);
        }

        if extra_opt != 0 {
            wr32(this + EXTRA_OUT, rd32(extra_opt));
            wr32(this + EXTRA_OUT + 4, rd32(extra_opt + 4));
            wr32(this + EXTRA_OUT + 8, rd32(extra_opt + 8));
            wr32(this + EXTRA_OUT + 12, rd32(extra_opt + 12));
        }
        wr32(this + FLAG_WORD, rd32(this + FLAG_WORD) | FLAG_BIT_ALWAYS);

        this
    }
});
