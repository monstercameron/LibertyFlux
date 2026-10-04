// original: 0x00875510 animation_dtor
/// Destroy an animation motion request.
///
/// Drops the reference-counted child in slot `0x14` (decrementing its
/// count word at offset 4 and calling its vtable slot 0 with argument 1
/// when the count reaches zero), clears the slot, then steps the member
/// vtables down while running the member release helper (stubbed) for
/// each live member, leaving the terminal vtable.
export!(thiscall, rw_00875510(this: u32) -> u32 {
    unsafe {
        const CHILD: usize = 0x14 / 4;
        const MEMBER_GUARD: usize = 8 / 4;
        let base = this as *mut u32;
        let child = base.add(CHILD).read();
        base.write(relocated(0xFE814C));
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
        let member = this.wrapping_add(4);
        let live = (member as *const u32).add(MEMBER_GUARD).read() != 0;
        if live {
            let _: u32 = callee_stdcall!(2, u32, member);
        }
        base.write(relocated(0xFE7FC8));
        if (member as *const u32).add(MEMBER_GUARD).read() != 0 {
            let _: u32 = callee_stdcall!(2, u32, member);
        }
        (member as *mut u32).write(relocated(0xFE7FB4));
        if (member as *const u32).add(MEMBER_GUARD).read() != 0 {
            let _: u32 = callee_stdcall!(2, u32, member);
        }
        (member as *mut u32).write(relocated(0xE86AFC));
        0
    }
});
