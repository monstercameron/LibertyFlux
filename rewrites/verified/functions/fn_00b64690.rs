//! Proof scope: complete target body within its declared stock fixture.
//! Synthetic object and planted vtables; all three collaborator effects are recorder stubs.
//! Both early-return and continued call traces are exercised; nontrivial writes are not.
//! Invocation-time runner hash binding is absent; observed pre-build and current hashes agree.
//! All nine declared comparisons passed in 1000 completed original trials.
//! The same-contract semantic mutant failed three times across 5 completed originals.
//! Acceptance binds the preserved tested source and DLL, not a new projection binary.
//! This unbuilt projection removes only the mutant export and comments; helper and positive
//! export code are unchanged. No portable or universal native-equivalence credit is claimed.
use lf_checker_rt::{callee_thiscall, export};

const SUBOBJECT_OFFSET: u32 = 0x18;
const FIRST_SLOT_OFFSET: u32 = 0xAC;
const SECOND_SLOT_OFFSET: u32 = 0x30;

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn call_slot_without_stack_arg(object: u32, slot_offset: u32) {
    unsafe {
        let vtable = read_u32(object);
        let target = read_u32(vtable.wrapping_add(slot_offset));
        let function: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        let _ = function(object);
    }
}

#[inline(always)]
unsafe fn call_slot_with_one_stack_arg(object: u32, slot_offset: u32, argument: u32) {
    unsafe {
        let vtable = read_u32(object);
        let target = read_u32(vtable.wrapping_add(slot_offset));
        let function: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let _ = function(object, argument);
    }
}

unsafe fn implementation(this_ptr: u32, parameter: u32) -> u32 {
    unsafe {
        let object = callee_thiscall!(
            1,
            u32,
            this_ptr,
            this_ptr.wrapping_add(SUBOBJECT_OFFSET),
            parameter,
            1
        );
        if object == 0 {
            return 0;
        }

        call_slot_without_stack_arg(object, FIRST_SLOT_OFFSET);
        call_slot_with_one_stack_arg(object, SECOND_SLOT_OFFSET, 1);
        object
    }
}

export!(thiscall, rw_00b64690(this_ptr: u32, parameter: u32) -> u32 {
    unsafe { implementation(this_ptr, parameter) }
});




unsafe fn implementation_mutant(this_ptr: u32, parameter: u32) -> u32 {
    unsafe {
        let object = callee_thiscall!(
            1,
            u32,
            this_ptr,
            this_ptr.wrapping_add(SUBOBJECT_OFFSET),
            parameter,
            1
        );
        if object == 0 {
            return 0;
        }

        call_slot_without_stack_arg(object, FIRST_SLOT_OFFSET);
        call_slot_with_one_stack_arg(object, SECOND_SLOT_OFFSET, 2);
        object
    }
}

