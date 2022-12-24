#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Glitch and clock monitor control"]
    pub monitor_glitch0_control: MONITOR_GLITCH0_CONTROL,
    #[doc = "0x04 - Glitch and clock monitor status"]
    pub monitor_glitch0_status: MONITOR_GLITCH0_STATUS,
    #[doc = "0x08 - Glitch and clock monitor control"]
    pub monitor_glitch1_control: MONITOR_GLITCH1_CONTROL,
    #[doc = "0x0c - Glitch and clock monitor status"]
    pub monitor_glitch1_status: MONITOR_GLITCH1_STATUS,
    #[doc = "0x10 - Glitch and clock monitor control"]
    pub monitor_clock0_control: MONITOR_CLOCK0_CONTROL,
    #[doc = "0x14 - Glitch and clock monitor status"]
    pub monitor_clock0_status: MONITOR_CLOCK0_STATUS,
    #[doc = "0x18 - Glitch and clock monitor control"]
    pub monitor_clock1_control: MONITOR_CLOCK1_CONTROL,
    #[doc = "0x1c - Glitch and clock monitor status"]
    pub monitor_clock1_status: MONITOR_CLOCK1_STATUS,
    _reserved8: [u8; 0x20],
    #[doc = "0x40 - No description avaiable"]
    pub irq_flag: IRQ_FLAG,
    #[doc = "0x44 - No description avaiable"]
    pub irq_enable: IRQ_ENABLE,
}
#[doc = "MONITOR_GLITCH0_CONTROL (rw) register accessor: an alias for `Reg<MONITOR_GLITCH0_CONTROL_SPEC>`"]
pub type MONITOR_GLITCH0_CONTROL =
    crate::Reg<monitor_glitch0_control::MONITOR_GLITCH0_CONTROL_SPEC>;
#[doc = "Glitch and clock monitor control"]
pub mod monitor_glitch0_control;
#[doc = "MONITOR_GLITCH0_STATUS (rw) register accessor: an alias for `Reg<MONITOR_GLITCH0_STATUS_SPEC>`"]
pub type MONITOR_GLITCH0_STATUS = crate::Reg<monitor_glitch0_status::MONITOR_GLITCH0_STATUS_SPEC>;
#[doc = "Glitch and clock monitor status"]
pub mod monitor_glitch0_status;
#[doc = "MONITOR_GLITCH1_CONTROL (rw) register accessor: an alias for `Reg<MONITOR_GLITCH1_CONTROL_SPEC>`"]
pub type MONITOR_GLITCH1_CONTROL =
    crate::Reg<monitor_glitch1_control::MONITOR_GLITCH1_CONTROL_SPEC>;
#[doc = "Glitch and clock monitor control"]
pub mod monitor_glitch1_control;
#[doc = "MONITOR_GLITCH1_STATUS (rw) register accessor: an alias for `Reg<MONITOR_GLITCH1_STATUS_SPEC>`"]
pub type MONITOR_GLITCH1_STATUS = crate::Reg<monitor_glitch1_status::MONITOR_GLITCH1_STATUS_SPEC>;
#[doc = "Glitch and clock monitor status"]
pub mod monitor_glitch1_status;
#[doc = "MONITOR_CLOCK0_CONTROL (rw) register accessor: an alias for `Reg<MONITOR_CLOCK0_CONTROL_SPEC>`"]
pub type MONITOR_CLOCK0_CONTROL = crate::Reg<monitor_clock0_control::MONITOR_CLOCK0_CONTROL_SPEC>;
#[doc = "Glitch and clock monitor control"]
pub mod monitor_clock0_control;
#[doc = "MONITOR_CLOCK0_STATUS (rw) register accessor: an alias for `Reg<MONITOR_CLOCK0_STATUS_SPEC>`"]
pub type MONITOR_CLOCK0_STATUS = crate::Reg<monitor_clock0_status::MONITOR_CLOCK0_STATUS_SPEC>;
#[doc = "Glitch and clock monitor status"]
pub mod monitor_clock0_status;
#[doc = "MONITOR_CLOCK1_CONTROL (rw) register accessor: an alias for `Reg<MONITOR_CLOCK1_CONTROL_SPEC>`"]
pub type MONITOR_CLOCK1_CONTROL = crate::Reg<monitor_clock1_control::MONITOR_CLOCK1_CONTROL_SPEC>;
#[doc = "Glitch and clock monitor control"]
pub mod monitor_clock1_control;
#[doc = "MONITOR_CLOCK1_STATUS (rw) register accessor: an alias for `Reg<MONITOR_CLOCK1_STATUS_SPEC>`"]
pub type MONITOR_CLOCK1_STATUS = crate::Reg<monitor_clock1_status::MONITOR_CLOCK1_STATUS_SPEC>;
#[doc = "Glitch and clock monitor status"]
pub mod monitor_clock1_status;
#[doc = "IRQ_FLAG (rw) register accessor: an alias for `Reg<IRQ_FLAG_SPEC>`"]
pub type IRQ_FLAG = crate::Reg<irq_flag::IRQ_FLAG_SPEC>;
#[doc = "No description avaiable"]
pub mod irq_flag;
#[doc = "IRQ_ENABLE (rw) register accessor: an alias for `Reg<IRQ_ENABLE_SPEC>`"]
pub type IRQ_ENABLE = crate::Reg<irq_enable::IRQ_ENABLE_SPEC>;
#[doc = "No description avaiable"]
pub mod irq_enable;
