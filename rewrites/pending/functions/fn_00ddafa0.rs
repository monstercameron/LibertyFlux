// original: 0x00ddafa0 item_list_insert_from_spec
/// Build a container item from a caller-supplied spec and link it into the list.
///
/// Same shape as the eight-measurement variant at 0x00DDADE0, but the item
/// is configured from one spec pointer instead of computed scores: a
/// 0x200-byte buffer is allocated, the child's slot-0x48 query and the
/// token helper turn the global counter at 0x17AB558 into constructor
/// arguments, and the item constructor runs on the buffer (a failed
/// allocation still issues the configuration call with the spec and the
/// stored field at +0x1EC, then faults reading the missing item, exactly
/// like the original). Two descriptor blocks (tags 4 and 0x10) are fetched
/// through the +0x114 slot, the slot-0x4C seed is handed to the +0x170
/// slot, the list helper files the item without the priority flag, and the
/// +0x28 slot's answer is returned.
export!(thiscall, rw_00ddafa0(this_ptr: u32, spec: u32) -> u32 {
    unsafe {
        const CHILD_OFF: u32 = 0x1F8;
        const FIELD_OFF: u32 = 0x1EC;
        const QUERY_SLOT: u32 = 0x48;
        const TOKEN_CONST: u32 = 0xEFC408;
        const COUNTER_GLOB: u32 = 0x17AB558;
        const DESC_SLOT: u32 = 0x114;
        const SEED_SLOT: u32 = 0x4C;
        const ATTACH_SLOT: u32 = 0x170;
        const RELEASE_SLOT: u32 = 0x28;

        let child = ((this_ptr.wrapping_add(CHILD_OFF)) as *const u32).read();
        let buf: u32 = callee_cdecl!(11, u32, 0x200);
        let item: u32;
        if buf == 0 {
            item = 0;
        } else {
            let child_vt = (child as *const u32).read();
            let query: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute((((child_vt.wrapping_add(QUERY_SLOT))) as *const u32).read() as usize);
            let qans = query(child);
            let counter = global::<u32>(COUNTER_GLOB).read();
            let tok: u32 = callee_cdecl!(12, u32, relocated(TOKEN_CONST), counter);
            let built: u32 = callee_thiscall!(13, u32, buf, tok, qans);
            global::<u32>(COUNTER_GLOB).write(counter.wrapping_add(1));
            item = built;
        }
        let field = ((this_ptr.wrapping_add(FIELD_OFF)) as *const u32).read();
        let _: u32 = callee_thiscall!(14, u32, item, field, spec);
        let item_vt = (item as *const u32).read();
        let describe: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute((((item_vt.wrapping_add(DESC_SLOT))) as *const u32).read() as usize);
        let mut frame = [0u32; 8];
        let dp = frame.as_mut_ptr() as u32;
        for tag in [4u32, 0x10u32] {
            let ans: u32 = callee_thiscall!(15, u32, dp, 0, 0);
            let w = |i: u32| ((ans.wrapping_add(i.wrapping_mul(4))) as *const u32).read();
            let _: u32 = describe(item, tag, w(0), w(1), w(2), w(3), w(4), w(5));
            let _: u32 = callee_thiscall!(16, u32, dp);
        }
        let this_vt = (this_ptr as *const u32).read();
        let seed_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((((this_vt.wrapping_add(SEED_SLOT))) as *const u32).read() as usize);
        let seed = seed_of(this_ptr);
        let item_vt2 = (item as *const u32).read();
        let attach: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((item_vt2.wrapping_add(ATTACH_SLOT))) as *const u32).read() as usize);
        let _: u32 = attach(item, seed);
        let _: u32 = callee_thiscall!(17, u32, child, item, 0);
        let item_vt3 = (item as *const u32).read();
        let release: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute((((item_vt3.wrapping_add(RELEASE_SLOT))) as *const u32).read() as usize);
        release(item, 1)
    }
});
