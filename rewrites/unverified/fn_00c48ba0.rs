// original: 0x00c48ba0 ccamcinematic_ensure_slot (proposed)
/// Open slot 0x14 in the table at `this + TABLE` and register it.
///
/// Sets the shared slot count to `SLOT`, clears `this + COUNT`, writes
/// `KIND` (0x24) into the newly opened table entry, then registers it
/// with the owner at `this + OWNER` (callee 1, called with the entry
/// value, 0 and `this` on the stack) and marks the returned object
/// with `READY` (bit 3) at `MARK`. Returns 1 in the low byte.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c48ba0(this: u32) -> u32 {
    const SLOT_COUNT_GLOBAL: u32 = 0x01048f50;
    const SLOT: u32 = 0x14;
    const COUNT: u32 = 0x1e4;
    const TABLE: u32 = 0x140;
    const KIND: u32 = 0x24;
    const OWNER: u32 = 0x114;
    const MARK: u32 = 0x13c;
    const READY: u8 = 8;
    const REGISTER: u32 = 1;
    unsafe {
        lf_checker_rt::global::<u32>(SLOT_COUNT_GLOBAL).write_unaligned(SLOT);
        ((this + COUNT) as *mut u32).write_unaligned(0);
        let n = lf_checker_rt::global::<u32>(SLOT_COUNT_GLOBAL).read_unaligned();
        ((this + TABLE + n.wrapping_mul(4)) as *mut u32).write_unaligned(KIND);
        let entry = ((this + TABLE + n.wrapping_mul(4)) as *const u32).read_unaligned();
        let owner = ((this + OWNER) as *const u32).read_unaligned();
        let obj = lf_checker_rt::callee_thiscall!(REGISTER, u32, owner, entry, 0, this);
        let mark = (obj + MARK) as *mut u8;
        mark.write(mark.read() | READY);
    }
    1
});
