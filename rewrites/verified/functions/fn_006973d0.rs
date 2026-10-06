// original: 0x006973d0 pool_lookup_build_b (proposed)

/// Look up (`key_a`, `key_b`) in pool 0x14, building on a miss.
///
/// `a2` and `a3` supply the two keys (`a2+8`, `a3+0x10`); either being
/// zero skips straight to the tail. Otherwise the hash `rol7(key_a) ^ key_b`
/// is looked up in the pool at `this+0x14` under its mutex (`+0x10`,
/// waited on when set) via the rebind helper, whose low answer byte decides:
/// found skips the build. On a miss both lengths (`a2+0x10`, `a3+0x24`,
/// unsigned halves) must not exceed the signed limit at `this+0x3c`, else the
/// tail is taken too. The build helper then runs with the hash, callback
/// `0x006973b0`, a two-word scratch frame holding (`a3`, zero), and
/// `a1`. The tail returns whether `a1+4` points at a record whose `+0x0c`
/// word is set. The found flag and the pool pointer are spilled into the
/// incoming `a2`/`a3` slots (the stack comparison stays off for that reason).
///
/// Original: thiscall, four stack words, callee cleans 16.
lf_checker_rt::export!(thiscall, rw_006973d0(this: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const KEY_B: u32 = 8;
        const KEY_A: u32 = 0x10;
        const POOL: u32 = 0x14;
        const POOL_MUTEX: u32 = 0x10;
        const LEN_B: u32 = 0x10;
        const LEN_A: u32 = 0x24;
        const LIMIT: u32 = 0x3c;
        const CALLBACK_FILE_VA: u32 = 0x006973b0;
        const REBIND: u32 = 2;
        const BUILD: u32 = 3;
        const WAIT: u32 = 10;
        const RELEASE: u32 = 11;
        const INFINITE: u32 = 0xffff_ffff;
        let _ = a4;
        let key_b = (a2 as *const u32).byte_offset(KEY_B as isize).read_unaligned();
        let key_a = (a3 as *const u32).byte_offset(KEY_A as isize).read_unaligned();
        if key_b == 0 || key_a == 0 {
            return tail_006973d0(a1);
        }
        let hash = key_a.rotate_left(7) ^ key_b;
        let pool = this.wrapping_add(POOL);
        let mutex = (pool as *const u32).byte_offset(POOL_MUTEX as isize).read_unaligned();
        if mutex != 0 {
            let _ = lf_checker_rt::callee_stdcall!(WAIT, u32, mutex, INFINITE);
        }
        let found = lf_checker_rt::callee_thiscall!(REBIND, u32, pool, hash, a1);
        let mutex2 = (pool as *const u32).byte_offset(POOL_MUTEX as isize).read_unaligned();
        if mutex2 != 0 {
            let _ = lf_checker_rt::callee_stdcall!(RELEASE, u32, mutex2);
        }
        if found & 0xff != 0 {
            return tail_006973d0(a1);
        }
        let len_b = (a2 as *const u16).byte_offset(LEN_B as isize).read_unaligned() as u32;
        let limit = (this as *const u32).byte_offset(LIMIT as isize).read_unaligned();
        // Signed comparison, as the original's `jg`.
        if (len_b as i32) > (limit as i32) {
            return tail_006973d0(a1);
        }
        let len_a = (a3 as *const u16).byte_offset(LEN_A as isize).read_unaligned() as u32;
        if (len_a as i32) > (limit as i32) {
            return tail_006973d0(a1);
        }
        let callback = lf_checker_rt::relocated(CALLBACK_FILE_VA);
        let frame = [a2, a3];
        let _ = lf_checker_rt::callee_thiscall!(
            BUILD, u32, pool, hash, callback, frame.as_ptr() as u32, a1
        );
        tail_006973d0(a1)
    }
});

/// Tail shared above: 1 when `a1+4` leads to a record with `+0x0c` set.
#[allow(dead_code)]
fn tail_006973d0(a1: u32) -> u32 {
    unsafe {
        let next = (a1 as *const u32).byte_offset(4).read_unaligned();
        if next == 0 {
            return 0;
        }
        let child = (next as *const u32).byte_offset(0x0c).read_unaligned();
        (child != 0) as u32
    }
}
