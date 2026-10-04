// original: 0x009822c0 audAmbientAudioEntity::vf2
/// Original 0x009822c0 `audAmbientAudioEntity::vf2`: release all attachments.
///
/// Releases the array at +0x6f28, visits each of the `count` records at
/// +0x6f30, releases the array at +0x6f30, releases both 245-entry handle
/// blocks from +0x20, sweeps the 245 slots from +0x5c00 (releasing the owners
/// then the handles), and tail-calls the base implementation, returning its
/// result.
export!(thiscall, rw_009822c0(this_: u32) -> u32 {
    let lo = unsafe { ((this_ + 0x6f2e) as *const u16).read() };
    let base0 = unsafe { ((this_ + 0x6f28) as *const u32).read() };
    callee_thiscall!(1, u32, this_.wrapping_add(0x6f28), base0, lo as u32);
    unsafe {
        ((this_ + 0x6f28) as *mut u32).write(0);
        ((this_ + 0x6f2c) as *mut u32).write(0);
    }
    let n = unsafe { ((this_ + 0x6f34) as *const u16).read() } as u32;
    let mut i = 0u32;
    while i < n {
        let rec = unsafe { ((this_ + 0x6f30) as *const u32).read() }
            .wrapping_add(i.wrapping_mul(0x160));
        callee_thiscall!(2, u32, rec);
        i += 1;
    }
    let hi = unsafe { ((this_ + 0x6f36) as *const u16).read() };
    let base1 = unsafe { ((this_ + 0x6f30) as *const u32).read() };
    callee_thiscall!(3, u32, this_.wrapping_add(0x6f30), base1, hi as u32);
    unsafe {
        ((this_ + 0x6f30) as *mut u32).write(0);
        ((this_ + 0x6f34) as *mut u32).write(0);
    }
    let mut s = this_.wrapping_add(0x20);
    for _ in 0..2u32 {
        for _ in 0..0xf5u32 {
            let h = unsafe { (s as *const u32).read() };
            if h != 0 {
                callee_thiscall!(4, u32, h, s);
                unsafe { (s as *mut u32).write(0); }
            }
            s = s.wrapping_add(0x30);
        }
    }
    let mut e = this_.wrapping_add(0x5c00);
    for _ in 0..0xf5u32 {
        let owner = unsafe { ((e - 4) as *const u32).read() };
        if owner != 0 {
            callee_thiscall!(5, u32, owner, 0);
        }
        let h = unsafe { (e as *const u32).read() };
        if h != 0 {
            callee_thiscall!(6, u32, h);
            unsafe { (e as *mut u32).write(0); }
        }
        e = e.wrapping_add(0x14);
    }
    callee_thiscall!(7, u32, this_)
});
