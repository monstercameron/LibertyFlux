// original: 0x008dfdc0 free_buffers_and_release

/// Free an entry's three heap buffers, run this entry's own release helper,
/// then tail-release a second, related entry whose header sits 0x400 bytes
/// further into the same arena.
///
/// `this`: nullable entry header; word 0, word 1 and word 3 are buffers freed
/// with the arena free helper (each first copied to a register, then the slot
/// is cleared to zero), word 2 is passed to the entry-local release helper.
/// The second entry lands in its own registers before the tail jump: word 0
/// as its buffer count, words 1 and 3 as its pointers.
///
/// Original: thiscall, nullable this (null faults on the first buffer read).
lf_checker_rt::export!(thiscall, rw_008dfdc0(this: *mut u32) -> u32 {
    unsafe {
        let words = |o: usize| *this.add(o);
        let mut p = words(0);
        lf_checker_rt::callee_cdecl!(1, u32, p);
        *this.add(0) = 0;
        let second = words(1);
        let third = words(3);
        p = second;
        lf_checker_rt::callee_cdecl!(1, u32, p);
        *this.add(1) = 0;
        lf_checker_rt::callee_thiscall!(2, u32, this as u32);
        p = third;
        lf_checker_rt::callee_cdecl!(1, u32, p);
        *this.add(3) = 0;
        lf_checker_rt::callee_thiscall!(3, u32, this as u32)
    }
});
