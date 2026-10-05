// original: 0x00a91fc0 stream_init_entry_named (proposed)

/// Initialise an entry with a name copy, an id and bound vectors.
///
/// The name (the argument points at a short NUL-terminated string) selects
/// an id (callee 1, cdecl/1). The entry takes 0 at `+0x00`/`+0x04`, the id
/// at `+0x08` and the name bytes (with the terminator) at `+0x0c`; the
/// bound vectors at `+0x30`/`+0x34`/`+0x38` take 0x7f7fffff, at
/// `+0x40`/`+0x44`/`+0x48` take 0xff7fffff, and at `+0x3c`/`+0x4c` take
/// whatever the stack happened to hold (pinned to 0 by the contract's
/// stack fill).
///
/// Returns the id with its low byte cleared (the terminator the copy left
/// in `al`). Thiscall: object in ecx, the name pointer on the stack,
/// callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a91fc0(this: u32, name: u32) -> u32 {
    unsafe {
        const ID_OFF: u32 = 0x08;
        const NAME_OFF: u32 = 0x0c;
        const LO_MAGIC: u32 = 0x7f7fffff;
        const HI_MAGIC: u32 = 0xff7fffff;
        const FILL_MAGIC: u32 = 0;
        ((this) as *mut u32).write_unaligned(0);
        ((this + 4) as *mut u32).write_unaligned(0);
        let id = lf_checker_rt::callee_cdecl!(1, u32, name);
        ((this + ID_OFF) as *mut u32).write_unaligned(id);
        let mut i = 0u32;
        loop {
            let b = ((name + i) as *const u8).read();
            ((this + NAME_OFF + i) as *mut u8).write(b);
            if b == 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
        for k in 0..3 {
            ((this + 0x30 + k * 4) as *mut u32).write_unaligned(LO_MAGIC);
            ((this + 0x40 + k * 4) as *mut u32).write_unaligned(HI_MAGIC);
        }
        ((this + 0x3c) as *mut u32).write_unaligned(FILL_MAGIC);
        ((this + 0x4c) as *mut u32).write_unaligned(FILL_MAGIC);
        id & 0xffffff00
    }
});
