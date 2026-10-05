// original: 0x00A4C9D0 vehicle_emit_event (proposed)

/// Emits an event for a linked pair selected by the argument's low byte.
///
/// With a zero low byte the pair is (`[this + AUX]` (8), `[this + HEAD]` (0));
/// otherwise it is (`[this + AUX]`, `[this + ALT]` (4)). Each side resolves
/// its target through `+0x6C`; any missing link returns at once (leaving
/// `eax` as the failed step left it: zero for a missing object, the object
/// for a missing target). On the full path: calls the classifying callee
/// (stdcall, `(arg-or-0, -1)`), then the measuring callee twice (thiscall,
/// each target in `ecx`, low 16 bits of each answer kept), then the emitting
/// callee (thiscall, constant object in `ecx`, the three answers on the
/// stack with the emit order's last answer first), and finally stores
/// `DONE` (0xC8) into the word at `this + STATE` (0x12) and 1 into the byte
/// at `this + READY` (0x10). The emit object's address is an immediate with
/// a relocation, so it arrives relocated and the rewrite relocates it too.
///
/// Original: 0x00A4C9D0 (thiscall, one stack word), four callees.
lf_checker_rt::export!(thiscall, rw_00A4C9D0(this: u32, arg: u32) -> u32 {
    unsafe {
        const AUX: u32 = 0x08;
        const HEAD: u32 = 0x00;
        const ALT: u32 = 0x04;
        const TARGET: u32 = 0x6C;
        const STATE: u32 = 0x12;
        const READY: u32 = 0x10;
        const EMIT_THIS_FILE_VA: u32 = 0x018ECFB0;
        const DONE: u16 = 0xC8;
        const CLASSIFY_CALLEE: u32 = 1;
        const MEASURE_CALLEE: u32 = 2;
        const EMIT_CALLEE: u32 = 3;
        let aux = ((this + AUX) as *const u32).read_unaligned();
        if aux == 0 {
            return 0;
        }
        let first = ((aux + TARGET) as *const u32).read_unaligned();
        if first == 0 {
            return aux;
        }
        let (second_obj, class_arg) = if (arg & 0xFF) != 0 {
            let o = ((this + ALT) as *const u32).read_unaligned();
            (o, arg)
        } else {
            let o = ((this + HEAD) as *const u32).read_unaligned();
            (o, 0)
        };
        if second_obj == 0 {
            return 0;
        }
        let second = ((second_obj + TARGET) as *const u32).read_unaligned();
        if second == 0 {
            return second_obj;
        }
        let class: u32 = lf_checker_rt::callee_stdcall!(
            CLASSIFY_CALLEE,
            u32,
            class_arg,
            0xFFFF_FFFF
        );
        let m1: u32 =
            lf_checker_rt::callee_thiscall!(MEASURE_CALLEE, u32, second);
        let m2: u32 =
            lf_checker_rt::callee_thiscall!(MEASURE_CALLEE, u32, first);
        lf_checker_rt::callee_thiscall!(
            EMIT_CALLEE,
            u32,
            lf_checker_rt::relocated(EMIT_THIS_FILE_VA),
            m2 & 0xFFFF,
            m1 & 0xFFFF,
            class
        );
        ((this + STATE) as *mut u16).write_unaligned(DONE);
        ((this + READY) as *mut u8).write(1);
        0xC8
    }
});
