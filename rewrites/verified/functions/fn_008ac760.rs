//! Proof scope: complete linked-effect dispatch body under valid null-link and child fixtures.
//! Direct helper and child virtual method are scripted; native collaborator effects are untested.
//! Physical EAX is compared; semantic return type, runtime class and saved non-return registers are unknown.
//! Both fixture shapes and all configured comparisons passed; arbitrary engine objects are outside scope.
//! Only the mutant export is removed; the tested positive helper is unchanged and projection unbuilt.
//! Historical reconstructed generation evidence is excluded; final immutable generation receipts bind this proof.
//! Checker-only rewrite candidate for the audio effect's linked-effect dispatch.
//! The checker's direct helper and virtual method are scripted recorder stubs.

use lf_checker_rt::{callee_thiscall, export};

const NEXT_EFFECT_OFFSET: usize = 0x08;
const MUTANT_NEXT_EFFECT_OFFSET: usize = 0x0C;
const FOLLOWUP_VIRTUAL_SLOT: usize = 5;
const FOLLOWUP_SLOT_BYTE_OFFSET: usize = FOLLOWUP_VIRTUAL_SLOT * core::mem::size_of::<u32>();

#[inline(always)]
unsafe fn read_effect_link(receiver: u32, offset: usize) -> u32 {
    let address = (receiver as usize).wrapping_add(offset) as *const u32;
    // SAFETY: the contract supplies receiver as a live heap object and pins
    // both candidate link words inside that segment.
    unsafe { core::ptr::read_unaligned(address) }
}

#[inline(always)]
unsafe fn invoke_followup(receiver: u32) -> u32 {
    // SAFETY: the contract supplies a live child object and a fabricated
    // vtable with a recorder stub in slot 5.
    unsafe {
        let vtable = core::ptr::read_unaligned(receiver as *const u32);
        let slot = (vtable as usize).wrapping_add(FOLLOWUP_SLOT_BYTE_OFFSET) as *const u32;
        let target = core::ptr::read_unaligned(slot);
        let method: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        method(receiver)
    }
}

#[inline(always)]
fn dispatch_effect(receiver: u32, link_offset: usize) -> u32 {
    let base_answer = callee_thiscall!(1, u32, receiver);
    // SAFETY: each contract trial provides a mapped receiver object.
    let next = unsafe { read_effect_link(receiver, link_offset) };
    if next == 0 {
        base_answer
    } else {
        // SAFETY: the non-null child pointer is backed by a heap segment,
        // and its slot 5 points to the contract's second recorder stub.
        unsafe { invoke_followup(next) }
    }
}

/// Runs the base effect update, then forwards to the linked effect's slot 5
/// when that link is non-null. Returns the base answer for a null link.
export!(thiscall, rw_fn_008ac760(receiver: u32) -> u32 {
    dispatch_effect(receiver, NEXT_EFFECT_OFFSET)
});

