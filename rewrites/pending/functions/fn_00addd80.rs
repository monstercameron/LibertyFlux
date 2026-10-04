// original: 0x00addd80 run_script2d_phase
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Run the script-2d render phase (original 0x00ADDD80).
///
/// Vtable slot 7 of the script-2d phase: publishes the phase object while it
/// runs, builds the phase's sub-objects through the shared allocator (callee 1)
/// and constructors (callees 2, 6, 7, 9), rescales two of them through the
/// shared slot-8 query, registers each built piece (callee 5), runs the guard
/// blocks selected by the phase flags, issues the slot-32 self call, and
/// clears the published pointer before returning the final rescale mask.
/// The allocator-failed paths would dereference null on two sites; the
/// contract keeps the allocator answers non-null so those paths never run.
export!(thiscall, rw_00addd80(this: u32) -> u32 {
    unsafe {
        let rd = |off: u32| (this.wrapping_add(off) as *const u32).read();
        let rb = |off: u32| (this.wrapping_add(off) as *const u8).read() as u32;
        let gr = |va: u32| (global::<u32>(va) as *const u32).read();
        let gw = |va: u32, v: u32| global::<u32>(va).write(v);
        gw(0x12FB1B8, this);
        let p1 = callee_cdecl!(1, u32, 0x10, 1);
        let ebx = if p1 != 0 { callee_thiscall!(2, u32, p1, 0xD) } else { 0 };
        rescale_slots(ebx, slot8(ebx), slot8(ebx));
        if rb(0xD51) != 0 {
            let esi = this.wrapping_add(0x960);
            callee_thiscall!(3, u32, esi, this.wrapping_add(0xB0));
            let p2 = callee_cdecl!(1, u32, 0x420, 0);
            let r = if p2 != 0 {
                callee_thiscall!(
                    4, u32, p2, rd(0x950), esi,
                    this.wrapping_add(0xD60), this.wrapping_add(0x944),
                    rd(0x94C), rb(0xD50), rb(0xD51)
                )
            } else {
                0
            };
            callee_cdecl!(5, u32, r);
        }
        let w890 = rd(0x890);
        if w890 != 0 {
            if rd(0x898) != 0 {
                let p = callee_cdecl!(1, u32, 0x18, 0);
                let r = if p != 0 { callee_thiscall!(6, u32, p, 0, w890, rd(0x898), 0) } else { 0 };
                callee_cdecl!(5, u32, r);
            } else {
                let p = callee_cdecl!(1, u32, 0x18, 0);
                let r = if p != 0 { callee_thiscall!(6, u32, p, 0, w890, 0, 0) } else { 0 };
                callee_cdecl!(5, u32, r);
            }
        }
        if rb(0x1C) != 0 {
            let p = callee_cdecl!(1, u32, 0x410, 0);
            let r = if p != 0 { callee_thiscall!(7, u32, p, this.wrapping_add(0xB0)) } else { 0 };
            callee_cdecl!(5, u32, r);
        }
        slot32(this);
        if rd(0x890) != 0 {
            let p = callee_cdecl!(1, u32, 0x2C, 0);
            let r = if p != 0 {
                // Seven-word parameter block; the last word is a single set
                // byte over zeroed stack, which the contract's stack fill gives.
                let args = [0x3F800000u32, 0x3F800000, 0xFF000000, 0, 0, 0, 1];
                callee_thiscall!(8, u32, p, 0, args.as_ptr() as u32)
            } else {
                0
            };
            callee_cdecl!(5, u32, r);
        }
        if rb(0x1C) != 0 {
            let p = callee_cdecl!(1, u32, 8, 0);
            let r = if p != 0 {
                (p as *mut u32).write(relocated(0xE7E048));
                let c = ((p.wrapping_add(4)) as *const u32).read() ^ gr(0x10327A0);
                let c = c & 0x3FFF;
                let cell = p.wrapping_add(4) as *mut u32;
                cell.write(cell.read() ^ c);
                gw(0x10327A0, gr(0x10327A0).wrapping_add(1));
                (p as *mut u32).write(relocated(0xE8668C));
                p
            } else {
                0
            };
            callee_cdecl!(5, u32, r);
        }
        let p8 = callee_cdecl!(1, u32, 8, 0);
        let esi = if p8 != 0 { callee_thiscall!(9, u32, p8) } else { 0 };
        let ret = rescale_slots(esi, slot8(esi), slot8(esi));
        gw(0x12FB1B8, 0);
        ret
    }
});

/// Shared slot-8 virtual query: loads the object's table and calls through it.
#[inline(always)]
fn slot8(obj: u32) -> i32 {
    unsafe {
        let vt = (obj as *const u32).read();
        let tgt = (vt.wrapping_add(8) as *const u32).read();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(tgt as usize);
        f(obj) as i32
    }
}

/// Shared slot-32 virtual self call; the result is ignored by the original.
#[inline(always)]
fn slot32(obj: u32) {
    unsafe {
        let vt = (obj as *const u32).read();
        let tgt = (vt.wrapping_add(0x20) as *const u32).read();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(tgt as usize);
        f(obj);
    }
}

/// Fold two slot-8 answers into the object's rescale field and return the mask.
///
/// Matches the original's two signed-modulo-16 idioms, the round-up-to-16 of
/// the sum, and the masked xor into the word at +4.
#[inline(always)]
fn rescale_slots(obj: u32, first: i32, second: i32) -> u32 {
    let r1 = first % 16;
    let s = (16 - r1) % 16;
    let tmp = second.wrapping_add(s);
    let q = (tmp / 16).wrapping_mul(0x4000);
    unsafe {
        let cell = obj.wrapping_add(4) as *mut u32;
        let masked = ((q as u32) ^ cell.read()) & 0x1FFC000;
        cell.write(cell.read() ^ masked);
        masked
    }
}
