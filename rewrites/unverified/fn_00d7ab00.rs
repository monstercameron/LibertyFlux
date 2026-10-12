// original: 0x00D7AB00 compute_input_response

// File VAs of the constants (loaded from the image) and globals this function uses.
const RADIUS_CONST: u32 = 0x00EE_C91C; // 21.0: half-width of the grid square
const SCALE_CONST: u32 = 0x00FE_8734; // 0.02: grid scale
const BIAS_CONST: u32 = 0x00FE_8B80; // 60.0: grid bias
const MAGIC_CONST: u32 = 0x00FE_8CF8; // 2^23: round-to-integer magic
const SIGN_CONST: u32 = 0x00FE_8D1C; // -0.0: sign-bit mask (as a float)
const ONE_CONST: u32 = 0x00FE_88E8; // 1.0
const THRESHOLD_CONST: u32 = 0x00FE_870C; // 0.01: level threshold
const HALF_CONST: u32 = 0x00FE_8830; // 0.5: final scale
const WRITE_CONST_100: u32 = 0x0179_76AC; // dword written with 100.0 (0x42C80000)
const GLOBAL_ADD: u32 = 0x0117_35B4; // dword added to the lookup result
const CELL_TABLE: u32 = 0x011A_8918; // 256 cell records of 20 bytes
const CELL_RECORD_SIZE: u32 = 20;
const ID_E40_CALLEE: u32 = 1; // thiscall/1 on the counter word at +0xE40
const ID_GRID_COUNTER: u32 = 2; // cdecl/0 counter
const ID_FLOAT_HELPER: u32 = 3; // cdecl/2 of two floats, float result in ST0
const ID_LOOKUP: u32 = 4; // cdecl/1 result of the lookup added to a global
const ID_SIDE_CALL: u32 = 5; // cdecl/1 side effect on the struct
const ID_CELL_A: u32 = 6; // cdecl/10 on a cell record (skipped for mode 6)
const ID_CELL_B: u32 = 7; // cdecl/10 on a cell record, second part
const ID_E44_HELPER: u32 = 8; // cdecl/6 on the +0xE44 object, the level copy and the struct
const ID_POOL_CELL_A: u32 = 9; // cdecl/10 of the pool path, first form (return slot first)
const ID_POOL_CELL_B: u32 = 10; // cdecl/10 of the pool path, second form (return slot first)
const ID_SLOT_HELPER: u32 = 11; // thiscall/0 on the return slot (this = slot address)
const ID_POOL_ADD: u32 = 12; // thiscall/2 on the pool object (slot address, a size)
const ID_POOL_GET: u32 = 13; // thiscall/1 pool lookup (this = pool, the limit)
const POOL_GLOBAL: u32 = 0x0012_FB214; // dword: the pool object
const GLOBAL_WORD: u32 = 0x0011_A8908; // 16-bit global copied into the +0xE44 object's +0x3C word

// Struct offsets of the three objects (bigstruct, modeobj, aux) and the geometry block.
const BIG_GEO_PTR: u32 = 0x20;
const BIG_COUNTER: u32 = 0xE40;
const BIG_E44: u32 = 0xE44;
const BIG_LIMIT: u32 = 0x48;
const BIG_E70: u32 = 0xE70;
const BIG_EF0: u32 = 0xEF0;
const BIG_EF8: u32 = 0xEF8;
const BIG_F15: u32 = 0xF15;
const BIG_F50: u32 = 0xF50;
const MODE_BYTE: u32 = 0x28;
const MODE_WEIGHT: u32 = 0x27;
const AUX_FLAG: u32 = 0x2A;
const AUX_RESULT: u32 = 0x10;
const GEO_A: u32 = 0x30;
const GEO_B: u32 = 0x34;
const MODE_CELL_SKIP: u8 = 6;

#[inline(always)]
unsafe fn rd32(va: u32) -> u32 {
    *(va as *const u32)
}

#[inline(always)]
unsafe fn wr32(va: u32, v: u32) {
    *(va as *mut u32) = v;
}

#[inline(always)]
unsafe fn rd8(va: u32) -> u8 {
    *(va as *const u8)
}

#[inline(always)]
unsafe fn wr8(va: u32, v: u8) {
    *(va as *mut u8) = v;
}

#[inline(always)]
unsafe fn image_f32(file_va: u32) -> f32 {
    *lf_checker_rt::global::<f32>(file_va)
}

#[inline(always)]
unsafe fn image_u32(file_va: u32) -> u32 {
    *lf_checker_rt::global::<u32>(file_va)
}

// Float operations in the order the original's SSE instructions use: destination first,
// both operands pinned so the compiler cannot swap them.
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

// cvttss2si: truncation to a 32-bit integer; the indefinite value 0x80000000 for a
// value that is not a number or lies outside the 32-bit range.
#[inline(always)]
fn cvtt_i32(f: f32) -> i32 {
    if f.is_nan() || f >= 2147483648.0 || f < -2147483648.0 {
        i32::MIN
    } else {
        // Bounded above: the test just before this cast keeps f inside the i32 range.
        f as i32
    }
}

// One rounding block of the original: X = base*S + B; the value is rounded to an integer
// with the 2^23 magic (the sign of X is carried into the magic), corrected by one when the
// rounded value lies on the far side of X for the sign of X, and truncated.
#[inline(always)]
unsafe fn grid_index(base: f32, scale: f32, bias: f32, magic: f32, sign_mask: u32, one: f32) -> i32 {
    let x = fadd(fmul(base, scale), bias);
    let sgn = x.to_bits() & sign_mask;
    let abs_bits = x.to_bits() ^ sgn;
    let in_range: u32 = if f32::from_bits(abs_bits) < magic { 0xFFFF_FFFF } else { 0 };
    let r = f32::from_bits((magic.to_bits() & in_range) | sgn);
    let y = fsub(fadd(x, r), r);
    let diff = fsub(y, x);
    let far: u32 = if !(diff <= f32::from_bits(sgn)) { 0xFFFF_FFFF } else { 0 };
    let corr = f32::from_bits(far & one.to_bits());
    let floored = fsub(y, corr);
    cvtt_i32(floored)
}

// Per-call float and pointer arguments shared by both cell callees.
#[derive(Clone, Copy)]
struct CellArgs {
    big: u32,
    a_lo: f32,
    b_lo: f32,
    a_hi: f32,
    b_hi: f32,
    level_local: u32,
    r_w: f32,
    modeobj: u32,
    aux: u32,
}

// First cell callee: on the cell record, with the record address as first argument.
#[inline(always)]
unsafe fn call_cell_a(rec: u32, args: CellArgs) {
    let f: extern "cdecl" fn(u32, u32, f32, f32, f32, f32, u32, f32, u32, u32) =
        core::mem::transmute(lf_checker_rt::callee_addr(ID_CELL_A) as usize);
    f(rec, args.big, args.a_lo, args.b_lo, args.a_hi, args.b_hi, args.level_local, args.r_w,
        args.modeobj, args.aux);
}

// Second cell callee: on the cell record's second part (record + 4).
#[inline(always)]
unsafe fn call_cell_b(rec_part: u32, args: CellArgs) {
    let f: extern "cdecl" fn(u32, u32, f32, f32, f32, f32, u32, f32, u32, u32) =
        core::mem::transmute(lf_checker_rt::callee_addr(ID_CELL_B) as usize);
    f(rec_part, args.big, args.a_lo, args.b_lo, args.a_hi, args.b_hi, args.level_local, args.r_w,
        args.modeobj, args.aux);
}

// Pool path helpers: the slot helper takes the return slot as its this; the cell callees of
// the pool path take the return slot first and the same arguments as the grid cells.
#[inline(always)]
unsafe fn call_slot_helper(slot: u32) {
    let f: extern "thiscall" fn(u32) -> u32 =
        core::mem::transmute(lf_checker_rt::callee_addr(ID_SLOT_HELPER) as usize);
    f(slot);
}

#[inline(always)]
unsafe fn call_pool_cell_a(slot: u32, args: CellArgs) {
    let f: extern "cdecl" fn(u32, u32, f32, f32, f32, f32, u32, f32, u32, u32) =
        core::mem::transmute(lf_checker_rt::callee_addr(ID_POOL_CELL_A) as usize);
    f(slot, args.big, args.a_lo, args.b_lo, args.a_hi, args.b_hi, args.level_local, args.r_w,
        args.modeobj, args.aux);
}

#[inline(always)]
unsafe fn call_pool_cell_b(slot: u32, args: CellArgs) {
    let f: extern "cdecl" fn(u32, u32, f32, f32, f32, f32, u32, f32, u32, u32) =
        core::mem::transmute(lf_checker_rt::callee_addr(ID_POOL_CELL_B) as usize);
    f(slot, args.big, args.a_lo, args.b_lo, args.a_hi, args.b_hi, args.level_local, args.r_w,
        args.modeobj, args.aux);
}

// The +0xE44 helper: six arguments (the object, the struct, the level copy by address, the
// float, the mode object and the aux object).
#[inline(always)]
unsafe fn call_e44_helper(obj: u32, big: u32, level_local: u32, r_w: f32, modeobj: u32, aux: u32) {
    let f: extern "cdecl" fn(u32, u32, u32, f32, u32, u32) -> u32 =
        core::mem::transmute(lf_checker_rt::callee_addr(ID_E44_HELPER) as usize);
    f(obj, big, level_local, r_w, modeobj, aux);
}

/// Compute the input response for a state object (cdecl, four arguments: the big struct,
/// the mode object, an aux object and a level float; the result is a float in ST0).
///
/// A mode byte of 2, 3, 5 or 7 returns the level at once. Otherwise: a non-zero counter word
/// goes to a counter callee and is cleared; a fixed float (100.0) is stored; four corners
/// around the geometry point (each offset by a radius, scaled and biased, then rounded to
/// an integer by the 2^23 magic with the sign carried over, corrected and truncated with the
/// instruction's indefinite value for out-of-range input) give the grid bounds, clamped
/// to 0 below and 119 above. A grid counter callee runs, then a helper gets the weight byte
/// and a struct float. A non-null object at +0xE44 is handled first (its counter callee,
/// the word cleared, the 16-bit global copied into the object, the helper call). If the
/// limit at +0x48 is non-negative, the pool lookup decides: a found object takes the pool
/// path (return slot zeroed, helpers and cell callees in pool form, return slot taken from
/// the level copy); otherwise, and whenever the bounds are not reversed, each grid cell whose
/// level is positive gets two cell callees (the first only when the mode is not 6) with the
/// record address; the level copy is carried from one cell to the next. The final section
/// sets bit 0x80 of a struct byte and, when the level is above 0.01, takes the flags path
/// (bit 1 cleared; a lookup for the aux object when its flag is zero; the aux flag and result
/// set); otherwise a side callee runs when the level is not above zero; the result is the
/// level copy for the kinds 0, 1 and 4 and otherwise the return value plus the level, halved.
lf_checker_rt::export!(cdecl, rw_d7ab00(
    bigstruct: u32,
    modeobj: u32,
    aux: u32,
    level_bits: u32,
) -> f32 {
    unsafe {
        let level = f32::from_bits(level_bits);
        let mode = rd8(modeobj.wrapping_add(MODE_BYTE));
        if mode == 2 || mode == 3 || mode == 5 || mode == 7 {
            return level;
        }

        // Counter word: when non-zero, the counter callee takes it as `this` and the word's
        // address as its argument; the word is then cleared.
        let counter_addr = bigstruct.wrapping_add(BIG_COUNTER);
        let counter = rd32(counter_addr);
        if counter != 0 {
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(lf_checker_rt::callee_addr(ID_E40_CALLEE) as usize);
            f(counter, counter_addr);
        }
        let c0 = image_f32(RADIUS_CONST);
        let scale = image_f32(SCALE_CONST);
        let bias = image_f32(BIAS_CONST);
        let magic = image_f32(MAGIC_CONST);
        let sign_mask = image_u32(SIGN_CONST);
        let one = image_f32(ONE_CONST);
        wr32(counter_addr, 0);
        *lf_checker_rt::global::<u32>(WRITE_CONST_100) = 0x42C8_0000;

        // Geometry corners: the object at +0x20 holds two floats; each gets a radius of C0
        // on either side. Each of the four corners goes through one rounding block.
        let geo = rd32(bigstruct.wrapping_add(BIG_GEO_PTR));
        let a = f32::from_bits(rd32(geo.wrapping_add(GEO_A)));
        let b = f32::from_bits(rd32(geo.wrapping_add(GEO_B)));
        let a_hi = fadd(a, c0);
        let a_lo = fsub(a, c0);
        let b_lo = fsub(b, c0);
        let b_hi = fadd(b, c0);
        let idx_1 = grid_index(a_lo, scale, bias, magic, sign_mask, one);
        let idx_2 = grid_index(b_lo, scale, bias, magic, sign_mask, one);
        let idx_3 = grid_index(a_hi, scale, bias, magic, sign_mask, one);
        let idx_4 = grid_index(b_hi, scale, bias, magic, sign_mask, one);
        // Bounds: lower clamped at 0 (first and second), upper clamped at 119 (third and fourth).
        let lo_x = if idx_1 > 0 { idx_1 } else { 0 };
        let lo_y = if idx_2 > 0 { idx_2 } else { 0 };
        let hi_x = if idx_3 < 0x77 { idx_3 } else { 0x77 };
        let hi_y = if idx_4 < 0x77 { idx_4 } else { 0x77 };

        // The grid counter callee runs with no arguments; its answer is not used.
        let g: extern "cdecl" fn() -> u32 =
            core::mem::transmute(lf_checker_rt::callee_addr(ID_GRID_COUNTER) as usize);
        g();

        // Float helper: the mode weight byte as a float, and the struct float at +0xEF0.
        let weight = (rd8(modeobj.wrapping_add(MODE_WEIGHT)) as u32 as i32) as f32;
        let ef0 = f32::from_bits(rd32(bigstruct.wrapping_add(BIG_EF0)));
        let helper: extern "cdecl" fn(f32, f32) -> f32 =
            core::mem::transmute(lf_checker_rt::callee_addr(ID_FLOAT_HELPER) as usize);
        let r_w = helper(weight, ef0);

        // ret is the value the function returns through the arguments; lvl is the frame's
        // copy of the level that the callees receive by address.
        let mut ret = level;
        let mut lvl = level;
        // A non-null object at +0xE44: the counter callee takes the object and the word's
        // address; the word is cleared, the object's +0x3C word takes the 16-bit global, and
        // the object goes to the second helper with the level copy.
        let e44_addr = bigstruct.wrapping_add(BIG_E44);
        let e44 = rd32(e44_addr);
        if e44 != 0 {
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(lf_checker_rt::callee_addr(ID_E40_CALLEE) as usize);
            f(e44, e44_addr);
            wr32(e44_addr, 0);
            let word = *(lf_checker_rt::relocated(GLOBAL_WORD) as *const u16) as u32;
            wr32(e44.wrapping_add(0x3C), word);
            call_e44_helper(e44, bigstruct, &mut lvl as *mut f32 as u32, r_w, modeobj, aux);
            ret = core::ptr::read_volatile(&lvl);
        }
        // A non-negative limit at +0x48 asks the pool for an object. A null answer goes on to
        // the grid loops below; an object takes the pool path, which skips the loops, sets
        // the return value to zero, runs the helpers on it, and ends with the level copy.
        let limit = rd32(bigstruct.wrapping_add(BIG_LIMIT)) as i32;
        let mut pool_taken = false;
        if limit >= 0 {
            let pool_global = *lf_checker_rt::global::<u32>(POOL_GLOBAL);
            let pool_get: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(lf_checker_rt::callee_addr(ID_POOL_GET) as usize);
            let found = pool_get(pool_global, limit as u32);
            if found != 0 {
                pool_taken = true;
                // ret := 0 (the return slot), then the helper on the slot.
                ret = 0.0;
                if rd8(modeobj.wrapping_add(MODE_BYTE)) != MODE_CELL_SKIP {
                    call_slot_helper(&mut ret as *mut f32 as u32);
                    let pool_second: extern "thiscall" fn(u32, u32, u32) -> u32 =
                        core::mem::transmute(lf_checker_rt::callee_addr(ID_POOL_ADD) as usize);
                    pool_second(found, &mut ret as *mut f32 as u32, 4);
                    let pool_cell_a = CellArgs {
                        big: bigstruct,
                        a_lo,
                        b_lo,
                        a_hi,
                        b_hi,
                        level_local: &mut lvl as *mut f32 as u32,
                        r_w,
                        modeobj,
                        aux,
                    };
                    call_pool_cell_a(&mut ret as *mut f32 as u32, pool_cell_a);
                }
                call_slot_helper(&mut ret as *mut f32 as u32);
                let pool_second: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(lf_checker_rt::callee_addr(ID_POOL_ADD) as usize);
                pool_second(found, &mut ret as *mut f32 as u32, 8);
                let pool_cell_b = CellArgs {
                    big: bigstruct,
                    a_lo,
                    b_lo,
                    a_hi,
                    b_hi,
                    level_local: &mut lvl as *mut f32 as u32,
                    r_w,
                    modeobj,
                    aux,
                };
                call_pool_cell_b(&mut ret as *mut f32 as u32, pool_cell_b);
                call_slot_helper(&mut ret as *mut f32 as u32);
                ret = core::ptr::read_volatile(&lvl);
            }
        }

        if !pool_taken && lo_y <= hi_y {
            let args = CellArgs {
                big: bigstruct,
                a_lo,
                b_lo,
                a_hi,
                b_hi,
                level_local: 0,
                r_w,
                modeobj,
                aux,
            };
            let mut j = lo_y;
            loop {
                // The inner loop starts from the reloaded value of ret.
                if lo_x <= hi_x {
                    let mut x0 = ret;
                    let mut i = lo_x;
                    loop {
                        if x0 > 0.0 {
                            let cell = (((j as u32) & 0xF) << 4) + ((i as u32) & 0xF);
                            let rec = lf_checker_rt::relocated(CELL_TABLE)
                                .wrapping_add(cell * CELL_RECORD_SIZE);
                            let mut cargs = args;
                            cargs.level_local = &mut lvl as *mut f32 as u32;
                            if rd8(modeobj.wrapping_add(MODE_BYTE)) != MODE_CELL_SKIP {
                                call_cell_a(rec, cargs);
                            }
                            call_cell_b(rec.wrapping_add(4), cargs);
                            x0 = core::ptr::read_volatile(&lvl);
                        }
                        i += 1;
                        if i > hi_x {
                            break;
                        }
                    }
                    ret = x0;
                }
                j += 1;
                if j > hi_y {
                    break;
                }
            }
        }

        // Final section.
        wr8(bigstruct.wrapping_add(BIG_F15), rd8(bigstruct.wrapping_add(BIG_F15)) | 0x80);
        if ret > image_f32(THRESHOLD_CONST) {
            let flags = rd8(bigstruct.wrapping_add(BIG_EF8));
            if flags & 2 != 0 {
                wr8(bigstruct.wrapping_add(BIG_EF8), flags & 0xFD);
                if rd8(aux.wrapping_add(AUX_FLAG)) == 0 {
                    let side: extern "cdecl" fn(u32) -> u32 =
                        core::mem::transmute(lf_checker_rt::callee_addr(ID_LOOKUP) as usize);
                    let result = side(rd32(bigstruct.wrapping_add(BIG_F50)));
                    if result != 0 {
                        wr8(aux.wrapping_add(AUX_FLAG), 1);
                        wr32(
                            aux.wrapping_add(AUX_RESULT),
                            result.wrapping_add(rd32(lf_checker_rt::relocated(GLOBAL_ADD))),
                        );
                    }
                }
                return finish(bigstruct, level, lvl, ret);
            }
        }
        // Not above zero (an unordered value is not taken): the second side call runs.
        if ret <= 0.0 {
            wr8(bigstruct.wrapping_add(BIG_EF8), rd8(bigstruct.wrapping_add(BIG_EF8)) | 2);
            let side: extern "cdecl" fn(u32) -> u32 =
                core::mem::transmute(lf_checker_rt::callee_addr(ID_SIDE_CALL) as usize);
            side(bigstruct);
        }
        finish(bigstruct, level, lvl, ret)
    }
});

// Final value: the frame's level copy, unless the kind byte of the struct is 0, 1 or 4, when
// the result is the return value plus the argument level, scaled by one half.
#[inline(always)]
unsafe fn finish(bigstruct: u32, level: f32, lvl: f32, ret: f32) -> f32 {
    let kind = rd8(bigstruct.wrapping_add(BIG_E70));
    if kind == 0 || kind == 1 || kind == 4 {
        lvl
    } else {
        fmul(fadd(ret, level), image_f32(HALF_CONST))
    }
}
