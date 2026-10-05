// original: 0x00ddd790 uitextfield_build_children (proposed)

/// Build the field's six child items and wire them into the layout.
///
/// `this` is the field object; thirteen stack words configure the build.
/// Two entry bytes/words are stored first (low byte of word 10 at `+0x20f`,
/// word 7 as float bits at `+0x21c`). Then six items are created at
/// `+0x1e4`, `+0x1e0`, `+0x1e8`, `+0x1f4`, `+0x1f0` and `+0x1ec`, each by
/// allocating (0x25c or 0x610 bytes), chaining two parent slot-`0x48`
/// answers through the probe helper with a per-block key, and constructing.
/// Blocks 1, 2, 4 and 5 attach with (0, 0, word, -1) where the word is an
/// incoming argument (words 0, 1 and 4); block 4 instead zeroes word 7 and
/// attaches with its address. Blocks 3 and 6 run the probe/setup/tail
/// sequence (probe code 7, slot `0x1cc` with (18.0f/19.0f bits, probe
/// answer, 0, 2), then slots `0x208`/`0x1fc`/`0x118`/`0x1e0`/`0x200`).
/// Placement blocks carry the layout callee's 24-byte answer plus scalar
/// pairs into slot `0x114` (blocks 1, 3, 6), or scope an older sibling
/// through slot `0x4c` first and place into `0x10c` (block 2, scoping
/// block 1 with no scalar) or `0x104` (blocks 4 and 5, scoping blocks 3
/// and 4 with one scalar). Blocks 4 and 5 finish with a zero marker at
/// item `+0x1d8` and flag calls; blocks 3 and 6 run one extra placement
/// when the low byte of word 9 is nonzero. The tail stores the length of
/// the word-5 string (low byte) at `+0x208`, words 10-12 and several
/// constants across `+0x200`-`+0x218`, scopes the parent and ends with
/// parent slot `0x13c` (flag 1), whose answer is returned.
///
/// Cleanup notes (all forced by frame balance, verified by simulation to
/// net zero): the allocator pops its size word; parent slot `0x48` pops
/// nothing at either chained call; the probe helper is cdecl/3 with the
/// key deepest; slot `0x4c` pops only its own scalars (none on the two
/// block-2 sites and the tail site, one elsewhere), so sibling items use
/// two fabricated vtables differing only in that slot; slots `0x104` and
/// `0x10c` pop their scalars plus the carried 24-byte block (8 words) and
/// slot `0x114` pops its scalar plus the block (7 words).
///
/// Edge cases: a null allocator answer skips its block (never exercised:
/// the contract's allocator always succeeds); word 9 selects the two
/// conditional placements; word 8 is read by nobody.
///
/// Original: 0x00ddd790 (thiscall, thirteen stack words).
lf_checker_rt::export!(
    thiscall,
    rw_00ddd790(
        this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32,
        a6: u32, a7: u32, a8: u32, a9: u32, a10: u32, a11: u32, a12: u32
    ) -> u32 {
    unsafe {
        const SMALL: u32 = 0x25c;
        const BIG: u32 = 0x610;
        const KEY1: u32 = 0x00efcf74;
        const KEY2: u32 = 0x00efcf80;
        const KEY3: u32 = 0x00efcf90;
        const KEY4: u32 = 0x00efcfa0;
        const KEY5: u32 = 0x00efcfb0;
        const KEY6: u32 = 0x00efcfbc;
        const OFF_B1: u32 = 0x1e4;
        const OFF_B2: u32 = 0x1e0;
        const OFF_B3: u32 = 0x1e8;
        const OFF_B4: u32 = 0x1f4;
        const OFF_B5: u32 = 0x1f0;
        const OFF_B6: u32 = 0x1ec;
        const ITEM_MARK: u32 = 0x1d8;
        const SLOT_SCOPE: u32 = 0x48;
        const SLOT_SCOPE2: u32 = 0x4c;
        const SLOT_PLACE: u32 = 0x104;
        const SLOT_PLACE_B: u32 = 0x10c;
        const SLOT_PLACE_A: u32 = 0x114;
        const SLOT_118: u32 = 0x118;
        const SLOT_120: u32 = 0x120;
        const SLOT_170: u32 = 0x170;
        const SLOT_13C: u32 = 0x13c;
        const SLOT_94: u32 = 0x94;
        const SLOT_SETUP: u32 = 0x1cc;
        const SLOT_TEXT: u32 = 0x1e0;
        const SLOT_FC: u32 = 0x1fc;
        const SLOT_200: u32 = 0x200;
        const SLOT_COLOUR: u32 = 0x208;
        const F_18: u32 = 0x41700000;
        const F_19: u32 = 0x41900000;
        const F_3: u32 = 0x40400000;
        const F_NEG3: u32 = 0xc0400000;
        const F_2: u32 = 0x40000000;
        const CALLEE_NEW: u32 = 1;
        const CALLEE_SCOPE: u32 = 2;
        const CALLEE_PROBE: u32 = 3;
        const CALLEE_CTOR_A: u32 = 4;
        const CALLEE_CTOR_B: u32 = 5;
        const CALLEE_ATTACH_V: u32 = 6;
        const CALLEE_ATTACH_P: u32 = 7;
        const CALLEE_LAYOUT: u32 = 8;
        const CALLEE_RELEASE: u32 = 14;
        const CALLEE_PROBE2: u32 = 15;
        const CALLEE_SCOPE2_0: u32 = 12;
        const CALLEE_SCOPE2_1: u32 = 13;
        const CALLEE_T170: u32 = 24;
        const CALLEE_T13C: u32 = 25;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        unsafe fn vslot(obj: u32, slot: u32) -> u32 {
            unsafe { rd32(rd32(obj) + slot) }
        }
        unsafe fn call0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(vslot(obj, slot) as usize);
                f(obj)
            }
        }
        unsafe fn call1(obj: u32, slot: u32, a: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(vslot(obj, slot) as usize);
                f(obj, a)
            }
        }
        unsafe fn call2(obj: u32, slot: u32, a: u32, b: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(vslot(obj, slot) as usize);
                f(obj, a, b)
            }
        }
        /// Allocate, scope twice, probe and construct one item; store it.
        ///
        /// Stack discipline mirrors the original exactly: the allocator is
        /// invoked through the cdecl macro (so this side emits the same
        /// compensating pop as the original's `(an instruction of the original)`) while the stub
        /// pops the size word; the first scope answer stays on the stack
        /// and doubles as the probe's third word, so the probe is invoked
        /// with two words only.
        unsafe fn make(this: u32, off: u32, size: u32, key: u32, ctor: u32) -> u32 {
            unsafe {
                let fresh: u32 = lf_checker_rt::callee_cdecl!(CALLEE_NEW, u32, size);
                let r1 = call0(this, SLOT_SCOPE);
                // The second scope call carries the first answer on the stack;
                // the slot itself pops nothing (see the doc comment), so the
                // word is still there for the probe below.
                let scope1: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(vslot(this, SLOT_SCOPE) as usize);
                let r2 = scope1(this, r1);
                let f: u32 = lf_checker_rt::callee_cdecl!(
                    CALLEE_PROBE, u32,
                    lf_checker_rt::relocated(key),
                    r2
                );
                let obj: u32 = lf_checker_rt::callee_thiscall!(ctor, u32, fresh, f);
                ((this + off) as *mut u32).write_unaligned(obj);
                obj
            }
        }
        /// One placement into slot `0x114`: layout block plus one scalar.
        unsafe fn place_a(obj: u32, c: u32, f0: u32, f1: u32, lo: *mut u32) {
            unsafe {
                let l: u32 =
                    lf_checker_rt::callee_thiscall!(CALLEE_LAYOUT, u32, lo as u32, f0, f1);
                let put: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(vslot(obj, SLOT_PLACE_A) as usize);
                put(obj, c, rd32(l), rd32(l + 4), rd32(l + 8), rd32(l + 12), rd32(l + 16), rd32(l + 20));
                lf_checker_rt::callee_thiscall!(CALLEE_RELEASE, u32, lo as u32);
            }
        }
        /// One placement with a scoping call first (slots `0x104`/`0x10c`).
        unsafe fn place_scoped(
            obj: u32, scope_obj: u32, slot: u32, scope_id: u32, scope_arg: Option<u32>,
            place_arg: u32, f0: u32, f1: u32, lo: *mut u32,
        ) {
            unsafe {
                let _ = scope_id;
                let l: u32 =
                    lf_checker_rt::callee_thiscall!(CALLEE_LAYOUT, u32, lo as u32, f0, f1);
                let scoped = match scope_arg {
                    Some(s) => call1(scope_obj, SLOT_SCOPE2, s),
                    None => call0(scope_obj, SLOT_SCOPE2),
                };
                let put: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(vslot(obj, slot) as usize);
                put(
                    obj, place_arg, scoped,
                    rd32(l), rd32(l + 4), rd32(l + 8),
                    rd32(l + 12), rd32(l + 16), rd32(l + 20),
                );
                lf_checker_rt::callee_thiscall!(CALLEE_RELEASE, u32, lo as u32);
            }
        }
        /// The probe/setup/tail sequence shared by the two big blocks.
        unsafe fn setup_tail(
            obj: u32, fconst: u32, colour_arg: u32, fc_arg: u32, e118_arg: u32,
            text_arg: u32, po: *mut u32,
        ) {
            unsafe {
                let p: u32 = lf_checker_rt::callee_cdecl!(CALLEE_PROBE2, u32, po as u32, 7u32);
                let setup: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(vslot(obj, SLOT_SETUP) as usize);
                setup(obj, fconst, p, 0, 2);
                call1(obj, SLOT_COLOUR, colour_arg);
                call1(obj, SLOT_FC, fc_arg);
                call1(obj, SLOT_118, e118_arg);
                call2(obj, SLOT_TEXT, text_arg, 0);
                call1(obj, SLOT_200, 1);
            }
        }

        ((this + 0x20f) as *mut u8).write(a10 as u8);
        ((this + 0x21c) as *mut u32).write_unaligned(a7);
        let mut probe_out = [0u32; 2];
        let mut layout_out = [0u32; 2];
        let po = probe_out.as_mut_ptr();
        let lo = layout_out.as_mut_ptr();

        // Block 1.
        let b1 = make(this, OFF_B1, SMALL, KEY1, CALLEE_CTOR_A);
        lf_checker_rt::callee_thiscall!(CALLEE_ATTACH_V, u32, b1, 0u32, 0u32, a0, 0xFFFF_FFFF);
        place_a(b1, 6, 0, 0, lo);
        place_a(b1, 0x18, 0, 0, lo);

        // Block 2 (scopes block 1). Items are reloaded from the parent
        // after every later allocation, exactly like the original: holding
        // them in locals across the shared-push sequence below is not sound
        // (the second scope push doubles as the probe's third word, which
        // Rust cannot see, so its stack model drifts inside make()).
        let b2 = make(this, OFF_B2, SMALL, KEY2, CALLEE_CTOR_A);
        lf_checker_rt::callee_thiscall!(CALLEE_ATTACH_V, u32, b2, 0u32, 0u32, a1, 0xFFFF_FFFF);
        let b1r = rd32(this + OFF_B1);
        place_scoped(b2, b1r, SLOT_PLACE_B, CALLEE_SCOPE2_0, None, 6, F_3, F_3, lo);
        place_scoped(b2, b1r, SLOT_PLACE_B, CALLEE_SCOPE2_0, None, 0x18, F_NEG3, F_NEG3, lo);

        // Block 3.
        let b3 = make(this, OFF_B3, BIG, KEY3, CALLEE_CTOR_B);
        setup_tail(b3, F_18, a2, 1, 0, a5, po);
        if (a9 as u8) != 0 {
            place_a(b3, 2, 0, 0, lo);
        }

        // Block 4 (attaches with the zeroed word 7 by address).
        let b4 = make(this, OFF_B4, SMALL, KEY4, CALLEE_CTOR_A);
        let a7p = &a7 as *const u32 as u32;
        (a7p as *mut u32).write(0);
        lf_checker_rt::callee_thiscall!(CALLEE_ATTACH_P, u32, b4, 0u32, 0u32, a7p, 0xFFFF_FFFF);
        let b3r = rd32(this + OFF_B3);
        place_scoped(b4, b3r, SLOT_PLACE, CALLEE_SCOPE2_1, Some(4), 4, 0, 0, lo);
        place_scoped(b4, b3r, SLOT_PLACE, CALLEE_SCOPE2_1, Some(0x10), 0x10, 0, 0, lo);
        place_scoped(b4, b3r, SLOT_PLACE, CALLEE_SCOPE2_1, Some(2), 2, 0, 0, lo);
        ((b4 + ITEM_MARK) as *mut u32).write_unaligned(0);
        call1(b4, SLOT_94, 0);
        call1(b4, SLOT_118, 1);

        // Block 5.
        let b5 = make(this, OFF_B5, SMALL, KEY5, CALLEE_CTOR_A);
        lf_checker_rt::callee_thiscall!(CALLEE_ATTACH_V, u32, b5, 0u32, 0u32, a4, 0xFFFF_FFFF);
        let b3r2 = rd32(this + OFF_B3);
        let b4r = rd32(this + OFF_B4);
        place_scoped(b5, b3r2, SLOT_PLACE, CALLEE_SCOPE2_1, Some(4), 4, 0, 0, lo);
        place_scoped(b5, b3r2, SLOT_PLACE, CALLEE_SCOPE2_1, Some(0x10), 0x10, 0, 0, lo);
        place_scoped(b5, b4r, SLOT_PLACE, CALLEE_SCOPE2_1, Some(8), 2, 0, 0, lo);
        ((b5 + ITEM_MARK) as *mut u32).write_unaligned(0);
        call1(b5, SLOT_94, F_2);
        call1(b5, SLOT_118, 0);
        call1(b5, SLOT_120, 0);

        // Block 6.
        let b6 = make(this, OFF_B6, BIG, KEY6, CALLEE_CTOR_B);
        setup_tail(b6, F_19, a3, 0, 1, a6, po);
        if (a9 as u8) != 0 {
            place_a(b6, 2, 0, 0, lo);
        }

        // Tail.
        let mut len = 0u32;
        while rd8(a5.wrapping_add(len)) != 0 {
            len = len.wrapping_add(1);
        }
        ((this + 0x208) as *mut u8).write(len as u8);
        ((this + 0x20b) as *mut u8).write(a10 as u8);
        ((this + 0x218) as *mut u32).write_unaligned(a11);
        ((this + 0x200) as *mut u32).write_unaligned(0);
        ((this + 0x209) as *mut u8).write(0);
        ((this + 0x20c) as *mut u16).write_unaligned(1);
        ((this + 0x20e) as *mut u8).write(0);
        ((this + 0x210) as *mut u32).write_unaligned(1);
        ((this + 0x214) as *mut u32).write_unaligned(a12);
        let s = call0(this, SLOT_SCOPE2);
        call1(this, SLOT_170, s);
        call1(this, SLOT_13C, 1)
    }
    }
);
