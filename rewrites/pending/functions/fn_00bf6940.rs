// original: 0x00bf6940 veh_overheat_blend_emit
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

#[inline(always)]
unsafe fn rd8(addr: u32) -> u8 {
    unsafe { (addr as *const u8).read() }
}

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr8(addr: u32, v: u8) {
    unsafe { (addr as *mut u8).write(v) }
}

#[inline(always)]
unsafe fn rf32(addr: u32) -> f32 {
    unsafe { (addr as *const f32).read_unaligned() }
}

// original: 0x00bf6940 veh_overheat_blend_emit (proposed)
/// Blend three floats from this object toward a source vector by factor t
/// and emit each through a keyed sink, then dispatch on the stored name
/// hash to decide the fourth emission.
///
/// Behaviour: emits `lerp(this+0x18, src+0x18)`, `lerp(0x1c)` and
/// `lerp(0x24)` with sink keys K0..K2 (id 1), where
/// `lerp(a, b) = (b - a) * t + a` in that exact operation order. Then
/// hashes nine candidate names (id 2) against the key at `this+8`: a
/// match in the first two emits `lerp(0x20)` with key KA and returns the
/// sink answer; a match in the next three emits it with key KB; a match
/// in the last four emits it with key KC only when `aux+0x1304 == 4`
/// (else the aux pointer itself is returned: the compare reloads EAX).
/// Those two paths then set `dst+0x1e2`
/// when bit 0 of `this+0x28` is set. No match returns the last hash
/// answer with no fourth emission.
export!(thiscall, rw_bf6940(
    this_ptr: u32,
    dst: u32,
    aux: u32,
    src: u32,
    t_bits: u32,
) -> u32 {
    const EMIT: u32 = 1; // 0x607490 thiscall/2 (key, value)
    const HASH: u32 = 2; // 0x40BA60 cdecl/2 (string, 0) -> hash
    const K0: u32 = 0x00EBC258;
    const K1: u32 = 0x00EBC260;
    const K2: u32 = 0x00EBC268;
    const KA: u32 = 0x00EBC2A4;
    const KB: u32 = 0x00EBC2F4;
    const KC: u32 = 0x00EBB9C8;
    const NAMES: [u32; 9] = [
        0x00EBC270, 0x00EBC28C, 0x00EBC2AC, 0x00EBC2C4, 0x00EBC2DC, 0x00EBB968,
        0x00EBB980, 0x00EBB998, 0x00EBB9B0,
    ];
    /// Blend one lane: (src - base) * t + base, matching the original's
    /// subss/mulss/addss order exactly (matters for signed zeros/NaNs).
    #[inline(always)]
    unsafe fn lane(base: u32, src: u32, t: f32) -> u32 {
        unsafe {
            let d = rf32(src) - rf32(base);
            let m = d * t;
            (m + rf32(base)).to_bits()
        }
    }
    unsafe {
        let t = f32::from_bits(t_bits);
        callee_thiscall!(
            EMIT,
            u32,
            dst,
            relocated(K0),
            lane(this_ptr + 0x18, src + 0x18, t)
        );
        callee_thiscall!(
            EMIT,
            u32,
            dst,
            relocated(K1),
            lane(this_ptr + 0x1c, src + 0x1c, t)
        );
        callee_thiscall!(
            EMIT,
            u32,
            dst,
            relocated(K2),
            lane(this_ptr + 0x24, src + 0x24, t)
        );
        let mut ans: u32 = 0;
        let mut i: usize = 0;
        while i < NAMES.len() {
            ans = callee_cdecl!(HASH, u32, relocated(NAMES[i]), 0);
            if rd32(this_ptr + 8) == ans {
                let fourth = lane(this_ptr + 0x20, src + 0x20, t);
                if i < 2 {
                    return callee_thiscall!(EMIT, u32, dst, relocated(KA), fourth);
                }
                let r = if i < 5 {
                    callee_thiscall!(EMIT, u32, dst, relocated(KB), fourth)
                } else if rd32(aux + 0x1304) == 4 {
                    callee_thiscall!(EMIT, u32, dst, relocated(KC), fourth)
                } else {
                    aux
                };
                if rd8(this_ptr + 0x28) & 1 != 0 {
                    wr8(dst + 0x1e2, 1);
                }
                return r;
            }
            i += 1;
        }
        ans
    }
});
