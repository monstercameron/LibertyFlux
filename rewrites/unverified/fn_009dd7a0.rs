// original: 0x009DD7A0 frag_model_txd_request

//! Pin the model resource, temporarily link a callback context through TLS, then restore the chain.
//!
//! This cdecl function takes a model-table index and a resource pointer. It
//! pins the current TXD resource, swaps in the model's signed 16-bit TXD id,
//! clears the pending-request byte, and links a four-word callback context
//! through the indexed TLS slot while the child drawable is attached. It
//! releases the TLS batch, restores the callback-updated TLS link, balances
//! the active-context counter, conditionally requests streaming when the
//! pending byte is set, pops the TXD resource, and returns one in AL. Model
//! ids are sign-extended from the word at model + 0x48. All object words use
//! the original 32-bit layout.

const MODEL_POINTER_TABLE_FILE_VA: u32 = 0x01295CD8;
const TLS_INDEX_FILE_VA: u32 = 0x017ABA14;
const PENDING_REQUEST_FILE_VA: u32 = 0x017F58EC;
const ACTIVE_CONTEXT_FILE_VA: u32 = 0x018B7A50;
const TXD_GLOBAL_FILE_VA: u32 = 0x012B4138;
const MODEL_TXD_ID_OFFSET: u32 = 0x48;
const RESOURCE_CHILD_OFFSET: u32 = 8;
const TLS_NEXT_OFFSET: u32 = 4;
const CONTEXT_RESOURCE_OFFSET: u32 = 0;
const CONTEXT_NEXT_OFFSET: u32 = 4;
const CONTEXT_TABLE_OFFSET: u32 = 8;
const CONTEXT_FLAGS_OFFSET: u32 = 12;
const CONTEXT_TABLE_FILE_VA: u32 = 0x00E97314;
const CONTEXT_FLAGS_WORD: u32 = 0xAAAAAA00;
const PIN_RESOURCE_CALLEE: u32 = 1;
const SWAP_RESOURCE_CALLEE: u32 = 2;
const DRAWABLE_ATTACH_CALLEE: u32 = 3;
const TLS_RELEASE_CALLEE: u32 = 4;
const STREAM_REQUEST_CALLEE: u32 = 5;
const POP_RESOURCE_CALLEE: u32 = 6;

#[inline(always)]
unsafe fn read_word(address: u32) -> u32 {
    // SAFETY: the caller supplies an aligned mapped image, TLS or heap word.
    unsafe { core::ptr::read_volatile(address as *const u32) }
}

#[inline(always)]
unsafe fn write_word(address: u32, value: u32) {
    // SAFETY: the caller supplies an aligned writable TLS, global or heap word.
    unsafe { core::ptr::write_volatile(address as *mut u32, value) }
}

#[inline(always)]
unsafe fn read_byte(address: u32) -> u8 {
    // SAFETY: the caller supplies a mapped byte address.
    unsafe { core::ptr::read_volatile(address as *const u8) }
}

#[inline(always)]
unsafe fn write_byte(address: u32, value: u8) {
    // SAFETY: the caller supplies a writable mapped byte address.
    unsafe { core::ptr::write_volatile(address as *mut u8, value) }
}

lf_checker_rt::export!(cdecl, rw_009dd7a0(model_index: u32, resource: u32) -> u8 {
    let model_table = lf_checker_rt::relocated(MODEL_POINTER_TABLE_FILE_VA);
    let model = unsafe { read_word(model_table.wrapping_add(model_index.wrapping_mul(4))) };
    let txd_id = unsafe {
        core::ptr::read_volatile(model.wrapping_add(MODEL_TXD_ID_OFFSET) as *const i16)
    } as i32 as u32;

    let _: u32 = lf_checker_rt::callee_cdecl!(PIN_RESOURCE_CALLEE, u32);
    let _: u32 = lf_checker_rt::callee_cdecl!(SWAP_RESOURCE_CALLEE, u32, txd_id);

    let tls_index = unsafe { read_word(lf_checker_rt::relocated(TLS_INDEX_FILE_VA)) } as usize;
    let tls_context = lf_checker_rt::tls_slot(tls_index);
    let previous_next = unsafe { read_word(tls_context.wrapping_add(TLS_NEXT_OFFSET)) };
    let mut context_words = [0xAAAAAAAAu32; 4];
    let context = context_words.as_mut_ptr() as u32;
    let active_counter = lf_checker_rt::relocated(ACTIVE_CONTEXT_FILE_VA);
    let pending_request = lf_checker_rt::relocated(PENDING_REQUEST_FILE_VA);

    unsafe {
        write_byte(pending_request, 0);
        write_word(tls_context.wrapping_add(TLS_NEXT_OFFSET), context);
        write_word(active_counter, read_word(active_counter).wrapping_add(1));
        write_word(context.wrapping_add(CONTEXT_RESOURCE_OFFSET), resource);
        write_word(context.wrapping_add(CONTEXT_NEXT_OFFSET), previous_next);
        write_word(
            context.wrapping_add(CONTEXT_TABLE_OFFSET),
            lf_checker_rt::relocated(CONTEXT_TABLE_FILE_VA),
        );
        write_word(context.wrapping_add(CONTEXT_FLAGS_OFFSET), CONTEXT_FLAGS_WORD);
    }

    let child = unsafe { read_word(resource.wrapping_add(RESOURCE_CHILD_OFFSET)) };
    if child != 0 {
        let _: u32 = lf_checker_rt::callee_thiscall!(
            DRAWABLE_ATTACH_CALLEE, u32, child, context
        );
    }

    let _: u32 = lf_checker_rt::callee_cdecl!(TLS_RELEASE_CALLEE, u32, resource);
    let next_after_callback = unsafe { read_word(context.wrapping_add(CONTEXT_NEXT_OFFSET)) };
    unsafe {
        write_word(tls_context.wrapping_add(TLS_NEXT_OFFSET), next_after_callback);
        write_word(active_counter, read_word(active_counter).wrapping_sub(1));
    }

    if unsafe { read_byte(pending_request) } != 0 {
        let txd_global = unsafe { read_word(lf_checker_rt::relocated(TXD_GLOBAL_FILE_VA)) };
        let _: u32 = lf_checker_rt::callee_cdecl!(
            STREAM_REQUEST_CALLEE, u32, model_index, txd_global
        );
    }

    let _: u32 = lf_checker_rt::callee_cdecl!(POP_RESOURCE_CALLEE, u32);
    1
});
