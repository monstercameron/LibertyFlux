// original: 0x00DC8BE0 pick_scaled_row (proposed)

use lf_checker_rt::{callee_cdecl, callee_thiscall, export};

/// Pick one of two float rows by a classifier answer, copying it to the
/// output, negating it on two of the four answers.
///
/// `out` receives four words (one integer word plus three floats, then one
/// integer word). `src` points at an object whose word at `+0x20` points at
/// a float block; the float at `+0x38` of that block and `key` seed a
/// classifier object built on the stack. `key` also selects the row table:
/// the word at `key+0x20` points at a row pair, and the classifier's answer
/// picks row 0 or row 1 of it.
///
/// The classifier runs in two calls: the first (nine stack words: `key`,
/// the seed float, 0.25, six zeros) prepares the object, the second (the
/// source float block at `+0x30`) answers 0-3, or something larger to take
/// nothing. Answers 0 and 2 take the second row, 1 and 3 the first; answers
/// 1 and 2 additionally multiply the three output floats by -1.0, in the
/// original's operand order (value times constant). The return value is
/// always `out`, on every path including "take nothing".
///
/// Original: 0x00DC8BE0 (cdecl, three stack words).
export!(cdecl, rw_dc8be0(out: u32, src: u32, key: u32) -> u32 {
    const SRC_BLOCK: u32 = 0x20;
    const SEED_OFF: u32 = 0x38;
    const PROBE_OFF: u32 = 0x30;
    const ROW_TAB: u32 = 0x20;
    const ROW_STRIDE: u32 = 0x10;
    const QUARTER: f32 = 0.25;
    const NEG_ONE: f32 = -1.0;
    const CAL_PREP: u32 = 1;
    const CAL_ASK: u32 = 2;
    const CAL_COOKIE: u32 = 3;

    #[inline(always)]
    fn mul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn rdf(a: u32) -> f32 {
        unsafe { f32::from_bits(rd32(a)) }
    }
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe fn wrf(a: u32, v: f32) {
        unsafe { wr32(a, v.to_bits()) }
    }

    unsafe {
        let block = rd32(src.wrapping_add(SRC_BLOCK));
        let seed = rdf(block.wrapping_add(SEED_OFF));
        // Classifier object on scratch: first word holds `key`, the
        // prepare call takes the object at +4 (matching the original's
        // two shifted views of one buffer).
        let mut obj = [0u32; 2];
        obj[0] = key;
        let base = obj.as_mut_ptr() as u32;
        callee_thiscall!(CAL_PREP, u32, base.wrapping_add(4), key, seed.to_bits(), QUARTER.to_bits(), 0, 0, 0, 0, 0, 0);
        let answer: u32 = callee_thiscall!(CAL_ASK, u32, base, block.wrapping_add(PROBE_OFF));
        // The cookie check runs on every exit path.
        let done = |out: u32| -> u32 {
            callee_cdecl!(CAL_COOKIE, u32, );
            out
        };
        let tab = rd32(key.wrapping_add(ROW_TAB));
        match answer {
            0 => {
                let row = tab.wrapping_add(ROW_STRIDE);
                wr32(out, rd32(row));
                wrf(out.wrapping_add(4), rdf(row.wrapping_add(4)));
                wrf(out.wrapping_add(8), rdf(row.wrapping_add(8)));
                wr32(out.wrapping_add(12), rd32(row.wrapping_add(12)));
                done(out)
            }
            1 => {
                wr32(out, rd32(tab));
                wrf(out.wrapping_add(4), rdf(tab.wrapping_add(4)));
                wrf(out.wrapping_add(8), rdf(tab.wrapping_add(8)));
                wr32(out.wrapping_add(12), rd32(tab.wrapping_add(12)));
                wrf(out, mul(rdf(out), NEG_ONE));
                wrf(out.wrapping_add(4), mul(rdf(out.wrapping_add(4)), NEG_ONE));
                wrf(out.wrapping_add(8), mul(rdf(out.wrapping_add(8)), NEG_ONE));
                done(out)
            }
            2 => {
                let row = tab.wrapping_add(ROW_STRIDE);
                wr32(out, rd32(row));
                wrf(out.wrapping_add(4), rdf(row.wrapping_add(4)));
                wrf(out.wrapping_add(8), rdf(row.wrapping_add(8)));
                wr32(out.wrapping_add(12), rd32(row.wrapping_add(12)));
                wrf(out, mul(rdf(out), NEG_ONE));
                wrf(out.wrapping_add(4), mul(rdf(out.wrapping_add(4)), NEG_ONE));
                wrf(out.wrapping_add(8), mul(rdf(out.wrapping_add(8)), NEG_ONE));
                done(out)
            }
            3 => {
                wr32(out, rd32(tab));
                wrf(out.wrapping_add(4), rdf(tab.wrapping_add(4)));
                wrf(out.wrapping_add(8), rdf(tab.wrapping_add(8)));
                wr32(out.wrapping_add(12), rd32(tab.wrapping_add(12)));
                done(out)
            }
            _ => done(out),
        }
    }
});
