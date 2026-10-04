// original: 0x009b77f0 rw_009b77f0
/// Run the element initialiser, clear the status word at `this+0x531 and
/// report success in AL (upper bytes carry through from the answer).
export!(thiscall, rw_009b77f0(this_: u32) -> u32 {
    unsafe {
        const STATUS_OFF: u32 = 0x531;
        let init: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let ans = init(this_);
        core::ptr::write_unaligned((this_.wrapping_add(STATUS_OFF)) as *mut u16, 0);
        (ans & 0xFFFF_FF00) | 1
    }
});
