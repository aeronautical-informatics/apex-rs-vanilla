#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const SYSTEM_LIMIT_NUMBER_OF_PARTITIONS: u32 = 32;
pub const SYSTEM_LIMIT_NUMBER_OF_MESSAGES: u32 = 512;
pub const SYSTEM_LIMIT_MESSAGE_SIZE: u32 = 8192;
pub const SYSTEM_LIMIT_NUMBER_OF_PROCESSES: u32 = 128;
pub const SYSTEM_LIMIT_NUMBER_OF_SAMPLING_PORTS: u32 = 512;
pub const SYSTEM_LIMIT_NUMBER_OF_QUEUING_PORTS: u32 = 512;
pub const SYSTEM_LIMIT_NUMBER_OF_BUFFERS: u32 = 256;
pub const SYSTEM_LIMIT_NUMBER_OF_BLACKBOARDS: u32 = 256;
pub const SYSTEM_LIMIT_NUMBER_OF_SEMAPHORES: u32 = 256;
pub const SYSTEM_LIMIT_NUMBER_OF_EVENTS: u32 = 256;
pub const SYSTEM_LIMIT_NUMBER_OF_MUTEXES: u32 = 256;
pub const MAX_NAME_LENGTH: u32 = 32;
pub const INFINITE_TIME_VALUE: i32 = -1;
pub const CORE_AFFINITY_NO_PREFERENCE: i32 = -1;
pub const MAX_NUMBER_OF_PROCESSES: u32 = 128;
pub const MIN_PRIORITY_VALUE: u32 = 1;
pub const MAX_PRIORITY_VALUE: u32 = 239;
pub const MAX_LOCK_LEVEL: u32 = 16;
pub const NULL_PROCESS_ID: u32 = 0;
pub const MAIN_PROCESS_ID: i32 = -1;
pub const MAX_NUMBER_OF_PARTITIONS: u32 = 32;
pub const MAX_NUMBER_OF_SAMPLING_PORTS: u32 = 512;
pub const MAX_NUMBER_OF_QUEUING_PORTS: u32 = 512;
pub const MAX_NUMBER_OF_BUFFERS: u32 = 256;
pub const MAX_NUMBER_OF_BLACKBOARDS: u32 = 256;
pub const MAX_NUMBER_OF_SEMAPHORES: u32 = 256;
pub const MAX_SEMAPHORE_VALUE: u32 = 32767;
pub const MAX_NUMBER_OF_EVENTS: u32 = 256;
pub const MAX_NUMBER_OF_MUTEXES: u32 = 256;
pub const NO_MUTEX_OWNED: i32 = -2;
pub const PREEMPTION_LOCK_MUTEX: i32 = -3;
pub const MAX_ERROR_MESSAGE_SIZE: u32 = 128;
pub const SYSTEM_LIMIT_FILE_SIZE: u32 = 1073741824;
pub const MAX_FILE_SIZE: u32 = 1073741824;
pub const MAX_FILE_NAME_LENGTH: u32 = 512;
pub const MAX_DIRECTORY_ENTRY_LENGTH: u32 = 64;
pub const EPERM: u32 = 1;
pub const ENOENT: u32 = 2;
pub const EIO: u32 = 5;
pub const EBADF: u32 = 9;
pub const EACCES: u32 = 13;
pub const EBUSY: u32 = 16;
pub const EEXIST: u32 = 17;
pub const ENOTDIR: u32 = 20;
pub const EISDIR: u32 = 21;
pub const EINVAL: u32 = 22;
pub const EMFILE: u32 = 24;
pub const EFBIG: u32 = 27;
pub const ENOSPC: u32 = 28;
pub const EROFS: u32 = 30;
pub const ENAMETOOLONG: u32 = 78;
pub const EOVERFLOW: u32 = 79;
pub const ENOTEMPTY: u32 = 93;
pub const ESTALE: u32 = 151;
pub const NO_DATA: u32 = 0;
pub const NORMAL_OPERATION: u32 = 3;
pub const FUNCTIONAL_TEST: u32 = 12;
pub const NO_COMPUTED_DATA: u32 = 48;
pub const MAX_NUMBER_OF_SAP_PORTS: u32 = 1024;
pub const SAP_ADDR_TYPE_UNKNOWN: u32 = 0;
pub const SAP_ADDR_TYPE_INET: u32 = 1;
pub const CORE_AFFINITY_ANY: i32 = -2;
pub type APEX_BYTE = cty::c_uchar;
pub type APEX_INTEGER = cty::c_long;
pub type APEX_UNSIGNED = cty::c_ulong;
pub type APEX_LONG_INTEGER = cty::c_longlong;
pub const RETURN_CODE_TYPE_NO_ERROR: RETURN_CODE_TYPE = 0;
pub const RETURN_CODE_TYPE_NO_ACTION: RETURN_CODE_TYPE = 1;
pub const RETURN_CODE_TYPE_NOT_AVAILABLE: RETURN_CODE_TYPE = 2;
pub const RETURN_CODE_TYPE_INVALID_PARAM: RETURN_CODE_TYPE = 3;
pub const RETURN_CODE_TYPE_INVALID_CONFIG: RETURN_CODE_TYPE = 4;
pub const RETURN_CODE_TYPE_INVALID_MODE: RETURN_CODE_TYPE = 5;
pub const RETURN_CODE_TYPE_TIMED_OUT: RETURN_CODE_TYPE = 6;
pub type RETURN_CODE_TYPE = cty::c_uint;
pub type NAME_TYPE = [cty::c_char; 32usize];
pub type SYSTEM_ADDRESS_TYPE = *mut cty::c_void;
pub type MESSAGE_ADDR_TYPE = *mut APEX_BYTE;
pub type MESSAGE_SIZE_TYPE = APEX_INTEGER;
pub type MESSAGE_RANGE_TYPE = APEX_INTEGER;
pub const PORT_DIRECTION_TYPE_SOURCE: PORT_DIRECTION_TYPE = 0;
pub const PORT_DIRECTION_TYPE_DESTINATION: PORT_DIRECTION_TYPE = 1;
pub type PORT_DIRECTION_TYPE = cty::c_uint;
pub const QUEUING_DISCIPLINE_TYPE_FIFO: QUEUING_DISCIPLINE_TYPE = 0;
pub const QUEUING_DISCIPLINE_TYPE_PRIORITY: QUEUING_DISCIPLINE_TYPE = 1;
pub type QUEUING_DISCIPLINE_TYPE = cty::c_uint;
pub type SYSTEM_TIME_TYPE = APEX_LONG_INTEGER;
pub type PROCESSOR_CORE_ID_TYPE = APEX_INTEGER;
extern "C" {
    pub fn TIMED_WAIT(DELAY_TIME: SYSTEM_TIME_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn PERIODIC_WAIT(RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn GET_TIME(SYSTEM_TIME: *mut SYSTEM_TIME_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn REPLENISH(BUDGET_TIME: SYSTEM_TIME_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
pub type PROCESS_NAME_TYPE = NAME_TYPE;
pub type PROCESS_ID_TYPE = cty::c_int;
pub type PROCESS_INDEX_TYPE = APEX_INTEGER;
pub type LOCK_LEVEL_TYPE = APEX_INTEGER;
pub type STACK_SIZE_TYPE = APEX_UNSIGNED;
pub type WAITING_RANGE_TYPE = APEX_INTEGER;
pub type PRIORITY_TYPE = APEX_INTEGER;
pub const PROCESS_STATE_TYPE_DORMANT: PROCESS_STATE_TYPE = 0;
pub const PROCESS_STATE_TYPE_READY: PROCESS_STATE_TYPE = 1;
pub const PROCESS_STATE_TYPE_RUNNING: PROCESS_STATE_TYPE = 2;
pub const PROCESS_STATE_TYPE_WAITING: PROCESS_STATE_TYPE = 3;
pub const PROCESS_STATE_TYPE_FAULTED: PROCESS_STATE_TYPE = 4;
pub type PROCESS_STATE_TYPE = cty::c_uint;
pub const DEADLINE_TYPE_SOFT: DEADLINE_TYPE = 0;
pub const DEADLINE_TYPE_HARD: DEADLINE_TYPE = 1;
pub type DEADLINE_TYPE = cty::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PROCESS_ATTRIBUTE_TYPE {
    pub PERIOD: SYSTEM_TIME_TYPE,
    pub TIME_CAPACITY: SYSTEM_TIME_TYPE,
    pub ENTRY_POINT: SYSTEM_ADDRESS_TYPE,
    pub STACK_SIZE: STACK_SIZE_TYPE,
    pub BASE_PRIORITY: PRIORITY_TYPE,
    pub DEADLINE: DEADLINE_TYPE,
    pub NAME: PROCESS_NAME_TYPE,
}
#[test]
fn bindgen_test_layout_PROCESS_ATTRIBUTE_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<PROCESS_ATTRIBUTE_TYPE>(),
        80usize,
        concat!("Size of: ", stringify!(PROCESS_ATTRIBUTE_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<PROCESS_ATTRIBUTE_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(PROCESS_ATTRIBUTE_TYPE))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<PROCESS_ATTRIBUTE_TYPE>())).PERIOD as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PROCESS_ATTRIBUTE_TYPE),
            "::",
            stringify!(PERIOD)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<PROCESS_ATTRIBUTE_TYPE>())).TIME_CAPACITY as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PROCESS_ATTRIBUTE_TYPE),
            "::",
            stringify!(TIME_CAPACITY)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<PROCESS_ATTRIBUTE_TYPE>())).ENTRY_POINT as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PROCESS_ATTRIBUTE_TYPE),
            "::",
            stringify!(ENTRY_POINT)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<PROCESS_ATTRIBUTE_TYPE>())).STACK_SIZE as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(PROCESS_ATTRIBUTE_TYPE),
            "::",
            stringify!(STACK_SIZE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<PROCESS_ATTRIBUTE_TYPE>())).BASE_PRIORITY as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(PROCESS_ATTRIBUTE_TYPE),
            "::",
            stringify!(BASE_PRIORITY)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<PROCESS_ATTRIBUTE_TYPE>())).DEADLINE as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(PROCESS_ATTRIBUTE_TYPE),
            "::",
            stringify!(DEADLINE)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<PROCESS_ATTRIBUTE_TYPE>())).NAME as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(PROCESS_ATTRIBUTE_TYPE),
            "::",
            stringify!(NAME)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PROCESS_STATUS_TYPE {
    pub DEADLINE_TIME: SYSTEM_TIME_TYPE,
    pub CURRENT_PRIORITY: PRIORITY_TYPE,
    pub PROCESS_STATE: PROCESS_STATE_TYPE,
    pub ATTRIBUTES: PROCESS_ATTRIBUTE_TYPE,
}
#[test]
fn bindgen_test_layout_PROCESS_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<PROCESS_STATUS_TYPE>(),
        104usize,
        concat!("Size of: ", stringify!(PROCESS_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<PROCESS_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(PROCESS_STATUS_TYPE))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<PROCESS_STATUS_TYPE>())).DEADLINE_TIME as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PROCESS_STATUS_TYPE),
            "::",
            stringify!(DEADLINE_TIME)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<PROCESS_STATUS_TYPE>())).CURRENT_PRIORITY as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PROCESS_STATUS_TYPE),
            "::",
            stringify!(CURRENT_PRIORITY)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<PROCESS_STATUS_TYPE>())).PROCESS_STATE as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PROCESS_STATUS_TYPE),
            "::",
            stringify!(PROCESS_STATE)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<PROCESS_STATUS_TYPE>())).ATTRIBUTES as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(PROCESS_STATUS_TYPE),
            "::",
            stringify!(ATTRIBUTES)
        )
    );
}
extern "C" {
    pub fn CREATE_PROCESS(
        ATTRIBUTES: *mut PROCESS_ATTRIBUTE_TYPE,
        PROCESS_ID: *mut PROCESS_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn SET_PRIORITY(
        PROCESS_ID: PROCESS_ID_TYPE,
        PRIORITY: PRIORITY_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn SUSPEND_SELF(TIME_OUT: SYSTEM_TIME_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn SUSPEND(PROCESS_ID: PROCESS_ID_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn RESUME(PROCESS_ID: PROCESS_ID_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn STOP_SELF();
}
extern "C" {
    pub fn STOP(PROCESS_ID: PROCESS_ID_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn START(PROCESS_ID: PROCESS_ID_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn DELAYED_START(
        PROCESS_ID: PROCESS_ID_TYPE,
        DELAY_TIME: SYSTEM_TIME_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn LOCK_PREEMPTION(LOCK_LEVEL: *mut LOCK_LEVEL_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn UNLOCK_PREEMPTION(LOCK_LEVEL: *mut LOCK_LEVEL_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn GET_MY_ID(PROCESS_ID: *mut PROCESS_ID_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn GET_PROCESS_ID(
        PROCESS_NAME: *mut cty::c_char,
        PROCESS_ID: *mut PROCESS_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_PROCESS_STATUS(
        PROCESS_ID: PROCESS_ID_TYPE,
        PROCESS_STATUS: *mut PROCESS_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn INITIALIZE_PROCESS_CORE_AFFINITY(
        PROCESS_ID: PROCESS_ID_TYPE,
        PROCESSOR_CORE_ID: PROCESSOR_CORE_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_MY_PROCESSOR_CORE_ID(
        PROCESSOR_CORE_ID: *mut PROCESSOR_CORE_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_MY_INDEX(PROCESS_INDEX: *mut PROCESS_INDEX_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
pub const OPERATING_MODE_TYPE_IDLE: OPERATING_MODE_TYPE = 0;
pub const OPERATING_MODE_TYPE_COLD_START: OPERATING_MODE_TYPE = 1;
pub const OPERATING_MODE_TYPE_WARM_START: OPERATING_MODE_TYPE = 2;
pub const OPERATING_MODE_TYPE_NORMAL: OPERATING_MODE_TYPE = 3;
pub type OPERATING_MODE_TYPE = cty::c_uint;
pub type PARTITION_ID_TYPE = cty::c_int;
pub type NUM_CORES_TYPE = APEX_UNSIGNED;
pub const START_CONDITION_TYPE_NORMAL_START: START_CONDITION_TYPE = 0;
pub const START_CONDITION_TYPE_PARTITION_RESTART: START_CONDITION_TYPE = 1;
pub const START_CONDITION_TYPE_HM_MODULE_RESTART: START_CONDITION_TYPE = 2;
pub const START_CONDITION_TYPE_HM_PARTITION_RESTART: START_CONDITION_TYPE = 3;
pub type START_CONDITION_TYPE = cty::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PARTITION_STATUS_TYPE {
    pub PERIOD: SYSTEM_TIME_TYPE,
    pub DURATION: SYSTEM_TIME_TYPE,
    pub IDENTIFIER: PARTITION_ID_TYPE,
    pub LOCK_LEVEL: LOCK_LEVEL_TYPE,
    pub OPERATING_MODE: OPERATING_MODE_TYPE,
    pub START_CONDITION: START_CONDITION_TYPE,
    pub NUM_ASSIGNED_CORES: NUM_CORES_TYPE,
}
#[test]
fn bindgen_test_layout_PARTITION_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<PARTITION_STATUS_TYPE>(),
        48usize,
        concat!("Size of: ", stringify!(PARTITION_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<PARTITION_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(PARTITION_STATUS_TYPE))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<PARTITION_STATUS_TYPE>())).PERIOD as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PARTITION_STATUS_TYPE),
            "::",
            stringify!(PERIOD)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<PARTITION_STATUS_TYPE>())).DURATION as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PARTITION_STATUS_TYPE),
            "::",
            stringify!(DURATION)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<PARTITION_STATUS_TYPE>())).IDENTIFIER as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PARTITION_STATUS_TYPE),
            "::",
            stringify!(IDENTIFIER)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<PARTITION_STATUS_TYPE>())).LOCK_LEVEL as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(PARTITION_STATUS_TYPE),
            "::",
            stringify!(LOCK_LEVEL)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<PARTITION_STATUS_TYPE>())).OPERATING_MODE as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(PARTITION_STATUS_TYPE),
            "::",
            stringify!(OPERATING_MODE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<PARTITION_STATUS_TYPE>())).START_CONDITION as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(PARTITION_STATUS_TYPE),
            "::",
            stringify!(START_CONDITION)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<PARTITION_STATUS_TYPE>())).NUM_ASSIGNED_CORES as *const _
                as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(PARTITION_STATUS_TYPE),
            "::",
            stringify!(NUM_ASSIGNED_CORES)
        )
    );
}
extern "C" {
    pub fn GET_PARTITION_STATUS(
        PARTITION_STATUS: *mut PARTITION_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn SET_PARTITION_MODE(
        OPERATING_MODE: OPERATING_MODE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
pub type SAMPLING_PORT_NAME_TYPE = NAME_TYPE;
pub type SAMPLING_PORT_ID_TYPE = cty::c_int;
pub const VALIDITY_TYPE_INVALID: VALIDITY_TYPE = 0;
pub const VALIDITY_TYPE_VALID: VALIDITY_TYPE = 1;
pub type VALIDITY_TYPE = cty::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SAMPLING_PORT_STATUS_TYPE {
    pub REFRESH_PERIOD: SYSTEM_TIME_TYPE,
    pub MAX_MESSAGE_SIZE: MESSAGE_SIZE_TYPE,
    pub PORT_DIRECTION: PORT_DIRECTION_TYPE,
    pub LAST_MSG_VALIDITY: VALIDITY_TYPE,
}
#[test]
fn bindgen_test_layout_SAMPLING_PORT_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<SAMPLING_PORT_STATUS_TYPE>(),
        24usize,
        concat!("Size of: ", stringify!(SAMPLING_PORT_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<SAMPLING_PORT_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(SAMPLING_PORT_STATUS_TYPE))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAMPLING_PORT_STATUS_TYPE>())).REFRESH_PERIOD as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SAMPLING_PORT_STATUS_TYPE),
            "::",
            stringify!(REFRESH_PERIOD)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAMPLING_PORT_STATUS_TYPE>())).MAX_MESSAGE_SIZE as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SAMPLING_PORT_STATUS_TYPE),
            "::",
            stringify!(MAX_MESSAGE_SIZE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAMPLING_PORT_STATUS_TYPE>())).PORT_DIRECTION as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SAMPLING_PORT_STATUS_TYPE),
            "::",
            stringify!(PORT_DIRECTION)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAMPLING_PORT_STATUS_TYPE>())).LAST_MSG_VALIDITY as *const _
                as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(SAMPLING_PORT_STATUS_TYPE),
            "::",
            stringify!(LAST_MSG_VALIDITY)
        )
    );
}
extern "C" {
    pub fn CREATE_SAMPLING_PORT(
        SAMPLING_PORT_NAME: *mut cty::c_char,
        MAX_MESSAGE_SIZE: MESSAGE_SIZE_TYPE,
        PORT_DIRECTION: PORT_DIRECTION_TYPE,
        REFRESH_PERIOD: SYSTEM_TIME_TYPE,
        SAMPLING_PORT_ID: *mut SAMPLING_PORT_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn WRITE_SAMPLING_MESSAGE(
        SAMPLING_PORT_ID: SAMPLING_PORT_ID_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: MESSAGE_SIZE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn READ_SAMPLING_MESSAGE(
        SAMPLING_PORT_ID: SAMPLING_PORT_ID_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: *mut MESSAGE_SIZE_TYPE,
        VALIDITY: *mut VALIDITY_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_SAMPLING_PORT_ID(
        SAMPLING_PORT_NAME: *mut cty::c_char,
        SAMPLING_PORT_ID: *mut SAMPLING_PORT_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_SAMPLING_PORT_STATUS(
        SAMPLING_PORT_ID: SAMPLING_PORT_ID_TYPE,
        SAMPLING_PORT_STATUS: *mut SAMPLING_PORT_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
pub type QUEUING_PORT_NAME_TYPE = NAME_TYPE;
pub type QUEUING_PORT_ID_TYPE = cty::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct QUEUING_PORT_STATUS_TYPE {
    pub NB_MESSAGE: MESSAGE_RANGE_TYPE,
    pub MAX_NB_MESSAGE: MESSAGE_RANGE_TYPE,
    pub MAX_MESSAGE_SIZE: MESSAGE_SIZE_TYPE,
    pub PORT_DIRECTION: PORT_DIRECTION_TYPE,
    pub WAITING_PROCESSES: WAITING_RANGE_TYPE,
}
#[test]
fn bindgen_test_layout_QUEUING_PORT_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<QUEUING_PORT_STATUS_TYPE>(),
        40usize,
        concat!("Size of: ", stringify!(QUEUING_PORT_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<QUEUING_PORT_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(QUEUING_PORT_STATUS_TYPE))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<QUEUING_PORT_STATUS_TYPE>())).NB_MESSAGE as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(QUEUING_PORT_STATUS_TYPE),
            "::",
            stringify!(NB_MESSAGE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<QUEUING_PORT_STATUS_TYPE>())).MAX_NB_MESSAGE as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(QUEUING_PORT_STATUS_TYPE),
            "::",
            stringify!(MAX_NB_MESSAGE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<QUEUING_PORT_STATUS_TYPE>())).MAX_MESSAGE_SIZE as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(QUEUING_PORT_STATUS_TYPE),
            "::",
            stringify!(MAX_MESSAGE_SIZE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<QUEUING_PORT_STATUS_TYPE>())).PORT_DIRECTION as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(QUEUING_PORT_STATUS_TYPE),
            "::",
            stringify!(PORT_DIRECTION)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<QUEUING_PORT_STATUS_TYPE>())).WAITING_PROCESSES as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(QUEUING_PORT_STATUS_TYPE),
            "::",
            stringify!(WAITING_PROCESSES)
        )
    );
}
extern "C" {
    pub fn CREATE_QUEUING_PORT(
        QUEUING_PORT_NAME: *mut cty::c_char,
        MAX_MESSAGE_SIZE: MESSAGE_SIZE_TYPE,
        MAX_NB_MESSAGE: MESSAGE_RANGE_TYPE,
        PORT_DIRECTION: PORT_DIRECTION_TYPE,
        QUEUING_DISCIPLINE: QUEUING_DISCIPLINE_TYPE,
        QUEUING_PORT_ID: *mut QUEUING_PORT_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn SEND_QUEUING_MESSAGE(
        QUEUING_PORT_ID: QUEUING_PORT_ID_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: MESSAGE_SIZE_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn RECEIVE_QUEUING_MESSAGE(
        QUEUING_PORT_ID: QUEUING_PORT_ID_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: *mut MESSAGE_SIZE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_QUEUING_PORT_ID(
        QUEUING_PORT_NAME: *mut cty::c_char,
        QUEUING_PORT_ID: *mut QUEUING_PORT_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_QUEUING_PORT_STATUS(
        QUEUING_PORT_ID: QUEUING_PORT_ID_TYPE,
        QUEUING_PORT_STATUS: *mut QUEUING_PORT_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn CLEAR_QUEUING_PORT(
        QUEUING_PORT_ID: QUEUING_PORT_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
pub type BUFFER_NAME_TYPE = NAME_TYPE;
pub type BUFFER_ID_TYPE = cty::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BUFFER_STATUS_TYPE {
    pub NB_MESSAGE: MESSAGE_RANGE_TYPE,
    pub MAX_NB_MESSAGE: MESSAGE_RANGE_TYPE,
    pub MAX_MESSAGE_SIZE: MESSAGE_SIZE_TYPE,
    pub WAITING_PROCESSES: WAITING_RANGE_TYPE,
}
#[test]
fn bindgen_test_layout_BUFFER_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<BUFFER_STATUS_TYPE>(),
        32usize,
        concat!("Size of: ", stringify!(BUFFER_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<BUFFER_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(BUFFER_STATUS_TYPE))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<BUFFER_STATUS_TYPE>())).NB_MESSAGE as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BUFFER_STATUS_TYPE),
            "::",
            stringify!(NB_MESSAGE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<BUFFER_STATUS_TYPE>())).MAX_NB_MESSAGE as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(BUFFER_STATUS_TYPE),
            "::",
            stringify!(MAX_NB_MESSAGE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<BUFFER_STATUS_TYPE>())).MAX_MESSAGE_SIZE as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(BUFFER_STATUS_TYPE),
            "::",
            stringify!(MAX_MESSAGE_SIZE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<BUFFER_STATUS_TYPE>())).WAITING_PROCESSES as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(BUFFER_STATUS_TYPE),
            "::",
            stringify!(WAITING_PROCESSES)
        )
    );
}
extern "C" {
    pub fn CREATE_BUFFER(
        BUFFER_NAME: *mut cty::c_char,
        MAX_MESSAGE_SIZE: MESSAGE_SIZE_TYPE,
        MAX_NB_MESSAGE: MESSAGE_RANGE_TYPE,
        QUEUING_DISCIPLINE: QUEUING_DISCIPLINE_TYPE,
        BUFFER_ID: *mut BUFFER_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn SEND_BUFFER(
        BUFFER_ID: BUFFER_ID_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: MESSAGE_SIZE_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn RECEIVE_BUFFER(
        BUFFER_ID: BUFFER_ID_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: *mut MESSAGE_SIZE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_BUFFER_ID(
        BUFFER_NAME: *mut cty::c_char,
        BUFFER_ID: *mut BUFFER_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_BUFFER_STATUS(
        BUFFER_ID: BUFFER_ID_TYPE,
        BUFFER_STATUS: *mut BUFFER_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
pub type BLACKBOARD_NAME_TYPE = NAME_TYPE;
pub type BLACKBOARD_ID_TYPE = cty::c_int;
pub const EMPTY_INDICATOR_TYPE_EMPTY: EMPTY_INDICATOR_TYPE = 0;
pub const EMPTY_INDICATOR_TYPE_OCCUPIED: EMPTY_INDICATOR_TYPE = 1;
pub type EMPTY_INDICATOR_TYPE = cty::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BLACKBOARD_STATUS_TYPE {
    pub EMPTY_INDICATOR: EMPTY_INDICATOR_TYPE,
    pub MAX_MESSAGE_SIZE: MESSAGE_SIZE_TYPE,
    pub WAITING_PROCESSES: WAITING_RANGE_TYPE,
}
#[test]
fn bindgen_test_layout_BLACKBOARD_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<BLACKBOARD_STATUS_TYPE>(),
        24usize,
        concat!("Size of: ", stringify!(BLACKBOARD_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<BLACKBOARD_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(BLACKBOARD_STATUS_TYPE))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<BLACKBOARD_STATUS_TYPE>())).EMPTY_INDICATOR as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BLACKBOARD_STATUS_TYPE),
            "::",
            stringify!(EMPTY_INDICATOR)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<BLACKBOARD_STATUS_TYPE>())).MAX_MESSAGE_SIZE as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(BLACKBOARD_STATUS_TYPE),
            "::",
            stringify!(MAX_MESSAGE_SIZE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<BLACKBOARD_STATUS_TYPE>())).WAITING_PROCESSES as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(BLACKBOARD_STATUS_TYPE),
            "::",
            stringify!(WAITING_PROCESSES)
        )
    );
}
extern "C" {
    pub fn CREATE_BLACKBOARD(
        BLACKBOARD_NAME: *mut cty::c_char,
        MAX_MESSAGE_SIZE: MESSAGE_SIZE_TYPE,
        BLACKBOARD_ID: *mut BLACKBOARD_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn DISPLAY_BLACKBOARD(
        BLACKBOARD_ID: BLACKBOARD_ID_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: MESSAGE_SIZE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn READ_BLACKBOARD(
        BLACKBOARD_ID: BLACKBOARD_ID_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: *mut MESSAGE_SIZE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn CLEAR_BLACKBOARD(BLACKBOARD_ID: BLACKBOARD_ID_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn GET_BLACKBOARD_ID(
        BLACKBOARD_NAME: *mut cty::c_char,
        BLACKBOARD_ID: *mut BLACKBOARD_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_BLACKBOARD_STATUS(
        BLACKBOARD_ID: BLACKBOARD_ID_TYPE,
        BLACKBOARD_STATUS: *mut BLACKBOARD_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
pub type SEMAPHORE_NAME_TYPE = NAME_TYPE;
pub type SEMAPHORE_ID_TYPE = cty::c_int;
pub type SEMAPHORE_VALUE_TYPE = APEX_INTEGER;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SEMAPHORE_STATUS_TYPE {
    pub CURRENT_VALUE: SEMAPHORE_VALUE_TYPE,
    pub MAXIMUM_VALUE: SEMAPHORE_VALUE_TYPE,
    pub WAITING_PROCESSES: WAITING_RANGE_TYPE,
}
#[test]
fn bindgen_test_layout_SEMAPHORE_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<SEMAPHORE_STATUS_TYPE>(),
        24usize,
        concat!("Size of: ", stringify!(SEMAPHORE_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<SEMAPHORE_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(SEMAPHORE_STATUS_TYPE))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SEMAPHORE_STATUS_TYPE>())).CURRENT_VALUE as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SEMAPHORE_STATUS_TYPE),
            "::",
            stringify!(CURRENT_VALUE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SEMAPHORE_STATUS_TYPE>())).MAXIMUM_VALUE as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SEMAPHORE_STATUS_TYPE),
            "::",
            stringify!(MAXIMUM_VALUE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SEMAPHORE_STATUS_TYPE>())).WAITING_PROCESSES as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SEMAPHORE_STATUS_TYPE),
            "::",
            stringify!(WAITING_PROCESSES)
        )
    );
}
extern "C" {
    pub fn CREATE_SEMAPHORE(
        SEMAPHORE_NAME: *mut cty::c_char,
        CURRENT_VALUE: SEMAPHORE_VALUE_TYPE,
        MAXIMUM_VALUE: SEMAPHORE_VALUE_TYPE,
        QUEUING_DISCIPLINE: QUEUING_DISCIPLINE_TYPE,
        SEMAPHORE_ID: *mut SEMAPHORE_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn WAIT_SEMAPHORE(
        SEMAPHORE_ID: SEMAPHORE_ID_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn SIGNAL_SEMAPHORE(SEMAPHORE_ID: SEMAPHORE_ID_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn GET_SEMAPHORE_ID(
        SEMAPHORE_NAME: *mut cty::c_char,
        SEMAPHORE_ID: *mut SEMAPHORE_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_SEMAPHORE_STATUS(
        SEMAPHORE_ID: SEMAPHORE_ID_TYPE,
        SEMAPHORE_STATUS: *mut SEMAPHORE_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
pub type EVENT_NAME_TYPE = NAME_TYPE;
pub type EVENT_ID_TYPE = cty::c_int;
pub const EVENT_STATE_TYPE_DOWN: EVENT_STATE_TYPE = 0;
pub const EVENT_STATE_TYPE_UP: EVENT_STATE_TYPE = 1;
pub type EVENT_STATE_TYPE = cty::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EVENT_STATUS_TYPE {
    pub EVENT_STATE: EVENT_STATE_TYPE,
    pub WAITING_PROCESSES: WAITING_RANGE_TYPE,
}
#[test]
fn bindgen_test_layout_EVENT_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<EVENT_STATUS_TYPE>(),
        16usize,
        concat!("Size of: ", stringify!(EVENT_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<EVENT_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(EVENT_STATUS_TYPE))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<EVENT_STATUS_TYPE>())).EVENT_STATE as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(EVENT_STATUS_TYPE),
            "::",
            stringify!(EVENT_STATE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<EVENT_STATUS_TYPE>())).WAITING_PROCESSES as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(EVENT_STATUS_TYPE),
            "::",
            stringify!(WAITING_PROCESSES)
        )
    );
}
extern "C" {
    pub fn CREATE_EVENT(
        EVENT_NAME: *mut cty::c_char,
        EVENT_ID: *mut EVENT_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn SET_EVENT(EVENT_ID: EVENT_ID_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn RESET_EVENT(EVENT_ID: EVENT_ID_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn WAIT_EVENT(
        EVENT_ID: EVENT_ID_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_EVENT_ID(
        EVENT_NAME: *mut cty::c_char,
        EVENT_ID: *mut EVENT_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_EVENT_STATUS(
        EVENT_ID: EVENT_ID_TYPE,
        EVENT_STATUS: *mut EVENT_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
pub type MUTEX_NAME_TYPE = NAME_TYPE;
pub type MUTEX_ID_TYPE = cty::c_int;
pub type LOCK_COUNT_TYPE = APEX_INTEGER;
pub const MUTEX_STATE_TYPE_AVAILABLE: MUTEX_STATE_TYPE = 0;
pub const MUTEX_STATE_TYPE_OWNED: MUTEX_STATE_TYPE = 1;
pub type MUTEX_STATE_TYPE = cty::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MUTEX_STATUS_TYPE {
    pub MUTEX_OWNER: PROCESS_ID_TYPE,
    pub MUTEX_STATE: MUTEX_STATE_TYPE,
    pub MUTEX_PRIORITY: PRIORITY_TYPE,
    pub LOCK_COUNT: LOCK_COUNT_TYPE,
    pub WAITING_PROCESSES: WAITING_RANGE_TYPE,
}
#[test]
fn bindgen_test_layout_MUTEX_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<MUTEX_STATUS_TYPE>(),
        32usize,
        concat!("Size of: ", stringify!(MUTEX_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<MUTEX_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(MUTEX_STATUS_TYPE))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MUTEX_STATUS_TYPE>())).MUTEX_OWNER as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MUTEX_STATUS_TYPE),
            "::",
            stringify!(MUTEX_OWNER)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MUTEX_STATUS_TYPE>())).MUTEX_STATE as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MUTEX_STATUS_TYPE),
            "::",
            stringify!(MUTEX_STATE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<MUTEX_STATUS_TYPE>())).MUTEX_PRIORITY as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MUTEX_STATUS_TYPE),
            "::",
            stringify!(MUTEX_PRIORITY)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MUTEX_STATUS_TYPE>())).LOCK_COUNT as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MUTEX_STATUS_TYPE),
            "::",
            stringify!(LOCK_COUNT)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<MUTEX_STATUS_TYPE>())).WAITING_PROCESSES as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(MUTEX_STATUS_TYPE),
            "::",
            stringify!(WAITING_PROCESSES)
        )
    );
}
extern "C" {
    pub fn CREATE_MUTEX(
        MUTEX_NAME: *mut cty::c_char,
        MUTEX_PRIORITY: PRIORITY_TYPE,
        QUEUING_DISCIPLINE: QUEUING_DISCIPLINE_TYPE,
        MUTEX_ID: *mut MUTEX_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn ACQUIRE_MUTEX(
        MUTEX_ID: MUTEX_ID_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn RELEASE_MUTEX(MUTEX_ID: MUTEX_ID_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn RESET_MUTEX(
        MUTEX_ID: MUTEX_ID_TYPE,
        PROCESS_ID: PROCESS_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_MUTEX_ID(
        MUTEX_NAME: *mut cty::c_char,
        MUTEX_ID: *mut MUTEX_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_MUTEX_STATUS(
        MUTEX_ID: MUTEX_ID_TYPE,
        MUTEX_STATUS: *mut MUTEX_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_PROCESS_MUTEX_STATE(
        PROCESS_ID: PROCESS_ID_TYPE,
        MUTEX_ID: *mut MUTEX_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
pub type ERROR_MESSAGE_SIZE_TYPE = APEX_INTEGER;
pub type ERROR_MESSAGE_TYPE = [APEX_BYTE; 128usize];
pub const ERROR_CODE_TYPE_DEADLINE_MISSED: ERROR_CODE_TYPE = 0;
pub const ERROR_CODE_TYPE_APPLICATION_ERROR: ERROR_CODE_TYPE = 1;
pub const ERROR_CODE_TYPE_NUMERIC_ERROR: ERROR_CODE_TYPE = 2;
pub const ERROR_CODE_TYPE_ILLEGAL_REQUEST: ERROR_CODE_TYPE = 3;
pub const ERROR_CODE_TYPE_STACK_OVERFLOW: ERROR_CODE_TYPE = 4;
pub const ERROR_CODE_TYPE_MEMORY_VIOLATION: ERROR_CODE_TYPE = 5;
pub const ERROR_CODE_TYPE_HARDWARE_FAULT: ERROR_CODE_TYPE = 6;
pub const ERROR_CODE_TYPE_POWER_FAIL: ERROR_CODE_TYPE = 7;
pub type ERROR_CODE_TYPE = cty::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ERROR_STATUS_TYPE {
    pub FAILED_ADDRESS: SYSTEM_ADDRESS_TYPE,
    pub FAILED_PROCESS_ID: PROCESS_ID_TYPE,
    pub ERROR_CODE: ERROR_CODE_TYPE,
    pub LENGTH: ERROR_MESSAGE_SIZE_TYPE,
    pub MESSAGE: ERROR_MESSAGE_TYPE,
}
#[test]
fn bindgen_test_layout_ERROR_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<ERROR_STATUS_TYPE>(),
        152usize,
        concat!("Size of: ", stringify!(ERROR_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<ERROR_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(ERROR_STATUS_TYPE))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<ERROR_STATUS_TYPE>())).FAILED_ADDRESS as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ERROR_STATUS_TYPE),
            "::",
            stringify!(FAILED_ADDRESS)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<ERROR_STATUS_TYPE>())).FAILED_PROCESS_ID as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ERROR_STATUS_TYPE),
            "::",
            stringify!(FAILED_PROCESS_ID)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ERROR_STATUS_TYPE>())).ERROR_CODE as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ERROR_STATUS_TYPE),
            "::",
            stringify!(ERROR_CODE)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ERROR_STATUS_TYPE>())).LENGTH as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ERROR_STATUS_TYPE),
            "::",
            stringify!(LENGTH)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<ERROR_STATUS_TYPE>())).MESSAGE as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ERROR_STATUS_TYPE),
            "::",
            stringify!(MESSAGE)
        )
    );
}
pub const ERROR_HANDLER_CONCURRENCY_CONTROL_TYPE_PROCESSES_PAUSE:
    ERROR_HANDLER_CONCURRENCY_CONTROL_TYPE = 0;
pub const ERROR_HANDLER_CONCURRENCY_CONTROL_TYPE_PROCESSES_SCHEDULED:
    ERROR_HANDLER_CONCURRENCY_CONTROL_TYPE = 1;
pub type ERROR_HANDLER_CONCURRENCY_CONTROL_TYPE = cty::c_uint;
extern "C" {
    pub fn REPORT_APPLICATION_MESSAGE(
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: MESSAGE_SIZE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn CREATE_ERROR_HANDLER(
        ENTRY_POINT: SYSTEM_ADDRESS_TYPE,
        STACK_SIZE: STACK_SIZE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_ERROR_STATUS(
        ERROR_STATUS: *mut ERROR_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn RAISE_APPLICATION_ERROR(
        ERROR_CODE: ERROR_CODE_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: ERROR_MESSAGE_SIZE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn CONFIGURE_ERROR_HANDLER(
        CONCURRENCY_CONTROL: ERROR_HANDLER_CONCURRENCY_CONTROL_TYPE,
        PROCESSOR_CORE_ID: PROCESSOR_CORE_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
pub type FILE_NAME_TYPE = [cty::c_char; 512usize];
pub type DIRECTORY_ENTRY_TYPE = [cty::c_char; 64usize];
pub type FILE_ID_TYPE = cty::c_int;
pub type DIRECTORY_ID_TYPE = cty::c_int;
pub type FILE_SIZE_TYPE = cty::c_int;
pub type FILE_ERRNO_TYPE = APEX_INTEGER;
pub const FILE_MODE_TYPE_READ: FILE_MODE_TYPE = 0;
pub const FILE_MODE_TYPE_READ_WRITE: FILE_MODE_TYPE = 1;
pub type FILE_MODE_TYPE = cty::c_uint;
pub const FILE_SEEK_TYPE_SEEK_SET: FILE_SEEK_TYPE = 0;
pub const FILE_SEEK_TYPE_SEEK_CUR: FILE_SEEK_TYPE = 1;
pub const FILE_SEEK_TYPE_SEEK_END: FILE_SEEK_TYPE = 2;
pub type FILE_SEEK_TYPE = cty::c_uint;
pub const ENTRY_KIND_TYPE_FILE_ENTRY: ENTRY_KIND_TYPE = 0;
pub const ENTRY_KIND_TYPE_DIRECTORY_ENTRY: ENTRY_KIND_TYPE = 1;
pub const ENTRY_KIND_TYPE_OTHER_ENTRY: ENTRY_KIND_TYPE = 2;
pub const ENTRY_KIND_TYPE_END_OF_DIRECTORY: ENTRY_KIND_TYPE = 3;
pub type ENTRY_KIND_TYPE = cty::c_uint;
pub const TIME_SET_TYPE_UNSET: TIME_SET_TYPE = 0;
pub const TIME_SET_TYPE_SET: TIME_SET_TYPE = 1;
pub type TIME_SET_TYPE = cty::c_uint;
pub const MEDIA_TYPE_VOLATILE: MEDIA_TYPE = 0;
pub const MEDIA_TYPE_NONVOLATILE: MEDIA_TYPE = 1;
pub const MEDIA_TYPE_REMOTE: MEDIA_TYPE = 2;
pub type MEDIA_TYPE = cty::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct COMPOSITE_TIME_TYPE {
    pub TM_SEC: APEX_INTEGER,
    pub TM_MIN: APEX_INTEGER,
    pub TM_HOUR: APEX_INTEGER,
    pub TM_MDAY: APEX_INTEGER,
    pub TM_MON: APEX_INTEGER,
    pub TM_YEAR: APEX_INTEGER,
    pub TM_WDAY: APEX_INTEGER,
    pub TM_YDAY: APEX_INTEGER,
    pub TM_ISDST: APEX_INTEGER,
    pub TM_IS_SET: TIME_SET_TYPE,
}
#[test]
fn bindgen_test_layout_COMPOSITE_TIME_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<COMPOSITE_TIME_TYPE>(),
        80usize,
        concat!("Size of: ", stringify!(COMPOSITE_TIME_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<COMPOSITE_TIME_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(COMPOSITE_TIME_TYPE))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<COMPOSITE_TIME_TYPE>())).TM_SEC as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(COMPOSITE_TIME_TYPE),
            "::",
            stringify!(TM_SEC)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<COMPOSITE_TIME_TYPE>())).TM_MIN as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(COMPOSITE_TIME_TYPE),
            "::",
            stringify!(TM_MIN)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<COMPOSITE_TIME_TYPE>())).TM_HOUR as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(COMPOSITE_TIME_TYPE),
            "::",
            stringify!(TM_HOUR)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<COMPOSITE_TIME_TYPE>())).TM_MDAY as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(COMPOSITE_TIME_TYPE),
            "::",
            stringify!(TM_MDAY)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<COMPOSITE_TIME_TYPE>())).TM_MON as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(COMPOSITE_TIME_TYPE),
            "::",
            stringify!(TM_MON)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<COMPOSITE_TIME_TYPE>())).TM_YEAR as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(COMPOSITE_TIME_TYPE),
            "::",
            stringify!(TM_YEAR)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<COMPOSITE_TIME_TYPE>())).TM_WDAY as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(COMPOSITE_TIME_TYPE),
            "::",
            stringify!(TM_WDAY)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<COMPOSITE_TIME_TYPE>())).TM_YDAY as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(COMPOSITE_TIME_TYPE),
            "::",
            stringify!(TM_YDAY)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<COMPOSITE_TIME_TYPE>())).TM_ISDST as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(COMPOSITE_TIME_TYPE),
            "::",
            stringify!(TM_ISDST)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<COMPOSITE_TIME_TYPE>())).TM_IS_SET as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(COMPOSITE_TIME_TYPE),
            "::",
            stringify!(TM_IS_SET)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FILE_STATUS_TYPE {
    pub CREATE_TIME: COMPOSITE_TIME_TYPE,
    pub LAST_UPDATE: COMPOSITE_TIME_TYPE,
    pub POSITION: FILE_SIZE_TYPE,
    pub SIZE: FILE_SIZE_TYPE,
    pub NB_OF_CHANGES: APEX_INTEGER,
    pub NB_OF_WRITE_ERRORS: APEX_INTEGER,
}
#[test]
fn bindgen_test_layout_FILE_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<FILE_STATUS_TYPE>(),
        184usize,
        concat!("Size of: ", stringify!(FILE_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<FILE_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(FILE_STATUS_TYPE))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<FILE_STATUS_TYPE>())).CREATE_TIME as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FILE_STATUS_TYPE),
            "::",
            stringify!(CREATE_TIME)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<FILE_STATUS_TYPE>())).LAST_UPDATE as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(FILE_STATUS_TYPE),
            "::",
            stringify!(LAST_UPDATE)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<FILE_STATUS_TYPE>())).POSITION as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(FILE_STATUS_TYPE),
            "::",
            stringify!(POSITION)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<FILE_STATUS_TYPE>())).SIZE as *const _ as usize },
        164usize,
        concat!(
            "Offset of field: ",
            stringify!(FILE_STATUS_TYPE),
            "::",
            stringify!(SIZE)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<FILE_STATUS_TYPE>())).NB_OF_CHANGES as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(FILE_STATUS_TYPE),
            "::",
            stringify!(NB_OF_CHANGES)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<FILE_STATUS_TYPE>())).NB_OF_WRITE_ERRORS as *const _ as usize
        },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(FILE_STATUS_TYPE),
            "::",
            stringify!(NB_OF_WRITE_ERRORS)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VOLUME_STATUS_TYPE {
    pub TOTAL_BYTES: APEX_LONG_INTEGER,
    pub USED_BYTES: APEX_LONG_INTEGER,
    pub FREE_BYTES: APEX_LONG_INTEGER,
    pub MAX_ATOMIC_SIZE: APEX_LONG_INTEGER,
    pub BLOCK_SIZE: APEX_INTEGER,
    pub ACCESS_RIGHTS: FILE_MODE_TYPE,
    pub MEDIA: MEDIA_TYPE,
}
#[test]
fn bindgen_test_layout_VOLUME_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<VOLUME_STATUS_TYPE>(),
        48usize,
        concat!("Size of: ", stringify!(VOLUME_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<VOLUME_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(VOLUME_STATUS_TYPE))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<VOLUME_STATUS_TYPE>())).TOTAL_BYTES as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VOLUME_STATUS_TYPE),
            "::",
            stringify!(TOTAL_BYTES)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<VOLUME_STATUS_TYPE>())).USED_BYTES as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VOLUME_STATUS_TYPE),
            "::",
            stringify!(USED_BYTES)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<VOLUME_STATUS_TYPE>())).FREE_BYTES as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VOLUME_STATUS_TYPE),
            "::",
            stringify!(FREE_BYTES)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<VOLUME_STATUS_TYPE>())).MAX_ATOMIC_SIZE as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VOLUME_STATUS_TYPE),
            "::",
            stringify!(MAX_ATOMIC_SIZE)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<VOLUME_STATUS_TYPE>())).BLOCK_SIZE as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(VOLUME_STATUS_TYPE),
            "::",
            stringify!(BLOCK_SIZE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<VOLUME_STATUS_TYPE>())).ACCESS_RIGHTS as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(VOLUME_STATUS_TYPE),
            "::",
            stringify!(ACCESS_RIGHTS)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<VOLUME_STATUS_TYPE>())).MEDIA as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(VOLUME_STATUS_TYPE),
            "::",
            stringify!(MEDIA)
        )
    );
}
extern "C" {
    pub fn OPEN_NEW_FILE(
        FILE_NAME: *mut cty::c_char,
        FILE_ID: *mut FILE_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn OPEN_FILE(
        FILE_NAME: *mut cty::c_char,
        FILE_MODE: FILE_MODE_TYPE,
        FILE_ID: *mut FILE_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn CLOSE_FILE(
        FILE_ID: FILE_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn READ_FILE(
        FILE_ID: FILE_ID_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        IN_LENGTH: MESSAGE_SIZE_TYPE,
        OUT_LENGTH: *mut MESSAGE_SIZE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn WRITE_FILE(
        FILE_ID: FILE_ID_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: MESSAGE_SIZE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn SEEK_FILE(
        FILE_ID: FILE_ID_TYPE,
        OFFSET: FILE_SIZE_TYPE,
        WHENCE: FILE_SEEK_TYPE,
        POSITION: *mut FILE_SIZE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn RESIZE_FILE(
        FILE_ID: FILE_ID_TYPE,
        NEW_SIZE: FILE_SIZE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn SYNC_FILE(
        FILE_ID: FILE_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn REMOVE_FILE(
        FILE_NAME: *mut cty::c_char,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn RENAME_FILE(
        OLD_FILE_NAME: *mut cty::c_char,
        NEW_FILE_NAME: *mut cty::c_char,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn GET_FILE_STATUS(
        FILE_ID: FILE_ID_TYPE,
        FILE_STATUS: *mut FILE_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn GET_VOLUME_STATUS(
        FILE_NAME: *mut cty::c_char,
        VOLUME_STATUS: *mut VOLUME_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn OPEN_DIRECTORY(
        DIRECTORY_NAME: *mut cty::c_char,
        DIRECTORY_ID: *mut DIRECTORY_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn CLOSE_DIRECTORY(
        DIRECTORY_ID: DIRECTORY_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn READ_DIRECTORY(
        DIRECTORY_ID: DIRECTORY_ID_TYPE,
        ENTRY_NAME: *mut DIRECTORY_ENTRY_TYPE,
        ENTRY_KIND: *mut ENTRY_KIND_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn REWIND_DIRECTORY(
        DIRECTORY_ID: DIRECTORY_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn MAKE_DIRECTORY(
        DIRECTORY_NAME: *mut cty::c_char,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn REMOVE_DIRECTORY(
        DIRECTORY_NAME: *mut cty::c_char,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn SYNC_DIRECTORY(
        DIRECTORY_ID: DIRECTORY_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
extern "C" {
    pub fn RENAME_DIRECTORY(
        OLD_DIRECTORY_NAME: *mut cty::c_char,
        NEW_DIRECTORY_NAME: *mut cty::c_char,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
        ERRNO: *mut FILE_ERRNO_TYPE,
    );
}
pub type APEX_CHAR = cty::c_char;
pub type APEX_SHORT_INTEGER = cty::c_short;
pub type APEX_WORD = cty::c_ushort;
pub type APEX_UNSIGNED_LONG = cty::c_ulonglong;
pub type APEX_FLOAT = f32;
pub type APEX_DOUBLE = f64;
pub type APEX_ENUM8 = APEX_CHAR;
pub type APEX_ENUM16 = APEX_SHORT_INTEGER;
pub type APEX_ENUM32 = APEX_INTEGER;
pub type FS_TYPE = APEX_ENUM8;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct FSS_TYPE {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout_FSS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<FSS_TYPE>(),
        4usize,
        concat!("Size of: ", stringify!(FSS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<FSS_TYPE>(),
        1usize,
        concat!("Alignment of ", stringify!(FSS_TYPE))
    );
}
impl FSS_TYPE {
    #[inline]
    pub fn fs1(&self) -> FS_TYPE {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u8) }
    }
    #[inline]
    pub fn set_fs1(&mut self, val: FS_TYPE) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn fs2(&self) -> FS_TYPE {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u8) }
    }
    #[inline]
    pub fn set_fs2(&mut self, val: FS_TYPE) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn fs3(&self) -> FS_TYPE {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(16usize, 8u8) as u8) }
    }
    #[inline]
    pub fn set_fs3(&mut self, val: FS_TYPE) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(16usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn fs4(&self) -> FS_TYPE {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(24usize, 8u8) as u8) }
    }
    #[inline]
    pub fn set_fs4(&mut self, val: FS_TYPE) {
        unsafe {
            let val: u8 = ::core::mem::transmute(val);
            self._bitfield_1.set(24usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        fs1: FS_TYPE,
        fs2: FS_TYPE,
        fs3: FS_TYPE,
        fs4: FS_TYPE,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 8u8, {
            let fs1: u8 = unsafe { ::core::mem::transmute(fs1) };
            fs1 as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let fs2: u8 = unsafe { ::core::mem::transmute(fs2) };
            fs2 as u64
        });
        __bindgen_bitfield_unit.set(16usize, 8u8, {
            let fs3: u8 = unsafe { ::core::mem::transmute(fs3) };
            fs3 as u64
        });
        __bindgen_bitfield_unit.set(24usize, 8u8, {
            let fs4: u8 = unsafe { ::core::mem::transmute(fs4) };
            fs4 as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type SCHEDULE_ID_TYPE = cty::c_int;
pub type SCHEDULE_NAME_TYPE = NAME_TYPE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCHEDULE_STATUS_TYPE {
    pub TIME_OF_LAST_SCHEDULE_SWITCH: SYSTEM_TIME_TYPE,
    pub CURRENT_SCHEDULE: SCHEDULE_ID_TYPE,
    pub NEXT_SCHEDULE: SCHEDULE_ID_TYPE,
}
#[test]
fn bindgen_test_layout_SCHEDULE_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<SCHEDULE_STATUS_TYPE>(),
        16usize,
        concat!("Size of: ", stringify!(SCHEDULE_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<SCHEDULE_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(SCHEDULE_STATUS_TYPE))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SCHEDULE_STATUS_TYPE>())).TIME_OF_LAST_SCHEDULE_SWITCH
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SCHEDULE_STATUS_TYPE),
            "::",
            stringify!(TIME_OF_LAST_SCHEDULE_SWITCH)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SCHEDULE_STATUS_TYPE>())).CURRENT_SCHEDULE as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SCHEDULE_STATUS_TYPE),
            "::",
            stringify!(CURRENT_SCHEDULE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SCHEDULE_STATUS_TYPE>())).NEXT_SCHEDULE as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SCHEDULE_STATUS_TYPE),
            "::",
            stringify!(NEXT_SCHEDULE)
        )
    );
}
extern "C" {
    pub fn SET_MODULE_SCHEDULE(SCHEDULE_ID: SCHEDULE_ID_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn GET_MODULE_SCHEDULE_STATUS(
        SCHEDULE_STATUS: *mut SCHEDULE_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_MODULE_SCHEDULE_ID(
        SCHEDULE_NAME: *mut cty::c_char,
        SCHEDULE_ID: *mut SCHEDULE_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
pub type LOGBOOK_ID_TYPE = cty::c_int;
pub type LOGBOOK_NAME_TYPE = NAME_TYPE;
pub const WRITE_STATUS_TYPE_ABORTED: WRITE_STATUS_TYPE = 0;
pub const WRITE_STATUS_TYPE_IN_PROGRESS: WRITE_STATUS_TYPE = 1;
pub const WRITE_STATUS_TYPE_COMPLETE: WRITE_STATUS_TYPE = 2;
pub type WRITE_STATUS_TYPE = cty::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LOGBOOK_STATUS_TYPE {
    pub MAX_MESSAGE_SIZE: APEX_INTEGER,
    pub MAX_NB_LOGGED_MESSAGES: APEX_INTEGER,
    pub MAX_NB_IN_PROGRESS_MESSAGES: APEX_INTEGER,
    pub NB_LOGGED_MESSAGES: APEX_INTEGER,
    pub NB_IN_PROGRESS_MESSAGES: APEX_INTEGER,
    pub NB_ABORTED_MESSAGES: APEX_INTEGER,
}
#[test]
fn bindgen_test_layout_LOGBOOK_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<LOGBOOK_STATUS_TYPE>(),
        48usize,
        concat!("Size of: ", stringify!(LOGBOOK_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<LOGBOOK_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(LOGBOOK_STATUS_TYPE))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<LOGBOOK_STATUS_TYPE>())).MAX_MESSAGE_SIZE as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(LOGBOOK_STATUS_TYPE),
            "::",
            stringify!(MAX_MESSAGE_SIZE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<LOGBOOK_STATUS_TYPE>())).MAX_NB_LOGGED_MESSAGES as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(LOGBOOK_STATUS_TYPE),
            "::",
            stringify!(MAX_NB_LOGGED_MESSAGES)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<LOGBOOK_STATUS_TYPE>())).MAX_NB_IN_PROGRESS_MESSAGES as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(LOGBOOK_STATUS_TYPE),
            "::",
            stringify!(MAX_NB_IN_PROGRESS_MESSAGES)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<LOGBOOK_STATUS_TYPE>())).NB_LOGGED_MESSAGES as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(LOGBOOK_STATUS_TYPE),
            "::",
            stringify!(NB_LOGGED_MESSAGES)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<LOGBOOK_STATUS_TYPE>())).NB_IN_PROGRESS_MESSAGES as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(LOGBOOK_STATUS_TYPE),
            "::",
            stringify!(NB_IN_PROGRESS_MESSAGES)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<LOGBOOK_STATUS_TYPE>())).NB_ABORTED_MESSAGES as *const _
                as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(LOGBOOK_STATUS_TYPE),
            "::",
            stringify!(NB_ABORTED_MESSAGES)
        )
    );
}
extern "C" {
    pub fn CREATE_LOGBOOK(
        LOGBOOK_NAME: *mut cty::c_char,
        MAX_MESSAGE_SIZE: MESSAGE_SIZE_TYPE,
        MAX_NB_LOGGED_MESSAGES: MESSAGE_RANGE_TYPE,
        MAX_NB_IN_PROGRESS_MESSAGES: MESSAGE_RANGE_TYPE,
        LOGBOOK_ID: *mut LOGBOOK_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn WRITE_LOGBOOK(
        LOGBOOK_ID: LOGBOOK_ID_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: MESSAGE_SIZE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn OVERWRITE_LOGBOOK(
        LOGBOOK_ID: LOGBOOK_ID_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: MESSAGE_SIZE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn READ_LOGBOOK(
        LOGBOOK_ID: LOGBOOK_ID_TYPE,
        LOGBOOK_ENTRY: MESSAGE_RANGE_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: *mut MESSAGE_SIZE_TYPE,
        WRITE_STATUS: *mut WRITE_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn CLEAR_LOGBOOK(LOGBOOK_ID: LOGBOOK_ID_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
extern "C" {
    pub fn GET_LOGBOOK_ID(
        LOGBOOK_NAME: *mut cty::c_char,
        LOGBOOK_ID: *mut LOGBOOK_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_LOGBOOK_STATUS(
        LOGBOOK_ID: LOGBOOK_ID_TYPE,
        LOGBOOK_STATUS: *mut LOGBOOK_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
pub const UPDATED_TYPE_EMPTY_PORT: UPDATED_TYPE = 0;
pub const UPDATED_TYPE_CONSUMED_MESSAGE: UPDATED_TYPE = 1;
pub const UPDATED_TYPE_NEW_MESSAGE: UPDATED_TYPE = 2;
pub type UPDATED_TYPE = cty::c_uint;
pub const AGE_TYPE_STALE: AGE_TYPE = 0;
pub const AGE_TYPE_FRESH: AGE_TYPE = 1;
pub type AGE_TYPE = cty::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SAMPLING_PORT_CURRENT_STATUS_TYPE {
    pub REFRESH_PERIOD: SYSTEM_TIME_TYPE,
    pub TIME_STAMP: SYSTEM_TIME_TYPE,
    pub MAX_MESSAGE_SIZE: MESSAGE_SIZE_TYPE,
    pub PORT_DIRECTION: PORT_DIRECTION_TYPE,
    pub MESSAGE_AGE: AGE_TYPE,
    pub UPDATED: UPDATED_TYPE,
}
#[test]
fn bindgen_test_layout_SAMPLING_PORT_CURRENT_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<SAMPLING_PORT_CURRENT_STATUS_TYPE>(),
        40usize,
        concat!("Size of: ", stringify!(SAMPLING_PORT_CURRENT_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<SAMPLING_PORT_CURRENT_STATUS_TYPE>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(SAMPLING_PORT_CURRENT_STATUS_TYPE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAMPLING_PORT_CURRENT_STATUS_TYPE>())).REFRESH_PERIOD
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SAMPLING_PORT_CURRENT_STATUS_TYPE),
            "::",
            stringify!(REFRESH_PERIOD)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAMPLING_PORT_CURRENT_STATUS_TYPE>())).TIME_STAMP as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SAMPLING_PORT_CURRENT_STATUS_TYPE),
            "::",
            stringify!(TIME_STAMP)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAMPLING_PORT_CURRENT_STATUS_TYPE>())).MAX_MESSAGE_SIZE
                as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SAMPLING_PORT_CURRENT_STATUS_TYPE),
            "::",
            stringify!(MAX_MESSAGE_SIZE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAMPLING_PORT_CURRENT_STATUS_TYPE>())).PORT_DIRECTION
                as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SAMPLING_PORT_CURRENT_STATUS_TYPE),
            "::",
            stringify!(PORT_DIRECTION)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAMPLING_PORT_CURRENT_STATUS_TYPE>())).MESSAGE_AGE as *const _
                as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(SAMPLING_PORT_CURRENT_STATUS_TYPE),
            "::",
            stringify!(MESSAGE_AGE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAMPLING_PORT_CURRENT_STATUS_TYPE>())).UPDATED as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SAMPLING_PORT_CURRENT_STATUS_TYPE),
            "::",
            stringify!(UPDATED)
        )
    );
}
extern "C" {
    pub fn READ_UPDATED_SAMPLING_MESSAGE(
        SAMPLING_PORT_ID: SAMPLING_PORT_ID_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: *mut MESSAGE_SIZE_TYPE,
        UPDATED: *mut UPDATED_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_SAMPLING_PORT_CURRENT_STATUS(
        SAMPLING_PORT_ID: SAMPLING_PORT_ID_TYPE,
        SAMPLING_PORT_CURRENT_STATUS: *mut SAMPLING_PORT_CURRENT_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn READ_SAMPLING_MESSAGE_CONDITIONAL(
        SAMPLING_PORT_ID: SAMPLING_PORT_ID_TYPE,
        REF_TIME_STAMP: SYSTEM_TIME_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: *mut MESSAGE_SIZE_TYPE,
        TIME_STAMP: *mut SYSTEM_TIME_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
pub type SAP_PORT_NAME_TYPE = NAME_TYPE;
pub type SAP_PORT_ID_TYPE = cty::c_int;
pub type APEX_UNSIGNED_SHORT = cty::c_ushort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SAP_ADDRESS_TYPE {
    pub Addr_Length: APEX_BYTE,
    pub Addr_Type: APEX_BYTE,
    pub UDP_Port_Number: APEX_UNSIGNED_SHORT,
    pub IP_Address: APEX_UNSIGNED,
    pub Addr_Spare: [APEX_BYTE; 8usize],
}
#[test]
fn bindgen_test_layout_SAP_ADDRESS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<SAP_ADDRESS_TYPE>(),
        24usize,
        concat!("Size of: ", stringify!(SAP_ADDRESS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<SAP_ADDRESS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(SAP_ADDRESS_TYPE))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<SAP_ADDRESS_TYPE>())).Addr_Length as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SAP_ADDRESS_TYPE),
            "::",
            stringify!(Addr_Length)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<SAP_ADDRESS_TYPE>())).Addr_Type as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(SAP_ADDRESS_TYPE),
            "::",
            stringify!(Addr_Type)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAP_ADDRESS_TYPE>())).UDP_Port_Number as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(SAP_ADDRESS_TYPE),
            "::",
            stringify!(UDP_Port_Number)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<SAP_ADDRESS_TYPE>())).IP_Address as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SAP_ADDRESS_TYPE),
            "::",
            stringify!(IP_Address)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<SAP_ADDRESS_TYPE>())).Addr_Spare as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SAP_ADDRESS_TYPE),
            "::",
            stringify!(Addr_Spare)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SAP_PORT_STATUS_TYPE {
    pub NB_MESSAGE: MESSAGE_RANGE_TYPE,
    pub MAX_NB_MESSAGE: MESSAGE_RANGE_TYPE,
    pub MAX_MESSAGE_SIZE: MESSAGE_SIZE_TYPE,
    pub PORT_DIRECTION: PORT_DIRECTION_TYPE,
    pub WAITING_PROCESSES: WAITING_RANGE_TYPE,
    pub CURRENT_DEST_ADDR: SAP_ADDRESS_TYPE,
    pub CURRENT_SRC_ADDR: SAP_ADDRESS_TYPE,
}
#[test]
fn bindgen_test_layout_SAP_PORT_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<SAP_PORT_STATUS_TYPE>(),
        88usize,
        concat!("Size of: ", stringify!(SAP_PORT_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<SAP_PORT_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(SAP_PORT_STATUS_TYPE))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAP_PORT_STATUS_TYPE>())).NB_MESSAGE as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SAP_PORT_STATUS_TYPE),
            "::",
            stringify!(NB_MESSAGE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAP_PORT_STATUS_TYPE>())).MAX_NB_MESSAGE as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SAP_PORT_STATUS_TYPE),
            "::",
            stringify!(MAX_NB_MESSAGE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAP_PORT_STATUS_TYPE>())).MAX_MESSAGE_SIZE as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SAP_PORT_STATUS_TYPE),
            "::",
            stringify!(MAX_MESSAGE_SIZE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAP_PORT_STATUS_TYPE>())).PORT_DIRECTION as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SAP_PORT_STATUS_TYPE),
            "::",
            stringify!(PORT_DIRECTION)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAP_PORT_STATUS_TYPE>())).WAITING_PROCESSES as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SAP_PORT_STATUS_TYPE),
            "::",
            stringify!(WAITING_PROCESSES)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAP_PORT_STATUS_TYPE>())).CURRENT_DEST_ADDR as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SAP_PORT_STATUS_TYPE),
            "::",
            stringify!(CURRENT_DEST_ADDR)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<SAP_PORT_STATUS_TYPE>())).CURRENT_SRC_ADDR as *const _ as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(SAP_PORT_STATUS_TYPE),
            "::",
            stringify!(CURRENT_SRC_ADDR)
        )
    );
}
extern "C" {
    pub fn CREATE_SAP_PORT(
        SAP_PORT_NAME: *mut cty::c_char,
        MAX_MESSAGE_SIZE: MESSAGE_SIZE_TYPE,
        MAX_NB_MESSAGE: MESSAGE_RANGE_TYPE,
        PORT_DIRECTION: PORT_DIRECTION_TYPE,
        QUEUING_DISCIPLINE: QUEUING_DISCIPLINE_TYPE,
        SAP_PORT_ID: *mut SAP_PORT_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn SEND_SAP_MESSAGE(
        SAP_PORT_ID: SAP_PORT_ID_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: MESSAGE_SIZE_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        DESTINATION_ADDR: SAP_ADDRESS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn RECEIVE_SAP_MESSAGE(
        SAP_PORT_ID: SAP_PORT_ID_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: *mut MESSAGE_SIZE_TYPE,
        SOURCE_ADDR: *mut SAP_ADDRESS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_SAP_PORT_ID(
        SAP_PORT_NAME: *mut cty::c_char,
        SAP_PORT_ID: *mut SAP_PORT_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_SAP_PORT_STATUS(
        SAP_PORT_ID: SAP_PORT_ID_TYPE,
        SAP_PORT_STATUS: *mut SAP_PORT_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn SEND_EXTENDED_SAP_MESSAGE(
        SAP_PORT_ID: SAP_PORT_ID_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: MESSAGE_SIZE_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        SOURCE_ADDR: SAP_ADDRESS_TYPE,
        DESTINATION_ADDR: SAP_ADDRESS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn RECEIVE_EXTENDED_SAP_MESSAGE(
        SAP_PORT_ID: SAP_PORT_ID_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        LENGTH: *mut MESSAGE_SIZE_TYPE,
        SOURCE_ADDR: *mut SAP_ADDRESS_TYPE,
        DESTINATION_ADDR: *mut SAP_ADDRESS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn CLEAR_SAP_PORT(SAP_PORT_ID: SAP_PORT_ID_TYPE, RETURN_CODE: *mut RETURN_CODE_TYPE);
}
pub type NAME_SERVICE_ID_TYPE = cty::c_int;
pub type NAME_SERVICE_NAME_TYPE = NAME_TYPE;
pub type NAME_SERVICE_ADDR_TYPE = *mut APEX_BYTE;
pub type NAME_SERVICE_LENGTH_TYPE = APEX_INTEGER;
extern "C" {
    pub fn CREATE_NAME_SERVICE(
        NAME_SERVICE_NAME: *mut cty::c_char,
        QUEUING_DISCIPLINE: QUEUING_DISCIPLINE_TYPE,
        SERVICE_ID: *mut NAME_SERVICE_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_NAME(
        SERVICE_ID: NAME_SERVICE_ID_TYPE,
        SAP_ADDRESS: SAP_ADDRESS_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        NAME_ADDR: NAME_SERVICE_ADDR_TYPE,
        MAX_NAME_LENGTH: NAME_SERVICE_LENGTH_TYPE,
        NAME_LENGTH: *mut NAME_SERVICE_LENGTH_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_ADDRESS(
        SERVICE_ID: NAME_SERVICE_ID_TYPE,
        NAME: NAME_SERVICE_ADDR_TYPE,
        NAME_LENGTH: *mut NAME_SERVICE_LENGTH_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        SAP_ADDRESS: *mut SAP_ADDRESS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
pub type MEMORY_BLOCK_NAME_TYPE = NAME_TYPE;
pub const MEMORY_BLOCK_MODE_TYPE_MB_READ: MEMORY_BLOCK_MODE_TYPE = 0;
pub const MEMORY_BLOCK_MODE_TYPE_MB_READ_WRITE: MEMORY_BLOCK_MODE_TYPE = 1;
pub type MEMORY_BLOCK_MODE_TYPE = cty::c_uint;
pub type MEMORY_BLOCK_SIZE_TYPE = APEX_INTEGER;
pub type MEMORY_BLOCK_ADDR_TYPE = *mut APEX_BYTE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MEMORY_BLOCK_STATUS_TYPE {
    pub MEMORY_BLOCK_ADDR: MEMORY_BLOCK_ADDR_TYPE,
    pub MEMORY_BLOCK_MODE: MEMORY_BLOCK_MODE_TYPE,
    pub MEMORY_BLOCK_SIZE: MEMORY_BLOCK_SIZE_TYPE,
}
#[test]
fn bindgen_test_layout_MEMORY_BLOCK_STATUS_TYPE() {
    assert_eq!(
        ::core::mem::size_of::<MEMORY_BLOCK_STATUS_TYPE>(),
        24usize,
        concat!("Size of: ", stringify!(MEMORY_BLOCK_STATUS_TYPE))
    );
    assert_eq!(
        ::core::mem::align_of::<MEMORY_BLOCK_STATUS_TYPE>(),
        8usize,
        concat!("Alignment of ", stringify!(MEMORY_BLOCK_STATUS_TYPE))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<MEMORY_BLOCK_STATUS_TYPE>())).MEMORY_BLOCK_ADDR as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MEMORY_BLOCK_STATUS_TYPE),
            "::",
            stringify!(MEMORY_BLOCK_ADDR)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<MEMORY_BLOCK_STATUS_TYPE>())).MEMORY_BLOCK_MODE as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MEMORY_BLOCK_STATUS_TYPE),
            "::",
            stringify!(MEMORY_BLOCK_MODE)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<MEMORY_BLOCK_STATUS_TYPE>())).MEMORY_BLOCK_SIZE as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MEMORY_BLOCK_STATUS_TYPE),
            "::",
            stringify!(MEMORY_BLOCK_SIZE)
        )
    );
}
extern "C" {
    pub fn GET_MEMORY_BLOCK_STATUS(
        MEMORY_BLOCK_NAME: *mut cty::c_char,
        MEMORY_BLOCK_STATUS: *mut MEMORY_BLOCK_STATUS_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn ENABLE_ERROR_HANDLER(RETURN_CODE: *mut RETURN_CODE_TYPE);
}
pub const PORT_ACTION_TYPE_INACTIVE: PORT_ACTION_TYPE = 0;
pub const PORT_ACTION_TYPE_ACTIVE: PORT_ACTION_TYPE = 1;
pub type PORT_ACTION_TYPE = cty::c_uint;
pub const RESET_LIST_TYPE_RESET: RESET_LIST_TYPE = 0;
pub const RESET_LIST_TYPE_CONTINUE: RESET_LIST_TYPE = 1;
pub type RESET_LIST_TYPE = cty::c_uint;
pub type LIST_SIZE_TYPE = APEX_INTEGER;
pub type QUEUING_PORT_LIST_ID_TYPE = cty::c_int;
pub type QUEUING_PORT_LIST_NAME_TYPE = NAME_TYPE;
extern "C" {
    pub fn CREATE_QUEUING_PORT_LIST(
        QUEUING_PORT_LIST_NAME: *mut cty::c_char,
        MAX_LIST_SIZE: LIST_SIZE_TYPE,
        QUEUING_PORT_LIST_ID: *mut QUEUING_PORT_LIST_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn ADD_PORT_TO_QUEUING_PORT_LIST(
        QUEUING_PORT_LIST_ID: QUEUING_PORT_LIST_ID_TYPE,
        QUEUING_PORT_ID: QUEUING_PORT_ID_TYPE,
        PORT_ACTION: PORT_ACTION_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn SET_PORT_ACTION_IN_QUEUING_PORT_LIST(
        QUEUING_PORT_LIST_ID: QUEUING_PORT_LIST_ID_TYPE,
        QUEUING_PORT_ID: QUEUING_PORT_ID_TYPE,
        PORT_ACTION: PORT_ACTION_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_QUEUING_PORT_ACTION_STATUS(
        QUEUING_PORT_LIST_ID: QUEUING_PORT_LIST_ID_TYPE,
        QUEUING_PORT_ID: QUEUING_PORT_ID_TYPE,
        PORT_ACTION_STATUS: *mut PORT_ACTION_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn RECEIVE_MESSAGE_FROM_QUEUING_PORT_LIST(
        QUEUING_PORT_LIST_ID: QUEUING_PORT_LIST_ID_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        MESSAGE_ADDR: MESSAGE_ADDR_TYPE,
        BUFFER_SIZE: MESSAGE_SIZE_TYPE,
        RESET_LIST: RESET_LIST_TYPE,
        QUEUING_PORT_ID: *mut QUEUING_PORT_ID_TYPE,
        LENGTH: *mut MESSAGE_SIZE_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn WAIT_FOR_MESSAGE_FROM_QUEUING_PORT_LIST(
        QUEUING_PORT_LIST_ID: QUEUING_PORT_LIST_ID_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        RESET_LIST: RESET_LIST_TYPE,
        QUEUING_PORT_ID: *mut QUEUING_PORT_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
pub const SET_AFFINITY_PERMISSION_TYPE_NOT_PERMITTED: SET_AFFINITY_PERMISSION_TYPE = 0;
pub const SET_AFFINITY_PERMISSION_TYPE_INIT_ONLY: SET_AFFINITY_PERMISSION_TYPE = 1;
pub const SET_AFFINITY_PERMISSION_TYPE_ALL_MODES: SET_AFFINITY_PERMISSION_TYPE = 2;
pub type SET_AFFINITY_PERMISSION_TYPE = cty::c_uint;
extern "C" {
    pub fn SET_PROCESS_CORE_AFFINITY(
        PROCESS_ID: PROCESS_ID_TYPE,
        PROCESSOR_CORE_ID: PROCESSOR_CORE_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_PROCESS_CORE_AFFINITY(
        PROCESS_ID: PROCESS_ID_TYPE,
        PROCESSOR_CORE_ID: *mut PROCESSOR_CORE_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn SET_DEFAULT_PROCESS_CORE_AFFINITY(
        PROCESSOR_CORE_ID: PROCESSOR_CORE_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
pub const INTERRUPT_PENDING_TYPE_CLEAR: INTERRUPT_PENDING_TYPE = 0;
pub const INTERRUPT_PENDING_TYPE_NO_CLEAR: INTERRUPT_PENDING_TYPE = 1;
pub type INTERRUPT_PENDING_TYPE = cty::c_uint;
pub type INTERRUPT_ID_TYPE = cty::c_int;
pub type INTERRUPT_NAME_TYPE = NAME_TYPE;
extern "C" {
    pub fn CREATE_INTERRUPT(
        INTERRUPT_NAME: *mut cty::c_char,
        INTERRUPT_ID: *mut INTERRUPT_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn GET_INTERRUPT_ID(
        INTERRUPT_NAME: *mut cty::c_char,
        INTERRUPT_ID: *mut INTERRUPT_ID_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
extern "C" {
    pub fn WAIT_INTERRUPT(
        INTERRUPT_ID: INTERRUPT_ID_TYPE,
        CLEAR_PENDING: INTERRUPT_PENDING_TYPE,
        TIME_OUT: SYSTEM_TIME_TYPE,
        RETURN_CODE: *mut RETURN_CODE_TYPE,
    );
}
