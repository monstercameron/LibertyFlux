// original: 0x00c56420 CEventPedCollisionWithPlayer::~CEventPedCollisionWithPlayer
/// Clone the CEventPedCollisionWithPlayer event. After allocation succeeds, forward the unsigned word at +0x0c, the raw float bits at +0x10, the dword at +0x14, pointers to the blocks at +0x20 and +0x30, and sign-extended i16 fields at +0x40 and +0x42 to the event copy helper. Stamp the derived vtable and return the new object.
lf_checker_rt::export!(thiscall, rw_00c56420(this: u32) -> u32 {
unsafe {
    const MANAGER_GLOBAL_VA: u32 = 0x0166_69c8;
    const EVENT_VTABLE_VA: u32 = 0x00ecaab4;
    let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
    let fresh: u32 = lf_checker_rt::callee_thiscall!(1, u32, manager);
    if fresh == 0 { return 0; }
    let event_kind = core::ptr::read_unaligned(this.wrapping_add(0x0c) as *const u16) as u32;
    let time_bits = core::ptr::read_unaligned(this.wrapping_add(0x10) as *const u32);
    let event_value = core::ptr::read_unaligned(this.wrapping_add(0x14) as *const u32);
    let first_block = this.wrapping_add(0x20);
    let second_block = this.wrapping_add(0x30);
    let part_id = core::ptr::read_unaligned(this.wrapping_add(0x40) as *const u16) as i16 as i32 as u32;
    let sector_id = core::ptr::read_unaligned(this.wrapping_add(0x42) as *const u16) as i16 as i32 as u32;
    let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, fresh, event_kind, time_bits, event_value, first_block, second_block, part_id, sector_id);
    core::ptr::write_unaligned(fresh as *mut u32, lf_checker_rt::relocated(EVENT_VTABLE_VA));
    fresh
}
});
