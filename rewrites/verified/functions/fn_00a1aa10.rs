// original: 0x00A1AA10 peds_task_state_init (proposed)

/// Construct a ped task-state object: reset the header words, clear the
/// scratch buffers, then attach it to its owner through the factory call.
///
/// `this` points to the object (at least 0xDA4 bytes). The header words are
/// the same set the reset routine clears (35 zero words plus the all-ones
/// status word at `+0x494`); four scratch ranges are cleared (`+0x714` len
/// 0x80, `+0x50a` len 0x80, `+0xa70` len 0x100, `+0xb70` len 0x200); scalar
/// fields around them are set (notably the flag byte at `+0xd98` is 7 and
/// the handle words at `+0xd90`/`+0xd94` are -1). The factory callee takes
/// (`this`, 0, template pointer, out-pointer), answers an id stored at
/// `+0xd78`, and writes a second id through the out-pointer, stored at
/// `+0xd7c`. Returns `this`.
///
/// The reset and the four clears run natively in the original (plain callee
/// code, effects compared byte-wise); only the factory call is intercepted.
///
/// Original: 0x00A1AA10 (thiscall, no stack arguments, returns `this`).
lf_checker_rt::export!(thiscall, rw_00A1AA10(this: u32) -> u32 {
    unsafe {
        const ZERO_WORDS: [u32; 35] = [
            0x020, 0x024, 0x040, 0x044, 0x048, 0x04c, 0x0b0, 0x0b4, 0x0d0, 0x0d4,
            0x0d8, 0x0dc, 0x160, 0x164, 0x180, 0x184, 0x188, 0x18c, 0x220, 0x224,
            0x240, 0x244, 0x248, 0x24c, 0x300, 0x304, 0x320, 0x324, 0x328, 0x32c,
            0x400, 0x404, 0x408, 0x40c, 0x410,
        ];
        const STATUS_WORD: u32 = 0x494;
        const STATUS_EMPTY: u32 = 0xffff_ffff;
        const TEMPLATE_PTR: u32 = 0x00a1_ef50;
        const FACTORY_CALLEE: u32 = 2;
        for off in ZERO_WORDS {
            ((this + off) as *mut u32).write_unaligned(0);
        }
        ((this + STATUS_WORD) as *mut u32).write_unaligned(STATUS_EMPTY);
        ((this + 0x504) as *mut u32).write_unaligned(0);
        ((this + 0x508) as *mut u16).write_unaligned(0);
        ((this + 0x794) as *mut u32).write_unaligned(0);
        ((this + 0x714) as *mut u8).write_bytes(0, 0x80);
        ((this + 0x50a) as *mut u8).write_bytes(0, 0x80);
        ((this + 0x58a) as *mut u8).write(0);
        ((this + 0xa70) as *mut u8).write_bytes(0, 0x100);
        ((this + 0xb70) as *mut u8).write_bytes(0, 0x200);
        ((this + 0xd70) as *mut u32).write_unaligned(0);
        ((this + 0xd74) as *mut u16).write_unaligned(0);
        ((this + 0xd78) as *mut u32).write_unaligned(0);
        ((this + 0xd7c) as *mut u32).write_unaligned(0);
        ((this + 0xd80) as *mut u32).write_unaligned(0);
        ((this + 0xd84) as *mut u32).write_unaligned(0);
        ((this + 0xd88) as *mut u8).write(0);
        let mut out_id: u32 = 0;
        let out_ptr = (&mut out_id as *mut u32) as u32;
        let id: u32 = lf_checker_rt::callee_cdecl!(
            FACTORY_CALLEE, u32, this, 0, lf_checker_rt::relocated(TEMPLATE_PTR), out_ptr
        );
        ((this + 0xd78) as *mut u32).write_unaligned(id);
        ((this + 0xd7c) as *mut u32).write_unaligned(out_id);
        ((this + 0xd8c) as *mut u32).write_unaligned(0);
        ((this + 0xd90) as *mut u32).write_unaligned(0xffff_ffff);
        ((this + 0xd94) as *mut u32).write_unaligned(0xffff_ffff);
        ((this + 0xd98) as *mut u8).write(7);
        ((this + 0xd9c) as *mut u32).write_unaligned(0);
        ((this + 0xda0) as *mut u32).write_unaligned(0);
        this
    }
});
