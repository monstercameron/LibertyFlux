// original: 0x00D7EF80 input_slot_resolve_and_emit (proposed)

/// Resolve one input slot of a controller object and emit its events.
///
/// `obj` points to a controller object; `flag` is nonzero (low byte only)
/// when the slot array must be reset first. The object holds a switch word
/// at `+0xF4A` (nonzero triggers a notification callee with the word),
/// a 14-entry dword slot array at `+0xDD4`, a parameter block at `+0xE48`,
/// a mode byte at `+0xE73` (bit 1, inverted, is passed on), two selector
/// dwords at `+0xDE0`/`+0xDE8` (slots 3 and 5 of the same array), a status
/// word at `+0xDE4` (slot 4), and a version dword at `+0x1304`.
///
/// Behaviour: optionally reset the slot array, ask the array for its
/// current index, fetch the entry there, and translate it through a
/// runtime table: the low word selects the row, the high word scales by
/// the row stride (32). Three signed 16-bit values at row offsets
/// `+0x14`/`+0x16`/`+0x18` are converted to float and scaled (the first
/// two by one constant, the third by another), then handed with the
/// entry, the object and several flags to an emit callee with a fixed
/// object handle. A status of `0xFFFF` returns 1 at once; otherwise two
/// per-slot callees run, entries are walked forward while below slot 14
/// and not `0xFFFF` (each emitted), and the array is reset again when
/// the two selectors are equal and neither is `0xFFFF`. Returns 1 on
/// the early path, 0 otherwise (low byte only; the upper bytes keep
/// whatever the last callee left).
///
/// Original: 0x00D7EF80 (cdecl, two stack words). The parameter-block
/// callee is a normal callee-cleanup function (the callee pops 8 bytes at its end); its
/// out-word is then fed to the emit callee, and the saved slot pointer
/// below it survives for the two per-slot calls.
lf_checker_rt::export!(cdecl, rw_00d7ef80(obj: u32, flag: u32) -> u32 {
    unsafe {
        const SLOT_BASE: u32 = 0x0DD4;
        const PARAM_BASE: u32 = 0x0E48;
        const SWITCH_WORD: u32 = 0x0F4A;
        const MODE_BYTE: u32 = 0x0E73;
        const SELECTOR_A: u32 = 0x0DE0;
        const STATUS_WORD: u32 = 0x0DE4;
        const SELECTOR_B: u32 = 0x0DE8;
        const VERSION_WORD: u32 = 0x1304;
        const SLOT_COUNT: u32 = 0x0E;
        const ROW_STRIDE: u32 = 32;
        const ROW_V0: u32 = 0x14;
        const ROW_V1: u32 = 0x16;
        const ROW_V2: u32 = 0x18;
        const MISSING: u32 = 0xFFFF;
        const EMIT_CONST: u32 = 0x497423FE;
        const ROW_TABLE: u32 = 0x01178284;
        const EMIT_HANDLE: u32 = 0x01177A80;
        const SCALE_LO: u32 = 0x00FE87A4;
        const SCALE_HI: u32 = 0x00FE8720;
        const ARG_DWORD: u32 = 0x010330DC;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd16s(a: u32) -> i32 {
            unsafe { (a as *const u16).read_unaligned() as i16 as i32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let switch = rd16(obj.wrapping_add(SWITCH_WORD));
        if switch != 0 {
            lf_checker_rt::callee_cdecl!(1, u32, switch);
        }
        let slots = obj.wrapping_add(SLOT_BASE);
        if (flag as u8) != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, slots);
        }
        lf_checker_rt::callee_thiscall!(3, u32, slots);
        let index = lf_checker_rt::callee_thiscall!(4, u32, slots);
        let mut scratch: u32 = 0;
        lf_checker_rt::callee_thiscall!(
            5,
            u32,
            obj.wrapping_add(PARAM_BASE),
            &mut scratch as *mut u32 as u32,
            obj
        );
        let entry_ptr = slots.wrapping_add(index.wrapping_mul(4));
        let entry = rd32(entry_ptr);
        let row = rd32(
            lf_checker_rt::relocated(ROW_TABLE).wrapping_add((entry & 0xFFFF).wrapping_mul(4)),
        );
        let fields = row.wrapping_add((entry >> 16).wrapping_mul(ROW_STRIDE));
        let scale_lo = (lf_checker_rt::global::<f32>(SCALE_LO) as *const f32).read_unaligned();
        let scale_hi = (lf_checker_rt::global::<f32>(SCALE_HI) as *const f32).read_unaligned();
        let mut scaled = [
            mul(rd16s(fields.wrapping_add(ROW_V0)) as f32, scale_lo),
            mul(rd16s(fields.wrapping_add(ROW_V1)) as f32, scale_lo),
            mul(rd16s(fields.wrapping_add(ROW_V2)) as f32, scale_hi),
        ];
        // Dead stack slot the emit callee is handed (one word past the
        // saved slot pointer); never read afterwards, so it keeps the
        // initial fill, which the contract fixes to zero.
        let dead: u32 = 0;
        let version_eq = (rd32(obj.wrapping_add(VERSION_WORD)) == 2) as u32;
        let mode_bit = (((rd8(obj.wrapping_add(MODE_BYTE)) >> 1) & 1) == 0) as u32;
        let arg_dword = rd32(lf_checker_rt::relocated(ARG_DWORD));
        let handle = lf_checker_rt::relocated(EMIT_HANDLE);
        lf_checker_rt::callee_thiscall!(
            6,
            u32,
            handle,
            scaled.as_mut_ptr() as u32,
            entry,
            &mut scratch as *mut u32 as u32,
            entry_ptr,
            &dead as *const u32 as u32,
            SLOT_COUNT.wrapping_sub(index),
            0,
            EMIT_CONST,
            0,
            EMIT_CONST,
            mode_bit,
            arg_dword,
            0,
            version_eq,
            0,
            0,
            0,
            0
        );
        if rd16(obj.wrapping_add(STATUS_WORD)) == MISSING {
            return 1;
        }
        lf_checker_rt::callee_thiscall!(7, u32, slots, index);
        lf_checker_rt::callee_thiscall!(8, u32, slots, index);
        let mut i = index;
        let mut p = entry_ptr;
        while i < SLOT_COUNT {
            let e = rd32(p);
            if (e & 0xFFFF) == MISSING {
                break;
            }
            lf_checker_rt::callee_thiscall!(9, u32, handle, e);
            i = i.wrapping_add(1);
            p = p.wrapping_add(4);
        }
        let sel_a = rd32(obj.wrapping_add(SELECTOR_A));
        if (sel_a & 0xFFFF) != MISSING {
            let sel_b = rd32(obj.wrapping_add(SELECTOR_B));
            if (sel_b & 0xFFFF) != MISSING && sel_a == sel_b {
                lf_checker_rt::callee_thiscall!(2, u32, slots);
            }
        }
        0
    }
});
