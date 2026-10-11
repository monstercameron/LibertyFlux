// original: 0x009DC0E0 gtaFragType::defrag

//! Temporarily link a defragmentation context through TLS while the resource callback runs.
//!
//! This cdecl callback takes an unused first word and a resource pointer as
//! its second word. It reads the child pointer at resource + 8, installs a
//! four-word stack context in TLS slot selected by the global index, raises
//! the active-context counter for the callback, then restores the TLS link
//! from the context after the callback has had a chance to update it. The
//! child's pointer is returned. The native object layout remains 32-bit.

const TLS_INDEX_FILE_VA: u32 = 0x017ABA14;
const ACTIVE_CONTEXT_FILE_VA: u32 = 0x018B7A50;
const RESOURCE_CHILD_OFFSET: u32 = 8;
const TLS_NEXT_OFFSET: u32 = 4;
const CONTEXT_RESOURCE_OFFSET: u32 = 0;
const CONTEXT_NEXT_OFFSET: u32 = 4;
const CONTEXT_TABLE_OFFSET: u32 = 8;
const CONTEXT_FLAG_OFFSET: u32 = 12;
const CONTEXT_TABLE_FILE_VA: u32 = 0x00E97320;
const CONTEXT_FLAG_WORD: u32 = 0xAAAAAA01;
const CALLBACK_CALLEE: u32 = 1;

#[inline(always)]
unsafe fn read_word(address: u32) -> u32 {
    // SAFETY: the caller supplies an aligned mapped image or heap word.
    unsafe { core::ptr::read_volatile(address as *const u32) }
}

#[inline(always)]
unsafe fn write_word(address: u32, value: u32) {
    // SAFETY: the caller supplies an aligned writable mapped word.
    unsafe { core::ptr::write_volatile(address as *mut u32, value) }
}

lf_checker_rt::export!(cdecl, rw_009dc0e0(_unused_context: u32, resource: u32) -> u32 {
    let tls_index = unsafe { read_word(lf_checker_rt::relocated(TLS_INDEX_FILE_VA)) } as usize;
    let tls_context = lf_checker_rt::tls_slot(tls_index);
    let previous_next = unsafe { read_word(tls_context.wrapping_add(TLS_NEXT_OFFSET)) };
    let mut context_words = [0xAAAAAAAA; 4];
    let context = context_words.as_mut_ptr() as u32;

    unsafe {
        write_word(tls_context.wrapping_add(TLS_NEXT_OFFSET), context);
        let active_counter = lf_checker_rt::relocated(ACTIVE_CONTEXT_FILE_VA);
        let active = read_word(active_counter);
        write_word(active_counter, active.wrapping_add(1));
        write_word(context.wrapping_add(CONTEXT_RESOURCE_OFFSET), resource);
        write_word(
            context.wrapping_add(CONTEXT_TABLE_OFFSET),
            lf_checker_rt::relocated(CONTEXT_TABLE_FILE_VA),
        );
        write_word(context.wrapping_add(CONTEXT_FLAG_OFFSET), CONTEXT_FLAG_WORD);
        write_word(context.wrapping_add(CONTEXT_NEXT_OFFSET), previous_next);
    }

    let child = unsafe { read_word(resource.wrapping_add(RESOURCE_CHILD_OFFSET)) };
    if child != 0 {
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLBACK_CALLEE, u32, child, context);
    }

    unsafe {
        let next_after_callback = read_word(context.wrapping_add(CONTEXT_NEXT_OFFSET));
        write_word(tls_context.wrapping_add(TLS_NEXT_OFFSET), next_after_callback);
        let active_counter = lf_checker_rt::relocated(ACTIVE_CONTEXT_FILE_VA);
        let active = read_word(active_counter);
        write_word(active_counter, active.wrapping_sub(1));
    }
    child
});
