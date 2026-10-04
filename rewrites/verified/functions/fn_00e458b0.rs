// original: 0x00E458B0 LB_RANKED
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

#[inline]
unsafe fn rd32(base: u32, off: u32) -> u32 {
    unsafe { ((base + off) as *const u32).read() }
}

// original: 0x00E458B0 LB_RANKED
// Ranked-leaderboard line builder: appends the "LB_RANKED" template, the
// "NET_TRUESKILL" segment when the detail flag is set, and a mapped per-mode
// segment into the wide-character buffer at +0x452, then formats the race
// selector. It never stores to the object itself; everything observable is
// the call sequence. The function makes the security-cookie call like any
// protected function, but that call is not part of the behaviour contract.
export!(thiscall, rw_e458b0(this: u32) -> u32 {
    unsafe {
        const QOBJ: u32 = 0x116BFF0;
        let obj = this;
        let flag = ((obj + 0x396) as *const u8).read();
        let mut tmp = [0u32; 8];
        let mut buf2 = [0u32; 4];

        callee_cdecl!(1u32, u32, relocated(0xF15C24), tmp.as_mut_ptr() as u32);
        let s = callee_thiscall!(2u32, u32, relocated(QOBJ), relocated(0xF15C28));
        callee_cdecl!(3u32, u32, obj.wrapping_add(0x452), s, 0xFFFFFFFFu32);
        callee_cdecl!(4u32, u32, obj.wrapping_add(0x452), tmp.as_mut_ptr() as u32);
        let mut acc = callee_cdecl!(5u32, u32, obj.wrapping_add(0x452));
        if flag != 0 {
            let s = callee_thiscall!(2u32, u32, relocated(QOBJ), relocated(0xF15C34));
            let p = obj.wrapping_add(0x452).wrapping_add(acc.wrapping_mul(2));
            callee_cdecl!(3u32, u32, p, s, 0xFFFFFFFFu32);
            callee_cdecl!(4u32, u32, p, tmp.as_mut_ptr() as u32);
            acc = acc.wrapping_add(callee_cdecl!(5u32, u32, p));
        }
        let v = callee_cdecl!(6u32, u32, rd32(obj, 0x388));
        let s = callee_thiscall!(2u32, u32, relocated(QOBJ), v);
        let p = obj.wrapping_add(0x452).wrapping_add(acc.wrapping_mul(2));
        callee_cdecl!(3u32, u32, p, s, 0xFFFFFFFFu32);
        if flag == 0 {
            callee_cdecl!(4u32, u32, p, tmp.as_mut_ptr() as u32);
        }
        let n3 = callee_cdecl!(5u32, u32, p);
        acc = acc.wrapping_add(n3);
        if flag != 0 {
            return n3;
        }
        // Mode selector: formats one race line for the known modes.
        let sw = rd32(obj, 0x384);
        if sw == 7 || sw == 6 {
            callee_cdecl!(8u32, u32, buf2.as_mut_ptr() as u32, relocated(0xF15C44), rd32(obj, 0x390));
        } else if sw == 0x14 {
            let v = rd32(obj, 0x390);
            let sv = v as i32;
            let (val, fmt) = if sv >= 0x79 {
                (v.wrapping_sub(0x78), relocated(0xF15C50))
            } else if sv >= 0x6A {
                (v.wrapping_sub(0x69), relocated(0xF15C5C))
            } else if sv >= 0x5B {
                (v.wrapping_sub(0x5A), relocated(0xF15C68))
            } else if sv >= 0x3D {
                (v.wrapping_sub(0x3C), relocated(0xF15C74))
            } else if sv >= 0x1F {
                (v.wrapping_sub(0x1E), relocated(0xF15C80))
            } else {
                (v, relocated(0xF15C8C))
            };
            callee_cdecl!(8u32, u32, buf2.as_mut_ptr() as u32, fmt, val);
        } else if sw == 0x1C || sw == 0x1D {
            let v = rd32(obj, 0x390);
            let sv = v as i32;
            let (val, fmt) = if sv > 0x3C {
                (v.wrapping_sub(0x3C), relocated(0xF15C98))
            } else if sv > 0x2D {
                (v.wrapping_sub(0x2D), relocated(0xF15CA4))
            } else if sv > 0x1E {
                (v.wrapping_sub(0x1E), relocated(0xF15CB0))
            } else if sv > 0x0F {
                (v.wrapping_sub(0x0F), relocated(0xF15CBC))
            } else {
                (v, relocated(0xF15CC8))
            };
            callee_cdecl!(8u32, u32, buf2.as_mut_ptr() as u32, fmt, val);
        }
        let s = callee_thiscall!(7u32, u32, relocated(QOBJ), buf2.as_mut_ptr() as u32);
        let p = obj.wrapping_add(0x452).wrapping_add(acc.wrapping_mul(2));
        callee_cdecl!(3u32, u32, p, s, 0xFFFFFFFFu32);
        callee_cdecl!(5u32, u32, p)
    }
});
