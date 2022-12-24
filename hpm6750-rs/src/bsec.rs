#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Secure state"]
    pub secure_state: SECURE_STATE,
    #[doc = "0x04 - secure state configuration"]
    pub secure_state_config: SECURE_STATE_CONFIG,
    #[doc = "0x08 - Security violation config"]
    pub violation_config: VIOLATION_CONFIG,
    #[doc = "0x0c - Escalate behavior on security event"]
    pub escalate_config: ESCALATE_CONFIG,
    #[doc = "0x10 - Event and escalate status"]
    pub event: EVENT,
}
#[doc = "SECURE_STATE (rw) register accessor: an alias for `Reg<SECURE_STATE_SPEC>`"]
pub type SECURE_STATE = crate::Reg<secure_state::SECURE_STATE_SPEC>;
#[doc = "Secure state"]
pub mod secure_state;
#[doc = "SECURE_STATE_CONFIG (rw) register accessor: an alias for `Reg<SECURE_STATE_CONFIG_SPEC>`"]
pub type SECURE_STATE_CONFIG = crate::Reg<secure_state_config::SECURE_STATE_CONFIG_SPEC>;
#[doc = "secure state configuration"]
pub mod secure_state_config;
#[doc = "VIOLATION_CONFIG (rw) register accessor: an alias for `Reg<VIOLATION_CONFIG_SPEC>`"]
pub type VIOLATION_CONFIG = crate::Reg<violation_config::VIOLATION_CONFIG_SPEC>;
#[doc = "Security violation config"]
pub mod violation_config;
#[doc = "ESCALATE_CONFIG (rw) register accessor: an alias for `Reg<ESCALATE_CONFIG_SPEC>`"]
pub type ESCALATE_CONFIG = crate::Reg<escalate_config::ESCALATE_CONFIG_SPEC>;
#[doc = "Escalate behavior on security event"]
pub mod escalate_config;
#[doc = "EVENT (r) register accessor: an alias for `Reg<EVENT_SPEC>`"]
pub type EVENT = crate::Reg<event::EVENT_SPEC>;
#[doc = "Event and escalate status"]
pub mod event;
