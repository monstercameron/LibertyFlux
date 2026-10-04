// original: 0x00c65ea0 cutscene_copy_row_and_accumulate_chain
/// Copies one table row into the caller's buffer, accumulating chained rows.
///
/// Resolves the row base through vtable slot 0xa0 (falling back to the
/// word at [this+0x100] when the fetch fails, and through slot 0xe0 of the
/// fetched object otherwise), copies row `index` (224 bytes each, reached
/// through two indirections), then adds each chained row's float triplet
/// at [+0x20,+0x24,+0x28], following [+0x10] links. Always returns 0.
export!(thiscall, rw_c65ea0(this: u32, index: u32, out: u32) -> u32 {
    unsafe {
        let dst = out as *mut u32;
        dst.write(0);
        dst.add(1).write(0);
        dst.add(2).write(0);
    }
    let vtable = unsafe { (this as *const u32).read() };
    let slot = unsafe { (vtable as *const u32).byte_add(0xa0).read() };
    let fetch: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(slot as usize) };
    let base = if fetch(this) == 0 {
        unsafe { (this as *const u32).byte_add(0x100).read() }
    } else {
        let inner = fetch(this);
        let inner_table = unsafe { (inner as *const u32).read() };
        let inner_slot = unsafe { (inner_table as *const u32).byte_add(0xe0).read() };
        let resolve: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(inner_slot as usize) };
        resolve(inner)
    };
    let step = unsafe { (base as *const u32).byte_add(4).read() };
    let rows = unsafe { (step as *const u32).read() };
    let row = rows.wrapping_add(index.wrapping_mul(0xe0));
    let head = unsafe { (row as *const u32).byte_add(0x20).read() };
    let mut acc0 = f32::from_bits(head);
    let mut acc1 =
        unsafe { ((row as *const u8).byte_add(0x24) as *const f32).read() };
    let mut acc2 =
        unsafe { ((row as *const u8).byte_add(0x28) as *const f32).read() };
    let tail = unsafe { (row as *const u32).byte_add(0x2c).read() };
    let mut chain = unsafe { (row as *const u32).byte_add(0x10).read() };
    while chain != 0 {
        unsafe {
            acc0 += ((chain as *const u8).byte_add(0x20) as *const f32).read();
            acc1 += ((chain as *const u8).byte_add(0x24) as *const f32).read();
            acc2 += ((chain as *const u8).byte_add(0x28) as *const f32).read();
            chain = (chain as *const u32).byte_add(0x10).read();
        }
    }
    unsafe {
        (out as *mut u32).write(acc0.to_bits());
        ((out as *mut u8).byte_add(4) as *mut f32).write(acc1);
        ((out as *mut u8).byte_add(8) as *mut f32).write(acc2);
        (out as *mut u32).byte_add(0xc).write(tail);
    }
    0
});
