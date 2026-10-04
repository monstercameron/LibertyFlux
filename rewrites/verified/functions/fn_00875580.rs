// original: 0x00875580 rage::crmtRequestAnimation::vf1
/// Detach an animation request's child and member references.
///
/// Drops the reference-counted child in slot `0x14` (decrementing its
/// count word at offset 4 and calling its vtable slot 0 with argument 1
/// when the count reaches zero), clears the slot, then runs the member
/// release helper (stubbed) when slot `0xC` is non-null.
export!(thiscall, rw_00875580(this: u32) -> u32 {
    unsafe {
        const CHILD: usize = 0x14 / 4;
        const MEMBER_GUARD: usize = 0xC / 4;
        let base = this as *mut u32;
        let child = base.add(CHILD).read();
        if child != 0 {
            let count = (child as *mut u16).add(2);
            let left = count.read().wrapping_sub(1);
            count.write(left);
            if left == 0 {
                let vt = (child as *const u32).read();
                let target = (vt as *const u32).read();
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                let _: u32 = release(child, 1);
            }
        }
        base.add(CHILD).write(0);
        if base.add(MEMBER_GUARD).read() != 0 {
            let _: u32 = callee_stdcall!(2, u32, this.wrapping_add(4));
        }
        0
    }
});
