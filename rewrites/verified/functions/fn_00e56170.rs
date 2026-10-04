// original: 0x00E56170 UIRawClipViewer::vf94
/// Dispatch a viewer request by tag, or resolve it through the float tail (original 0x00E56170).
///
/// Compares the argument against four name-service tags; on a match it runs
/// the inner request, optionally notifies the attached object through slot
/// 0x120, raises the flag and records the block code. When no tag matches
/// and the head cell is clear it resolves a target through the keeper,
/// selects the first ready round object, evaluates the two float chains
/// with min/max reductions and clamped gate cells, and either returns
/// through the shared epilogue or tail-dispatches with argument 1.
use crate::{callee_cdecl, callee_thiscall, export, global, relocated};

fn vcall0(obj: u32, off: u32) -> u32 {
    let vt = unsafe { (obj as *const u32).read() };
    let f: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute((((vt + off)) as *const u32).read() as usize) };
    f(obj)
}

fn vcallf(obj: u32, off: u32) -> f32 {
    let vt = unsafe { (obj as *const u32).read() };
    let f: extern "thiscall" fn(u32) -> f32 =
        unsafe { core::mem::transmute((((vt + off)) as *const u32).read() as usize) };
    f(obj)
}

export!(thiscall, rw_e56170(this: u32, arg: u32) -> u32 {
    const TAGS: [u32; 4] = [0x00F1_A578, 0x00F1_A58C, 0x00F1_A5A0, 0x00F1_A5B4];
    const KINDS: [u32; 4] = [2, 3, 3, 3];
    const CODES: [u32; 4] = [1, 2, 3, 4];
    const INNER: u32 = 0x210;
    const OBJ_SLOT: u32 = 0x214;
    const FLAG_OFF: u32 = 0x21C;
    const CODE_OFF: u32 = 0x344;
    const KEEPER: u32 = 0x0198_1A4C;
    const HEAD_CELL: u32 = 0x018B_6C8C;
    const SCALE_G: u32 = 0x00FE_8830;
    const CLAMP_G: u32 = 0x00FE_88E8;
    const SCALE2_G: u32 = 0x017A_CCF0;
    const SCALE3_G: u32 = 0x017A_CCE8;
    const CELL1: u32 = 0x018B_7A8C;
    const CELL2: u32 = 0x018B_7A80;
    const ROUNDS: [u32; 4] = [0x1E0, 0x1E4, 0x1EC, 0x1E8];
    let mut i = 0usize;
    while i < 4 {
        let want = callee_cdecl!(1, u32, relocated(TAGS[i]));
        if arg == want {
            let a2 = callee_thiscall!(2, u32, this.wrapping_add(INNER), KINDS[i]);
            let obj = unsafe { ((this + OBJ_SLOT) as *const u32).read() };
            let ret = if obj != 0 {
                let vt = unsafe { (obj as *const u32).read() };
                let f: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
                    core::mem::transmute((((vt + 0x120)) as *const u32).read() as usize)
                };
                f(obj, 1)
            } else {
                a2
            };
            unsafe {
                ((this + FLAG_OFF) as *mut u8).write(1);
                ((this + CODE_OFF) as *mut u32).write(CODES[i]);
            }
            return ret;
        }
        i += 1;
    }
    unsafe {
        let gptr = global::<u32>(HEAD_CELL).read();
        if ((gptr + 0x200) as *const u32).read() != 0 {
            return gptr;
        }
        let target = callee_thiscall!(5, u32, relocated(KEEPER), arg);
        let mut sel = 0u32;
        let mut last = 0u32;
        let mut r = 0usize;
        while r < 4 {
            let o = ((this + ROUNDS[r]) as *const u32).read();
            last = vcall0(o, 0x11C);
            if last as u8 != 0 {
                sel = o;
                break;
            }
            r += 1;
        }
        if r >= 4 || sel == 0 {
            return last;
        }
        let scale = global::<f32>(SCALE_G).read();
        let mut lastf = 0u32;
        let mut get = |obj: u32, off: u32, lastf: &mut u32| -> f32 {
            let v = vcallf(obj, off);
            *lastf = v.to_bits();
            v
        };
        let mut s08 = get(target, 0xC8, &mut lastf);
        let mut s2c = get(target, 0xB8, &mut lastf);
        let mut x0 = s2c;
        x0 *= scale;
        let mut x1 = s08;
        x1 -= x0;
        let s0c = x1;
        s2c = get(target, 0xB8, &mut lastf);
        x0 = s2c;
        x0 *= scale;
        s08 = x0;
        s2c = get(target, 0xC8, &mut lastf);
        x0 = s2c;
        x0 += s08;
        let s14 = x0;
        s08 = get(target, 0xD0, &mut lastf);
        s2c = get(target, 0xC0, &mut lastf);
        x0 = s2c;
        x0 *= scale;
        x1 = s08;
        x1 -= x0;
        let s1c = x1;
        s2c = get(target, 0xC0, &mut lastf);
        x0 = s2c;
        x0 *= scale;
        s08 = x0;
        s2c = get(target, 0xD0, &mut lastf);
        x0 = s2c;
        x0 += s08;
        let s24 = x0;
        s08 = get(sel, 0xC8, &mut lastf);
        s2c = get(sel, 0xB8, &mut lastf);
        x0 = s2c;
        x0 *= scale;
        x1 = s08;
        x1 -= x0;
        let s10 = x1;
        s2c = get(sel, 0xB8, &mut lastf);
        x0 = s2c;
        x0 *= scale;
        s08 = x0;
        s2c = get(sel, 0xC8, &mut lastf);
        x0 = s2c;
        x0 += s08;
        let s18 = x0;
        s08 = get(sel, 0xD0, &mut lastf);
        s2c = get(sel, 0xC0, &mut lastf);
        x0 = s2c;
        x0 *= scale;
        x1 = s08;
        x1 -= x0;
        let s20 = x1;
        s2c = get(sel, 0xC0, &mut lastf);
        x0 = s2c;
        x0 *= scale;
        s08 = x0;
        let mut x7 = s0c;
        s2c = get(sel, 0xD0, &mut lastf);
        x1 = s10;
        x0 = s2c;
        x0 += s08;
        if x1 > x7 {
            x7 = x1;
        }
        let mut x6 = s14;
        x1 = s18;
        if x6 > x1 {
            x6 = x1;
        }
        let mut x5 = s1c;
        x1 = s20;
        if x1 > x5 {
            x5 = x1;
        }
        let mut x4 = s24;
        if x4 > x0 {
            x4 = x0;
        }
        let x3 = global::<f32>(CLAMP_G).read();
        x0 = (global::<u32>(CELL1).read() as i32) as f32;
        x1 = 0.0;
        x0 *= global::<f32>(SCALE2_G).read();
        if x1 > x0 {
            x0 = x1;
        } else if x0 > x3 {
            x0 = x3;
        }
        let mut x2 = (global::<u32>(CELL2).read() as i32) as f32;
        x2 *= global::<f32>(SCALE3_G).read();
        if !(x1 > x2) {
            if x2 > x3 {
                x1 = x3;
            } else {
                x1 = x2;
            }
        }
        if !(x1 > x7) {
            return lastf;
        }
        if !(x6 > x1) {
            return lastf;
        }
        if !(x0 > x5) {
            return lastf;
        }
        if !(x4 > x0) {
            return lastf;
        }
        if ((gptr + 0x200) as *const u32).read() != 0 {
            return lastf;
        }
        // E9 tail path: the worker patched the jump to the tail stub
        // variant; the rewrite calls the same callee normally (k2_tail1
        // pattern) and returns, matching ESP on both sides.
        callee_thiscall!(10, u32, gptr, 1)
    }
});
