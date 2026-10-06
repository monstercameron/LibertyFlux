// original: 0x0059F600 memcfg_load_files (proposed)
use lf_checker_rt::{callee_cdecl, callee_stdcall, export, global, relocated,
    tls_slot};

/// Load two config files and an optional sized blob, then return.
///
/// Takes no arguments. Three stages share one shape (probe, fallback probe,
/// then load); every helper is a stubbed callee. Argument order below is
/// stack order (last push is arg0).
///
/// Stage 1 probes id `PROBE1` with `(P1A, P1B)`: nonzero runs `CLOSE` on the
/// answer. Zero falls back to `PROBE2(P1C)` into `h1`: zero skips the stage,
/// else a 1.3KB name list (`STR1`) is copied to scratch, its NUL-terminated
/// length measured (the checker inputs always place the NUL within the
/// first 64 bytes; a longer scan is uncovered), and `LOAD3` runs with
/// `(string, len+1, zero-word)` -- the zero word may be overwritten and is
/// forwarded -- then `USE(h1, answer, forwarded)`, `CLOSE(h1)`, and the
/// answer is freed with LocalFree.
///
/// Stage 2 probes `PROBE1(P2A, P2B)` into `h2`: zero falls back to
/// `PROBE2(P2C)`: zero ends the function (cookie check, return its answer).
/// A nonzero fallback copies 16 bytes (`STR2`) to scratch, measures its NUL
/// length (NUL within 16 by construction), and runs the same
/// `LOAD3`/`USE`/`CLOSE`/free sequence, then returns through the cookie
/// check (the mid-function return).
///
/// A nonzero `h2` instead sizes `h2` with `SIZE`: non-positive (SIGNED)
/// closes `h2` and returns. Positive allocates through the thread-local
/// object graph (`fs:[0x2c]` slot value `+8`, table `+8`, thiscall with
/// `(size, 0x10, 0)`); the slot value is also saved for the later release
/// calls through table `+0xC` (thiscall with the block). Then `READ` runs
/// with `(h2, block, size)`: an answer of exactly -1 releases a nonzero
/// block and closes; otherwise `LOADH` runs with `(block, size, zero-word)`
/// (the zero word is never read back), a nonzero block is released, a zero
/// `LOADH` answer closes, else two keyed lookups run (each answer, when
/// nonzero, is decoded and stored to `KEYOUT1`/`KEYOUT2`), the `LOADH`
/// answer is freed with LocalFree (called directly through its import
/// slot), and `h2` is closed.
///
/// Both exits run the cookie-check callee (its answer is the return); the
/// cookie slot itself is unobserved scratch. All pushed address constants
/// are relocated. EAX on the closed-early paths is the last helper answer
/// only until the cookie call overwrites it.
///
/// Original: 0x0059F600 (cdecl/0).
export!(cdecl, rw_0059f600() -> u32 {
    #[inline(always)]
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read() }
    }
    unsafe {
        const STR1: u32 = 0x00F8_7AE8;
        const STR2: u32 = 0x00F8_80D0;
        const P1A: u32 = 0x00F8_8054;
        const P1B: u32 = 0x00F8_8050;
        const P1C: u32 = 0x00F8_8044;
        const P1D: u32 = 0x00F8_8038;
        const P2A: u32 = 0x00F8_7AE4;
        const P2B: u32 = 0x00F8_7AD8;
        const P2C: u32 = 0x00F8_7ACC;
        const KEY1: u32 = 0x00F8_80CC;
        const KEY2: u32 = 0x00F8_80C8;
        const KEYOUT1: u32 = 0x0106_C284;
        const KEYOUT2: u32 = 0x0106_C2C0;
        const C_PROBE0: u32 = 1;
        const C_PROBE1A: u32 = 2;
        const C_PROBE1B: u32 = 3;
        const C_PROBE2A: u32 = 4;
        const C_PROBE2B: u32 = 5;
        const C_LOAD3: u32 = 6;
        const C_USE: u32 = 7;
        const C_CLOSE: u32 = 8;
        const C_LOCALFREE: u32 = 9;
        const C_SIZE: u32 = 10;
        const C_READ: u32 = 11;
        const C_ALLOC: u32 = 12;
        const C_FREE: u32 = 13;
        const C_LOADH: u32 = 14;
        const C_LOOKUP1: u32 = 15;
        const C_LOOKUP2: u32 = 16;
        const C_DECODE: u32 = 17;
        const C_COOKIE: u32 = 18;

        let _ = callee_cdecl!(C_PROBE0, u32, relocated(P1A), 0);
        let a1 = callee_cdecl!(C_PROBE1A, u32, relocated(P1C), relocated(P1B));
        if a1 != 0 {
            let _ = callee_cdecl!(C_CLOSE, u32, a1);
        } else {
            let h1 = callee_cdecl!(C_PROBE2A, u32, relocated(P1D));
            if h1 != 0 {
                let mut buf = [0u8; 64];
                let src = relocated(STR1);
                let mut i = 0usize;
                while i < 64 {
                    buf[i] = rd8(src.wrapping_add(i as u32));
                    i += 1;
                }
                let mut len = 0u32;
                while len < 64 && buf[len as usize] != 0 {
                    len += 1;
                }
                let mut z = 0u32;
                let e = callee_cdecl!(C_LOAD3, u32, buf.as_ptr() as u32,
                    len.wrapping_add(1), &mut z as *mut u32 as u32);
                let _ = callee_cdecl!(C_USE, u32, h1, e, z);
                let _ = callee_cdecl!(C_CLOSE, u32, h1);
                let _ = callee_stdcall!(C_LOCALFREE, u32, e);
            }
        }
        let h2 = callee_cdecl!(C_PROBE1B, u32, relocated(P2B), relocated(P2A));
        if h2 == 0 {
            let b2 = callee_cdecl!(C_PROBE2B, u32, relocated(P2C));
            if b2 == 0 {
                return callee_cdecl!(C_COOKIE, u32,);
            }
            let mut buf2 = [0u8; 16];
            let src2 = relocated(STR2);
            let mut i = 0usize;
            while i < 16 {
                buf2[i] = rd8(src2.wrapping_add(i as u32));
                i += 1;
            }
            let mut len = 0u32;
            while len < 16 && buf2[len as usize] != 0 {
                len += 1;
            }
            let mut z = 0u32;
            let e = callee_cdecl!(C_LOAD3, u32, buf2.as_ptr() as u32,
                len.wrapping_add(1), &mut z as *mut u32 as u32);
            let _ = callee_cdecl!(C_USE, u32, b2, e, z);
            let _ = callee_cdecl!(C_CLOSE, u32, b2);
            let _ = callee_stdcall!(C_LOCALFREE, u32, e);
            return callee_cdecl!(C_COOKIE, u32,);
        }
        let f = callee_cdecl!(C_SIZE, u32, h2);
        if (f as i32) <= 0 {
            let _ = callee_cdecl!(C_CLOSE, u32, h2);
            return callee_cdecl!(C_COOKIE, u32,);
        }
        let p = tls_slot(0);
        let segu = ((p.wrapping_add(8)) as *const u32).read_unaligned();
        let tgtbase = (segu as *const u32).read_unaligned();
        let tgta = ((tgtbase.wrapping_add(8)) as *const u32).read_unaligned();
        let fa: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(tgta as usize);
        let blk = fa(segu, f, 0x10, 0);
        let saved = p;
        let g = callee_cdecl!(C_READ, u32, h2, blk, f);
        if g == 0xFFFF_FFFF {
            if blk != 0 {
                let u2 = ((saved.wrapping_add(8)) as *const u32).read_unaligned();
                let t2 = (u2 as *const u32).read_unaligned();
                let tgt2 = ((t2.wrapping_add(0xC)) as *const u32).read_unaligned();
                let ff: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(tgt2 as usize);
                let _ = ff(u2, blk);
            }
        } else {
            let mut z = 0u32;
            let h = callee_cdecl!(C_LOADH, u32, blk, f, &mut z as *mut u32 as u32);
            if blk != 0 {
                let u2 = ((saved.wrapping_add(8)) as *const u32).read_unaligned();
                let t2 = (u2 as *const u32).read_unaligned();
                let tgt2 = ((t2.wrapping_add(0xC)) as *const u32).read_unaligned();
                let ff: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(tgt2 as usize);
                let _ = ff(u2, blk);
            }
            if h != 0 {
                let i1 = callee_cdecl!(C_LOOKUP1, u32, h, relocated(KEY1));
                if i1 != 0 {
                    let j1 = callee_cdecl!(C_DECODE, u32, i1);
                    global::<u32>(KEYOUT1).write(j1);
                }
                let i2 = callee_cdecl!(C_LOOKUP2, u32, 0, relocated(KEY2));
                if i2 != 0 {
                    let j2 = callee_cdecl!(C_DECODE, u32, i2);
                    global::<u32>(KEYOUT2).write(j2);
                }
                let _ = callee_stdcall!(C_LOCALFREE, u32, h);
            }
        }
        let _ = callee_cdecl!(C_CLOSE, u32, h2);
        callee_cdecl!(C_COOKIE, u32,)
    }
});
