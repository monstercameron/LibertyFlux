// original: 0x009DECA0 world_pools_entity_query_visit_node

//! Walk two linked entity records and notify entities whose pool query is not the excluded state.
//!
//! This thiscall routine takes a status-byte pointer in ECX and a pointer to
//! a linked-list head word as its one stack argument. Each node stores an
//! entity pointer at +0 and the next node pointer at +4. The entity's signed
//! 16-bit pool index at +0x2e selects a model record in the global pointer
//! table. When the pool flags at +0x40 contain bit 3, it calls pool vtable
//! slot +0x0c; if the returned byte is not 2 and the status byte is nonzero,
//! it calls entity vtable slot +0x40 and stores 0xff at entity +0x63. The
//! implementation follows the original 32-bit object layout.

const MODEL_POINTER_TABLE_FILE_VA: u32 = 0x01295CD8;
const ENTITY_POOL_INDEX_OFFSET: u32 = 0x2E;
const POOL_FLAGS_OFFSET: u32 = 0x40;
const QUERY_SLOT_OFFSET: u32 = 0x0C;
const NOTIFY_SLOT_OFFSET: u32 = 0x40;
const ENTITY_FLAGS_OFFSET: u32 = 0x63;

#[inline(always)]
unsafe fn read_word(address: u32) -> u32 {
    // SAFETY: the caller supplies an aligned mapped image or heap word.
    unsafe { core::ptr::read_volatile(address as *const u32) }
}

#[inline(always)]
unsafe fn read_u16(address: u32) -> u16 {
    // SAFETY: the caller supplies a mapped 16-bit object field.
    unsafe { core::ptr::read_volatile(address as *const u16) }
}

#[inline(always)]
unsafe fn read_byte(address: u32) -> u8 {
    // SAFETY: the caller supplies a mapped object byte.
    unsafe { core::ptr::read_volatile(address as *const u8) }
}

#[inline(always)]
unsafe fn write_byte(address: u32, value: u8) {
    // SAFETY: the caller supplies a mapped writable object byte.
    unsafe { core::ptr::write_volatile(address as *mut u8, value) }
}

#[inline(always)]
unsafe fn query_pool_state(pool: u32) -> u8 {
    let vtable = unsafe { read_word(pool) };
    let target = unsafe { read_word(vtable.wrapping_add(QUERY_SLOT_OFFSET)) };
    let call: extern "thiscall" fn(u32) -> u8 = unsafe {
        core::mem::transmute(target as usize)
    };
    call(pool)
}

#[inline(always)]
unsafe fn notify_entity(entity: u32) {
    let vtable = unsafe { read_word(entity) };
    let target = unsafe { read_word(vtable.wrapping_add(NOTIFY_SLOT_OFFSET)) };
    let call: extern "thiscall" fn(u32) -> u32 = unsafe {
        core::mem::transmute(target as usize)
    };
    let _ = call(entity);
}

lf_checker_rt::export!(thiscall, rw_009deca0(status: u32, head_slot: u32) -> u32 {
    let mut node = unsafe { read_word(head_slot) };
    let status_is_active = unsafe { read_byte(status) } != 0;
    let model_table = lf_checker_rt::relocated(MODEL_POINTER_TABLE_FILE_VA);

    while node != 0 {
        let entity = unsafe { read_word(node) };
        node = unsafe { read_word(node.wrapping_add(4)) };
        let pool_index = unsafe {
            core::ptr::read_volatile(
                entity.wrapping_add(ENTITY_POOL_INDEX_OFFSET) as *const i16
            )
        } as i32 as u32;
        let pool = unsafe { read_word(model_table.wrapping_add(pool_index.wrapping_mul(4))) };
        if unsafe { read_byte(pool.wrapping_add(POOL_FLAGS_OFFSET)) } & 8 != 0 {
            let query_state = unsafe { query_pool_state(pool) };
            if query_state != 2 && status_is_active {
                unsafe { notify_entity(entity); }
                unsafe { write_byte(entity.wrapping_add(ENTITY_FLAGS_OFFSET), 0xFF); }
            }
        }
    }
    0
});
