// original: 0x00E4FF20 clips_favorite_row_build (proposed)
#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn rd16(a: u32) -> u32 {
    unsafe { (a as *const u16).read_unaligned() as u32 }
}
#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}
#[inline(always)]
unsafe fn rdf(a: u32) -> f32 {
    unsafe { f32::from_bits(rd32(a)) }
}
#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

const STORE: u32 = 0x1ec;
const AUX: u32 = 0x1f0;
const METER: u32 = 0x200;
const MATCH_ID: u32 = 0x328;
const RATE2: u32 = 0x32c;
const RATE: u32 = 0x330;
const VAL33C: u32 = 0x33c;
const COUNTER: u32 = 0x34c;
const LIST: u32 = 0x1e0;
const SUB: u32 = 0x1ec;
const DONE: u32 = 0x218;

const V_COUNT: u32 = 0x1d4;
const V_ITEMS: u32 = 0x1d0;
const V_TAG: u32 = 0x48;
const V_SETUP: u32 = 0x1fc;
const V_SETF: u32 = 0xa0;
const V_SETI: u32 = 0x24;
const V_TRACK: u32 = 0x4c;
const V_BIND0: u32 = 0x170;
const V_BIND1: u32 = 0x17c;
const V_BIND2: u32 = 0x188;
const V_ENABLE: u32 = 0x28;
const V_STAT: u32 = 0x23c;
const V_QRY: u32 = 0x4c;
const V_CFG: u32 = 0x104;
const V_ACT: u32 = 0x120;
const V_GAUGE: u32 = 0x98;
const V_FIN: u32 = 0x238;

const NEW1: u32 = 1;
const NEW2: u32 = 2;
const NEW300: u32 = 3;
const NEW25C: u32 = 4;
const CTOR1A: u32 = 5;
const CTOR1B: u32 = 6;
const FMT: u32 = 7;
const CTOR2: u32 = 8;
const INIT: u32 = 9;
const CHK: u32 = 10;
const STRA: u32 = 11;
const SPRINTF: u32 = 12;
const CTOR3: u32 = 13;
const CTOR4: u32 = 14;
const QUERY: u32 = 15;
const DISPOSE: u32 = 16;
const TAIL1: u32 = 17;
const TAIL2: u32 = 18;

const STR_ROW0: u32 = 0x00f1a2f4;
const STR_FMT: u32 = 0x00f1a338;
const STR_X: u32 = 0x00f1a354;
const STR_Y: u32 = 0x00f1a228;
const STR_Z: u32 = 0x00f1a360;
const STR_W: u32 = 0x00f1a38c;
const STR_V: u32 = 0x00f1a394;
const DTAB: u32 = 0x00fe8f50;
const TWO: u32 = 0x00fe8a24;

#[inline(always)]
unsafe fn vt0(obj: u32, slot: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + slot) as usize);
        f(obj)
    }
}
#[inline(always)]
unsafe fn vt1(obj: u32, slot: u32, a: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + slot) as usize);
        f(obj, a)
    }
}
#[inline(always)]
unsafe fn vt1f(obj: u32, slot: u32, a: f32) -> u32 {
    unsafe { vt1(obj, slot, a.to_bits()) }
}
#[inline(always)]
unsafe fn vt3(obj: u32, slot: u32, a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + slot) as usize);
        f(obj, a, b, c)
    }
}
#[inline(always)]
unsafe fn vt9(
    obj: u32, slot: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32,
    a7: u32, a8: u32, a9: u32,
) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + slot) as usize);
        f(obj, a1, a2, a3, a4, a5, a6, a7, a8, a9)
    }
}
#[inline(always)]
unsafe fn vtgauge(obj: u32) -> f32 {
    unsafe {
        let f: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(rd32(rd32(obj) + V_GAUGE) as usize);
        f(obj)
    }
}

/// Build the "favorite" rows of the clip gallery view and bind their tracks.
///
/// `this` points to the gallery object. It reads the store object at `+0x1ec`
/// (with the item list at store `+0x1e0` and a sub object at store `+0x1ec`),
/// two sibling objects at `+0x1f0` and `+0x200`, a match id at `+0x328`, two
/// rate floats at `+0x32c`/`+0x330`, a value word at `+0x33c`, and a counter
/// at `+0x34c` which it increments. It returns the store object.
///
/// Algorithm: ask the store for its row count. When the count is zero, create
/// a row builder through the fixed layout name; otherwise walk the item list
/// (each entry reached as base plus count times four, count read as an
/// UNSIGNED 16-bit word) counting entries whose id EQUALS the match id
/// (equality, so signedness is irrelevant) and keeping the last id that
/// differs. When no builder resulted, create one through the counted layout
/// name and remember that rates must be scaled. Then create the track set,
/// bind three tracks resolved through this object's own table, enable it,
/// and, when the store accepts the set, format the detail block, create the
/// detail object and run four query/configure passes over it. When scaling
/// was remembered, convert the row count to a float (through the double
/// table, so values above `i32::MAX` stay exact), scale the rates, and store
/// the results back. Finally, unless the row count is at most 2 (compared
/// UNSIGNED: `jbe`), activate the sub object; finalize the builder, copy the
/// last gauge value across, run the two tail calls, mark the store done, and
/// return it.
///
/// Edge cases: a failed creation yields a null builder and the function
/// faults on the next virtual call, exactly like the original. Floats follow
/// the original's operation order; the maximum keeps the running value on
/// unordered (NaN) comparison, matching `comiss`/`ja`.
///
/// Original: 0x00E4FF20 (thiscall, no stack arguments, returns a pointer).
/// `tail_signed` selects the wrong-version hook for the brief's signedness
/// check: false is the original's UNSIGNED `jbe`, true is a signed
/// comparison, which diverges on counts above `i32::MAX`.
unsafe fn run(this: u32, tail_signed: bool) -> u32 {
    unsafe {
        let store = rd32(this + STORE);
        let aux = rd32(this + AUX);
        let meter = rd32(this + METER);
        let mut count: u32 = 0;
        let mut flag: u8 = 0;
        let mut ebx: u32 = 0;
        let mut ebp: u32 = 0;
        let v = vt0(store, V_COUNT);
        if v == 0 {
            let raw = lf_checker_rt::callee_cdecl!(NEW1, u32, 0x1f8);
            if raw != 0 {
                let x = vt0(store, V_TAG);
                ebx = lf_checker_rt::callee_thiscall!(
                    CTOR1A,
                    u32,
                    raw,
                    lf_checker_rt::relocated(STR_ROW0),
                    x
                );
            }
            vt3(ebx, V_SETUP, 2, 0, 0);
            vt1f(ebx, V_SETF, rdf(this + RATE));
            vt1(ebx, V_SETI, 0);
            vt1f(store, V_SETF, rdf(this + RATE));
        } else {
            let list = rd32(store + LIST);
            let p1 = vt0(list, V_ITEMS);
            let mut esi = rd32(p1);
            let p2 = vt0(list, V_ITEMS);
            let mut end = rd32(p2).wrapping_add(rd16(p2 + 4).wrapping_mul(4));
            if esi != end {
                loop {
                    let item = rd32(esi);
                    let vid = vt0(item, V_COUNT);
                    if vid == rd32(this + MATCH_ID) {
                        count = count.wrapping_add(1);
                    } else {
                        ebx = item;
                    }
                    ebp = count;
                    let p3 = vt0(list, V_ITEMS);
                    end = rd32(p3).wrapping_add(rd16(p3 + 4).wrapping_mul(4));
                    esi = esi.wrapping_add(4);
                    if esi == end {
                        break;
                    }
                }
            }
        }
        if ebx == 0 {
            let raw2 = lf_checker_rt::callee_cdecl!(NEW2, u32, 0x1f8);
            if raw2 != 0 {
                let x = vt0(store, V_TAG);
                let w = lf_checker_rt::callee_cdecl!(
                    FMT,
                    u32,
                    lf_checker_rt::relocated(STR_FMT),
                    ebp
                );
                ebx = lf_checker_rt::callee_thiscall!(CTOR1B, u32, raw2, w, x);
            }
            vt3(ebx, V_SETUP, 2, 0, 0);
            vt1f(ebx, V_SETF, rdf(this + RATE));
            vt1(ebx, V_SETI, 0);
            flag = 1;
        }
        let raw3 = lf_checker_rt::callee_cdecl!(NEW300, u32, 0x300);
        if raw3 != 0 {
            let x = vt0(ebx, V_TAG);
            let w = lf_checker_rt::callee_cdecl!(
                FMT,
                u32,
                lf_checker_rt::relocated(STR_X),
                rd32(this + VAL33C)
            );
            ebp = lf_checker_rt::callee_thiscall!(CTOR2, u32, raw3, w, x);
        } else {
            ebp = 0;
        }
        count = 0xffff_ffff;
        lf_checker_rt::callee_thiscall!(
            INIT,
            u32,
            ebp,
            rd32(this + VAL33C),
            lf_checker_rt::relocated(STR_Y),
            &count as *const u32 as u32,
            rdf(this + RATE2).to_bits(),
            rdf(this + RATE).to_bits(),
            0,
            1,
            2
        );
        let t = vt0(this, V_TRACK);
        vt1(ebp, V_BIND0, t);
        let t = vt0(this, V_TRACK);
        vt1(ebp, V_BIND1, t);
        let t = vt0(this, V_TRACK);
        vt1(ebp, V_BIND2, t);
        vt1(ebp, V_ENABLE, 1);
        wr32(this + COUNTER, rd32(this + COUNTER).wrapping_add(1));
        let rc = vt0(ebp, V_STAT);
        let ok = lf_checker_rt::callee_thiscall!(CHK, u32, this, rc);
        if (ok & 0xff) != 0 {
            let s = lf_checker_rt::callee_cdecl!(STRA, u32, lf_checker_rt::relocated(STR_Z));
            let mut buf = [0u32; 8];
            lf_checker_rt::callee_cdecl!(
                SPRINTF,
                u32,
                buf.as_mut_ptr() as u32,
                0x3e
            );
            let raw4 = lf_checker_rt::callee_cdecl!(NEW25C, u32, 0x25c);
            let esi: u32;
            if raw4 != 0 {
                let x1 = vt0(ebp, V_TAG);
                let x2 = vt0(ebp, V_TAG);
                let w = lf_checker_rt::callee_cdecl!(
                    FMT,
                    u32,
                    lf_checker_rt::relocated(STR_W),
                    x2
                );
                esi = lf_checker_rt::callee_thiscall!(CTOR3, u32, raw4, w, x1);
                count = esi;
            } else {
                esi = 0;
                count = 0;
            }
            lf_checker_rt::callee_thiscall!(
                CTOR4,
                u32,
                esi,
                s,
                lf_checker_rt::relocated(STR_V),
                buf.as_mut_ptr() as u32,
                0xffff_ffff
            );
            let qthis = (buf.as_mut_ptr() as u32).wrapping_add(8);
            // Four query/configure passes; each answers 24 bytes that are
            // passed on to the detail object's wide configure call.
            let passes: [(u32, u32, u32, u32); 4] = [
                (0, 0xc1600000, 0x10, 4),
                (0, 0xc0800000, 0x10, 0x10),
                (0x40800000, 0, 2, 2),
                (0x41c00000, 0, 2, 8),
            ];
            for (a1, a2, pre, k) in passes {
                let ans = lf_checker_rt::callee_thiscall!(QUERY, u32, qthis, a1, a2);
                let w0 = rd32(ans);
                let w1 = rd32(ans.wrapping_add(4));
                let w2 = rd32(ans.wrapping_add(8));
                let w3 = rd32(ans.wrapping_add(12));
                let w4 = rd32(ans.wrapping_add(16));
                let w5 = rd32(ans.wrapping_add(20));
                let x = vt0(ebp, V_QRY);
                vt9(esi, V_CFG, k, x, pre, w0, w1, w2, w3, w4, w5);
                lf_checker_rt::callee_thiscall!(DISPOSE, u32, qthis);
            }
            ebp = esi;
            vt1(ebp, V_ACT, 1);
        }
        if flag != 0 {
            let c = vt0(store, V_COUNT);
            let tab = lf_checker_rt::global::<u64>(DTAB);
            let magic = f64::from_bits(unsafe {
                ((tab as *const u64).add((c >> 31) as usize)).read_unaligned()
            });
            let d = core::hint::black_box((c as i32) as f64) + core::hint::black_box(magic);
            let rate = rdf(this + RATE);
            let mut sum = fmul(d as f32, rate);
            let g1 = vtgauge(meter);
            sum = fadd(g1, sum);
            let g2 = vtgauge(meter);
            let t = fmul(rate, f32::from_bits(rd32(lf_checker_rt::relocated(TWO))));
            let u = fadd(g2, t);
            if u > sum {
                sum = u;
            }
            vt1f(aux, V_SETF, sum);
            let g3 = vtgauge(store);
            vt1f(store, V_SETF, fadd(g3, rate));
        }
        let c2 = vt0(store, V_COUNT);
        let skip: bool = if tail_signed {
            (c2 as i32) <= 2
        } else {
            c2 <= 2
        };
        if !skip {
            let sub = rd32(store + SUB);
            vt1(sub, V_ACT, 1);
        }
        vt0(ebx, V_FIN);
        let g4 = vtgauge(meter);
        vt1f(aux, V_SETF, g4);
        lf_checker_rt::callee_thiscall!(TAIL2, u32, aux);
        lf_checker_rt::callee_thiscall!(TAIL1, u32, store);
        wr8(store + DONE, 1);
        store
    }
}

lf_checker_rt::export!(thiscall, rw_00e4ff20(this: u32) -> u32 {
    unsafe { run(this, false) }
});
