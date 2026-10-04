// original: 0x00e72a30 reinit_array_2x60
/// Re-init sweep over 2 records of 0x60 bytes ending at 0x16C8940.
///
/// Per element, back to front: stamp the relocated tag word 0xEC5E0C (the
/// original's immediate has a HIGHLOW fixup, so it follows the image base),
/// run the init step (thiscall/0, stubbed as id 1), then the follow-up step
/// (thiscall/0, stubbed as id 2). Returns the last follow-up answer,
/// matching EAX.
export!(cdecl, rw_00e72a30() -> u32 {
    unsafe {
        const END: u32 = 0x16C8940;
        const COUNT: u32 = 2;
        const STRIDE: u32 = 0x60;
        const TAG: u32 = 0xEC5E0C;
        let init: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let follow: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let mut ptr = relocated(END);
        let mut last = 0u32;
        let mut remaining = COUNT;
        loop {
            ptr = ptr.wrapping_sub(STRIDE);
            *(ptr as *mut u32) = relocated(TAG);
            init(ptr);
            last = follow(ptr);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
        last
    }
});
