// original: 0x00bea800 event_ctor_tag7677 (proposed)

/// Initialise an event/task record of kind 0x76 or 0x77 from a descriptor.
///
/// `this` points to the record, `src` (the seventh argument) to a descriptor
/// whose dword at `+0x04` is a key. Validates the key through callee 1
/// (cdecl, one word; only its low byte matters) and returns quietly when it
/// answers zero. Otherwise writes the shared tick counter (file address
/// `0x11735A4`) at `+0x04`, the fifth argument at `+0x08`, the first at
/// `+0x0c`, the low bytes of the second and third at `+0x02`/`+0x01`, the
/// descriptor's first qword at `+0x10` and its third dword at `+0x18`, then
/// invokes callee 2 (thiscall on `this+0x1c`, one word: the sixth argument),
/// stamps the kind tag at `+0x00` (0x77 when the fourth argument's low byte
/// is non-zero, else 0x76) and registers the record through callee 3 (cdecl,
/// two words: the key and `this+3`). No branches on the tag; `ret: none`.
///
/// Original: thiscall, seven stack words, the callee pops 0x1c bytes.
lf_checker_rt::export!(thiscall, rw_00bea800(this: u32, first: u32, second: u32, third: u32, fourth: u32, fifth: u32, sixth: u32, src: u32) -> u32 {
    unsafe {
        const TICK_GLOBAL: u32 = 0x11735A4;
        const OFF_KIND: u32 = 0x00;
        const OFF_FLAG_B: u32 = 0x01;
        const OFF_FLAG_A: u32 = 0x02;
        const OFF_TICK: u32 = 0x04;
        const OFF_FIFTH: u32 = 0x08;
        const OFF_FIRST: u32 = 0x0c;
        const OFF_QWORD: u32 = 0x10;
        const OFF_THIRD_DW: u32 = 0x18;
        const OFF_SUBOBJECT: u32 = 0x1c;
        const SRC_KEY: u32 = 0x04;
        const SRC_THIRD_DW: u32 = 0x08;
        const TAG_CLEAR: u8 = 0x76;
        const TAG_SET: u8 = 0x77;
        const VALIDATE: u32 = 1;
        const CONFIGURE: u32 = 2;
        const REGISTER: u32 = 3;

        let key = ((src + SRC_KEY) as *const u32).read_unaligned();
        let ok: u32 = lf_checker_rt::callee_cdecl!(VALIDATE, u32, key);
        if ok & 0xFF == 0 {
            return 0;
        }
        let tick = lf_checker_rt::global::<u32>(TICK_GLOBAL).read_unaligned();
        ((this + OFF_TICK) as *mut u32).write_unaligned(tick);
        ((this + OFF_FIFTH) as *mut u32).write_unaligned(fifth);
        ((this + OFF_FIRST) as *mut u32).write_unaligned(first);
        ((this + OFF_FLAG_A) as *mut u8).write(second as u8);
        ((this + OFF_FLAG_B) as *mut u8).write(third as u8);
        let q0 = (src as *const u32).read_unaligned();
        let q1 = ((src + SRC_KEY) as *const u32).read_unaligned();
        ((this + OFF_QWORD) as *mut u32).write_unaligned(q0);
        ((this + OFF_QWORD + 4) as *mut u32).write_unaligned(q1);
        let w2 = ((src + SRC_THIRD_DW) as *const u32).read_unaligned();
        ((this + OFF_THIRD_DW) as *mut u32).write_unaligned(w2);
        lf_checker_rt::callee_thiscall!(CONFIGURE, u32, this.wrapping_add(OFF_SUBOBJECT), sixth);
        let tag = if fourth as u8 != 0 { TAG_SET } else { TAG_CLEAR };
        ((this + OFF_KIND) as *mut u8).write(tag);
        lf_checker_rt::callee_cdecl!(REGISTER, u32, key, this.wrapping_add(3));
        0
    }
});
