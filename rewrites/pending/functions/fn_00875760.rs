// original: 0x00875760 filter_ctor_args
/// Construct a motion-request filter object around a child.
///
/// Initialises the embedded member at offset 4 (vtable plus three zero
/// words, then its constructor through the shared data slot), stores the
/// tag argument in slot `0x14`, takes a reference on the child argument
/// through its vtable (slot 1) when non-null, and stores it in slot
/// `0x18`. A release check of slot `0x18` in the original is dead (the
/// slot was just cleared) and is not reproduced.
export!(thiscall, rw_00875760(this: u32, tag: u32, child: u32) -> u32 {
    unsafe {
        const MEMBER_VT: usize = 1;
        const TAG: usize = 0x14 / 4;
        const CHILD: usize = 0x18 / 4;
        let base = this as *mut u32;
        base.write(relocated(0xFE7FC8));
        base.add(2).write(0);
        base.add(MEMBER_VT).write(relocated(0xFE7FB4));
        base.add(3).write(0);
        base.add(4).write(0);
        let slot = global::<u32>(0xFE7FB8);
        let member_ctor: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute::<u32, extern "thiscall" fn(u32) -> u32>(*slot);
        let _: u32 = member_ctor(this.wrapping_add(4));
        base.write(relocated(0xFE815C));
        base.add(CHILD).write(0);
        base.add(TAG).write(tag);
        if child != 0 {
            let vt = (child as *const u32).read();
            let target = (vt as *const u32).add(1).read();
            let addref: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            let _: u32 = addref(child);
        }
        base.add(CHILD).write(child);
        this
    }
});
