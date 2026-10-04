// original: 0x006f52d0 factory_attach_node
/// Creates a node through the owner factory and attaches it to the list.
///
/// Releases the slot-13 channel when occupied, asks the factory stored in the
/// owner object for a node, and on success fills the node's header from the
/// template argument, registers it through the shared registrar, and links it
/// after the current tail (or as the head when the list is empty). Returns
/// whether a node was created.
export!(thiscall, rw_006f52d0(this: u32, tpl: u32, tag: u32, ctx: u32) -> u8 {
    unsafe {
        let obj = this as *mut u32;
        let close_a: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(*global::<u32>(0xE731CC));
        let close_b: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(*global::<u32>(0xE731C8));
        if *obj.add(0x34 / 4) != 0 {
            close_a(this.wrapping_add(0x34));
        }
        let owner = *obj.add(2);
        let table = *(owner as *const u32);
        let factory: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(*(table as *const u32).add(2));
        let node = factory(owner, ctx.wrapping_add(0x14), 0, 0);
        if node != 0 {
            *(node as *mut u32) = *(tpl as *const u32);
            *((node + 4) as *mut u16) = *((tpl + 4) as *const u16);
            let body = node.wrapping_add(0x14);
            *((node + 8) as *mut u32) = body;
            *((node + 12) as *mut u32) = ctx;
            *((node + 16) as *mut u32) = 0;
            callee_cdecl!(4, u32, body, tag, ctx);
            let tail = *obj.add(4);
            if tail != 0 {
                *((tail + 0x10) as *mut u32) = node;
            } else {
                *obj.add(3) = node;
            }
            *obj.add(4) = node;
        }
        let created = (node != 0) as u8;
        if *obj.add(0x34 / 4) != 0 {
            close_b(this.wrapping_add(0x34));
        }
        created
    }
});
