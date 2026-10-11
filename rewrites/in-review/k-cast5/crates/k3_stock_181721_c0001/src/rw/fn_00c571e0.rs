// original: 0x00c571e0 peds_task_convert_float_fields (proposed)

/// Refresh a ped task-worker's cached narrow fields from two float sources.
///
/// `this` is the worker record, `obj` the task object. The worker keeps a
/// byte at `+0x10`, a copy of the previous byte at `+0x11`, a word at
/// `+0x12`, a copy of the previous word at `+0x14`, and a tag byte at `+0x0`
/// (copied to `+0x1` at the end). The object supplies a float through its
/// virtual slot `+0xFC` (called with the object in ECX, result in ST0),
/// converted with x87 truncate-to-integer and stored low byte first into
/// the worker's `+0x10`; when the object's flag bit 2 (`+0x26C`) is set, a
/// second float comes from the sub-object at `+0xB30` (same slot) and its
/// low word goes to `+0x12`.
///
/// Afterwards one of three things happens: nothing (epilogue only), or a
/// three-call sequence on a scratch slot initialised with a table pointer
/// (`TAG_A` when the compared bytes fall in the middle band, `TAG_B`
/// otherwise). Both tags are file addresses of address tables in read-only
/// data and carry relocation entries, so the value observed is the
/// relocated address on each side. The flag-set path compares the copied
/// word against `LO_W`
/// and `HI_W`; the flag-clear path compares the copied previous byte
/// against `LO_B` and `HI_B`. The scratch slot lives in the caller's frame
/// on both sides, so only its contents at the second call are observed.
///
/// The conversions reproduce `fistp` exactly, including its out-of-range
/// and NaN response (the indefinite integer, whose kept bytes are zero):
/// any float at or above 2^31 or below -2^31 converts to zero, otherwise
/// Rust's truncating `as` cast matches chop mode bit for bit.
///
/// Original: 0x00C571E0 (thiscall, ECX = worker, one stack word = object,
/// returns the worker's tag byte zero-extended).
lf_checker_rt::export!(thiscall, rw_00c571e0(this: u32, obj: u32) -> u32 {
    unsafe {
        const VT_SLOT: u32 = 0xFC;
        const FLAG_OFF: u32 = 0x26C;
        const FLAG_BIT: u8 = 4;
        const SUB_OFF: u32 = 0xB30;
        const CTX_OFF: u32 = 0x224;
        const CTX_BIAS: u32 = 0x84;
        const LO_B: u8 = 0x6E;
        const HI_B: u8 = 0x96;
        const LO_W: u16 = 0x12C;
        const HI_W: u16 = 0x258;
        const TAG_A: u32 = 0xECAD44;
        const TAG_B: u32 = 0xECADA4;
        const CAL_INIT: u32 = 2;
        const CAL_USE: u32 = 3;
        const CAL_DONE: u32 = 4;
        const F32_TWO31: f32 = 2147483648.0;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }

        /// Virtual slot call the original makes through `(an instruction of the original)`: load the
        /// table, take the slot, call with the object in ECX. Both sides land
        /// on the same planted stub. The float result arrives in ST0.
        #[inline(always)]
        unsafe fn slot_float(target: u32) -> f32 {
            unsafe {
                let slot: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(rd32(rd32(target) + VT_SLOT) as usize);
                slot(target)
            }
        }

        /// Truncate like `fistp` with chop mode, keeping the low byte.
        #[inline(always)]
        fn trunc_byte(v: f32) -> u8 {
            if v >= F32_TWO31 || v < -F32_TWO31 {
                0
            } else {
                (v as i64 & 0xFF) as u8
            }
        }

        /// Truncate like `fistp` with chop mode, keeping the low word.
        #[inline(always)]
        fn trunc_word(v: f32) -> u16 {
            if v >= F32_TWO31 || v < -F32_TWO31 {
                0
            } else {
                (v as i64 & 0xFFFF) as u16
            }
        }

        /// The three-call sequence on a scratch slot holding table `tag`
        /// (a relocated file address, as the original's operand is).
        #[inline(always)]
        unsafe fn run_calls(obj: u32, tag: u32) {
            unsafe {
                let mut slot: u32 = 0;
                lf_checker_rt::callee_thiscall!(CAL_INIT, u32, &mut slot as *mut u32 as u32);
                slot = lf_checker_rt::relocated(tag);
                let ctx = rd32(obj + CTX_OFF).wrapping_add(CTX_BIAS);
                lf_checker_rt::callee_thiscall!(CAL_USE, u32, ctx, &slot as *const u32 as u32, 0u32, 1u32);
                slot = lf_checker_rt::relocated(tag);
                lf_checker_rt::callee_thiscall!(CAL_DONE, u32, &slot as *const u32 as u32);
            }
        }

        /// Copy current fields over the previous ones and return the tag.
        #[inline(always)]
        unsafe fn epilogue(this: u32) -> u32 {
            unsafe {
                wr8(this + 0x11, rd8(this + 0x10));
                wr16(this + 0x14, rd16(this + 0x12));
                let tag = rd8(this);
                wr8(this + 1, tag);
                tag as u32
            }
        }

        wr8(this + 0x10, trunc_byte(slot_float(obj)));
        if rd8(obj + FLAG_OFF) & FLAG_BIT != 0 {
            let sub = rd32(obj + SUB_OFF);
            if sub != 0 {
                wr16(this + 0x12, trunc_word(slot_float(sub)));
            }
        }
        // The original tests the flag again here; replicate the second test.
        if rd8(obj + FLAG_OFF) & FLAG_BIT != 0 {
            let prev = rd16(this + 0x14);
            if prev < HI_W {
                if prev < LO_W {
                    return epilogue(this);
                }
                if rd16(this + 0x12) >= LO_W {
                    return epilogue(this);
                }
                run_calls(obj, TAG_B);
            } else {
                let cur = rd16(this + 0x12);
                if cur < LO_W {
                    run_calls(obj, TAG_B);
                } else if cur >= HI_W {
                    // Jumps into the middle of the byte-band test, so the
                    // surviving flags are the word's own comparison: a word
                    // at or above the high bound skips the calls.
                    return epilogue(this);
                } else {
                    run_calls(obj, TAG_A);
                }
            }
        } else {
            let prev = rd8(this + 0x11);
            if prev < HI_B {
                if prev < LO_B {
                    return epilogue(this);
                }
                if rd8(this + 0x10) >= LO_B {
                    return epilogue(this);
                }
                run_calls(obj, TAG_B);
            } else {
                let cur = rd8(this + 0x10);
                if cur < LO_B {
                    run_calls(obj, TAG_B);
                } else if cur >= HI_B {
                    return epilogue(this);
                } else {
                    run_calls(obj, TAG_A);
                }
            }
        }
        epilogue(this)
    }
});
