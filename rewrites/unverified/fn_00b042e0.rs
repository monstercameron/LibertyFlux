// original: 0x00b042e0 chain_probe_outputs (proposed)

/// Walk a chained index over word tables, test six rows per step, and fill
/// two output structs unless a test diverts to the next step.
///
/// `arg` selects a bank (`BANK_STRIDE` bytes past the bank table); the dword
/// at `COUNT_OFF` past the bank base bounds the walk. The entry probe
/// (virtual slot `+0x24` of the shared object) decides once whether the
/// distance stage runs (`0x15` skips it). Each step chains the index through
/// the word at `CHAIN_OFF` past its slice (`(2*w)>>1` on the signed word),
/// scales four table words into factors, and scores six rows of the shared
/// object against them; a strictly negative score ends the step early. The
/// distance stage compares a squared distance against three bounds and may
/// also end the step early, otherwise the body decodes five more table words
/// and stores the node words, a 24-word block and a 32-word block before the
/// scan call (bank base, node, 1). The step counter, a 100-step cap and a
/// chain end (`-1`) bound the walk; the count check uses a signed compare.
/// The distance stage's fourth bound word survives in its slot across steps
/// (zero until the stage first runs); every other slot is rewritten per step.
/// Returns the bank base, or the global just loaded on the two early exits.
/// The original stores the bank base over its own argument slot, which no
/// rewrite can reproduce, so the stack check is off for this function.
///
/// Original: 0x00b042e0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00b042e0(arg: u32) -> u32 {
    unsafe {
        const BANK_STRIDE: u32 = 0x5580;
        const COUNT_OFF: u32 = 0x4000;
        const COUNT_LIM: i32 = 0x40;
        const STEP_CAP: i32 = 100;
        const SKIP_PROBE: u32 = 0x15;
        const VT_PROBE: u32 = 0x24;
        const ROW_BASE: u32 = 0x3b8;
        const CHAIN_OFF: u32 = 18;
        const M_SCAN: u32 = 2;
        const G_BANKTAB: u32 = 0x0160_1098;
        const G_OBJ: u32 = 0x012f_b1b8;
        const G_INIT: u32 = 0x0104_0050;
        const G_NODE: u32 = 0x0160_109c;
        const G_TAB: u32 = 0x0160_10e0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16i(a: u32) -> i16 {
            unsafe { (a as *const u16).read_unaligned() as i16 }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn frd(a: u32) -> f32 {
            unsafe { rdf(a) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn xorf(a: f32, b: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ b.to_bits())
        }

        let tab = lf_checker_rt::relocated(G_TAB);
    let c_eaa030_0: f32 = frd(lf_checker_rt::relocated(0x00eaa030) + 0x0);
    let c_eaa034_0: f32 = frd(lf_checker_rt::relocated(0x00eaa034) + 0x0);
    let c_eaa038_0: f32 = frd(lf_checker_rt::relocated(0x00eaa038) + 0x0);
    let c_eaa03c_0: f32 = frd(lf_checker_rt::relocated(0x00eaa03c) + 0x0);
    let c_eaa040_0: f32 = frd(lf_checker_rt::relocated(0x00eaa040) + 0x0);
    let c_eaa044_0: f32 = frd(lf_checker_rt::relocated(0x00eaa044) + 0x0);
    let c_eaa048_0: f32 = frd(lf_checker_rt::relocated(0x00eaa048) + 0x0);
    let c_eaa04c_0: f32 = frd(lf_checker_rt::relocated(0x00eaa04c) + 0x0);
    let c_eaa050_0: f32 = frd(lf_checker_rt::relocated(0x00eaa050) + 0x0);
    let c_eaa054_0: f32 = frd(lf_checker_rt::relocated(0x00eaa054) + 0x0);
    let c_eaa058_0: f32 = frd(lf_checker_rt::relocated(0x00eaa058) + 0x0);
    let c_eaa05c_0: f32 = frd(lf_checker_rt::relocated(0x00eaa05c) + 0x0);
    let c_fe8628_0: f32 = frd(lf_checker_rt::relocated(0x00fe8628) + 0x0);
    let c_fe8688_0: f32 = frd(lf_checker_rt::relocated(0x00fe8688) + 0x0);
    let c_fe87a4_0: f32 = frd(lf_checker_rt::relocated(0x00fe87a4) + 0x0);
    let c_fe87e4_0: f32 = frd(lf_checker_rt::relocated(0x00fe87e4) + 0x0);
    let c_fe8830_0: f32 = frd(lf_checker_rt::relocated(0x00fe8830) + 0x0);
    let c_fe8d1c_0: f32 = frd(lf_checker_rt::relocated(0x00fe8d1c) + 0x0);
    let c_fe8fa0_0: f32 = frd(lf_checker_rt::relocated(0x00fe8fa0) + 0x0);
    let c_110db70_0: f32 = frd(lf_checker_rt::relocated(0x0110db70) + 0x0);
    let c_110db74_0: f32 = frd(lf_checker_rt::relocated(0x0110db74) + 0x0);
    let c_110db78_0: f32 = frd(lf_checker_rt::relocated(0x0110db78) + 0x0);
        let banktab = lf_checker_rt::relocated(G_BANKTAB);
        let bankptr = rd32(banktab);
        let bankbase = arg
            .wrapping_mul(BANK_STRIDE)
            .wrapping_add(rd32(bankptr));
        if bankbase == 0 {
            return bankptr;
        }
        let gobj = rd32(lf_checker_rt::relocated(G_OBJ));
        let vt = rd32(gobj);
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(VT_PROBE)) as usize);
        let skip_dist = probe(gobj) == SKIP_PROBE;
        let g6 = rdf(gobj.wrapping_add(0x120));
        let g7 = rdf(gobj.wrapping_add(0x124));
        let mut esi = rd32(lf_checker_rt::relocated(G_INIT)) as i32;
        if esi == -1 {
            return gobj;
        }
        let mut edi: i32 = 0;
        let mut sbc: f32 = 0.0;
        let rowbase = gobj.wrapping_add(ROW_BASE);
        loop {
            let edx = (esi as u32).wrapping_mul(5);
            let edx4 = edx.wrapping_mul(4);
            let w9 = rd16i(tab.wrapping_add(edx4).wrapping_add(CHAIN_OFF));
            esi = (w9.wrapping_mul(2) as i32) >> 1;
            let eax2: i16 = rd16i(tab.wrapping_add(edx4).wrapping_add(0));
            let eax3: i16 = rd16i(tab.wrapping_add(edx4).wrapping_add(2));
            let eax4: i16 = rd16i(tab.wrapping_add(edx4).wrapping_add(4));
            let eax5: i16 = rd16i(tab.wrapping_add(edx4).wrapping_add(6));
            let eax6: i16 = rd16i(tab.wrapping_add(edx4).wrapping_add(8));
            let eax7: i16 = rd16i(tab.wrapping_add(edx4).wrapping_add(10));
            let eax8: i16 = rd16i(tab.wrapping_add(edx4).wrapping_add(12));
            let eax9: i16 = rd16i(tab.wrapping_add(edx4).wrapping_add(14));
            let eax10: i16 = rd16i(tab.wrapping_add(edx4).wrapping_add(16));
            let s10_1: f32 = mul((eax2 as f32), c_fe87e4_0);
            let s2c_1: f32 = mul((eax3 as f32), c_fe87e4_0);
            let s30_1: f32 = mul((eax4 as f32), c_fe87e4_0);
            let s20_1: f32 = mul((eax5 as f32), c_fe87e4_0);
            let mut ok6 = true;
            for k in 0..6u32 {
                let rb = rowbase.wrapping_add(k.wrapping_mul(0x10));
                let r0 = rdf(rb.wrapping_sub(8));
                let r1 = rdf(rb.wrapping_sub(4));
                let r2 = rdf(rb);
                let r3 = rdf(rb.wrapping_add(4));
                let t = add(
                    add(add(mul(r1, s2c_1), mul(r0, s10_1)), mul(s30_1, r2)),
                    r3,
                );
                let t = add(t, s20_1);
                if 0.0 > t {
                    ok6 = false;
                    break;
                }
            }
            let mut run_body = true;
            if ok6 && !skip_dist {
            let s150_1: f32 = add(mul(sub(g7, mul((eax3 as f32), c_fe87e4_0)), sub(g7, mul((eax3 as f32), c_fe87e4_0))), mul(sub(g6, mul((eax2 as f32), c_fe87e4_0)), sub(g6, mul((eax2 as f32), c_fe87e4_0))));
            let s154_1: f32 = add(mul(sub(g7, mul((eax3 as f32), c_fe87e4_0)), sub(g7, mul((eax3 as f32), c_fe87e4_0))), mul(sub(g6, mul((eax2 as f32), c_fe87e4_0)), sub(g6, mul((eax2 as f32), c_fe87e4_0))));
            let s158_1: f32 = add(mul(sub(g7, mul((eax3 as f32), c_fe87e4_0)), sub(g7, mul((eax3 as f32), c_fe87e4_0))), mul(sub(g6, mul((eax2 as f32), c_fe87e4_0)), sub(g6, mul((eax2 as f32), c_fe87e4_0))));
            let s15c_1: f32 = add(mul(sub(g7, mul((eax3 as f32), c_fe87e4_0)), sub(g7, mul((eax3 as f32), c_fe87e4_0))), mul(sub(g6, mul((eax2 as f32), c_fe87e4_0)), sub(g6, mul((eax2 as f32), c_fe87e4_0))));
            let s170_1: f32 = s150_1;
            let s174_1: f32 = s154_1;
            let s178_1: f32 = s158_1;
            let s17c_1: f32 = s15c_1;
            let s16c_1: f32 = mul(add(c_eaa03c_0, add(mul(c_eaa05c_0, s20_1), mul(c_eaa04c_0, mul(mul((eax5 as f32), c_fe87e4_0), mul((eax5 as f32), c_fe87e4_0))))), add(c_eaa03c_0, add(mul(c_eaa05c_0, s20_1), mul(c_eaa04c_0, mul(mul((eax5 as f32), c_fe87e4_0), mul((eax5 as f32), c_fe87e4_0))))));
            let s160_1: f32 = mul(add(add(mul(c_eaa040_0, mul(mul((eax5 as f32), c_fe87e4_0), mul((eax5 as f32), c_fe87e4_0))), mul(c_eaa050_0, mul((eax5 as f32), c_fe87e4_0))), c_eaa030_0), add(add(mul(c_eaa040_0, mul(mul((eax5 as f32), c_fe87e4_0), mul((eax5 as f32), c_fe87e4_0))), mul(c_eaa050_0, mul((eax5 as f32), c_fe87e4_0))), c_eaa030_0));
            let s164_1: f32 = mul(add(add(mul(c_eaa044_0, mul(mul((eax5 as f32), c_fe87e4_0), mul((eax5 as f32), c_fe87e4_0))), mul(c_eaa054_0, s20_1)), c_eaa034_0), add(add(mul(c_eaa044_0, mul(mul((eax5 as f32), c_fe87e4_0), mul((eax5 as f32), c_fe87e4_0))), mul(c_eaa054_0, s20_1)), c_eaa034_0));
            let s168_1: f32 = mul(add(add(mul(c_eaa048_0, mul(mul((eax5 as f32), c_fe87e4_0), mul((eax5 as f32), c_fe87e4_0))), mul(c_eaa058_0, s20_1)), c_eaa038_0), add(add(mul(c_eaa048_0, mul(mul((eax5 as f32), c_fe87e4_0), mul((eax5 as f32), c_fe87e4_0))), mul(c_eaa058_0, s20_1)), c_eaa038_0));
                if s150_1 > s160_1 {
            let sb0_1: f32 = s160_1;
            let sb4_1: f32 = s164_1;
            let sb8_1: f32 = s168_1;
            sbc = s16c_1;
                    if s174_1 > sb4_1 && s178_1 > sb8_1 {
                        run_body = false;
                    }
                }
            }
            if ok6 && run_body {
                let sn = rd32(lf_checker_rt::relocated(G_NODE));
                let out2 = rd32(sn.wrapping_add(0x10));
                let outp = rd32(sn.wrapping_add(0x18));
            let sa4_1: f32 = mul((eax9 as f32), c_fe8688_0);
            let s50_1: f32 = xorf(mul((eax9 as f32), c_fe8688_0), c_fe8fa0_0);
            let s40_1: f32 = mul(mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0), 0.0);
            wrf(sn + 0x0, s10_1);
            wrf(sn + 0x4, s2c_1);
            let s68_1: f32 = mul((eax8 as f32), c_fe8688_0);
            wrf(sn + 0x8, s30_1);
            wrf(sn + 0xc, s20_1);
            let s20_2: f32 = add(mul((eax10 as f32), c_fe87a4_0), s30_1);
            let s30_2: f32 = sub(s30_1, mul((eax10 as f32), c_fe87a4_0));
            let sf0_1: f32 = add(add(mul(xorf(mul((eax9 as f32), c_fe8688_0), c_fe8fa0_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0)), mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0))), s10_1);
            let s138_1: f32 = add(add(mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0)), mul(mul((eax9 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0))), s2c_1);
            let s110_1: f32 = add(add(mul(mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0), 0.0), s40_1), s20_2);
            let s40_2: f32 = add(add(mul(xorf(mul((eax9 as f32), c_fe8688_0), c_fe8fa0_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0)), mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0))), s10_1);
            let sac_1: f32 = add(add(mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0)), mul(mul((eax9 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0))), s2c_1);
            let s80_1: f32 = add(add(mul(mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0), 0.0), s40_1), s30_2);
            let sc0_1: f32 = add(sub(mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0)), mul(xorf(mul((eax9 as f32), c_fe8688_0), c_fe8fa0_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0))), s10_1);
            let se4_1: f32 = add(sub(mul(mul((eax9 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0)), mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0))), s2c_1);
            let s128_1: f32 = add(sub(s40_1, mul(mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0), 0.0)), s20_2);
            let s88_1: f32 = sub(s20_2, add(mul(mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0), 0.0), s40_1));
            let sd0_1: f32 = add(sub(mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0)), mul(xorf(mul((eax9 as f32), c_fe8688_0), c_fe8fa0_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0))), s10_1);
            let s100_1: f32 = sub(s10_1, add(mul(xorf(mul((eax9 as f32), c_fe8688_0), c_fe8fa0_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0)), mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0))));
            let se0_1: f32 = add(sub(mul(mul((eax9 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0)), mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0))), s2c_1);
            let se8_1: f32 = add(sub(s40_1, mul(mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0), 0.0)), s30_2);
            let s70_1: f32 = sub(s10_1, add(mul(xorf(mul((eax9 as f32), c_fe8688_0), c_fe8fa0_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0)), mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0))));
            let s118_1: f32 = sub(s2c_1, add(mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0)), mul(mul((eax9 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0))));
            let s120_1: f32 = sub(s30_2, add(mul(mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0), 0.0), s40_1));
            let s13c_1: f32 = sub(s2c_1, add(mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0)), mul(mul((eax9 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0))));
            let s140_1: f32 = sub(s10_1, sub(mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0)), mul(xorf(mul((eax9 as f32), c_fe8688_0), c_fe8fa0_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0))));
            let s10_2: f32 = sub(s10_1, sub(mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0)), mul(xorf(mul((eax9 as f32), c_fe8688_0), c_fe8fa0_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0))));
            let s11c_1: f32 = sub(s2c_1, sub(mul(mul((eax9 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0)), mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0))));
            let s2c_2: f32 = sub(s2c_1, sub(mul(mul((eax9 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0)), mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0))));
            let s20_3: f32 = sub(s20_2, sub(s40_1, mul(mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0), 0.0)));
            let s30_3: f32 = sub(s30_2, sub(s40_1, mul(mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0), 0.0)));
            wrf(out2 + 0x0, c_110db70_0);
            wrf(out2 + 0x4, c_110db74_0);
            wrf(out2 + 0x8, c_110db78_0);
            wrf(out2 + 0xc, add(add(mul(sub(s2c_1, add(mul(mul((eax8 as f32), c_fe8688_0), mul(mul((eax6 as f32), c_fe8830_0), c_fe87e4_0)), mul(mul((eax9 as f32), c_fe8688_0), mul(mul((eax7 as f32), c_fe8830_0), c_fe87e4_0)))), c_110db74_0), mul(s70_1, c_110db70_0)), mul(s88_1, c_110db78_0)));
            wrf(out2 + 0x10, s50_1);
            wrf(out2 + 0x14, s68_1);
            wr32(out2 + 0x18, 0x0);
            wrf(out2 + 0x1c, add(add(mul(s40_2, s50_1), mul(sac_1, s68_1)), mul(s80_1, c_fe8628_0)));
            wrf(out2 + 0x28, xorf(c_110db78_0, c_fe8fa0_0));
            wrf(out2 + 0x20, xorf(c_110db70_0, c_fe8fa0_0));
            wrf(out2 + 0x24, xorf(c_110db74_0, c_fe8fa0_0));
            wrf(out2 + 0x2c, add(add(mul(xorf(c_110db74_0, c_fe8fa0_0), sac_1), mul(xorf(c_110db70_0, c_fe8fa0_0), s40_2)), mul(xorf(c_110db78_0, c_fe8fa0_0), s80_1)));
            wrf(out2 + 0x30, xorf(s50_1, c_fe8fa0_0));
            wrf(out2 + 0x34, xorf(s68_1, c_fe8fa0_0));
            wr32(out2 + 0x38, 0x80000000);
            wrf(out2 + 0x3c, add(add(mul(s13c_1, xorf(s68_1, c_fe8fa0_0)), mul(xorf(s50_1, c_fe8fa0_0), s70_1)), mul(s88_1, c_fe8d1c_0)));
            wrf(out2 + 0x40, xorf(s68_1, c_fe8fa0_0));
            wrf(out2 + 0x44, s50_1);
            wr32(out2 + 0x48, 0x80000000);
            wrf(out2 + 0x4c, add(add(mul(s70_1, xorf(s68_1, c_fe8fa0_0)), mul(s13c_1, s50_1)), mul(s88_1, c_fe8d1c_0)));
            wr32(out2 + 0x58, 0x0);
            wrf(out2 + 0x50, s68_1);
            wrf(out2 + 0x54, sa4_1);
            wrf(out2 + 0x5c, add(add(mul(sac_1, sa4_1), mul(s40_2, s68_1)), mul(s80_1, c_fe8628_0)));
            wrf(outp + 0x0, sc0_1);
            wrf(outp + 0x4, se4_1);
            wrf(outp + 0x8, s128_1);
            wrf(outp + 0xc, sbc);
            wrf(outp + 0x1c, sbc);
            wrf(outp + 0x10, s70_1);
            wrf(outp + 0x14, s13c_1);
            wrf(outp + 0x18, s88_1);
            wrf(outp + 0x20, sf0_1);
            wrf(outp + 0x24, s138_1);
            wrf(outp + 0x28, s110_1);
            wrf(outp + 0x2c, sbc);
            wrf(outp + 0x30, s140_1);
            wrf(outp + 0x34, s11c_1);
            wrf(outp + 0x38, s20_3);
            wrf(outp + 0x3c, sbc);
            wrf(outp + 0x48, s80_1);
            wrf(outp + 0x4c, sbc);
            wrf(outp + 0x40, s40_2);
            wrf(outp + 0x44, sac_1);
            wrf(outp + 0x50, s10_2);
            wrf(outp + 0x54, s2c_2);
            wrf(outp + 0x58, s30_3);
            wrf(outp + 0x5c, sbc);
            wrf(outp + 0x60, sd0_1);
            wrf(outp + 0x64, se0_1);
            wrf(outp + 0x68, se8_1);
            wrf(outp + 0x6c, sbc);
            wrf(outp + 0x70, s100_1);
            wrf(outp + 0x74, s118_1);
            wrf(outp + 0x78, s120_1);
            wrf(outp + 0x7c, sbc);
                let _: u32 = lf_checker_rt::callee_cdecl!(M_SCAN, u32, bankbase, sn, 1u32,);
            }
            let count = rd32(bankbase.wrapping_add(COUNT_OFF)) as i32;
            if count >= COUNT_LIM {
                break;
            }
            edi = edi.wrapping_add(1);
            if edi >= STEP_CAP {
                break;
            }
            if esi == -1 {
                break;
            }
        }
        bankbase
    }
});
