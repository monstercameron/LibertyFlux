// original: 0x008757d0 filter_dtor
/// Destroy a motion-request filter object.
///
/// Installs the filter vtable, releases the child in slot `0x18` through
/// its vtable (slot 2) when non-null, clears slots `0x18` and `0x14`,
/// steps the member vtables down while running the member release helper
/// (stubbed) for each live member, and leaves the terminal vtable.
export!(thiscall, rw_008757d0(this: u32) -> u32 {
    unsafe {
        const CHILD: usize = 0x18 / 4;
        const TAG: usize = 0x14 / 4;
        const MEMBER_GUARD: usize = 0xC / 4;
        let base = this as *mut u32;
        let child = base.add(CHILD).read();
        base.write(relocated(0xFE815C));
        if child != 0 {
            let vt = (child as *const u32).read();
            let target = (vt as *const u32).add(2).read();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            let _: u32 = release(child);
        }
        base.add(CHILD).write(0);
        base.add(TAG).write(0);
        base.write(relocated(0xFE7FC8));
        let member = this.wrapping_add(4);
        if base.add(MEMBER_GUARD).read() != 0 {
            let _: u32 = callee_stdcall!(2, u32, member);
        }
        let live = base.add(MEMBER_GUARD).read() != 0;
        (member as *mut u32).write(relocated(0xFE7FB4));
        if live {
            let _: u32 = callee_stdcall!(2, u32, member);
        }
        (member as *mut u32).write(relocated(0xE86AFC));
        0
    }
});
