// original: 0x008756e0 filter_ctor
/// Construct a motion-request filter object.
///
/// Initialises the embedded member at offset 4 (vtable plus three zero
/// words, then its constructor through the shared data slot), clears the
/// payload slots at `0x14` and `0x18`, installs the filter vtable and
/// returns the object.
export!(thiscall, rw_008756e0(this: u32) -> u32 {
    unsafe {
        const MEMBER_VT: usize = 1;
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
        base.add(0x14 / 4).write(0);
        base.write(relocated(0xFE815C));
        base.add(0x18 / 4).write(0);
        this
    }
});
