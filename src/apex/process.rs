use core::mem::MaybeUninit;

use apex_rs::bindings::*;

use super::VanillaHypervisor;
use crate::bindings::*;

impl ApexProcessP4 for VanillaHypervisor {
    fn create_process<L: Locked>(
        attributes: &ApexProcessAttribute,
    ) -> Result<ProcessId, ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut process_id = MaybeUninit::uninit();
        let mut process_attr = MaybeUninit::new(PROCESS_ATTRIBUTE_TYPE {
            PERIOD: attributes.period,
            TIME_CAPACITY: attributes.time_capacity,
            ENTRY_POINT: attributes.entry_point as SYSTEM_ADDRESS_TYPE,
            STACK_SIZE: attributes.stack_size as STACK_SIZE_TYPE,
            BASE_PRIORITY: attributes.base_priority as PRIORITY_TYPE,
            DEADLINE: attributes.deadline as DEADLINE_TYPE,
            NAME: attributes.name.map(|c| c as cty::c_char),
        });
        unsafe {
            CREATE_PROCESS(
                process_attr.as_mut_ptr(),
                process_id.as_mut_ptr(),
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())?;
            Ok(process_id.assume_init() as ProcessId)
        }
    }

    fn start<L: Locked>(process_id: ProcessId) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            START(process_id as PROCESS_ID_TYPE, return_code.as_mut_ptr());
            ErrorReturnCode::from(return_code.assume_init())
        }
    }
}

impl ApexProcessP1 for VanillaHypervisor {
    fn set_priority<L: Locked>(
        process_id: ProcessId,
        priority: Priority,
    ) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            SET_PRIORITY(
                process_id as PROCESS_ID_TYPE,
                priority as PRIORITY_TYPE,
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())
        }
    }

    fn suspend_self<L: Locked>(time_out: ApexSystemTime) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            SUSPEND_SELF(time_out, return_code.as_mut_ptr());
            ErrorReturnCode::from(return_code.assume_init())
        }
    }

    fn suspend<L: Locked>(process_id: ProcessId) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            SUSPEND(process_id as PROCESS_ID_TYPE, return_code.as_mut_ptr());
            ErrorReturnCode::from(return_code.assume_init())
        }
    }

    fn resume<L: Locked>(process_id: ProcessId) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            RESUME(process_id as PROCESS_ID_TYPE, return_code.as_mut_ptr());
            ErrorReturnCode::from(return_code.assume_init())
        }
    }

    fn stop_self<L: Locked>() {
        unsafe {
            STOP_SELF();
        }
    }

    fn stop<L: Locked>(process_id: ProcessId) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            STOP(process_id as PROCESS_ID_TYPE, return_code.as_mut_ptr());
            ErrorReturnCode::from(return_code.assume_init())
        }
    }

    fn delayed_start<L: Locked>(
        process_id: ProcessId,
        delay_time: ApexSystemTime,
    ) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            DELAYED_START(
                process_id as PROCESS_ID_TYPE,
                delay_time,
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())
        }
    }

    fn lock_preemption<L: Locked>() -> Result<LockLevel, ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut lock_level = MaybeUninit::uninit();
        unsafe {
            LOCK_PREEMPTION(lock_level.as_mut_ptr(), return_code.as_mut_ptr());
            ErrorReturnCode::from(return_code.assume_init())?;
            Ok(lock_level.assume_init() as LockLevel)
        }
    }

    fn unlock_preemption<L: Locked>() -> Result<LockLevel, ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut lock_level = MaybeUninit::uninit();
        unsafe {
            UNLOCK_PREEMPTION(lock_level.as_mut_ptr(), return_code.as_mut_ptr());
            ErrorReturnCode::from(return_code.assume_init())?;
            Ok(lock_level.assume_init() as LockLevel)
        }
    }

    fn get_my_id<L: Locked>() -> Result<ProcessId, ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut process_id = MaybeUninit::uninit();
        unsafe {
            GET_MY_ID(process_id.as_mut_ptr(), return_code.as_mut_ptr());
            ErrorReturnCode::from(return_code.assume_init())?;
            Ok(process_id.assume_init() as ProcessId)
        }
    }

    fn get_process_id<L: Locked>(process_name: ProcessName) -> Result<ProcessId, ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut process_id = MaybeUninit::uninit();
        unsafe {
            GET_PROCESS_ID(
                process_name.map(|c| c as cty::c_char).as_ptr() as *mut _,
                process_id.as_mut_ptr(),
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())?;
            Ok(process_id.assume_init() as ProcessId)
        }
    }

    fn get_process_status<L: Locked>(
        process_id: ProcessId,
    ) -> Result<ApexProcessStatus, ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut status = MaybeUninit::uninit();
        unsafe {
            GET_PROCESS_STATUS(
                process_id as PROCESS_ID_TYPE,
                status.as_mut_ptr(),
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())?;
            let status = status.assume_init();
            Ok(ApexProcessStatus {
                deadline_time: status.DEADLINE_TIME as ApexSystemTime,
                current_priority: status.CURRENT_PRIORITY as Priority,
                process_state: ProcessState::from_repr(status.PROCESS_STATE).unwrap(),
                attributes: ApexProcessAttribute {
                    period: status.ATTRIBUTES.PERIOD as ApexSystemTime,
                    time_capacity: status.ATTRIBUTES.TIME_CAPACITY as ApexSystemTime,
                    entry_point: core::mem::transmute(status.ATTRIBUTES.ENTRY_POINT as *const ()),
                    stack_size: status.ATTRIBUTES.STACK_SIZE as StackSize,
                    base_priority: status.ATTRIBUTES.BASE_PRIORITY as Priority,
                    deadline: Deadline::from_repr(status.ATTRIBUTES.DEADLINE).unwrap(),
                    name: status.ATTRIBUTES.NAME.map(|c| c as APEX_BYTE),
                },
            })
        }
    }

    fn initialize_process_core_affinity<L: Locked>(
        process_id: ProcessId,
        processor_core_id: ProcessorCoreId,
    ) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            INITIALIZE_PROCESS_CORE_AFFINITY(
                process_id as PROCESS_ID_TYPE,
                processor_core_id as PROCESSOR_CORE_ID_TYPE,
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())
        }
    }

    fn get_my_processor_core_id<L: Locked>() -> ProcessorCoreId {
        let mut core_id = MaybeUninit::uninit();
        unsafe {
            GET_MY_PROCESSOR_CORE_ID(core_id.as_mut_ptr(), MaybeUninit::uninit().as_mut_ptr());
            core_id.assume_init() as ProcessorCoreId
        }
    }

    fn get_my_index<L: Locked>() -> Result<ProcessIndex, ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        let mut process_index = MaybeUninit::uninit();
        unsafe {
            GET_MY_INDEX(process_index.as_mut_ptr(), return_code.as_mut_ptr());
            ErrorReturnCode::from(return_code.assume_init())?;
            Ok(process_index.assume_init() as ProcessIndex)
        }
    }
}
