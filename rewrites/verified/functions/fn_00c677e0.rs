// original: 0x00c677e0 cutscene_notify_gate_then_flag
/// Notifies the gate object at [this+0x34]+4 when present, then sets the
/// flag byte at 0x2b1.
export!(thiscall, rw_c677e0(this: u32) -> u32 {
    let gate = unsafe { (this as *const u32).byte_add(0x34).read() };
    let target = unsafe { (gate as *const u32).byte_add(4).read() };
    if target != 0 {
        callee_thiscall!(1, u32, target, this, 0xFFFF_FFFFu32);
    }
    unsafe { ((this as *mut u8).byte_add(0x2b1)).write(1) };
    0
});
