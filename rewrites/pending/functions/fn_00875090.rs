// original: 0x00875090 source202_ctor
/// Construct a motion-source object.
///
/// Initialises the embedded member at offset 4 (vtable plus three zero
/// words, then its constructor through the shared data slot), installs
/// the source vtable, zeroes offsets `0x14..=0x90` and returns the object.
export!(thiscall, rw_00875090(this: u32) -> u32 {
    unsafe {
        const MEMBER_VT: usize = 1;
        const FIRST_WORD: usize = 0x14 / 4;
        const COUNT: usize = 32;
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
        base.write(relocated(0xFE811C));
        let mut i: usize = 0;
        while i < COUNT {
            base.add(FIRST_WORD + i).write(0);
            i += 1;
        }
        this
    }
});
