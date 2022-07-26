#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BANGGAP control"]
    pub bandgap: crate::Reg<bandgap::BANDGAP_SPEC>,
    #[doc = "0x04 - 1V LDO config"]
    pub ldo1p1: crate::Reg<ldo1p1::LDO1P1_SPEC>,
    #[doc = "0x08 - 2.5V LDO config"]
    pub ldo2p5: crate::Reg<ldo2p5::LDO2P5_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - DCDC mode select"]
    pub dcdc_mode: crate::Reg<dcdc_mode::DCDC_MODE_SPEC>,
    #[doc = "0x14 - DCDC low power mode"]
    pub dcdc_lpmode: crate::Reg<dcdc_lpmode::DCDC_LPMODE_SPEC>,
    #[doc = "0x18 - DCDC protection"]
    pub dcdc_prot: crate::Reg<dcdc_prot::DCDC_PROT_SPEC>,
    #[doc = "0x1c - DCDC current estimation"]
    pub dcdc_current: crate::Reg<dcdc_current::DCDC_CURRENT_SPEC>,
    #[doc = "0x20 - DCDC advance setting"]
    pub dcdc_advmode: crate::Reg<dcdc_advmode::DCDC_ADVMODE_SPEC>,
    #[doc = "0x24 - DCDC advance parameter"]
    pub dcdc_advparam: crate::Reg<dcdc_advparam::DCDC_ADVPARAM_SPEC>,
    #[doc = "0x28 - DCDC misc parameter"]
    pub dcdc_misc: crate::Reg<dcdc_misc::DCDC_MISC_SPEC>,
    #[doc = "0x2c - DCDC Debug"]
    pub dcdc_debug: crate::Reg<dcdc_debug::DCDC_DEBUG_SPEC>,
    #[doc = "0x30 - DCDC ramp time"]
    pub dcdc_start_time: crate::Reg<dcdc_start_time::DCDC_START_TIME_SPEC>,
    #[doc = "0x34 - DCDC resume time"]
    pub dcdc_resume_time: crate::Reg<dcdc_resume_time::DCDC_RESUME_TIME_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x40 - SOC power trap"]
    pub power_trap: crate::Reg<power_trap::POWER_TRAP_SPEC>,
    #[doc = "0x44 - Wake up source"]
    pub wake_cause: crate::Reg<wake_cause::WAKE_CAUSE_SPEC>,
    #[doc = "0x48 - Wake up mask"]
    pub wake_mask: crate::Reg<wake_mask::WAKE_MASK_SPEC>,
    #[doc = "0x4c - Clock gate control in PMIC"]
    pub scg_ctrl: crate::Reg<scg_ctrl::SCG_CTRL_SPEC>,
    #[doc = "0x50 - Debug stop config"]
    pub debug_stop: crate::Reg<debug_stop::DEBUG_STOP_SPEC>,
    _reserved18: [u8; 0x0c],
    #[doc = "0x60 - RC 24M config"]
    pub rc24m: crate::Reg<rc24m::RC24M_SPEC>,
    #[doc = "0x64 - RC 24M track mode"]
    pub rc24m_track: crate::Reg<rc24m_track::RC24M_TRACK_SPEC>,
    #[doc = "0x68 - RC 24M track target"]
    pub track_target: crate::Reg<track_target::TRACK_TARGET_SPEC>,
    #[doc = "0x6c - RC 24M track status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
}
#[doc = "BANDGAP register accessor: an alias for `Reg<BANDGAP_SPEC>`"]
pub type BANDGAP = crate::Reg<bandgap::BANDGAP_SPEC>;
#[doc = "BANGGAP control"]
pub mod bandgap;
#[doc = "LDO1P1 register accessor: an alias for `Reg<LDO1P1_SPEC>`"]
pub type LDO1P1 = crate::Reg<ldo1p1::LDO1P1_SPEC>;
#[doc = "1V LDO config"]
pub mod ldo1p1;
#[doc = "LDO2P5 register accessor: an alias for `Reg<LDO2P5_SPEC>`"]
pub type LDO2P5 = crate::Reg<ldo2p5::LDO2P5_SPEC>;
#[doc = "2.5V LDO config"]
pub mod ldo2p5;
#[doc = "DCDC_MODE register accessor: an alias for `Reg<DCDC_MODE_SPEC>`"]
pub type DCDC_MODE = crate::Reg<dcdc_mode::DCDC_MODE_SPEC>;
#[doc = "DCDC mode select"]
pub mod dcdc_mode;
#[doc = "DCDC_LPMODE register accessor: an alias for `Reg<DCDC_LPMODE_SPEC>`"]
pub type DCDC_LPMODE = crate::Reg<dcdc_lpmode::DCDC_LPMODE_SPEC>;
#[doc = "DCDC low power mode"]
pub mod dcdc_lpmode;
#[doc = "DCDC_PROT register accessor: an alias for `Reg<DCDC_PROT_SPEC>`"]
pub type DCDC_PROT = crate::Reg<dcdc_prot::DCDC_PROT_SPEC>;
#[doc = "DCDC protection"]
pub mod dcdc_prot;
#[doc = "DCDC_CURRENT register accessor: an alias for `Reg<DCDC_CURRENT_SPEC>`"]
pub type DCDC_CURRENT = crate::Reg<dcdc_current::DCDC_CURRENT_SPEC>;
#[doc = "DCDC current estimation"]
pub mod dcdc_current;
#[doc = "DCDC_ADVMODE register accessor: an alias for `Reg<DCDC_ADVMODE_SPEC>`"]
pub type DCDC_ADVMODE = crate::Reg<dcdc_advmode::DCDC_ADVMODE_SPEC>;
#[doc = "DCDC advance setting"]
pub mod dcdc_advmode;
#[doc = "DCDC_ADVPARAM register accessor: an alias for `Reg<DCDC_ADVPARAM_SPEC>`"]
pub type DCDC_ADVPARAM = crate::Reg<dcdc_advparam::DCDC_ADVPARAM_SPEC>;
#[doc = "DCDC advance parameter"]
pub mod dcdc_advparam;
#[doc = "DCDC_MISC register accessor: an alias for `Reg<DCDC_MISC_SPEC>`"]
pub type DCDC_MISC = crate::Reg<dcdc_misc::DCDC_MISC_SPEC>;
#[doc = "DCDC misc parameter"]
pub mod dcdc_misc;
#[doc = "DCDC_DEBUG register accessor: an alias for `Reg<DCDC_DEBUG_SPEC>`"]
pub type DCDC_DEBUG = crate::Reg<dcdc_debug::DCDC_DEBUG_SPEC>;
#[doc = "DCDC Debug"]
pub mod dcdc_debug;
#[doc = "DCDC_START_TIME register accessor: an alias for `Reg<DCDC_START_TIME_SPEC>`"]
pub type DCDC_START_TIME = crate::Reg<dcdc_start_time::DCDC_START_TIME_SPEC>;
#[doc = "DCDC ramp time"]
pub mod dcdc_start_time;
#[doc = "DCDC_RESUME_TIME register accessor: an alias for `Reg<DCDC_RESUME_TIME_SPEC>`"]
pub type DCDC_RESUME_TIME = crate::Reg<dcdc_resume_time::DCDC_RESUME_TIME_SPEC>;
#[doc = "DCDC resume time"]
pub mod dcdc_resume_time;
#[doc = "POWER_TRAP register accessor: an alias for `Reg<POWER_TRAP_SPEC>`"]
pub type POWER_TRAP = crate::Reg<power_trap::POWER_TRAP_SPEC>;
#[doc = "SOC power trap"]
pub mod power_trap;
#[doc = "WAKE_CAUSE register accessor: an alias for `Reg<WAKE_CAUSE_SPEC>`"]
pub type WAKE_CAUSE = crate::Reg<wake_cause::WAKE_CAUSE_SPEC>;
#[doc = "Wake up source"]
pub mod wake_cause;
#[doc = "WAKE_MASK register accessor: an alias for `Reg<WAKE_MASK_SPEC>`"]
pub type WAKE_MASK = crate::Reg<wake_mask::WAKE_MASK_SPEC>;
#[doc = "Wake up mask"]
pub mod wake_mask;
#[doc = "SCG_CTRL register accessor: an alias for `Reg<SCG_CTRL_SPEC>`"]
pub type SCG_CTRL = crate::Reg<scg_ctrl::SCG_CTRL_SPEC>;
#[doc = "Clock gate control in PMIC"]
pub mod scg_ctrl;
#[doc = "DEBUG_STOP register accessor: an alias for `Reg<DEBUG_STOP_SPEC>`"]
pub type DEBUG_STOP = crate::Reg<debug_stop::DEBUG_STOP_SPEC>;
#[doc = "Debug stop config"]
pub mod debug_stop;
#[doc = "RC24M register accessor: an alias for `Reg<RC24M_SPEC>`"]
pub type RC24M = crate::Reg<rc24m::RC24M_SPEC>;
#[doc = "RC 24M config"]
pub mod rc24m;
#[doc = "RC24M_TRACK register accessor: an alias for `Reg<RC24M_TRACK_SPEC>`"]
pub type RC24M_TRACK = crate::Reg<rc24m_track::RC24M_TRACK_SPEC>;
#[doc = "RC 24M track mode"]
pub mod rc24m_track;
#[doc = "TRACK_TARGET register accessor: an alias for `Reg<TRACK_TARGET_SPEC>`"]
pub type TRACK_TARGET = crate::Reg<track_target::TRACK_TARGET_SPEC>;
#[doc = "RC 24M track target"]
pub mod track_target;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "RC 24M track status"]
pub mod status;
