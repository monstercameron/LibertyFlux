// original: 0x00dbad70 ui_pause_menu_bars_build (proposed)

/// Build the pause-menu rollover bar set: pick a display aspect, create one
/// child item through the shared allocator and constructor, configure it
/// through a fixed sequence of virtual calls, and tail-forward to the
/// item's slot `0x200`.
///
/// `this` points to the menu object; the new item pointer is stored at
/// `+4`. Two display-dimension globals are selected by two polled answers
/// (each answer's low byte: zero keeps the first word, nonzero the second)
/// and converted to float: `ratio = float(w) / float(h)`. The bar scale is
/// then `1 / (gain / ratio) * (flag == 0 ? 0.4 : 0.3)` where `gain` and
/// `flag` are globals, computed with the original's divide order. The item
/// (0x610 bytes) is allocated, constructed with the two name strings, and
/// configured: slot `0x1cc` with (18.0, probe answer, 0, 2), slot `0x208`
/// with a second probe answer, slots `0x1f8` and `0x1e0` with (0) and
/// (caller's word, 0), two `0x100` placements each carrying a 24-byte block
/// copied from the layout callee's answer plus (4, name, 4) and (2, name, 8),
/// slot `0x1e8` with the bar scale, a marker byte set at `+0x20b`, and slots
/// `0x1d4` (0x12), `0x1ec` (1.0), `0x1f0`, `0x1fc` (0), `0x120` (0). The
/// outgoing tail jump becomes a forwarding call whose answer is returned.
///
/// Original: 0x00dbad70 (thiscall, one stack word, forwarded opaquely).
lf_checker_rt::export!(thiscall, rw_00dbad70(this: u32, arg0: u32) -> u32 {
    unsafe {
        const DIMS: u32 = 0x0105c87c;
        const FLAG: u32 = 0x01160cc8;
        const GAIN: u32 = 0x017a669c;
        const ONE: f32 = f32::from_bits(0x3f800000);
        const SCALE_A: f32 = f32::from_bits(0x3ecccccd); // 0.4
        const SCALE_B: f32 = f32::from_bits(0x3e99999a); // 0.3
        const ITEM_SIZE: u32 = 0x610;
        const NAME_PARENT: u32 = 0x00ef3b40;
        const NAME_ITEM: u32 = 0x00ef3b4c;
        const NAME_BAR_A: u32 = 0x00ef3b5c;
        const NAME_BAR_B: u32 = 0x00ef3b70;
        const ITEM_SLOT: u32 = 0x04;
        const MARKER: u32 = 0x20b;
        const SLOT_PLACE: u32 = 0x100;
        const SLOT_SHOW: u32 = 0x120;
        const SLOT_SETUP: u32 = 0x1cc;
        const SLOT_D4: u32 = 0x1d4;
        const SLOT_TEXT: u32 = 0x1e0;
        const SLOT_SCALE: u32 = 0x1e8;
        const SLOT_EC: u32 = 0x1ec;
        const SLOT_F0: u32 = 0x1f0;
        const SLOT_F8: u32 = 0x1f8;
        const SLOT_FC: u32 = 0x1fc;
        const SLOT_TAIL: u32 = 0x200;
        const SLOT_208: u32 = 0x208;
        const CALLEE_POLL_A: u32 = 1;
        const CALLEE_POLL_B: u32 = 2;
        const CALLEE_NEW: u32 = 3;
        const CALLEE_CTOR: u32 = 4;
        const CALLEE_PROBE: u32 = 5;
        const CALLEE_SETUP: u32 = 6;
        const CALLEE_ONE_ARG: u32 = 7;
        const CALLEE_TEXT: u32 = 8;
        const CALLEE_LAYOUT: u32 = 9;
        const CALLEE_PLACE: u32 = 10;
        const CALLEE_RELEASE: u32 = 11;
        const CALLEE_TAIL: u32 = 12;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        unsafe fn vslot(obj: u32, slot: u32) -> u32 {
            unsafe { rd32(rd32(obj) + slot) }
        }
        unsafe fn call1(obj: u32, slot: u32, a: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(vslot(obj, slot) as usize);
                f(obj, a)
            }
        }

        let base = lf_checker_rt::relocated(DIMS);
        let w = if lf_checker_rt::callee_thiscall!(CALLEE_POLL_A, u32, this) as u8 != 0 {
            rd32(base + 12)
        } else {
            rd32(base + 8)
        };
        let h = if lf_checker_rt::callee_thiscall!(CALLEE_POLL_B, u32, this) as u8 != 0 {
            rd32(base)
        } else {
            rd32(base + 4)
        };
        let ratio = div(w as i32 as f32, h as i32 as f32);
        let gain = f32::from_bits(rd32(lf_checker_rt::relocated(GAIN)));
        let flag = rd32(lf_checker_rt::relocated(FLAG));
        let inv = div(ONE, div(gain, ratio));
        let aspect = if flag == 0 { mul(inv, SCALE_A) } else { mul(inv, SCALE_B) };

        let fresh = lf_checker_rt::callee_cdecl!(CALLEE_NEW, u32, ITEM_SIZE);
        let obj = lf_checker_rt::callee_thiscall!(
            CALLEE_CTOR,
            u32,
            fresh,
            lf_checker_rt::relocated(NAME_ITEM),
            lf_checker_rt::relocated(NAME_PARENT)
        );
        ((this + ITEM_SLOT) as *mut u32).write_unaligned(obj);

        let mut probe_out = [0u32; 2];
        let p1 = lf_checker_rt::callee_cdecl!(
            CALLEE_PROBE,
            u32,
            probe_out.as_mut_ptr() as u32,
            7u32
        );
        let setup: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(vslot(obj, SLOT_SETUP) as usize);
        setup(obj, 0x41900000, p1, 0, 2);

        let p2 = lf_checker_rt::callee_cdecl!(
            CALLEE_PROBE,
            u32,
            probe_out.as_mut_ptr() as u32,
            0x44u32
        );
        call1(obj, SLOT_208, p2);
        call1(obj, SLOT_F8, 0);
        let text: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(vslot(obj, SLOT_TEXT) as usize);
        text(obj, arg0, 0);

        let mut layout_out = [0u32; 2];
        let l1 = lf_checker_rt::callee_thiscall!(
            CALLEE_LAYOUT,
            u32,
            layout_out.as_mut_ptr() as u32,
            0u32,
            0x41aecccd_u32
        );
        let place: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(vslot(obj, SLOT_PLACE) as usize);
        place(
            obj,
            4,
            lf_checker_rt::relocated(NAME_BAR_A),
            4,
            rd32(l1),
            rd32(l1 + 4),
            rd32(l1 + 8),
            rd32(l1 + 12),
            rd32(l1 + 16),
            rd32(l1 + 20),
        );
        lf_checker_rt::callee_thiscall!(CALLEE_RELEASE, u32, layout_out.as_mut_ptr() as u32);

        let l2 = lf_checker_rt::callee_thiscall!(
            CALLEE_LAYOUT,
            u32,
            layout_out.as_mut_ptr() as u32,
            0u32,
            0u32
        );
        place(
            obj,
            2,
            lf_checker_rt::relocated(NAME_BAR_B),
            8,
            rd32(l2),
            rd32(l2 + 4),
            rd32(l2 + 8),
            rd32(l2 + 12),
            rd32(l2 + 16),
            rd32(l2 + 20),
        );
        lf_checker_rt::callee_thiscall!(CALLEE_RELEASE, u32, layout_out.as_mut_ptr() as u32);

        call1(obj, SLOT_SCALE, aspect.to_bits());
        ((obj + MARKER) as *mut u8).write(1);
        call1(obj, SLOT_D4, 0x12);
        call1(obj, SLOT_EC, 0x3f800000);
        call1(obj, SLOT_F0, 0x3f8ccccd);
        call1(obj, SLOT_FC, 0);
        call1(obj, SLOT_SHOW, 0);
        call1(obj, SLOT_TAIL, 1)
    }
});
