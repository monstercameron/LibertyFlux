// original: 0x00dbb0e0 ui_montage_upload_panel_build (proposed)

/// Build the six-item montage upload panel, unless the parent's gate says
/// it already exists.
///
/// `this` is the parent object. Slot `0x140` of its table is polled first:
/// a nonzero low byte skips everything and returns that answer. Otherwise
/// six child items are created at `+0x1e8`, `+0x1e0`, `+0x1e4`, `+0x1ec`,
/// `+0x1f0` and `+0x1f4`, each through the shared allocator (0x610 bytes)
/// and constructor, then configured through a fixed virtual-call sequence.
/// The first three items are constructed with a name string and one parent
/// slot-`0x48` answer; the last three combine two slot-`0x48` answers with
/// the format helper before construction. Every item gets the common head
/// (probe slot `0x1cc` with (18.0, probe answer, 0, 2)) and a placement
/// block: the layout callee's 24-byte answer is carried with a scalar pair
/// into the item's slot `0x104`, after a scoping call to slot `0x4c` on the
/// parent (first item) or an older sibling. Per-item differences: colours
/// through slot `0x208`, flags through `0x1fc`/`0x118`/`0x120`, sizes
/// through `0x80`, enables through `0x200`, and text through `0x1e0`.
/// Between the fifth and sixth items a global tick counter is split into
/// quotient and remainder by 30 (the original's multiply-and-shift
/// sequence plus a multiply-back subtraction) and formatted as three
/// two-digit fields; the sixth item's text call carries that buffer.
/// The panel ends with parent slot `0x13c` (flag 1), whose answer is
/// returned.
///
/// Original: 0x00dbb0e0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00dbb0e0(this: u32) -> u32 {
    unsafe {
        const ITEM_SIZE: u32 = 0x610;
        const TICKS_GLOBAL: u32 = 0x0106c284;
        const DIV30_MAGIC: i64 = 0x88888889u32 as i32 as i64;
        const TIME_FORMAT: u32 = 0x00ef3d78;
        const NAME_DIVIDER: u32 = 0x00ef3cd4;
        const NAME_SELECTED: u32 = 0x00ef3cf4;
        const NAME_RUNNING: u32 = 0x00ef3d1c;
        const FMT_WARNING: u32 = 0x00ef3d44;
        const FMT_COLON: u32 = 0x00ef3d68;
        const FMT_MAXTIME: u32 = 0x00ef3d88;
        const TEXT_SLASH: u32 = 0x00ef3cf0;
        const TEXT_ZERO_A: u32 = 0x00ef3d10;
        const TEXT_ZERO_B: u32 = 0x00ef3d38;
        const TEXT_UPLOAD_LIM: u32 = 0x00ef3d58;
        const TEXT_COLON: u32 = 0x00ef3d74;
        const OFF_B1: u32 = 0x1e8;
        const OFF_B2: u32 = 0x1e0;
        const OFF_B3: u32 = 0x1e4;
        const OFF_B4: u32 = 0x1ec;
        const OFF_B5: u32 = 0x1f0;
        const OFF_B6: u32 = 0x1f4;
        const SLOT_SCOPE: u32 = 0x48;
        const SLOT_GATE: u32 = 0x140;
        const SLOT_DONE: u32 = 0x13c;
        const SLOT_PLACE2: u32 = 0x4c;
        const SLOT_SIZE: u32 = 0x80;
        const SLOT_PLACE: u32 = 0x104;
        const SLOT_118: u32 = 0x118;
        const SLOT_SHOW: u32 = 0x120;
        const SLOT_SETUP: u32 = 0x1cc;
        const SLOT_D4: u32 = 0x1d4;
        const SLOT_TEXT: u32 = 0x1e0;
        const SLOT_F8: u32 = 0x1f8;
        const SLOT_FC: u32 = 0x1fc;
        const SLOT_200: u32 = 0x200;
        const SLOT_COLOUR: u32 = 0x208;
        const COLOUR_DIM: u32 = 0xff6f6f6f;
        const COLOUR_WHITE: u32 = 0xffffffff;
        const COLOUR_HIGHLIGHT: u32 = 0xffc73103;
        const CALLEE_GATE: u32 = 1;
        const CALLEE_SCOPE: u32 = 2;
        const CALLEE_NEW: u32 = 3;
        const CALLEE_CTOR: u32 = 4;
        const CALLEE_FORMAT: u32 = 5;
        const CALLEE_PROBE: u32 = 6;
        const CALLEE_SETUP: u32 = 7;
        const CALLEE_ONE_ARG: u32 = 8;
        const CALLEE_COLOUR: u32 = 9;
        const CALLEE_SIZE: u32 = 10;
        const CALLEE_LAYOUT: u32 = 11;
        const CALLEE_SCOPE2: u32 = 12;
        const CALLEE_PLACE: u32 = 13;
        const CALLEE_RELEASE: u32 = 14;
        const CALLEE_TEXT: u32 = 15;
        const CALLEE_SPRINTF: u32 = 16;
        const CALLEE_COOKIE: u32 = 17;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        /// Allocate and construct one item of the first kind (name string
        /// plus one parent scope answer), store it at `this + off`.
        unsafe fn make_named(
            this: u32, off: u32, name: u32, probe_out: *mut u32, layout_out: *mut u32,
        ) -> u32 {
            unsafe {
                let fresh = lf_checker_rt::callee_cdecl!(CALLEE_NEW, u32, ITEM_SIZE);
                let _ = layout_out;
                let r = call0(this, SLOT_SCOPE);
                let obj = lf_checker_rt::callee_thiscall!(
                    CALLEE_CTOR,
                    u32,
                    fresh,
                    lf_checker_rt::relocated(name),
                    r
                );
                ((this + off) as *mut u32).write_unaligned(obj);
                let p = lf_checker_rt::callee_cdecl!(CALLEE_PROBE, u32, probe_out as u32, 7u32);
                let setup: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(vslot(obj, SLOT_SETUP) as usize);
                setup(obj, 0x41900000, p, 0, 2);
                obj
            }
        }
        /// Allocate and construct one item of the second kind (two parent
        /// scope answers combined by the format helper), store it at
        /// `this + off`.
        unsafe fn make_formatted(
            this: u32, off: u32, fmt: u32, probe_out: *mut u32, layout_out: *mut u32,
        ) -> u32 {
            unsafe {
                let fresh = lf_checker_rt::callee_cdecl!(CALLEE_NEW, u32, ITEM_SIZE);
                let _ = layout_out;
                let r1 = call0(this, SLOT_SCOPE);
                let r2 = call0(this, SLOT_SCOPE);
                let f = lf_checker_rt::callee_cdecl!(
                    CALLEE_FORMAT,
                    u32,
                    lf_checker_rt::relocated(fmt),
                    r2
                );
                let obj =
                    lf_checker_rt::callee_thiscall!(CALLEE_CTOR, u32, fresh, f, r1);
                ((this + off) as *mut u32).write_unaligned(obj);
                let p = lf_checker_rt::callee_cdecl!(CALLEE_PROBE, u32, probe_out as u32, 7u32);
                let setup: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(vslot(obj, SLOT_SETUP) as usize);
                setup(obj, 0x41900000, p, 0, 2);
                obj
            }
        }
        /// One placement block: layout answer plus scalar pair into slot
        /// `0x104` after scoping `scope_obj` through slot `0x4c`.
        unsafe fn place(
            obj: u32, scope_obj: u32, iconst: u32, scope_arg: u32, place_arg: u32,
            layout_out: *mut u32,
        ) {
            unsafe {
                let l = lf_checker_rt::callee_thiscall!(
                    CALLEE_LAYOUT,
                    u32,
                    layout_out as u32,
                    iconst,
                    0u32
                );
                let scoped = call1(scope_obj, SLOT_PLACE2, scope_arg);
                let put: extern "thiscall" fn(
                    u32, u32, u32, u32, u32, u32, u32, u32, u32,
                ) -> u32 = core::mem::transmute(vslot(obj, SLOT_PLACE) as usize);
                put(
                    obj,
                    place_arg,
                    scoped,
                    rd32(l),
                    rd32(l + 4),
                    rd32(l + 8),
                    rd32(l + 12),
                    rd32(l + 16),
                    rd32(l + 20),
                );
                lf_checker_rt::callee_thiscall!(CALLEE_RELEASE, u32, layout_out as u32);
            }
        }
        unsafe fn colour(obj: u32, value: u32) {
            unsafe {
                let mut c = value;
                call1(obj, SLOT_COLOUR, &mut c as *mut u32 as u32);
            }
        }

        let gate = call0(this, SLOT_GATE);
        if gate as u8 != 0 {
            lf_checker_rt::callee_cdecl!(CALLEE_COOKIE, u32,);
            return gate;
        }

        let mut probe_out = [0u32; 2];
        let mut layout_out = [0u32; 2];
        let po = probe_out.as_mut_ptr();
        let lo = layout_out.as_mut_ptr();

        // Item 1: divider.
        let b1 = make_named(this, OFF_B1, NAME_DIVIDER, po, lo);
        colour(b1, COLOUR_DIM);
        call1(b1, SLOT_FC, 1);
        call2(b1, SLOT_SIZE, 0x41000000, 0x41500000);
        call1(b1, SLOT_200, 1);
        place(b1, this, 0x42480000, 6, 6, lo);
        call2(b1, SLOT_TEXT, lf_checker_rt::relocated(TEXT_SLASH), 0);
        call1(b1, SLOT_F8, 0);
        call1(b1, SLOT_D4, 0x12);

        // Item 2: selected entry.
        let b2 = make_named(this, OFF_B2, NAME_SELECTED, po, lo);
        colour(b2, COLOUR_WHITE);
        call1(b2, SLOT_FC, 1);
        call1(b2, SLOT_200, 1);
        place(b2, b1, 0xbf800000, 6, 0x0c, lo);
        call2(b2, SLOT_TEXT, lf_checker_rt::relocated(TEXT_ZERO_A), 0);
        call1(b2, SLOT_D4, 0x12);
        call1(b2, SLOT_F8, 0);

        // Item 3: running time.
        let b3 = make_named(this, OFF_B3, NAME_RUNNING, po, lo);
        colour(b3, COLOUR_DIM);
        call1(b3, SLOT_FC, 1);
        call1(b3, SLOT_200, 1);
        place(b3, b1, 0x3f800000, 0x0c, 6, lo);
        call2(b3, SLOT_TEXT, lf_checker_rt::relocated(TEXT_ZERO_B), 0);
        call1(b3, SLOT_D4, 0x12);
        call1(b3, SLOT_F8, 0);

        // Item 4: warning line.
        let b4 = make_formatted(this, OFF_B4, FMT_WARNING, po, lo);
        place(b4, b1, 0x42700000, 0x0c, 6, lo);
        colour(b4, COLOUR_HIGHLIGHT);
        call1(b4, SLOT_118, 1);
        call1(b4, SLOT_SHOW, 0);
        call2(b4, SLOT_SIZE, 0x41200000, 0x41200000);
        call2(b4, SLOT_TEXT, lf_checker_rt::relocated(TEXT_UPLOAD_LIM), 0);
        call1(b4, SLOT_D4, 0x12);
        call1(b4, SLOT_F8, 0);

        // Item 5: colon separator.
        let b5 = make_formatted(this, OFF_B5, FMT_COLON, po, lo);
        place(b5, b4, 0x3f800000, 0x0c, 6, lo);
        colour(b5, COLOUR_HIGHLIGHT);
        call1(b5, SLOT_118, 1);
        call1(b5, SLOT_SHOW, 0);
        call2(b5, SLOT_TEXT, lf_checker_rt::relocated(TEXT_COLON), 0);
        call1(b5, SLOT_FC, 1);
        call1(b5, SLOT_D4, 0x12);
        call1(b5, SLOT_F8, 0);

        // Middle: split the tick counter into quotient and remainder by 30
        // and format all three lanes (the third lane is zeroes).
        let ticks = rd32(lf_checker_rt::relocated(TICKS_GLOBAL)) as i32;
        let prod = DIV30_MAGIC.wrapping_mul(ticks as i64);
        let mut q = (prod >> 32) as i32;
        q = q.wrapping_add(ticks);
        q >>= 5;
        q = q.wrapping_add((q as u32 >> 31) as i32);
        let mut back = q.wrapping_shl(4);
        back = back.wrapping_sub(q);
        back = back.wrapping_shl(2);
        let rem = ticks.wrapping_sub(back);
        let mut buf = [0u32; 8];
        lf_checker_rt::callee_cdecl!(
            CALLEE_SPRINTF,
            u32,
            buf.as_mut_ptr() as u32,
            lf_checker_rt::relocated(TIME_FORMAT),
            q as u32,
            rem as u32,
            0u32
        );

        // Item 6: max-time line, carrying the formatted buffer as its text.
        let b6 = make_formatted(this, OFF_B6, FMT_MAXTIME, po, lo);
        place(b6, b5, 0x3f800000, 0x0c, 6, lo);
        colour(b6, COLOUR_HIGHLIGHT);
        call1(b6, SLOT_118, 1);
        call1(b6, SLOT_SHOW, 0);
        call1(b6, SLOT_FC, 1);
        call2(b6, SLOT_TEXT, buf.as_mut_ptr() as u32, 0);
        call1(b6, SLOT_D4, 0x12);
        call1(b6, SLOT_F8, 0);

        let done = call1(this, SLOT_DONE, 1);
        lf_checker_rt::callee_cdecl!(CALLEE_COOKIE, u32,);
        done
    }
});
