// original: 0x00A1DA00 task_select_by_kind (proposed)

/// Select a task variant by its kind byte and resolve its code and stride.
///
/// `this` is the task object, `kind_obj` describes the requested kind (kind
/// byte at `+4`), `sel` is a selector object and `extra` an extra argument
/// passed to the predicate callee. The function zeroes the halfword at
/// `this+0x3a`, reads `idx = kind_obj[4] - 4` (wrapping) and dispatches:
///
/// - 0, 1: copy the halfword at `kind_obj+0xcc` to `this+0x38`. A zero word
///   returns 0; otherwise `this+0x3c` is zeroed and the dword at
///   `kind_obj+0x8c` is returned.
/// - 2, 3: run the predicate callee as `pred(sel, extra, kind_obj)`
///   (thiscall, two stack arguments). If its low byte is zero, continue at
///   the tail below. Otherwise square the float at `this+0x490`, run the
///   combine callee as `combine(sel, K1, K2, this+0x430, sqbits)` (thiscall,
///   four stack arguments; `K1`/`K2` are two constant addresses), then the
///   virtual slot at `+0xf0` of `kind_obj` as `slot(kind_obj, sel)`, and
///   continue at the tail.
/// - 6: like 2/3, except the triple at `this+0x430..0x438` is first staged
///   and passed by pointer to the prepare callee, the combine callee takes
///   frame pointers to that staging area plus a zero word, and the virtual
///   slot is `+0xf4`. The tail is shared with arms 2/3 except the code word
///   is rechecked before the stride lookup (see below).
/// - 4, 5, or anything above 6: return 0.
///
/// Tail (arms 2, 3, 6): read `i` from `sel+0x2f4`. The code word is 0 when
/// `i` is -1, else the halfword at `sel + i*2 + 0x6a`; it is stored to
/// `this+0x38` and a zero word returns 0. Otherwise `i` is read again, `c`
/// is the halfword at `sel + i*2 + 0x274`, `v` is the halfword at
/// `sel[0x64] + c*2` stored to `this+0x3c`, and the function returns
/// `(v << 5) + kind_obj[0x8c]`. (Arm 6 rejoins this computation after its
/// own code-word check, so the shape is one tail with two entries.)
///
/// Original: 0x00A1DA00 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00A1DA00(this: u32, kind_obj: u32, sel: u32, extra: u32) -> u32 {
    unsafe {
        const CODE: u32 = 0x38;
        const CLEAR: u32 = 0x3a;
        const STRIDE_OUT: u32 = 0x3c;
        const KIND_BYTE: u32 = 0x4;
        const KIND_WORD: u32 = 0xcc;
        const KIND_BASE: u32 = 0x8c;
        const RADIUS: u32 = 0x490;
        const TRIPLE: u32 = 0x430;
        const SEL_INDEX: u32 = 0x2f4;
        const SEL_CODE_ROW: u32 = 0x6a;
        const SEL_C_ROW: u32 = 0x274;
        const SEL_TABLE: u32 = 0x64;
        const SLOT_A: u32 = 0xf0;
        const SLOT_B: u32 = 0xf4;
        const K1: u32 = 0x01110090;
        const K2: u32 = 0x01b4b320;
        const PRED: u32 = 1;
        const COMBINE_DIRECT: u32 = 2;
        const COMBINE_STAGED: u32 = 3;
        const PREPARE: u32 = 4;
        const VSLOT_A: u32 = 10;
        const VSLOT_B: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }

        /// Shared tail: resolve the code word and stride for selector index.
        #[inline(always)]
        unsafe fn tail(this: u32, kind_obj: u32, sel: u32, code: u16) -> u32 {
            unsafe {
                wr16(this + CODE, code);
                if code == 0 {
                    return 0;
                }
                let i = rd32(sel + SEL_INDEX);
                let c = rd16(sel.wrapping_add(i.wrapping_mul(2)).wrapping_add(SEL_C_ROW));
                let t = rd32(sel + SEL_TABLE);
                let v = rd16(t.wrapping_add((c as u32).wrapping_mul(2)));
                wr16(this + STRIDE_OUT, v);
                (v as u32).wrapping_mul(32).wrapping_add(rd32(kind_obj + KIND_BASE))
            }
        }
        #[inline(always)]
        unsafe fn code_word(sel: u32) -> u16 {
            unsafe {
                let i = rd32(sel + SEL_INDEX);
                if i == 0xffff_ffff {
                    0
                } else {
                    rd16(sel.wrapping_add(i.wrapping_mul(2)).wrapping_add(SEL_CODE_ROW))
                }
            }
        }

        (this as *mut u16).add((CLEAR / 2) as usize).write_unaligned(0);
        let idx = ((kind_obj as *const u8).add(KIND_BYTE as usize).read() as u32).wrapping_sub(4);
        if idx > 6 {
            return 0;
        }
        match idx {
            0 | 1 => {
                let w = rd16(kind_obj + KIND_WORD);
                wr16(this + CODE, w);
                if w == 0 {
                    return 0;
                }
                wr16(this + STRIDE_OUT, 0);
                rd32(kind_obj + KIND_BASE)
            }
            2 | 3 => {
                let r = lf_checker_rt::callee_thiscall!(PRED, u32, sel, extra, kind_obj);
                if r & 0xff != 0 {
                    let x = f32::from_bits(rd32(this + RADIUS));
                    let sq = core::hint::black_box(x) * core::hint::black_box(x);
                    lf_checker_rt::callee_thiscall!(
                        COMBINE_DIRECT, u32, sel, K1, K2, this + TRIPLE, sq.to_bits()
                    );
                    let slot: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(rd32(rd32(kind_obj) + SLOT_A) as usize);
                    slot(kind_obj, sel);
                }
                tail(this, kind_obj, sel, code_word(sel))
            }
            6 => {
                let r = lf_checker_rt::callee_thiscall!(PRED, u32, sel, extra, kind_obj);
                if r & 0xff != 0 {
                    let mut triple = [
                        rd32(this + TRIPLE),
                        rd32(this + TRIPLE + 4),
                        rd32(this + TRIPLE + 8),
                    ];
                    let p = triple.as_mut_ptr() as u32;
                    lf_checker_rt::callee_thiscall!(PREPARE, u32, p + 16);
                    lf_checker_rt::callee_thiscall!(COMBINE_STAGED, u32, sel, p + 16, p, p, 0u32);
                    let slot: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(rd32(rd32(kind_obj) + SLOT_B) as usize);
                    slot(kind_obj, sel);
                }
                tail(this, kind_obj, sel, code_word(sel))
            }
            _ => 0,
        }
    }
});
