// original: 0x00e489b0 MO_START
//! Start-menu build: unless a readiness hook vetoes, allocates and wires four
//! menu-item objects (start, options, guide, exit) plus a social-club image
//! item, links them into the menu object, installs a callback record and
//! shows the menu, returning the final hook answer.
//!
//! Behaviour: call this+0x140; when its low byte is nonzero return its full
//! answer and do nothing else. Otherwise run the one-time init hook, then for
//! each of the four items in order: allocate its buffer, run two no-arg
//! queries on this, format its label, construct it (storing it at this+0x1e0
//! and up in steps of 4), convert a colour word and publish the row through
//! slot 0x1cc, run two float-keyed attach sequences (block 1 calls the item's
//! own slot 0x114 twice; blocks 2-4 call the previous item's slot 0x4c then
//! the item's own slot 0x104), run the scale/geometry calls (slots 0x80,
//! 0x1fc, 0x118, 0x1e0 with the item's tag string, 0x28, 0x200), then two
//! rounds of this+0x4c feeding the item's slots 0x170/0x17c, and the finish
//! calls (slots 0x28, 0x120, 0x1d4; block 4 skips the 0x28). Block 1 also publishes the first item at
//! this+0x1f0 with a zero word beside it; block 2 makes one extra convert
//! call (slot 0x208); block 4 picks the tag MO_BAK when any of three feature
//! checks is nonzero else MO_QUI and publishes it through slot 0x1e0. Then
//! allocate and construct the image item at this+0x1f8, attach it twice
//! through slot 0x114, scale it (slot 0xa0), tag it 2, finish it (slot
//! 0x120), link it, gate its slot 0x17c on this+0x4c, finish it (slot 0x28),
//! run the finish hook with the global picked by the argument's low byte,
//! install the callback record at this+0x204, register it, run this slots
//! 0x28/0x24/0x18/0x13c each with argument 1, and return the last answer.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

// Direct-callee ids (see contract).
const E_NEW1: u32 = 1; // new(0x610): item 1 buffer
const E_NEW2: u32 = 2; // new(0x610): item 2 buffer
const E_NEW3: u32 = 3; // new(0x610): item 3 buffer
const E_NEW4: u32 = 4; // new(0x610): item 4 buffer
const E_NEW5: u32 = 5; // new(0x25c): image-item buffer
const E_INIT: u32 = 6; // one-time init hook
const E_FMT: u32 = 7; // label formatter: (arg, format) -> string
const E_CTOR1: u32 = 8; // item 1 constructor (buf, string, query) -> object
const E_CTOR2: u32 = 9; // item 2 constructor
const E_CTOR3: u32 = 10; // item 3 constructor
const E_CTOR4: u32 = 11; // item 4 constructor
const E_CONV: u32 = 12; // colour convert: (out-word, index) -> out pointer
const E_MKSTR: u32 = 13; // keyed 24-byte record builder -> record pointer
const E_FREE: u32 = 14; // record release
const E_REG: u32 = 15; // feature check, low byte decides
const E_CHK1: u32 = 16; // manager check 1
const E_CHK2: u32 = 17; // manager check 2
const E_CTOR5: u32 = 18; // image-item constructor
const E_LINK: u32 = 19; // image-item link (4 args)
const E_FIN: u32 = 20; // finish hook with picked global
const E_REG2: u32 = 21; // callback registrar

// Object field offsets (this = ecx).
const F_ITEM1: u32 = 0x1e0;
const F_ITEM2: u32 = 0x1e4;
const F_ITEM3: u32 = 0x1e8;
const F_ITEM4: u32 = 0x1ec;
const F_FIRST: u32 = 0x1f0;
const F_ZERO: u32 = 0x1f4;
const F_ITEM5: u32 = 0x1f8;
const F_CB: u32 = 0x204;
const ITEM_TAG: u32 = 0x1d8;

// Label formats and tag strings (file VAs).
const S_FMT1: u32 = 0x00F16BD4;
const S_NM1: u32 = 0x00F16BE8;
const S_FMT2: u32 = 0x00F16BF4;
const S_NM2: u32 = 0x00F16C08;
const S_FMT3: u32 = 0x00F16C64;
const S_NM3: u32 = 0x00F16CB8;
const S_FMT4: u32 = 0x00F16CC8;
const S_BAK: u32 = 0x00F16CE4;
const S_QUI: u32 = 0x00F16CEC;
const S_PAR: u32 = 0x00F16CF4;
const S_IMG: u32 = 0x00F16D00;

// Globals and callback (file VAs).
const G_ALT: u32 = 0x019D2F3C;
const G_DEF: u32 = 0x019D2F40;
const G_MGR: u32 = 0x01BB5624;
const CB_FN: u32 = 0x00E48880;

// Float keys as bits.
const K_ROW: u32 = 0x42100000;
const K_S1A: u32 = 0x41E00000;
const K_S1B: u32 = 0x41200000;
const K_S20: u32 = 0x41A00000;
const K_G0: u32 = 0x43480000;
const K_G1: u32 = 0x41F00000;
const K_IMG: u32 = 0x43160000;

const CONV_IDX: u32 = 0x41;
const ITEM_SIZE: u32 = 0x610;
const IMG_SIZE: u32 = 0x25C;
const FIN_KEY: u32 = 0x79;

#[inline(always)]
unsafe fn rd(p: u32) -> u32 {
    unsafe { (p as *const u32).read() }
}

#[inline(always)]
unsafe fn vt(obj: u32) -> u32 {
    unsafe { rd(obj) }
}

#[inline(always)]
unsafe fn vt0(obj: u32, slot: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd(vt(obj).wrapping_add(slot)) as usize);
        f(obj)
    }
}

#[inline(always)]
unsafe fn vt1(obj: u32, slot: u32, a0: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd(vt(obj).wrapping_add(slot)) as usize);
        f(obj, a0)
    }
}

#[inline(always)]
unsafe fn vt2(obj: u32, slot: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd(vt(obj).wrapping_add(slot)) as usize);
        f(obj, a0, a1)
    }
}

#[inline(always)]
unsafe fn vt4(obj: u32, slot: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd(vt(obj).wrapping_add(slot)) as usize);
        f(obj, a0, a1, a2, a3)
    }
}

#[inline(always)]
unsafe fn vt7(
    obj: u32, slot: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32,
) -> u32 {
    unsafe {
        let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd(vt(obj).wrapping_add(slot)) as usize);
        f(obj, a0, a1, a2, a3, a4, a5, a6)
    }
}

/// The six words of a 24-byte record the builder returns.
#[inline(always)]
unsafe fn words6(p: u32) -> [u32; 6] {
    unsafe {
        let q = p as *const u32;
        [
            q.read(),
            q.add(1).read(),
            q.add(2).read(),
            q.add(3).read(),
            q.add(4).read(),
            q.add(5).read(),
        ]
    }
}

/// One item's head: allocate, query twice, format, construct, store, convert
/// the colour word and publish the row. Returns the item object.
unsafe fn item_head(obj: u32, field: u32, new_id: u32, ctor_id: u32, fmt: u32) -> u32 {
    unsafe {
        let buf: u32 = callee_cdecl!(new_id, u32, ITEM_SIZE);
        let q1 = vt0(obj, 0x48);
        let q2 = vt0(obj, 0x48);
        let s: u32 = callee_cdecl!(E_FMT, u32, relocated(fmt), q2);
        let item: u32 = callee_thiscall!(ctor_id, u32, buf, s, q1);
        ((obj + field) as *mut u32).write(item);
        let mut slot = 0u32;
        let p: u32 = callee_cdecl!(E_CONV, u32, &mut slot as *mut u32 as u32, CONV_IDX);
        vt4(item, 0x1cc, K_ROW, p, 0, 2);
        item
    }
}

/// One float-keyed attach pair feeding the item's own slot 0x114 (block 1
/// and the image item): build the record, pass its words, release it.
unsafe fn attach_direct(item: u32, k0: u32, k1: u32, n: u32) {
    unsafe {
        let mut tmp = [0u32; 6];
        let r: u32 = callee_thiscall!(E_MKSTR, u32, tmp.as_mut_ptr() as u32, k0, k1);
        let w = words6(r);
        vt7(item, 0x114, n, w[0], w[1], w[2], w[3], w[4], w[5]);
        callee_cdecl!(E_FREE, u32,);
    }
}

/// One float-keyed attach pair feeding the previous item's slot 0x4c then
/// the item's own slot 0x104 (blocks 2-4).
unsafe fn attach_chain(prev: u32, item: u32, k0: u32, k1: u32, n0: u32, n1: u32) {
    unsafe {
        let mut tmp = [0u32; 6];
        let r: u32 = callee_thiscall!(E_MKSTR, u32, tmp.as_mut_ptr() as u32, k0, k1);
        let w = words6(r);
        let a = vt7(prev, 0x4c, n0, w[0], w[1], w[2], w[3], w[4], w[5]);
        vt2(item, 0x104, n1, a);
        callee_cdecl!(E_FREE, u32,);
    }
}

/// The shared scale/geometry/finish tail of blocks 1-4: block 4 publishes
/// its tag later (the feature-gated selection), so its name is `None`.
/// Block 2's extra convert call is made by the caller between the geometry
/// and gate rounds.
unsafe fn item_tail(obj: u32, item: u32, name: Option<u32>) {
    unsafe {
        let _ = obj;
        vt2(item, 0x80, K_G0, K_G1);
        vt1(item, 0x1fc, 0);
        vt1(item, 0x118, 1);
        if let Some(n) = name {
            vt2(item, 0x1e0, relocated(n), 0);
        }
        vt1(item, 0x28, 1);
        vt1(item, 0x200, 1);
    }
}

/// The two gate rounds plus the finish calls shared by blocks 1-4. Block 4
/// skips the second slot-0x28 call.
unsafe fn item_gate(obj: u32, item: u32, with_28: bool) {
    unsafe {
        let t = vt0(obj, 0x4c);
        vt1(item, 0x170, t);
        let t = vt0(obj, 0x4c);
        vt1(item, 0x17c, t);
        if with_28 {
            vt1(item, 0x28, 1);
        }
        vt1(item, 0x120, 1);
        vt1(item, 0x1d4, FIN_KEY);
    }
}

unsafe fn run(obj: u32, arg: u32, is_mut: bool) -> u32 {
    unsafe {
        let v = vt0(obj, 0x140);
        if (v & 0xFF) != 0 {
            return v;
        }
        callee_cdecl!(E_INIT, u32,);
        // Block 1.
        let o1 = item_head(obj, F_ITEM1, E_NEW1, E_CTOR1, S_FMT1);
        attach_direct(o1, K_S1A, 0, 2);
        attach_direct(o1, 0, K_S1B, 4);
        item_tail(obj, o1, Some(S_NM1));
        item_gate(obj, o1, true);
        ((obj + F_FIRST) as *mut u32).write(o1);
        // Mutant: skip the zero-word publish beside the first item.
        if !is_mut {
            ((obj + F_ZERO) as *mut u32).write(0);
        }
        // Block 2.
        let o2 = item_head(obj, F_ITEM2, E_NEW2, E_CTOR2, S_FMT2);
        attach_chain(o1, o2, K_S20, 0, 8, 2);
        attach_chain(o1, o2, 0, 0, 4, 4);
        item_tail(obj, o2, Some(S_NM2));
        let mut slot2 = 0u32;
        let p2: u32 = callee_cdecl!(E_CONV, u32, &mut slot2 as *mut u32 as u32, CONV_IDX);
        vt1(o2, 0x208, p2);
        item_gate(obj, o2, true);
        // Block 3.
        let o3 = item_head(obj, F_ITEM3, E_NEW3, E_CTOR3, S_FMT3);
        attach_chain(o2, o3, K_S20, 0, 8, 2);
        attach_chain(o2, o3, 0, 0, 4, 4);
        item_tail(obj, o3, Some(S_NM3));
        item_gate(obj, o3, true);
        // Block 4.
        let o4 = item_head(obj, F_ITEM4, E_NEW4, E_CTOR4, S_FMT4);
        attach_chain(o3, o4, K_S20, 0, 8, 2);
        attach_chain(o3, o4, 0, 0, 4, 4);
        item_tail(obj, o4, None);
        item_gate(obj, o4, false);
        let r0: u32 = callee_cdecl!(E_REG, u32,);
        let sel = if (r0 & 0xFF) != 0 {
            S_BAK
        } else {
            let g: u32 = global::<u32>(G_MGR).read();
            let r1: u32 = callee_thiscall!(E_CHK1, u32, g, 1);
            if (r1 & 0xFF) != 0 {
                S_BAK
            } else {
                let g: u32 = global::<u32>(G_MGR).read();
                let r2: u32 = callee_thiscall!(E_CHK2, u32, g, 2);
                if (r2 & 0xFF) != 0 { S_BAK } else { S_QUI }
            }
        };
        vt2(o4, 0x1e0, relocated(sel), 0);
        // Block 5: the image item.
        let buf5: u32 = callee_cdecl!(E_NEW5, u32, IMG_SIZE);
        let o5: u32 = callee_thiscall!(E_CTOR5, u32, buf5, relocated(S_IMG), relocated(S_PAR));
        ((obj + F_ITEM5) as *mut u32).write(o5);
        attach_direct(o5, 0, 0, 8);
        attach_direct(o5, 0, K_S20, 0x10);
        vt1(o5, 0xa0, K_IMG);
        ((o5 + ITEM_TAG) as *mut u32).write(2);
        vt1(o5, 0x120, 1);
        let mut outw = 0u32;
        callee_thiscall!(
            E_LINK,
            u32,
            o5,
            0,
            0,
            &mut outw as *mut u32 as u32,
            0xFFFF_FFFFu32
        );
        let t = vt0(obj, 0x4c);
        vt1(o5, 0x17c, t);
        vt1(o5, 0x28, 1);
        let pick: u32 = if (arg & 0xFF) == 0 {
            global::<u32>(G_DEF).read()
        } else {
            global::<u32>(G_ALT).read()
        };
        callee_thiscall!(E_FIN, u32, o5, pick);
        // Callback record: installed unless the slot aliases a frame word
        // (impossible: heap against stack), then registered.
        let slotp = (obj + F_CB) as *mut u32;
        let probe = &mut outw as *mut u32;
        if slotp as u32 != probe as u32 {
            slotp.write(0);
            slotp.add(1).write(relocated(CB_FN));
        }
        callee_cdecl!(E_REG2, u32, slotp as u32);
        vt1(obj, 0x28, 1);
        vt1(obj, 0x24, 1);
        vt1(obj, 0x18, 1);
        vt1(obj, 0x13c, 1)
    }
}

export!(thiscall, rw_489b0(obj: u32, arg: u32) -> u32 {
    unsafe { run(obj, arg, false) }
});

export!(thiscall, mut_489b0(obj: u32, arg: u32) -> u32 {
    unsafe { run(obj, arg, true) }
});
