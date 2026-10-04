// original: 0x00875470 animation_ctor
/// Construct an animation motion request.
///
/// Initialises the embedded member at offset 4 (vtable plus three zero
/// words, then its constructor through the shared data slot), installs
/// the animation vtable with default payload (zero tag and weight, unit
/// blend factor, cleared mode bytes), takes a reference on the child
/// argument when non-null, and stores the child and the two weight words.
/// A release check of the tag slot in the original is dead (the slot was
/// just cleared) and is not reproduced.
export!(thiscall, rw_00875470(this: u32, child: u32, weight0: u32, weight1: u32) -> u32 {
    unsafe {
        const MEMBER_VT: usize = 1;
        const TAG: usize = 0x14 / 4;
        const WEIGHT0: usize = 0x18 / 4;
        const WEIGHT1: usize = 0x1C / 4;
        const UNIT_FACTOR: u32 = 0x3F800000;
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
        base.write(relocated(0xFE814C));
        base.add(TAG).write(0);
        base.add(WEIGHT0).write(0);
        base.add(WEIGHT1).write(UNIT_FACTOR);
        (this as *mut u16).add(0x20 / 2).write(0);
        (this as *mut u8).add(0x22).write(0);
        if child != 0 {
            let count = (child as *mut u16).add(2);
            count.write(count.read().wrapping_add(1));
        }
        base.add(WEIGHT0).write(weight0);
        base.add(TAG).write(child);
        base.add(WEIGHT1).write(weight1);
        this
    }
});
