lf_checker_rt::export!(thiscall, rw_00664d50(this: u32, first_arg: u32, second_arg: u32) -> u32 {
    unsafe {
        const SERVICE_OBJECT: u32 = 0x60;
        const REQUEST_POINTER: u32 = 0x98;
        const REQUEST_STATE: u32 = 0x90;
        const SERVICE_VTABLE_SLOT: u32 = 0x0c;
        const SERVICE_RESET_CALL: u32 = 2;
        const TASK_FINISH_CALL: u32 = 3;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn invoke_request_method(context: u32, request: u32) -> u32 {
            let vtable = unsafe { read_word(context) };
            let target = unsafe { read_word(vtable.wrapping_add(SERVICE_VTABLE_SLOT)) };
            let callback: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            callback(context, request)
        }

        let request = unsafe { read_word(this.wrapping_add(REQUEST_POINTER)) };
        if request != 0 {
            let service = unsafe { read_word(this.wrapping_add(SERVICE_OBJECT)) };
            let method_context = unsafe { read_word(service) };
            unsafe { invoke_request_method(method_context, request) };
        }
        let request_state = unsafe { read_word(this.wrapping_add(REQUEST_STATE)) };
        if request_state == 1 {
            let service = unsafe { read_word(this.wrapping_add(SERVICE_OBJECT)) };
            let service_this = service.wrapping_add(0x48);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                SERVICE_RESET_CALL,
                u32,
                service_this,
                this.wrapping_add(REQUEST_STATE)
            );
        }
        let result: u32 = lf_checker_rt::callee_thiscall!(
            TASK_FINISH_CALL,
            u32,
            this,
            first_arg,
            second_arg
        );
        unsafe { write_word(this.wrapping_add(SERVICE_OBJECT), 0) };
        result
    }
});

lf_checker_rt::export!(thiscall, lane_base_00664d50(this: u32, first_arg: u32, second_arg: u32) -> u32 {
    unsafe {
        const SERVICE_OBJECT: u32 = 0x60;
        const REQUEST_POINTER: u32 = 0x98;
        const REQUEST_STATE: u32 = 0x90;
        const SERVICE_VTABLE_SLOT: u32 = 0x0c;
        const SERVICE_RESET_CALL: u32 = 2;
        const TASK_FINISH_CALL: u32 = 3;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn invoke_request_method(context: u32, request: u32) -> u32 {
            let vtable = unsafe { read_word(context) };
            let target = unsafe { read_word(vtable.wrapping_add(SERVICE_VTABLE_SLOT)) };
            let callback: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            callback(context, request)
        }

        let request = unsafe { read_word(this.wrapping_add(REQUEST_POINTER)) };
        if request != 0 {
            let service = unsafe { read_word(this.wrapping_add(SERVICE_OBJECT)) };
            let method_context = unsafe { read_word(service) };
            unsafe { invoke_request_method(method_context, request) };
            unsafe { write_word(this.wrapping_add(REQUEST_POINTER), 0) };
        }

        let request_state = unsafe { read_word(this.wrapping_add(REQUEST_STATE)) };
        if request_state == 1 {
            let service = unsafe { read_word(this.wrapping_add(SERVICE_OBJECT)) };
            let service_this = service.wrapping_add(0x48);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                SERVICE_RESET_CALL,
                u32,
                service_this,
                this.wrapping_add(REQUEST_STATE)
            );
        }

        let result: u32 = lf_checker_rt::callee_thiscall!(
            TASK_FINISH_CALL,
            u32,
            this,
            first_arg,
            second_arg
        );
        unsafe { write_word(this.wrapping_add(SERVICE_OBJECT), 0) };
        result
    }
});
