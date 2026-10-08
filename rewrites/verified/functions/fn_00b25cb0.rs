//! Proof scope: complete target body within its declared stock fixture.
//! Six collaborator stubs and fabricated object/vtable topology; native effects are untested.
//! Table selectors are limited to 0 and 1. Output-pointer identity is skipped at calls 5 and 6.
//! Call 6 snapshots call 5's prior local write; call 6's final dead-local write is unobserved.
//! The bounded 60-trial stage is excluded from full-body credit.
//! Invocation-time runner hash is absent from preflight; historical runner evidence is preserved.
//! All nine declared comparisons passed in 1000 completed original trials.
//! The same-contract semantic mutant failed three times across 4 completed originals.
//! Acceptance binds the preserved tested source and DLL, not a new projection binary.
//! This unbuilt projection removes only the mutant export and comments; helper and positive
//! export code are unchanged. No portable or universal native-equivalence credit is claimed.
use core::mem::{transmute, MaybeUninit};
use core::ptr;
use lf_checker_rt::{callee_thiscall, export, global};

const TABLE_VA: u32 = 0x0129_5CD8;

unsafe fn vcall0(object: u32, slot: usize) -> u32 {
    let vtable = unsafe { *(object as *const u32) };
    let target = unsafe { *((vtable as *const u32).add(slot / 4)) };
    let f: extern "thiscall" fn(u32) -> u32 = unsafe { transmute(target as usize) };
    f(object)
}

unsafe fn vcall1(object: u32, slot: usize, arg0: u32) -> u32 {
    let vtable = unsafe { *(object as *const u32) };
    let target = unsafe { *((vtable as *const u32).add(slot / 4)) };
    let f: extern "thiscall" fn(u32, u32) -> u32 = unsafe { transmute(target as usize) };
    f(object, arg0)
}

unsafe fn vcall2(object: u32, slot: usize, arg0: u32, arg1: u32) -> u32 {
    let vtable = unsafe { *(object as *const u32) };
    let target = unsafe { *((vtable as *const u32).add(slot / 4)) };
    let f: extern "thiscall" fn(u32, u32, u32) -> u32 = unsafe { transmute(target as usize) };
    f(object, arg0, arg1)
}

unsafe fn body(this: u32, gain: f32, invert_registry_test: bool) -> u32 {
    let mut item = callee_thiscall!(1, u32, this);
    if item == 0 {
        return item;
    }

    let selector = unsafe { ptr::read_unaligned((this as *const u8).add(0x2e) as *const i16) } as isize;
    let record = unsafe { *global::<u32>(TABLE_VA).offset(selector) };
    if record != 0 {
        let registered_item = unsafe { *((record.wrapping_add(4)) as *const u32) };
        let matched = registered_item == item;
        if matched != invert_registry_test {
            let handoff = unsafe { vcall0(item, 0x18) };
            let context = unsafe { *((this.wrapping_add(0x38)) as *const u32) };
            item = callee_thiscall!(2, u32, context, handoff);
        }
    }

    let gain_bits = gain.to_bits();
    let _ = unsafe { vcall1(item, 0x3c, gain_bits) };

    let mut out = MaybeUninit::<u32>::uninit();
    let out_ptr = out.as_mut_ptr() as u32;
    let secondary = unsafe { *((item.wrapping_add(0x0c)) as *const u32) };
    let _ = unsafe { vcall2(secondary, 0x50, out_ptr, gain_bits) };
    unsafe { vcall1(item, 0x40, out_ptr) }
}

export!(thiscall, rw_00b25cb0(this: u32, gain: f32) -> u32 {
    unsafe { body(this, gain, false) }
});

