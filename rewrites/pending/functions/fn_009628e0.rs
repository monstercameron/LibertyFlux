// original: 0x009628E0 advance_stream_state
/// Advance a two-channel buffered stream held in globals, then run the exit hook.
///
/// With a zero low argument byte the hook runs at once and 0 is returned.
/// Otherwise a zero state counter runs the open sequence first (helpers 1
/// to 5 build and validate the handle; a null handle fails with 0 without
/// running the hook). The main step dispatches on the counter against its
/// limit: below the limit the active channel fills toward the 0x895440 cap
/// through helper 6, rolling to the next base through the divisor tables
/// when the tag mismatches; at the limit the tail either finishes through
/// helper 7 (returning 1) or tops the second channel up first. Every path
/// except the open failure runs the exit hook (helper 8) and the cookie
/// check (helper 9). Returns 1 only on the two tail paths.
///
/// Notes: the stack cookie check runs for real on the original side and is
/// omitted here (see the exit helper). Two paths return a frame byte that
/// the contract's zero stack fill defines as 0.
export!(cdecl, rw_009628E0(arg0: u32) -> u8 {
    unsafe {
        const CAP: u32 = 0x895440;
        if (arg0 as u8) == 0 {
            return fn3_exit(0);
        }
        let (al, cl, edi) = if fn3_g32(0x00) == 0 {
            let esi: u32 = callee_cdecl!(1, u32, relocated(0x11F7040));
            let mut buf = [0u8; 64];
            callee_cdecl!(2, u32, buf.as_mut_ptr() as u32, 0, 0x3F);
            let s0 = *(esi as *const u32);
            let s4 = *((esi.wrapping_add(4)) as *const u32);
            let s8 = *((esi.wrapping_add(8)) as *const u32);
            let s12 = *((esi.wrapping_add(12)) as *const u32);
            let s16 = *((esi.wrapping_add(0x10)) as *const u32);
            let s20 = *((esi.wrapping_add(0x14)) as *const u32);
            let mut buf2 = [0u8; 64];
            callee_cdecl!(
                3, u32,
                buf2.as_mut_ptr() as u32,
                relocated(0xE8AD84),
                s20.wrapping_sub(0x64),
                s16.wrapping_add(1),
                s12,
                s8,
                s4,
                s0
            );
            callee_cdecl!(4, u32, relocated(0xE8ADB4), 0);
            let mut buf3 = [0u8; 64];
            let handle: u32 = callee_cdecl!(5, u32, buf3.as_mut_ptr() as u32);
            if handle == 0 {
                return 0;
            }
            fn3_s32(0x3C, handle);
            fn3_s16(0x42, 0x101);
            (1u8, 1u8, handle)
        } else {
            (fn3_g8(0x43), fn3_g8(0x42), fn3_g32(0x3C))
        };
        if edi == 0 {
            return fn3_exit(0);
        }
        let limit = fn3_g32(0x04) as i32;
        let counter = fn3_g32(0x00) as i32;
        if counter >= limit {
            return fn3_tail(edi, al, cl);
        }
        if counter == 0 {
            callee_cdecl!(6, u32, edi, relocated(0x1037848), 0x14);
            let al = fn3_g8(0x43);
            let cl = fn3_g8(0x42);
            if cl == 0 {
                return fn3_chan2(edi, al);
            }
            return fn3_chan1(edi);
        }
        if cl == 0 {
            return fn3_chan2(edi, al);
        }
        fn3_chan1(edi)
    }
});

/// Common exit: exit hook, then the status byte. The original's stack
/// cookie check runs for real on the original side (it passes
/// self-consistently and preserves EAX); a Rust rewrite has no cookie, so
/// there is nothing to check. It must NOT be stubbed: every stub answer
/// channel overwrites EAX, which would clobber the status byte.
unsafe fn fn3_exit(bl: u8) -> u8 {
    unsafe {
        callee_cdecl!(8, u32, relocated(0xE89269));
        bl
    }
}

/// First channel fill step toward the cap.
unsafe fn fn3_chan1(edi: u32) -> u8 {
    unsafe {
        const CAP: u32 = 0x895440;
        let ecx = fn3_g32(0x14);
        let mut ebx = fn3_g32(0x38);
        if ecx.wrapping_add(ebx) > CAP {
            let step = CAP.wrapping_sub(ecx);
            callee_cdecl!(6, u32, edi, fn3_g32(0x18).wrapping_add(ecx), step);
            let tag = fn3_g8(0x1C);
            fn3_s32(0x14, fn3_g32(0x14).wrapping_add(step));
            ebx = ebx.wrapping_sub(step);
            if tag == fn3_g8(0x45) {
                callee_cdecl!(
                    6,
                    u32,
                    edi,
                    fn3_g32(0x08).wrapping_add(fn3_g32(0x0C)),
                    ebx
                );
                fn3_s32(0x08, fn3_g32(0x08).wrapping_add(ebx));
                fn3_s32(0x00, fn3_g32(0x00).wrapping_add(1));
                fn3_s8(0x42, 0);
                return fn3_exit(0);
            }
            let div = *(relocated(0x11F6FFD) as *const u8);
            let rem = (((tag as u32).wrapping_add(1)) % (div as u32)) as u8;
            fn3_s32(0x14, 0);
            fn3_s8(0x1C, rem);
            let next =
                *((relocated(0x11F7000).wrapping_add((rem as u32).wrapping_mul(4)))
                    as *const u32);
            fn3_s32(0x18, next);
            callee_cdecl!(6, u32, edi, next, ebx);
            fn3_s32(0x14, fn3_g32(0x14).wrapping_add(ebx));
            fn3_s32(0x00, fn3_g32(0x00).wrapping_add(1));
            return fn3_exit(0);
        }
        callee_cdecl!(6, u32, edi, fn3_g32(0x18).wrapping_add(ecx), ebx);
        let new = fn3_g32(0x14).wrapping_add(ebx);
        fn3_s32(0x14, new);
        if new != CAP {
            fn3_s32(0x00, fn3_g32(0x00).wrapping_add(1));
            return fn3_exit(0);
        }
        let tag = fn3_g8(0x1C);
        if tag == fn3_g8(0x45) {
            fn3_s32(0x00, fn3_g32(0x00).wrapping_add(1));
            fn3_s8(0x42, 0);
            return fn3_exit(0);
        }
        let div = *(relocated(0x11F6FFD) as *const u8);
        let rem = (((tag as u32).wrapping_add(1)) % (div as u32)) as u8;
        let next = *((relocated(0x11F7000).wrapping_add((rem as u32).wrapping_mul(4)))
            as *const u32);
        fn3_s32(0x00, fn3_g32(0x00).wrapping_add(1));
        fn3_s32(0x14, 0);
        fn3_s8(0x1C, rem);
        fn3_s32(0x18, next);
        fn3_exit(0)
    }
}

/// Second channel fill step toward the cap.
unsafe fn fn3_chan2(edi: u32, al: u8) -> u8 {
    unsafe {
        const CAP: u32 = 0x895440;
        if al == 0 {
            fn3_s32(0x00, fn3_g32(0x00).wrapping_add(1));
            return fn3_exit(0);
        }
        let ecx = fn3_g32(0x08);
        let mut ebx = fn3_g32(0x38);
        if ecx.wrapping_add(ebx) > CAP {
            let step = CAP.wrapping_sub(ecx);
            callee_cdecl!(6, u32, edi, fn3_g32(0x0C).wrapping_add(ecx), step);
            let tag = fn3_g8(0x10);
            fn3_s32(0x08, fn3_g32(0x08).wrapping_add(step));
            ebx = ebx.wrapping_sub(step);
            if tag == fn3_g8(0x44) {
                fn3_s32(0x00, fn3_g32(0x00).wrapping_add(1));
                fn3_s8(0x43, 0);
                return fn3_exit(0);
            }
            let div = *(relocated(0x11F6FFB) as *const u8);
            let rem = (((tag as u32).wrapping_add(1)) % (div as u32)) as u8;
            fn3_s32(0x08, 0);
            fn3_s8(0x10, rem);
            let next =
                *((relocated(0x11F6F7C).wrapping_add((rem as u32).wrapping_mul(4)))
                    as *const u32);
            fn3_s32(0x0C, next);
            callee_cdecl!(6, u32, edi, next, ebx);
            fn3_s32(0x08, fn3_g32(0x08).wrapping_add(ebx));
            fn3_s32(0x00, fn3_g32(0x00).wrapping_add(1));
            return fn3_exit(0);
        }
        callee_cdecl!(6, u32, edi, fn3_g32(0x0C).wrapping_add(ecx), ebx);
        let new = fn3_g32(0x08).wrapping_add(ebx);
        fn3_s32(0x08, new);
        if new != CAP {
            fn3_s32(0x00, fn3_g32(0x00).wrapping_add(1));
            return fn3_exit(0);
        }
        let tag = fn3_g8(0x10);
        if tag == fn3_g8(0x44) {
            fn3_s32(0x00, fn3_g32(0x00).wrapping_add(1));
            fn3_s8(0x43, 0);
            return fn3_exit(0);
        }
        let div = *(relocated(0x11F6FFB) as *const u8);
        let rem = (((tag as u32).wrapping_add(1)) % (div as u32)) as u8;
        let next = *((relocated(0x11F6F7C).wrapping_add((rem as u32).wrapping_mul(4)))
            as *const u32);
        fn3_s32(0x00, fn3_g32(0x00).wrapping_add(1));
        fn3_s32(0x08, 0);
        fn3_s8(0x10, rem);
        fn3_s32(0x0C, next);
        fn3_exit(0)
    }
}

/// Tail step at the limit: finish, or top the second channel up first.
unsafe fn fn3_tail(edi: u32, al: u8, cl: u8) -> u8 {
    unsafe {
        const CAP: u32 = 0x895440;
        if fn3_g32(0x00) != fn3_g32(0x04) {
            fn3_s32(0x00, fn3_g32(0x00).wrapping_add(1));
            return fn3_exit(0);
        }
        let ecx = fn3_g32(0x08);
        if cl != 0 || al != 0 || fn3_g32(0x14) != CAP || ecx != CAP {
            let step = CAP.wrapping_sub(ecx);
            callee_cdecl!(6, u32, edi, fn3_g32(0x0C).wrapping_add(ecx), step);
            fn3_s32(0x08, fn3_g32(0x08).wrapping_add(step));
            callee_cdecl!(7, u32, edi);
            fn3_s32(0x00, fn3_g32(0x00).wrapping_add(1));
            return fn3_exit(1);
        }
        callee_cdecl!(7, u32, edi);
        fn3_s32(0x00, fn3_g32(0x00).wrapping_add(1));
        fn3_exit(1)
    }
}

/// Read a 32-bit word of the state block at 0x1037828.
unsafe fn fn3_g32(off: u32) -> u32 {
    unsafe { *(relocated(0x1037828 + off) as *const u32) }
}

/// Write a 32-bit word of the state block.
unsafe fn fn3_s32(off: u32, v: u32) {
    unsafe {
        *(relocated(0x1037828 + off) as *mut u32) = v;
    }
}

/// Read a byte of the state block.
unsafe fn fn3_g8(off: u32) -> u8 {
    unsafe { *(relocated(0x1037828 + off) as *const u8) }
}

/// Write a byte of the state block.
unsafe fn fn3_s8(off: u32, v: u8) {
    unsafe {
        *(relocated(0x1037828 + off) as *mut u8) = v;
    }
}

/// Write a 16-bit word of the state block.
unsafe fn fn3_s16(off: u32, v: u16) {
    unsafe {
        *(relocated(0x1037828 + off) as *mut u16) = v;
    }
}
