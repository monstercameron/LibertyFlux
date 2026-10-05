// original: 0x00cd1630 die_task_reset_anim
/// Reset the die task's animation slot unless a blocker is found.
///
/// Marks byte `arg+0x28` and, unless the chain through `[arg+0x18]`,
/// `+0x6c` (with a non-zero flag at `+0xe`) and `+0x224`/`+0x2e0` holds a
/// node whose word at `+4` is 0xda (nodes link through `+0xc`), sets bits
/// 0x8000 then 0x4000 in the slot's flags at `[arg+0x24]+4` and drives it
/// with -50.0 (thiscall). The slot word itself is always cleared. Any null
/// or flag miss skips straight to the clear. (One compare inside the scan
/// tests a value against itself, so its branch is dead.) Cdecl of two
/// stack arguments, the first unused.
export!(cdecl, rw_00cd1630(_a0: u32, obj: u32) -> u32 {
    unsafe {
        const MARK_OFF: u32 = 0x28;
        const HEAD_OFF: u32 = 0x18;
        const INNER_OFF: u32 = 0x6c;
        const INNER_FLAG_OFF: u32 = 0x0e;
        const OUTER_OFF: u32 = 0x224;
        const LIST_OFF: u32 = 0x2e0;
        const NODE_NEXT_OFF: u32 = 0x0c;
        const NODE_VAL_OFF: u32 = 0x04;
        const BLOCKER: u32 = 0xda;
        const SLOT_OFF: u32 = 0x24;
        const RATE: u32 = 0xc1000000;
        (obj.wrapping_add(MARK_OFF) as *mut u8).write(1);
        let mut blocked = true;
        let head = (obj.wrapping_add(HEAD_OFF) as *const u32).read_unaligned();
        if head != 0 {
            let inner = (head.wrapping_add(INNER_OFF) as *const u32).read_unaligned();
            if inner != 0
                && (inner.wrapping_add(INNER_FLAG_OFF) as *const u8).read() != 0
            {
                let outer = (head.wrapping_add(OUTER_OFF) as *const u32).read_unaligned();
                let mut node = (outer.wrapping_add(LIST_OFF) as *const u32).read_unaligned();
                blocked = false;
                while node != 0 {
                    if (node.wrapping_add(NODE_VAL_OFF) as *const u32).read_unaligned()
                        == BLOCKER
                    {
                        blocked = true;
                        break;
                    }
                    node = (node.wrapping_add(NODE_NEXT_OFF) as *const u32).read_unaligned();
                }
            }
        }
        if !blocked {
            let slot = (obj.wrapping_add(SLOT_OFF) as *const u32).read_unaligned();
            let f1 = (slot.wrapping_add(4) as *const u32).read_unaligned();
            (slot.wrapping_add(4) as *mut u32).write_unaligned(f1 | 0x8000);
            let slot2 = (obj.wrapping_add(SLOT_OFF) as *const u32).read_unaligned();
            let f2 = (slot2.wrapping_add(4) as *const u32).read_unaligned();
            (slot2.wrapping_add(4) as *mut u32).write_unaligned(f2 | 0x4000);
            let slot3 = (obj.wrapping_add(SLOT_OFF) as *const u32).read_unaligned();
            let _: u32 = callee_thiscall!(1, u32, slot3, RATE);
        }
        (obj.wrapping_add(SLOT_OFF) as *mut u32).write_unaligned(0);
        0
    }
});
