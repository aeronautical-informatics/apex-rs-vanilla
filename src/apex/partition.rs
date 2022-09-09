use core::mem::MaybeUninit;

use apex_rs::bindings::*;

use super::VanillaHypervisor;
use crate::bindings::*;

impl ApexPartitionP4 for VanillaHypervisor {
    fn get_partition_status<L: apex_rs::Locked>() -> ApexPartitionStatus {
        let mut status = MaybeUninit::uninit();
        unsafe {
            GET_PARTITION_STATUS(status.as_mut_ptr(), MaybeUninit::uninit().as_mut_ptr());
            let status = status.assume_init();
            ApexPartitionStatus {
                period: status.PERIOD,
                duration: status.DURATION,
                identifier: status.IDENTIFIER as PartitionId,
                lock_level: status.LOCK_LEVEL as LockLevel,
                operating_mode: OperatingMode::from_repr(status.OPERATING_MODE).unwrap(),
                start_condition: StartCondition::from_repr(status.START_CONDITION).unwrap(),
                num_assigned_cores: status.NUM_ASSIGNED_CORES as NumCores,
            }
        }
    }

    fn set_partition_mode<L: apex_rs::Locked>(
        operating_mode: OperatingMode,
    ) -> Result<(), ErrorReturnCode> {
        let mut return_code = MaybeUninit::uninit();
        unsafe {
            SET_PARTITION_MODE(
                operating_mode as OPERATING_MODE_TYPE,
                return_code.as_mut_ptr(),
            );
            ErrorReturnCode::from(return_code.assume_init())
        }
    }
}
