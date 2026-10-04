// original: 0x00e488a0 mo_qui_select
//! Menu-option selector keyed off a four-way tag compare.
//!
//! Reads a key word and compares it against three stored tags in order:
//! a match on the first tag loads a value through a loader hook, publishes
//! it to two globals and, when positive, runs a manager check/apply/commit
//! sequence before selecting entry 6; a match on the second tag writes two
//! mode globals and selects entry 0x42; a match on the third tag chains into
//! a follow-up routine. Otherwise a sub-object is fetched and, when the key
//! matches its selector, a name obtained through its function table is
//! compared byte-wise against a fixed tag string: equal selects entry 0x35,
//! different sets a flag byte.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

// Callee ids (see contract).
const C_LOAD: u32 = 1; // 0x4102d0 thiscall/1: loader hook, out-word through arg
const C_POLL: u32 = 2; // 0x8c4950 cdecl/0: poll, result ignored
const C_CHECK: u32 = 3; // 0x934ba0 thiscall/1: manager check, low byte decides
const C_APPLY: u32 = 4; // 0x935ab0 thiscall/2: manager apply
const C_COMMIT: u32 = 5; // 0x9354f0 thiscall/0: manager commit
const C_SELECT: u32 = 6; // 0x8be0e0 cdecl/1: entry selector
const C_TAIL: u32 = 7; // 0x482250 tail-chained follow-up

// Object field offsets (this = ecx).
const F_TAG0: u32 = 0x1e0;
const F_TAG1: u32 = 0x1e4;
const F_TAG2: u32 = 0x1e8;
const F_SUB: u32 = 0x1ec;
const F_KEY: u32 = 0x1f0;

// Globals (file VAs).
const G_LOADER_OBJ: u32 = 0x011D6FE4;
const G_LOADED0: u32 = 0x011D6FD0;
const G_LOADED1: u32 = 0x011D6FDC;
const G_MGR: u32 = 0x01BB5624;
const G_MODE0: u32 = 0x01160C40;
const G_MODE1: u32 = 0x01030BA8;
const G_TAGSTR: u32 = 0x00F16D7C;
const G_DIFF_FLAG: u32 = 0x018B6E8A;

#[inline(always)]
unsafe fn rd(obj: u32, off: u32) -> u32 {
    unsafe { ((obj + off) as *const u32).read() }
}

/// Byte-wise unsigned compare of a NUL-terminated string against the fixed
/// tag: 0 when equal, else -1/+1 by the first differing byte (tag - value).
unsafe fn tag_cmp(val: u32) -> i32 {
    unsafe {
        let tag = relocated(G_TAGSTR);
        let mut i = 0u32;
        loop {
            let a = ((tag + i) as *const u8).read();
            let b = ((val + i) as *const u8).read();
            if a != b {
                return if (a as i32) < (b as i32) { -1 } else { 1 };
            }
            if a == 0 {
                return 0;
            }
            i += 1;
        }
    }
}

export!(thiscall, rw_488a0(obj: u32) -> u32 {
    unsafe {
        let key = rd(obj, F_KEY);
        if key == rd(obj, F_TAG0) {
            let mut slot = 0u32;
            callee_thiscall!(C_LOAD, u32, relocated(G_LOADER_OBJ), &mut slot as *mut u32 as u32);
            let v = slot;
            global::<u32>(G_LOADED0).write(v);
            global::<u32>(G_LOADED1).write(v);
            if (v as i32) > 0 {
                callee_cdecl!(C_POLL, u32,);
                let mgr = global::<u32>(G_MGR).read();
                let ok = callee_thiscall!(C_CHECK, u32, mgr, v);
                if (ok & 0xFF) == 0 {
                    let mgr = global::<u32>(G_MGR).read();
                    callee_thiscall!(C_APPLY, u32, mgr, v, 1);
                    let mgr = global::<u32>(G_MGR).read();
                    callee_thiscall!(C_COMMIT, u32, mgr);
                }
            }
            return callee_cdecl!(C_SELECT, u32, 6);
        }
        if key == rd(obj, F_TAG1) {
            global::<u32>(G_MODE0).write(0x3b);
            global::<u32>(G_MODE1).write(0xFFFF_FFFF);
            return callee_cdecl!(C_SELECT, u32, 0x42);
        }
        if key == rd(obj, F_TAG2) {
            return callee_thiscall!(C_TAIL, u32, obj);
        }
        let sub = rd(obj, F_SUB);
        if key != sub {
            return key;
        }
        let vt = (sub as *const u32).read();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt + 0x20c) as *const u32).read() as usize);
        let name = f(sub);
        let c = tag_cmp(name);
        if c == 0 {
            return callee_cdecl!(C_SELECT, u32, 0x35);
        }
        global::<u8>(G_DIFF_FLAG).write(1);
        c as u32
    }
});
