// original: 0x00A1A8B0 peds_task_state_reset (proposed)

/// Reset a ped task-state object to its freshly constructed values.
///
/// `this` points to the object. Every known state word is cleared to zero
/// except the status word at `+0x494`, which is set to all-ones (-1, meaning
/// "no task" / invalid). The cleared words fall into offset groups that read
/// like per-slot state (timer/counter words) plus a header word at `+0x20`.
/// No memory is read, no calls are made, and the object pointer is returned.
///
/// Original: 0x00A1A8B0 (thiscall, no stack arguments, returns `this`).
lf_checker_rt::export!(thiscall, rw_00A1A8B0(this: u32) -> u32 {
    unsafe {
        const ZERO_WORDS: [u32; 35] = [
            0x020, 0x024, 0x040, 0x044, 0x048, 0x04c, 0x0b0, 0x0b4, 0x0d0, 0x0d4,
            0x0d8, 0x0dc, 0x160, 0x164, 0x180, 0x184, 0x188, 0x18c, 0x220, 0x224,
            0x240, 0x244, 0x248, 0x24c, 0x300, 0x304, 0x320, 0x324, 0x328, 0x32c,
            0x400, 0x404, 0x408, 0x40c, 0x410,
        ];
        const STATUS_WORD: u32 = 0x494;
        const STATUS_EMPTY: u32 = 0xffff_ffff;
        for off in ZERO_WORDS {
            ((this + off) as *mut u32).write_unaligned(0);
        }
        ((this + STATUS_WORD) as *mut u32).write_unaligned(STATUS_EMPTY);
        this
    }
});
