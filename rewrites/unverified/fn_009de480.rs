// original: 0x009DE480 entity_ensure_interiors_loaded

//! Load the current interior's room data, request its TXD when present, and dispatch its two virtual hooks.
//!
//! This thiscall function takes a status-byte pointer in ECX and an interior
//! pointer as its one stack argument. It marks the status active, starts the
//! cycle runner, and repeatedly asks for the room count while ensuring each
//! room with threshold 32. Room indexes and counts use an unsigned compare.
//! It then sign-extends the interior's pool index, looks up its model record,
//! and requests the model's signed 16-bit TXD id unless it is 0xffff. After
//! the two interior-model requests, it dispatches vtable slots +0xa8 and,
//! when the portal reference's word at +8 is 0xffff, +0xac. It clears the
//! status byte before returning. Object words follow the game's 32-bit layout.

const MODEL_POINTER_TABLE_FILE_VA: u32 = 0x01295CD8;
const STREAM_DATA_FILE_VA: u32 = 0x0104B888;
const LOADER_GLOBAL_FILE_VA: u32 = 0x011737D0;
const INTERIOR_POOL_INDEX_OFFSET: u32 = 0x2e;
const PORTAL_REFERENCE_OFFSET: u32 = 0x38;
const POOL_TXD_ID_OFFSET: u32 = 0x52;
const PORTAL_KEY_OFFSET: u32 = 8;
const VTABLE_SLOT_ROOMS_LOADED: u32 = 0xa8;
const VTABLE_SLOT_PORTAL_READY: u32 = 0xac;
const ROOMS_LOADED_CALLEE: u32 = 7;
const PORTAL_READY_CALLEE: u32 = 8;
const LOADER_CALLEE: u32 = 1;
const ROOM_COUNT_CALLEE: u32 = 2;
const ROOM_LOAD_CALLEE: u32 = 3;
const TXD_REQUEST_CALLEE: u32 = 4;
const INTERIOR_MODELS_CALLEE: u32 = 5;
const INTERIOR_MODELS_DEFAULT_CALLEE: u32 = 6;

#[inline(always)]
unsafe fn read_word(address: u32) -> u32 {
    // SAFETY: the caller provides an aligned mapped image or heap word.
    unsafe { core::ptr::read_volatile(address as *const u32) }
}

#[inline(always)]
unsafe fn write_word(address: u32, value: u32) {
    // SAFETY: the caller provides an aligned writable heap word.
    unsafe { core::ptr::write_volatile(address as *mut u32, value) }
}

#[inline(always)]
unsafe fn read_u16(address: u32) -> u16 {
    // SAFETY: the caller provides a mapped 16-bit object field.
    unsafe { core::ptr::read_volatile(address as *const u16) }
}

#[inline(always)]
unsafe fn read_byte(address: u32) -> u8 {
    // SAFETY: the caller provides a mapped object byte.
    unsafe { core::ptr::read_volatile(address as *const u8) }
}

#[inline(always)]
unsafe fn write_byte(address: u32, value: u8) {
    // SAFETY: the caller provides a mapped writable object byte.
    unsafe { core::ptr::write_volatile(address as *mut u8, value) }
}

#[inline(always)]
unsafe fn call_table_slot(object: u32, slot: u32) -> u32 {
    let vtable = unsafe { read_word(object) };
    let target = unsafe { read_word(vtable.wrapping_add(slot)) };
    let call: extern "thiscall" fn(u32) -> u32 = unsafe {
        core::mem::transmute(target as usize)
    };
    call(object)
}

lf_checker_rt::export!(thiscall, rw_009de480(status: u32, interior: u32) -> u32 {
    unsafe { write_byte(status, 1); }

    let loader_object = lf_checker_rt::relocated(LOADER_GLOBAL_FILE_VA);
    let _: u32 = lf_checker_rt::callee_thiscall!(LOADER_CALLEE, u32, loader_object, 0);

    let mut room_index = 0u32;
    let mut room_count = lf_checker_rt::callee_thiscall!(
        ROOM_COUNT_CALLEE, u32, interior
    );
    while room_index < room_count {
        let _: u32 = lf_checker_rt::callee_thiscall!(
            ROOM_LOAD_CALLEE, u32, interior, room_index, 0x20
        );
        room_index = room_index.wrapping_add(1);
        room_count = lf_checker_rt::callee_thiscall!(
            ROOM_COUNT_CALLEE, u32, interior
        );
    }

    let pool_index = unsafe {
        core::ptr::read_volatile(interior.wrapping_add(INTERIOR_POOL_INDEX_OFFSET) as *const i16)
    } as i32 as u32;
    let model_table = lf_checker_rt::relocated(MODEL_POINTER_TABLE_FILE_VA);
    let model = unsafe { read_word(model_table.wrapping_add(pool_index.wrapping_mul(4))) };
    let txd_id_bits = unsafe { read_u16(model.wrapping_add(POOL_TXD_ID_OFFSET)) };

    if txd_id_bits != 0xFFFF {
        let txd_id = txd_id_bits as i16 as i32 as u32;
        let stream_data = unsafe { read_word(lf_checker_rt::relocated(STREAM_DATA_FILE_VA)) };
        let _: u32 = lf_checker_rt::callee_cdecl!(
            TXD_REQUEST_CALLEE, u32, txd_id, stream_data, 0x20
        );
    }

    let _: u32 = lf_checker_rt::callee_cdecl!(INTERIOR_MODELS_CALLEE, u32, 0);
    let _: u32 = lf_checker_rt::callee_cdecl!(INTERIOR_MODELS_DEFAULT_CALLEE, u32, 0x20);

    if unsafe { read_u16(model.wrapping_add(POOL_TXD_ID_OFFSET)) } != 0xFFFF {
        let _: u32 = unsafe { call_table_slot(interior, VTABLE_SLOT_ROOMS_LOADED) };
        let portal = unsafe { read_word(interior.wrapping_add(PORTAL_REFERENCE_OFFSET)) };
        if portal != 0 && unsafe { read_u16(portal.wrapping_add(PORTAL_KEY_OFFSET)) } == 0xFFFF {
            let _: u32 = unsafe { call_table_slot(interior, VTABLE_SLOT_PORTAL_READY) };
        }
    }

    unsafe { write_byte(status, 0); }
    0
});
