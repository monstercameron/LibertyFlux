//! Proof scope: complete wrapper under null, separate-member and self-alias fixtures.
//! Both collaborator bodies are scripted recorder stubs; their engine effects are untested.
//! Three call shapes are not instrumented native branch coverage.
//! Production assembly still requires the direct collaborator's per-function resolver mapping.
//! The tested source/DLL binding is preserved; this positive-only projection removes only the mutant module.
//! Historical preparation fields and the missing invocation-time wrapper hash are excluded from evidence.
//! Proof candidate for releasing the optional object stored in an owner's slot.
//!
//! The wrapper dispatches the object's virtual release method, invokes the
//! intercepted cleanup helper with the object and its owning slot, clears the
//! slot, and returns the object identity. Both collaborators are deliberately
//! left behind their contract stubs; this candidate checks the wrapper's call
//! order, arguments, cleanup, return value, and memory effect.


const MEMBER_SLOT_OFFSET: usize = 0x1C;
const VIRTUAL_RELEASE_SLOT_INDEX: usize = 0x114 / 4;
const CLEANUP_CALLEE_ID: u32 = 2;
const VIRTUAL_RELEASE_REASON: u32 = 11;

type VirtualRelease = unsafe extern "thiscall" fn(*mut u8, u32) -> u32;
type Cleanup = unsafe extern "thiscall" fn(*mut u8, *mut *mut u8) -> u32;

unsafe fn destroy_with_reason(owner: *mut u8, reason: u32) -> *mut u8 {
    let member_slot = unsafe { owner.add(MEMBER_SLOT_OFFSET).cast::<*mut u8>() };
    let member = unsafe { member_slot.read() };
    if member.is_null() {
        return core::ptr::null_mut();
    }

    let vtable = unsafe { member.cast::<*const usize>().read() };
    let virtual_address = unsafe { vtable.add(VIRTUAL_RELEASE_SLOT_INDEX).read() };
    let virtual_release: VirtualRelease = unsafe { core::mem::transmute(virtual_address) };
    let _virtual_result = unsafe { virtual_release(member, reason) };

    let cleanup_address = lf_checker_rt::callee_addr(CLEANUP_CALLEE_ID) as usize;
    let cleanup: Cleanup = unsafe { core::mem::transmute(cleanup_address) };
    // The native body reloads the owner's slot after virtual release. If the
    // virtual method cleared it, the direct cleanup helper is skipped.
    let current_member = unsafe { member_slot.read() };
    if !current_member.is_null() {
        let _cleanup_result = unsafe { cleanup(current_member, member_slot) };
    }

    unsafe { member_slot.write(core::ptr::null_mut()) };
    member
}

lf_checker_rt::export!(thiscall, destroy_slot_object(owner: *mut u8) -> *mut u8 {
    unsafe { destroy_with_reason(owner, VIRTUAL_RELEASE_REASON) }
});
