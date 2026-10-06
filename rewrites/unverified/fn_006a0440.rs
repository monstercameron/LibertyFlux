// original: 0x006a0440 joystick_classifier
// Joystick classifier: narrows two wide device names, matches them plus a
// VID/PID dword against tables, and records a type id (stdcall/2 -> u32).
//
// Behaviour. Takes a device record and an unused second word. It narrows the
// wide strings at record+0x230 and record+0x28 to byte buffers (low byte of
// each wide unit, null included) and runs an ordered chain: a scripted
// one-argument probe first (nonzero answer feeds a second scripted call
// whose answer becomes the type), then case-insensitive name matches, two
// counted case-sensitive prefix matches, VID/PID dword matches in groups,
// and a final case-sensitive inline match, falling through to type 0. The
// winning type is stored into the current table row (base plus index times
// row stride). A flag byte is set when the type is one of three values; a
// gate global selects a scripted device call whose nonzero answer sets a
// second flag and, when the first flag is clear, runs a scripted table call
// with an out-byte that can set the first flag. Two qwords are then copied
// from the record into the row and the index advanced, unless the type is
// zero without the second flag and neither entry flag bit is set. Returns
// whether the resulting index is below four.
//
// The string helpers run natively on the original side and are reimplemented
// here (the checker's anti-cheat revokes code pages under the rewrite): a
// C-locale case-insensitive compare matching the helper exactly, including
// its null-argument answer, and a counted byte compare. The locale flag the
// helper reads is pinned to zero by the contract.
use lf_checker_rt::{callee_cdecl, callee_stdcall, callee_thiscall, export, global, relocated};

/// Device slot index global.
const DEV_INDEX: u32 = 0x18b7dc4;
/// Device table base global; rows are ROW_STRIDE bytes.
const DEV_TABLE: u32 = 0x18b7e44;
/// Row stride in bytes.
const ROW_STRIDE: i32 = 0x98;
/// Gate global selecting the scripted device call.
const GATE: u32 = 0x18b8030;
/// Probe input: image constant handed to the first scripted call.
const ID0_CONST: u32 = 0xfa155c;
/// Inline match constant (case-sensitive byte loop).
const INLINE_CONST: u32 = 0xfa1408;
/// Wide name offsets in the device record.
const S1_OFF: u32 = 0x28;
/// Wide name offsets in the device record.
const S2_OFF: u32 = 0x230;
/// VID/PID dword offset in the device record.
const VID_OFF: u32 = 0x14;
/// Flag dword offset in the device record.
const FLAG_OFF: u32 = 0x24;
/// Types that set the first flag byte.
const FLAG_TYPES: [u32; 3] = [0x0e, 0x15, 0x16];

/// C-locale case-insensitive byte compare, matching the helper the original
/// calls (folds ASCII uppercase only, compares to the null, returns the
/// difference; 0x7FFFFFFF when either side is null).
fn stricmp_c(a: u32, b: u32) -> i32 {
    if a == 0 || b == 0 {
        return 0x7fffffff;
    }
    let mut i = 0u32;
    loop {
        let mut c1 = unsafe { ((a.wrapping_add(i)) as *const u8).read() };
        if (0x41..=0x5a).contains(&c1) {
            c1 += 0x20;
        }
        let mut c2 = unsafe { ((b.wrapping_add(i)) as *const u8).read() };
        if (0x41..=0x5a).contains(&c2) {
            c2 += 0x20;
        }
        if c1 == 0 || c1 != c2 {
            return c1 as i32 - c2 as i32;
        }
        i = i.wrapping_add(1);
    }
}

/// Counted case-sensitive byte compare, matching the helper the original
/// calls (first difference or the null, up to n bytes).
fn strncmp_c(a: u32, b: u32, n: u32) -> i32 {
    let mut i = 0u32;
    while i < n {
        let c1 = unsafe { ((a.wrapping_add(i)) as *const u8).read() };
        let c2 = unsafe { ((b.wrapping_add(i)) as *const u8).read() };
        if c1 != c2 {
            return c1 as i32 - c2 as i32;
        }
        if c1 == 0 {
            return 0;
        }
        i = i.wrapping_add(1);
    }
    0
}

/// Case-sensitive whole-string equality against the inline constant.
fn inline_eq(s2: u32) -> bool {
    let c = relocated(INLINE_CONST);
    let mut i = 0u32;
    loop {
        let a = unsafe { ((c.wrapping_add(i)) as *const u8).read() };
        let b = unsafe { ((s2.wrapping_add(i)) as *const u8).read() };
        if a != b {
            return false;
        }
        if a == 0 {
            return true;
        }
        i = i.wrapping_add(1);
    }
}

/// Narrow a wide string: length in wide units, then the low byte of each
/// unit including the terminator. Returns the narrowed byte count.
fn narrow(src: u32, buf: &mut [u8]) -> u32 {
    let mut len = 0u32;
    while unsafe { ((src.wrapping_add(len.wrapping_mul(2))) as *const u16).read_unaligned() } != 0 {
        len = len.wrapping_add(1);
    }
    let mut i = 0u32;
    while i <= len {
        buf[i as usize] =
            unsafe { ((src.wrapping_add(i.wrapping_mul(2))) as *const u8).read() };
        i = i.wrapping_add(1);
    }
    len.wrapping_add(1)
}

/// Classify the narrowed device names and VID/PID dword.
///
/// Mirrors the original's ordered chain: each string test compares
/// one narrowed buffer against an image constant (case-insensitive,
/// or a case-sensitive prefix for the two counted compares), each
/// VID/PID test matches one dword against a group of literals, and
/// the final inline test is a case-sensitive byte loop. Returns the
/// type id the original stores.
fn classify<const MUT: bool>(s1: u32, s2: u32, vid: u32) -> u32 {
    // MUTANT (MUT=true): the first match stores 8, not 7.
    if stricmp_c(s2, relocated(0xfa156c)) == 0 {
        if MUT { 8 } else { 0x7 }
    } else if stricmp_c(s2, relocated(0xfa1534)) == 0 {
        0x2
    } else if stricmp_c(s1, relocated(0xfa1548)) == 0 {
        0x2
    } else if stricmp_c(s1, relocated(0xfa1500)) == 0 {
        0x9
    } else if stricmp_c(s2, relocated(0xfa1518)) == 0 {
        0x1
    } else if strncmp_c(s2, relocated(0xfa14c4), 0x21) == 0 {
        0x3
    } else if strncmp_c(s1, relocated(0xfa14e8), 0x17) == 0 {
        0x3
    } else if stricmp_c(s2, relocated(0xfa1658)) == 0 {
        0x2
    } else if stricmp_c(s2, relocated(0xfa1678)) == 0 {
        0x2
    } else if stricmp_c(s2, relocated(0xfa1628)) == 0 {
        0x5
    } else if stricmp_c(s2, relocated(0xfa1640)) == 0 {
        0x6
    } else if stricmp_c(s2, relocated(0xfa15e8)) == 0 {
        0x5
    } else if stricmp_c(s2, relocated(0xfa1614)) == 0 {
        0x4
    } else if stricmp_c(s2, relocated(0xfa15a4)) == 0 {
        0x8
    } else if stricmp_c(s2, relocated(0xfa15cc)) == 0 {
        0x8
    } else if vid == 0xc216046d || vid == 0xc218046d || vid == 0xc219046d {
        0xd
    } else if stricmp_c(s2, relocated(0xfa1290)) == 0 {
        0xd
    } else if stricmp_c(s2, relocated(0xfa12a8)) == 0 {
        0xd
    } else if vid == 0xc299046d || vid == 0xc29b046d || vid == 0xca03046d || vid == 0xc295046d || vid == 0xc298046d || vid == 0xc294046d || vid == 0xca04046d || vid == 0xc291046d || vid == 0xc293046d || vid == 0xc20e046d {
        0xe
    } else if vid == 0xff0c06a3 || vid == 0x350906a3 || vid == 0x010906a3 {
        0xf
    } else if vid == 0xff0d06a3 || vid == 0x5f0d06a3 {
        0x10
    } else if vid == 0x01120f30 || vid == 0x01100f30 || vid == 0x01110f30 {
        0x11
    } else if vid == 0x062106a3 || vid == 0xf62006a3 {
        0x12
    } else if vid == 0x040c06a3 || vid == 0x040b06a3 {
        0x13
    } else if vid == 0xf51806a3 || vid == 0xf51a06a3 {
        0x14
    } else if vid == 0x050606a3 {
        0x15
    } else if vid == 0xff0406a3 || vid == 0xff3206a3 {
        0x16
    } else if vid == 0x4006047d {
        0x17
    } else if stricmp_c(s2, relocated(0xfa1250)) == 0 {
        0x5
    } else if stricmp_c(s2, relocated(0xfa1268)) == 0 {
        0x4
    } else if stricmp_c(s2, relocated(0xfa1218)) == 0 {
        0x4
    } else if stricmp_c(s2, relocated(0xfa123c)) == 0 {
        0x2
    } else if stricmp_c(s2, relocated(0xfa0c44)) == 0 {
        0x2
    } else if stricmp_c(s2, relocated(0xfa1200)) == 0 {
        0x2
    } else if stricmp_c(s2, relocated(0xfa1398)) == 0 {
        0xa
    } else if stricmp_c(s2, relocated(0xfa13ac)) == 0 {
        0xb
    } else if vid == 0x028e045e || vid == 0x02a1045e {
        0xc
    } else if stricmp_c(s2, relocated(0xfa1360)) == 0 {
        0xc
    } else if stricmp_c(s2, relocated(0xfa136c)) == 0 {
        0xc
    } else if stricmp_c(s2, relocated(0xfa1308)) == 0 {
        0xc
    } else if stricmp_c(s2, relocated(0xfa132c)) == 0 {
        0xc
    } else if stricmp_c(s2, relocated(0xfa12c8)) == 0 {
        0xc
    } else if stricmp_c(s2, relocated(0xfa12ec)) == 0 {
        0x5
    } else if inline_eq(s2) {
        0x4
    } else {
        0x0
    }
}

fn body<const MUT: bool>(dev: u32, _unused: u32) -> u32 {
    unsafe {
        let flagfull = ((dev.wrapping_add(FLAG_OFF)) as *const u32).read_unaligned();
        let flag2 = ((flagfull >> 8) & 0xff) == 2;
        let mut s2b = [0u8; 128];
        let s2base = dev.wrapping_add(S2_OFF);
        let s2: u32;
        if s2base == 0 {
            s2 = 0;
        } else {
            narrow(s2base, &mut s2b);
            s2 = core::ptr::addr_of!(s2b) as u32;
        }
        let mut s1b = [0u8; 64];
        let s1base = dev.wrapping_add(S1_OFF);
        let s1: u32;
        if s1base == 0 {
            s1 = 0;
        } else {
            narrow(s1base, &mut s1b);
            s1 = core::ptr::addr_of!(s1b) as u32;
        }
        let idx = global::<i32>(DEV_INDEX).read();
        let row = relocated(DEV_TABLE).wrapping_add(idx.wrapping_mul(ROW_STRIDE) as u32);
        let a0 = callee_cdecl!(1, u32, relocated(ID0_CONST));
        let ty: u32;
        if a0 != 0 {
            ty = callee_cdecl!(2, u32, a0);
        } else {
            let vid = ((dev.wrapping_add(VID_OFF)) as *const u32).read_unaligned();
            ty = classify::<MUT>(s1, s2, vid);
        }
        (row as *mut u32).write_unaligned(ty);
        // Tail: first flag, gated device call, table call with out-byte.
        // Row layout from the addressed bytes: type at +0, flags at +4/+5,
        // record qwords at +0x8/+0x10.
        if FLAG_TYPES.contains(&ty) {
            ((row.wrapping_add(0x05)) as *mut u8).write(1);
        } else {
            ((row.wrapping_add(0x05)) as *mut u8).write(0);
        }
        let gate = global::<u32>(GATE).read();
        let f48: u8;
        if gate == 0 {
            ((row.wrapping_add(0x04)) as *mut u8).write(0);
            f48 = 0;
        } else {
            let r2 = callee_thiscall!(3, u32, dev.wrapping_add(VID_OFF));
            if r2 == 0 {
                ((row.wrapping_add(0x04)) as *mut u8).write(0);
                f48 = 0;
            } else {
                ((row.wrapping_add(0x04)) as *mut u8).write(1);
                f48 = 1;
                let f49 = ((row.wrapping_add(0x05)) as *const u8).read();
                if f49 == 0 {
                    let mut out = [0u8; 8];
                    let r3 = callee_stdcall!(
                        4, u32, idx as u32, 1u32, core::ptr::addr_of_mut!(out) as u32
                    );
                    if r3 == 0 && out[1] == 2 {
                        ((row.wrapping_add(0x05)) as *mut u8).write(1);
                    }
                }
            }
        }
        // Record commit gate and index advance.
        let ty2 = (row as *const u32).read_unaligned();
        let mut take = false;
        if ty2 != 0 {
            if flag2 {
                take = true;
            } else if ((dev.wrapping_add(FLAG_OFF)) as *const u8).read() == 0x16 {
                take = true;
            } else if f48 != 0 {
                take = true;
            }
        } else if f48 != 0 {
            take = true;
        }
        let mut idxw = idx;
        if take {
            let q1 = ((dev.wrapping_add(4)) as *const u64).read_unaligned();
            ((row.wrapping_add(0x08)) as *mut u64).write_unaligned(q1);
            let q2 = ((dev.wrapping_add(0x0c)) as *const u64).read_unaligned();
            ((row.wrapping_add(0x10)) as *mut u64).write_unaligned(q2);
            idxw = idx.wrapping_add(1);
            global::<i32>(DEV_INDEX).write(idxw);
        }
        (idxw < 4) as u32
    }
}

export!(stdcall, rw_6a0440(dev: u32, _unused: u32) -> u32 {
    body::<false>(dev, _unused)
});

export!(stdcall, mut_6a0440(dev: u32, _unused: u32) -> u32 {
    body::<true>(dev, _unused)
});
