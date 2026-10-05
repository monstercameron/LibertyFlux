// original: 0x0089BF00 rage::audTwinLoopSound::vf7 (symbols)

/// Start a twin-loop sound from a parameter block.
///
/// `this` is an `audTwinLoopSound`, `arg1`/`arg2` are opaque words passed
/// through to the base start helper, and `arg3` points at a parameter
/// block whose first 24 bytes select the two loop slots. The base helper
/// (callee 0, `thiscall` with the three words) runs first; a zero answer
/// returns 0. The parameter head is copied to scratch and each loop slot
/// is resolved through the attach helper (callee 1 then callee 2, both
/// `thiscall` on the global registry with the selector dword from the
/// descriptor at `+0x1D`/`+0x25`, `this`, `arg2`, and `arg3` for the
/// first call but the scratch copy for the second): a null answer means
/// slot `0xFF`, otherwise the slot index is `(answer - row_base) / stride`
/// (unsigned divide; `row_base` from the global table by the category byte
/// at `+0x40`, `stride` global), stored at `+0x48`/`+0x49`. A `0xFF` slot
/// or a null resolved row returns 0. Otherwise descriptor words feed
/// `+0xB4`/`+0xB8`/`+0xBC`/`+0xC0`, four descriptor dwords (`+0xC`,
/// `+0x10`, `+0x14`, `+0x18`) each go through virtual slot `+0x10` of
/// `this` with the answers stored at `+0xCC`/`+0xD0`/`+0xC4`/`+0xC8` in
/// that order, the shape helper (callee 3, `thiscall` on `this+0xDC` with
/// `1, 1, 0, 1`) runs, the tone helper (callee 4, `thiscall` on `this`
/// with no arguments) answers a float that is handed twice to the gain
/// helper (callee 5, `thiscall` on `this+0xDC`), and the descriptor dword
/// at `+0x8` is matched against two global ids: the first match stores 0
/// at `+0xD8` and returns 1, the second stores 1 and returns 1, otherwise
/// 0 returns. The two id globals are guarded by initialisation flags
/// whose fill paths call into the encrypted first megabyte of the code
/// section; the proof pins the flags to the initialised state, so those
/// two paths are not covered. All comparisons on helper answers are
/// zero/non-zero or equality tests and the slot divide cannot overflow,
/// so no signedness applies. The original stages the tone answer in its
/// incoming third-argument slot; the rewrite keeps it in a local (the
/// stack check is off for that reason; the value is observed through the
/// gain helper's compared float arguments instead).
///
/// Original: 0x0089BF00 (thiscall, three stack words, returns `al`).
lf_checker_rt::export!(thiscall, rw_0089BF00(this: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    unsafe {
        const CAT_INDEX: u32 = 0x40;
        const DESCRIPTOR: u32 = 0x94;
        const SLOT_A: u32 = 0x48;
        const SLOT_B: u32 = 0x49;
        const CAT_STRIDE: u32 = 0x6F40;
        const TABLE_ROW: u32 = 0x6F10;
        const NO_SLOT: u8 = 0xFF;
        const G_REGISTRY: u32 = 0x115DC18;
        const G_STRIDE: u32 = 0x115D964;
        const G_TABLE_BASE: u32 = 0x115D988;
        const G_INIT_FLAGS: u32 = 0x115F81C;
        const G_ID_FIRST: u32 = 0x115F818;
        const G_ID_SECOND: u32 = 0x115F820;
        const ONE_BITS: u32 = 0x3F800000;
        const VTABLE_SLOT: u32 = 0x10;
        const SHAPE_OBJ: u32 = 0xDC;

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
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let base_ok: u32 = lf_checker_rt::callee_thiscall!(0, u32, this, arg1, arg2, arg3);
        if (base_ok & 0xFF) == 0 {
            return 0;
        }
        // Scratch copy of the parameter head; the second attach call
        // receives a pointer to it (compared by snapshot, not by address).
        let head = [rd64(arg3), rd64(arg3 + 8), rd64(arg3 + 0x10)];
        let desc = rd32(this + DESCRIPTOR);
        let table_base = (lf_checker_rt::global::<u32>(G_TABLE_BASE)).read_unaligned();
        let stride = (lf_checker_rt::global::<u32>(G_STRIDE)).read_unaligned();
        let cat = rd8(this + CAT_INDEX) as u32;
        let row = rd32(table_base.wrapping_add(cat.wrapping_mul(CAT_STRIDE)).wrapping_add(TABLE_ROW));
        let slot_of = |answer: u32| -> u8 {
            if answer == 0 {
                NO_SLOT
            } else {
                (answer.wrapping_sub(row) / stride) as u8
            }
        };
        let sel_a = rd32(desc + 0x1D);
        let ans_a: u32 = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            lf_checker_rt::relocated(G_REGISTRY),
            sel_a,
            this,
            arg2,
            arg3
        );
        let slot_a = slot_of(ans_a);
        ((this + SLOT_A) as *mut u8).write(slot_a);
        let sel_b = rd32(desc + 0x25);
        let ans_b: u32 = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            lf_checker_rt::relocated(G_REGISTRY),
            sel_b,
            this,
            arg2,
            head.as_ptr() as u32
        );
        let slot_b = slot_of(ans_b);
        ((this + SLOT_B) as *mut u8).write(slot_b);
        if slot_a == NO_SLOT {
            return 0;
        }
        let cat_row = rd32(table_base.wrapping_add(cat.wrapping_mul(CAT_STRIDE)).wrapping_add(TABLE_ROW));
        if stride.wrapping_mul(slot_a as u32).wrapping_add(cat_row) == 0 {
            return 0;
        }
        if slot_b == NO_SLOT {
            return 0;
        }
        if stride.wrapping_mul(slot_b as u32).wrapping_add(cat_row) == 0 {
            return 0;
        }
        wr32(this + 0xB4, rd16(desc + 4) as u32);
        wr32(this + 0xB8, rd16(desc + 6) as u32);
        wr32(this + 0xBC, rd16(desc) as u32);
        wr32(this + 0xC0, rd16(desc + 2) as u32);
        let vtable = rd32(this);
        let hook: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + VTABLE_SLOT) as usize);
        wr32(this + 0xCC, hook(this, rd32(desc + 0xC)));
        wr32(this + 0xD0, hook(this, rd32(desc + 0x10)));
        wr32(this + 0xC4, hook(this, rd32(desc + 0x14)));
        wr32(this + 0xC8, hook(this, rd32(desc + 0x18)));
        lf_checker_rt::callee_thiscall!(3, u32, this + SHAPE_OBJ, ONE_BITS, ONE_BITS, 0, ONE_BITS);
        let tone: f32 = lf_checker_rt::callee_thiscall!(4, f32, this);
        lf_checker_rt::callee_thiscall!(5, u32, this + SHAPE_OBJ, tone.to_bits(), tone.to_bits());
        let flags = (lf_checker_rt::global::<u32>(G_INIT_FLAGS)).read_unaligned();
        let _ = flags;
        let key = rd32(desc + 8);
        if key == (lf_checker_rt::global::<u32>(G_ID_FIRST)).read_unaligned() {
            wr32(this + 0xD8, 0);
            1
        } else if key == (lf_checker_rt::global::<u32>(G_ID_SECOND)).read_unaligned() {
            wr32(this + 0xD8, 1);
            1
        } else {
            0
        }
    }
});
