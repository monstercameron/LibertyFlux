// original: 0x00D4B7A0 CTaskComplexGun::vf1

#![allow(unsafe_code)]

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn wr16(a: u32, v: u16) {
    unsafe { (a as *mut u16).write_unaligned(v) }
}
#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}

/// Clone of the gun-task object (`vf1`).
///
/// Allocates a fresh object from the task pool, constructs it with this
/// object's own fields (selector byte, attachment, `+0x20` vector, the two
/// floats, `+0x74` and the two words), then copies the remaining tuning
/// fields: words `+0x36`/`+0x38`/`+0x3c`/`+0x3e`, dwords `+0x44`/`+0x48`,
/// words `+0x4c`..`+0x5a`, byte `+0x70`, dwords `+0x80`/`+0x84`/`+0x88`/
/// `+0x8c`/`+0x90`, and bytes `+0x6c`..`+0x71` (`+0x70` is stored twice).
/// The source words are read with mixed sign/zero extension but only the
/// low bits are stored, so every copy is a plain bit copy. When the pool
/// is exhausted the copy loop runs against a null pointer and faults, like
/// the original. Returns the new object.
///
/// Original: 0x00D4B7A0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00D4B7A0(this: u32) -> u32 {
    unsafe {
        const POOL_GLOBAL: u32 = 0x0167E2A0;
        const ALLOC: u32 = 0;
        const CTOR: u32 = 1;
        let pool: u32 = (lf_checker_rt::global::<u32>(POOL_GLOBAL) as *const u32).read_unaligned();
        let fresh: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        let target: u32;
        if fresh != 0 {
            let sel = (((rd8(this + 0x73)) as i8) as i32) as u32;
            let built: u32 = lf_checker_rt::callee_thiscall!(CTOR, u32, fresh, sel, rd32(this + 0x14),
                this + 0x20, rd32(this + 0x30), rd32(this + 0x74),
                rd16(this + 0x3a) as u32, rd16(this + 0x34) as u32, rd32(this + 0x40));
            target = built;
        } else {
            target = 0;
        }
        wr16(target + 0x36, rd16(this + 0x36));
        wr16(target + 0x38, rd16(this + 0x38));
        wr16(target + 0x3c, rd16(this + 0x3c));
        wr16(target + 0x3e, rd16(this + 0x3e));
        wr32(target + 0x44, rd32(this + 0x44));
        wr32(target + 0x48, rd32(this + 0x48));
        wr16(target + 0x4c, rd16(this + 0x4c));
        wr16(target + 0x4e, rd16(this + 0x4e));
        wr16(target + 0x50, rd16(this + 0x50));
        wr16(target + 0x52, rd16(this + 0x52));
        wr16(target + 0x54, rd16(this + 0x54));
        wr16(target + 0x56, rd16(this + 0x56));
        wr16(target + 0x58, rd16(this + 0x58));
        wr16(target + 0x5a, rd16(this + 0x5a));
        wr8(target + 0x70, rd8(this + 0x70));
        wr32(target + 0x80, rd32(this + 0x80));
        wr32(target + 0x84, rd32(this + 0x84));
        wr32(target + 0x88, rd32(this + 0x88));
        wr32(target + 0x8c, rd32(this + 0x8c));
        wr32(target + 0x90, rd32(this + 0x90));
        wr8(target + 0x6c, rd8(this + 0x6c));
        wr8(target + 0x6d, rd8(this + 0x6d));
        wr8(target + 0x6e, rd8(this + 0x6e));
        wr8(target + 0x6f, rd8(this + 0x6f));
        wr8(target + 0x70, rd8(this + 0x70));
        wr8(target + 0x71, rd8(this + 0x71));
        target
    }
});
