// original: 0x00AD2800 audio_channel_setup (proposed)

/// Set up one audio channel object in up to two parts.
///
/// `obj` is the channel object (null returns immediately); `mode` selects
/// the first part, `level` is a float trim, `count` drives the second part.
/// The channel's parameter word comes from virtual slot `PARAM_SLOT` of the
/// object (id 1). When `mode` equals 2 the first part runs: the parameter
/// word shifted arithmetically right by `count` becomes a divisor for the
/// `HALF_RATE` constant, the trim maps to `clamp(1.0 - level)` (negative
/// trims hold zero, trims above one hold one, NaN passes through), the
/// coefficient block is applied (id 2, selector 2) and a `[1, 1, 0, clamp]`
/// descriptor is committed with the object (id 3), a mode command follows
/// (id 4), and the part submits through the thread session (ids 5-7) with
/// the divisor and divisor-plus-one as its trailing floats. When `count` is
/// positive the second part runs the same shape with selector 0, a
/// `[1, 1, count-1, 1/(count+1)]` descriptor, and the divisor recomputed
/// from the same shift. Both parts end by running finalisers (ids 8-9).
/// All float operations keep the original's operand order.
///
/// Original: 0x00AD2800 (cdecl, four stack words: object, mode, level bits,
/// count; no meaningful return).
lf_checker_rt::export!(cdecl, rw_00ad2800(obj: u32, mode: u32, level: u32, count: u32) -> u32 {
    unsafe {
        const TLS_SLOT_IDX: u32 = 0x017A_BA14;
        const HALF_RATE: u32 = 0x00FE_8830;
        const UNITY: u32 = 0x00FE_88E8;
        const COEFFS: u32 = 0x00EA_69A0;
        const PARAM_SLOT: u32 = 0x20;
        const THREAD_REFCOUNT: u32 = 0x0c;
        const SENTINEL: u32 = 0xffff_ffff;
        const PARAM_CALL: u32 = 1;
        const APPLY: u32 = 2;
        const COMMIT: u32 = 3;
        const MODE: u32 = 4;
        const SESSION_OPEN: u32 = 5;
        const SUBMIT: u32 = 6;
        const SESSION_CLOSE: u32 = 7;
        const FIN1: u32 = 8;
        const FIN2: u32 = 9;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        // Pinned-order float helpers: both operands pass through black_box
        // so the compiler emits exactly the original's operand order.
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        if obj == 0 {
            return 0;
        }
        // Virtual slot call for the channel's parameter word.
        let vtable = rd32(obj);
        let target = rd32(vtable + PARAM_SLOT);
        let param_fn: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        let param = param_fn(obj);

        let half = f32::from_bits(rd32(lf_checker_rt::relocated(HALF_RATE)));
        let one = f32::from_bits(rd32(lf_checker_rt::relocated(UNITY)));
        let coeff_base = lf_checker_rt::relocated(COEFFS);
        // Scratch mirroring the original's frame area.
        let mut area = [0u32; 11];
        let shifted = (param as i32) >> ((count & 0x1f) as u32);

        // Opens the thread session on first reference; returns the object
        // so the matching release can reuse it. Emitted inline per part.
        if mode == 2 {
            for i in 0..4 {
                area[6 + i] = rd32(coeff_base + (i as u32) * 4);
            }
            let divisor = fdiv(half, shifted as f32);
            let trim = fsub(one, f32::from_bits(level));
            let clamped = if trim < 0.0 {
                0.0f32
            } else if trim > one {
                one
            } else {
                trim
            };
            area[4] = divisor.to_bits();
            area[5] = param;
            let coeff_ptr = (&area[6] as *const u32) as u32;
            lf_checker_rt::callee_cdecl!(APPLY, u32, 2, coeff_ptr);
            area[6] = one.to_bits();
            area[7] = one.to_bits();
            area[8] = 0;
            area[9] = clamped.to_bits();
            lf_checker_rt::callee_cdecl!(COMMIT, u32, 0, coeff_ptr, obj);
            lf_checker_rt::callee_cdecl!(MODE, u32, 0x16, 1);
            let slot = lf_checker_rt::global::<u32>(TLS_SLOT_IDX).read_unaligned();
            let refc = (lf_checker_rt::tls_slot(slot as usize) + THREAD_REFCOUNT) as *mut u32;
            refc.write_unaligned(refc.read_unaligned().wrapping_add(1));
            if refc.read_unaligned() == 1 {
                lf_checker_rt::callee_cdecl!(SESSION_OPEN, u32, 1);
            }
            area[3] = SENTINEL;
            let submit_ptr = (&area[3] as *const u32) as u32;
            let up = fadd(divisor, one);
            const ONE_BITS: u32 = 0x3f80_0000;
            lf_checker_rt::callee_cdecl!(SUBMIT, u32, 0, ONE_BITS, ONE_BITS, 0, 0,
                divisor.to_bits(), divisor.to_bits(), up.to_bits(), up.to_bits(), submit_ptr);
            let slot2 = lf_checker_rt::global::<u32>(TLS_SLOT_IDX).read_unaligned();
            let refc2 =
                (lf_checker_rt::tls_slot(slot2 as usize) + THREAD_REFCOUNT) as *mut u32;
            refc2.write_unaligned(refc2.read_unaligned().wrapping_sub(1));
            if refc2.read_unaligned() == 0 {
                lf_checker_rt::callee_cdecl!(SESSION_CLOSE, u32,);
            }
            lf_checker_rt::callee_cdecl!(FIN1, u32,);
            lf_checker_rt::callee_cdecl!(FIN2, u32,);
        }

        if (count as i32) > 0 {
            for i in 0..4 {
                area[7 + i] = rd32(coeff_base + (i as u32) * 4);
            }
            let divisor = fdiv(half, shifted as f32);
            let coeff_ptr = (&area[7] as *const u32) as u32;
            lf_checker_rt::callee_cdecl!(APPLY, u32, 0, coeff_ptr);
            let lower = (count.wrapping_sub(1) as i32) as f32;
            let upper = fadd((count as i32) as f32, one);
            area[7] = one.to_bits();
            area[8] = one.to_bits();
            area[9] = lower.to_bits();
            area[10] = fdiv(one, upper).to_bits();
            lf_checker_rt::callee_cdecl!(COMMIT, u32, 0, coeff_ptr, obj);
            lf_checker_rt::callee_cdecl!(MODE, u32, 0x16, 0);
            let slot = lf_checker_rt::global::<u32>(TLS_SLOT_IDX).read_unaligned();
            let refc = (lf_checker_rt::tls_slot(slot as usize) + THREAD_REFCOUNT) as *mut u32;
            refc.write_unaligned(refc.read_unaligned().wrapping_add(1));
            if refc.read_unaligned() == 1 {
                lf_checker_rt::callee_cdecl!(SESSION_OPEN, u32, 1);
            }
            area[6] = divisor.to_bits();
            area[5] = SENTINEL;
            let submit_ptr = (&area[5] as *const u32) as u32;
            let up = fadd(divisor, one);
            const ONE_BITS: u32 = 0x3f80_0000;
            lf_checker_rt::callee_cdecl!(SUBMIT, u32, 0, ONE_BITS, ONE_BITS, 0, 0,
                divisor.to_bits(), divisor.to_bits(), up.to_bits(), up.to_bits(), submit_ptr);
            let slot2 = lf_checker_rt::global::<u32>(TLS_SLOT_IDX).read_unaligned();
            let refc2 =
                (lf_checker_rt::tls_slot(slot2 as usize) + THREAD_REFCOUNT) as *mut u32;
            refc2.write_unaligned(refc2.read_unaligned().wrapping_sub(1));
            if refc2.read_unaligned() == 0 {
                lf_checker_rt::callee_cdecl!(SESSION_CLOSE, u32,);
            }
            lf_checker_rt::callee_cdecl!(FIN1, u32,);
            lf_checker_rt::callee_cdecl!(FIN2, u32,);
        }
        0
    }
});
