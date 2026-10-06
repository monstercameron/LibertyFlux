// original: 0x00697780 pool_lookup_build_indirect (proposed)

/// Look up a keyed pair in this pool, building on a miss.
///
/// Asks `a3`'s vtable slot 4 for the probe key, then combines it with `a2`'s
/// key (`+8`) as `rol16(answer) ^ key_b`; either being zero takes the tail.
/// The hash is looked up in this pool under its mutex (`+0x10`) via the
/// rebind helper (low answer byte decides; found skips the build). On a miss
/// the length (`a2+0x10`, unsigned half) must not exceed the signed limit at
/// `this+0x3c`. The build helper then runs with the hash, `a2`, a two-word
/// scratch frame holding (`a2`, `a3`), and `a1`. The tail returns whether
/// `a1+4` points at a record whose `+0x0c` word is set. The original aligns
/// its frame to 8 bytes; only the logic is reproduced here.
///
/// Original: thiscall, four stack words (the fourth unread), callee cleans 16.
lf_checker_rt::export!(thiscall, rw_00697780(this: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const PROBE_SLOT: u32 = 0x10;
        const KEY_B: u32 = 8;
        const POOL_MUTEX: u32 = 0x10;
        const LEN_B: u32 = 0x10;
        const LIMIT: u32 = 0x3c;
        const PROBER: u32 = 4;
        const REBIND: u32 = 2;
        const BUILD: u32 = 3;
        const WAIT: u32 = 10;
        const RELEASE: u32 = 11;
        const INFINITE: u32 = 0xffff_ffff;
        let _ = a4;
        let vtable = (a3 as *const u32).read_unaligned();
        let routine =
            (vtable as *const u32).byte_offset(PROBE_SLOT as isize).read_unaligned();
        let probe: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(routine as usize);
        let answer = probe(a3);
        let key_b = (a2 as *const u32).byte_offset(KEY_B as isize).read_unaligned();
        if key_b == 0 || answer == 0 {
            return tail_00697780(a1);
        }
        let hash = answer.rotate_left(16) ^ key_b;
        let mutex = (this as *const u32).byte_offset(POOL_MUTEX as isize).read_unaligned();
        if mutex != 0 {
            let _ = lf_checker_rt::callee_stdcall!(WAIT, u32, mutex, INFINITE);
        }
        let found = lf_checker_rt::callee_thiscall!(REBIND, u32, this, hash, a1);
        let mutex2 = (this as *const u32).byte_offset(POOL_MUTEX as isize).read_unaligned();
        if mutex2 != 0 {
            let _ = lf_checker_rt::callee_stdcall!(RELEASE, u32, mutex2);
        }
        if found & 0xff != 0 {
            return tail_00697780(a1);
        }
        let len_b = (a2 as *const u16).byte_offset(LEN_B as isize).read_unaligned() as u32;
        let limit = (this as *const u32).byte_offset(LIMIT as isize).read_unaligned();
        // Signed comparison, as the original's `jg`.
        if (len_b as i32) > (limit as i32) {
            return tail_00697780(a1);
        }
        let frame = [a2, a3];
        let _ = lf_checker_rt::callee_thiscall!(
            BUILD, u32, this, hash, a2, frame.as_ptr() as u32, a1
        );
        tail_00697780(a1)
    }
});

/// Tail: 1 when `a1+4` leads to a record with `+0x0c` set.
fn tail_00697780(a1: u32) -> u32 {
    unsafe {
        let next = (a1 as *const u32).byte_offset(4).read_unaligned();
        if next == 0 {
            return 0;
        }
        let child = (next as *const u32).byte_offset(0x0c).read_unaligned();
        (child != 0) as u32
    }
}
