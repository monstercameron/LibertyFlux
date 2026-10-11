// original: 0x00D4DE50 CTaskSimpleAffectSecondaryBehaviour::vf1
/// Return the secondary behavior selected by this task, or zero when its
/// manager is unavailable. If a child task exists, its virtual query result
/// is forwarded with this task's selector and stored argument to the manager.
/// The object stores its child pointer at `+0x1c`, an 8-bit selector at
/// `+0x14`, and the forwarded word at `+0x18`. The child vtable's slot at
/// `+0x04` supplies the query. All three calls preserve their observed order.
lf_checker_rt::export!(thiscall, rw_00d4de50(this: u32) -> u32 {
    const CHILD: u32 = 0x1c;
    const SELECTOR: u32 = 0x14;
    const ARGUMENT: u32 = 0x18;
    const CHILD_QUERY_SLOT: u32 = 0x04;
    const MANAGER_GLOBAL: u32 = 0x0167_E2A0;
    const MANAGER_LOOKUP: u32 = 2;
    const APPLY_SECONDARY: u32 = 3;

    unsafe {
        let child = ((this + CHILD) as *const u32).read_unaligned();
        let child_result = if child == 0 {
            0
        } else {
            let vtable = (child as *const u32).read_unaligned();
            let query_address = ((vtable + CHILD_QUERY_SLOT) as *const u32).read_unaligned();
            let query: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(query_address as usize);
            query(child)
        };

        let manager_global = lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned();
        let manager = lf_checker_rt::callee_thiscall!(MANAGER_LOOKUP, u32, manager_global);
        if manager == 0 {
            return 0;
        }

        let selector = ((this + SELECTOR) as *const u8).read() as u32;
        let argument = ((this + ARGUMENT) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            APPLY_SECONDARY,
            u32,
            manager,
            selector,
            argument,
            child_result,
        )
    }
});
