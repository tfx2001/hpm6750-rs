#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Second counter"]
    pub second: crate::Reg<second::SECOND_SPEC>,
    #[doc = "0x04 - Sub-second counter"]
    pub subsec: crate::Reg<subsec::SUBSEC_SPEC>,
    #[doc = "0x08 - Second counter snap shot"]
    pub sec_snap: crate::Reg<sec_snap::SEC_SNAP_SPEC>,
    #[doc = "0x0c - Sub-second counter snap shot"]
    pub sub_snap: crate::Reg<sub_snap::SUB_SNAP_SPEC>,
    #[doc = "0x10 - RTC alarm0"]
    pub alarm0: crate::Reg<alarm0::ALARM0_SPEC>,
    #[doc = "0x14 - Alarm0 incremental"]
    pub alarm0_inc: crate::Reg<alarm0_inc::ALARM0_INC_SPEC>,
    #[doc = "0x18 - RTC alarm1"]
    pub alarm1: crate::Reg<alarm1::ALARM1_SPEC>,
    #[doc = "0x1c - Alarm1 incremental"]
    pub alarm1_inc: crate::Reg<alarm1_inc::ALARM1_INC_SPEC>,
    #[doc = "0x20 - RTC alarm flag"]
    pub alarm_flag: crate::Reg<alarm_flag::ALARM_FLAG_SPEC>,
    #[doc = "0x24 - RTC alarm enable"]
    pub alarm_en: crate::Reg<alarm_en::ALARM_EN_SPEC>,
}
#[doc = "SECOND register accessor: an alias for `Reg<SECOND_SPEC>`"]
pub type SECOND = crate::Reg<second::SECOND_SPEC>;
#[doc = "Second counter"]
pub mod second;
#[doc = "SUBSEC register accessor: an alias for `Reg<SUBSEC_SPEC>`"]
pub type SUBSEC = crate::Reg<subsec::SUBSEC_SPEC>;
#[doc = "Sub-second counter"]
pub mod subsec;
#[doc = "SEC_SNAP register accessor: an alias for `Reg<SEC_SNAP_SPEC>`"]
pub type SEC_SNAP = crate::Reg<sec_snap::SEC_SNAP_SPEC>;
#[doc = "Second counter snap shot"]
pub mod sec_snap;
#[doc = "SUB_SNAP register accessor: an alias for `Reg<SUB_SNAP_SPEC>`"]
pub type SUB_SNAP = crate::Reg<sub_snap::SUB_SNAP_SPEC>;
#[doc = "Sub-second counter snap shot"]
pub mod sub_snap;
#[doc = "ALARM0 register accessor: an alias for `Reg<ALARM0_SPEC>`"]
pub type ALARM0 = crate::Reg<alarm0::ALARM0_SPEC>;
#[doc = "RTC alarm0"]
pub mod alarm0;
#[doc = "ALARM0_INC register accessor: an alias for `Reg<ALARM0_INC_SPEC>`"]
pub type ALARM0_INC = crate::Reg<alarm0_inc::ALARM0_INC_SPEC>;
#[doc = "Alarm0 incremental"]
pub mod alarm0_inc;
#[doc = "ALARM1 register accessor: an alias for `Reg<ALARM1_SPEC>`"]
pub type ALARM1 = crate::Reg<alarm1::ALARM1_SPEC>;
#[doc = "RTC alarm1"]
pub mod alarm1;
#[doc = "ALARM1_INC register accessor: an alias for `Reg<ALARM1_INC_SPEC>`"]
pub type ALARM1_INC = crate::Reg<alarm1_inc::ALARM1_INC_SPEC>;
#[doc = "Alarm1 incremental"]
pub mod alarm1_inc;
#[doc = "ALARM_FLAG register accessor: an alias for `Reg<ALARM_FLAG_SPEC>`"]
pub type ALARM_FLAG = crate::Reg<alarm_flag::ALARM_FLAG_SPEC>;
#[doc = "RTC alarm flag"]
pub mod alarm_flag;
#[doc = "ALARM_EN register accessor: an alias for `Reg<ALARM_EN_SPEC>`"]
pub type ALARM_EN = crate::Reg<alarm_en::ALARM_EN_SPEC>;
#[doc = "RTC alarm enable"]
pub mod alarm_en;
