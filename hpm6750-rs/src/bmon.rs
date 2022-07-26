#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Glitch and clock monitor control"]
    pub monitor_glitch0_control: crate::Reg<monitor_glitch0_control::MONITOR_GLITCH0_CONTROL_SPEC>,
    #[doc = "0x04 - Glitch and clock monitor status"]
    pub monitor_glitch0_status: crate::Reg<monitor_glitch0_status::MONITOR_GLITCH0_STATUS_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Glitch and clock monitor control"]
    pub monitor_clock0_control: crate::Reg<monitor_clock0_control::MONITOR_CLOCK0_CONTROL_SPEC>,
    #[doc = "0x14 - Glitch and clock monitor status"]
    pub monitor_clock0_status: crate::Reg<monitor_clock0_status::MONITOR_CLOCK0_STATUS_SPEC>,
}
#[doc = "MONITOR_GLITCH0_CONTROL register accessor: an alias for `Reg<MONITOR_GLITCH0_CONTROL_SPEC>`"]
pub type MONITOR_GLITCH0_CONTROL =
    crate::Reg<monitor_glitch0_control::MONITOR_GLITCH0_CONTROL_SPEC>;
#[doc = "Glitch and clock monitor control"]
pub mod monitor_glitch0_control;
#[doc = "MONITOR_GLITCH0_STATUS register accessor: an alias for `Reg<MONITOR_GLITCH0_STATUS_SPEC>`"]
pub type MONITOR_GLITCH0_STATUS = crate::Reg<monitor_glitch0_status::MONITOR_GLITCH0_STATUS_SPEC>;
#[doc = "Glitch and clock monitor status"]
pub mod monitor_glitch0_status;
#[doc = "MONITOR_CLOCK0_CONTROL register accessor: an alias for `Reg<MONITOR_CLOCK0_CONTROL_SPEC>`"]
pub type MONITOR_CLOCK0_CONTROL = crate::Reg<monitor_clock0_control::MONITOR_CLOCK0_CONTROL_SPEC>;
#[doc = "Glitch and clock monitor control"]
pub mod monitor_clock0_control;
#[doc = "MONITOR_CLOCK0_STATUS register accessor: an alias for `Reg<MONITOR_CLOCK0_STATUS_SPEC>`"]
pub type MONITOR_CLOCK0_STATUS = crate::Reg<monitor_clock0_status::MONITOR_CLOCK0_STATUS_SPEC>;
#[doc = "Glitch and clock monitor status"]
pub mod monitor_clock0_status;
