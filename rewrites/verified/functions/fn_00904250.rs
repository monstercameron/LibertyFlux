// original: 0x00904250 input_slot_find_free (proposed)
/// Find a free slot at or after `start` and install an object there.
///
/// Scans the handle table up from `start` for a null slot; past the end
/// (`0x5DC`) returns -1. When `flag`'s low byte is set a large object is
/// allocated and its constructor answer installed, otherwise a small object
/// is allocated and zeroed at `+8` before installing; a failed allocation
/// installs null. Returns the slot index. Cdecl with two stack words.
export!(cdecl, rw_00904250(flag: u32, start: u32) -> u32 {
    unsafe {
        /// Handle table base (file VA).
        const TABLE: u32 = 0x0118F6F8;
        /// Table length scanned.
        const LEN: u32 = 0x5DC;
        /// Large and small allocation sizes.
        const BIG: u32 = 0xA0;
        const SMALL: u32 = 0x28;
        const KIND_OFF: u32 = 0x08;
        const ALLOC_ID: u32 = 1;
        const CTOR_ID: u32 = 2;
        let mut i = start;
        loop {
            let s = ((relocated(TABLE).wrapping_add(i.wrapping_mul(4))) as *const u32)
                .read_unaligned();
            if s == 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i >= LEN {
            return 0xFFFFFFFF;
        }
        let cellp =
            (relocated(TABLE).wrapping_add(i.wrapping_mul(4))) as *mut u32;
        if (flag as u8) != 0 {
            let p: u32 = callee_cdecl!(ALLOC_ID, u32, BIG);
            if p == 0 {
                cellp.write_unaligned(0);
            } else {
                let obj: u32 = callee_thiscall!(CTOR_ID, u32, p);
                cellp.write_unaligned(obj);
            }
        } else {
            let p: u32 = callee_cdecl!(ALLOC_ID, u32, SMALL);
            if p == 0 {
                cellp.write_unaligned(0);
            } else {
                ((p.wrapping_add(KIND_OFF)) as *mut u8).write(0);
                cellp.write_unaligned(p);
            }
        }
        i
    }
});
