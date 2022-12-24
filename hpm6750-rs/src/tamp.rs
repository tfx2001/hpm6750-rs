#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Tamper n control"]
    pub tamp_tamp0_control: TAMP_TAMP0_CONTROL,
    #[doc = "0x04 - Tamper n Polynomial of LFSR"]
    pub tamp_tamp0_poly: TAMP_TAMP0_POLY,
    #[doc = "0x08 - Tamper n LFSR shift register"]
    pub tamp_tamp0_lfsr: TAMP_TAMP0_LFSR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Tamper1 control"]
    pub tamp_tamp1_control: TAMP_TAMP1_CONTROL,
    #[doc = "0x14 - Tamper1 Polynomial of LFSR"]
    pub tamp_tamp1_poly: TAMP_TAMP1_POLY,
    #[doc = "0x18 - Tamper1 LFSR shift register"]
    pub tamp_tamp1_lfsr: TAMP_TAMP1_LFSR,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Tamper2 control"]
    pub tamp_tamp2_control: TAMP_TAMP2_CONTROL,
    #[doc = "0x24 - Tamper2 Polynomial of LFSR"]
    pub tamp_tamp2_poly: TAMP_TAMP2_POLY,
    #[doc = "0x28 - Tamper2 LFSR shift register"]
    pub tamp_tamp2_lfsr: TAMP_TAMP2_LFSR,
    _reserved9: [u8; 0x04],
    #[doc = "0x30 - Tamper3 control"]
    pub tamp_tamp3_control: TAMP_TAMP3_CONTROL,
    #[doc = "0x34 - Tamper3 Polynomial of LFSR"]
    pub tamp_tamp3_poly: TAMP_TAMP3_POLY,
    #[doc = "0x38 - Tamper3 LFSR shift register"]
    pub tamp_tamp3_lfsr: TAMP_TAMP3_LFSR,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - Tamper4 control"]
    pub tamp_tamp4_control: TAMP_TAMP4_CONTROL,
    #[doc = "0x44 - Tamper4 Polynomial of LFSR"]
    pub tamp_tamp4_poly: TAMP_TAMP4_POLY,
    #[doc = "0x48 - Tamper4 LFSR shift register"]
    pub tamp_tamp4_lfsr: TAMP_TAMP4_LFSR,
    _reserved15: [u8; 0x04],
    #[doc = "0x50 - Tamper5 control"]
    pub tamp_tamp5_control: TAMP_TAMP5_CONTROL,
    #[doc = "0x54 - Tamper5 Polynomial of LFSR"]
    pub tamp_tamp5_poly: TAMP_TAMP5_POLY,
    #[doc = "0x58 - Tamper5 LFSR shift register"]
    pub tamp_tamp5_lfsr: TAMP_TAMP5_LFSR,
    _reserved18: [u8; 0x24],
    #[doc = "0x80 - Tamper flag"]
    pub tamp_flag: TAMP_FLAG,
    #[doc = "0x84 - Tamper interrupt enable"]
    pub irq_en: IRQ_EN,
}
#[doc = "TAMP_TAMP0_CONTROL (rw) register accessor: an alias for `Reg<TAMP_TAMP0_CONTROL_SPEC>`"]
pub type TAMP_TAMP0_CONTROL = crate::Reg<tamp_tamp0_control::TAMP_TAMP0_CONTROL_SPEC>;
#[doc = "Tamper n control"]
pub mod tamp_tamp0_control;
#[doc = "TAMP_TAMP0_POLY (rw) register accessor: an alias for `Reg<TAMP_TAMP0_POLY_SPEC>`"]
pub type TAMP_TAMP0_POLY = crate::Reg<tamp_tamp0_poly::TAMP_TAMP0_POLY_SPEC>;
#[doc = "Tamper n Polynomial of LFSR"]
pub mod tamp_tamp0_poly;
#[doc = "TAMP_TAMP0_LFSR (w) register accessor: an alias for `Reg<TAMP_TAMP0_LFSR_SPEC>`"]
pub type TAMP_TAMP0_LFSR = crate::Reg<tamp_tamp0_lfsr::TAMP_TAMP0_LFSR_SPEC>;
#[doc = "Tamper n LFSR shift register"]
pub mod tamp_tamp0_lfsr;
#[doc = "TAMP_TAMP1_CONTROL (rw) register accessor: an alias for `Reg<TAMP_TAMP1_CONTROL_SPEC>`"]
pub type TAMP_TAMP1_CONTROL = crate::Reg<tamp_tamp1_control::TAMP_TAMP1_CONTROL_SPEC>;
#[doc = "Tamper1 control"]
pub mod tamp_tamp1_control;
#[doc = "TAMP_TAMP1_POLY (rw) register accessor: an alias for `Reg<TAMP_TAMP1_POLY_SPEC>`"]
pub type TAMP_TAMP1_POLY = crate::Reg<tamp_tamp1_poly::TAMP_TAMP1_POLY_SPEC>;
#[doc = "Tamper1 Polynomial of LFSR"]
pub mod tamp_tamp1_poly;
#[doc = "TAMP_TAMP1_LFSR (w) register accessor: an alias for `Reg<TAMP_TAMP1_LFSR_SPEC>`"]
pub type TAMP_TAMP1_LFSR = crate::Reg<tamp_tamp1_lfsr::TAMP_TAMP1_LFSR_SPEC>;
#[doc = "Tamper1 LFSR shift register"]
pub mod tamp_tamp1_lfsr;
#[doc = "TAMP_TAMP2_CONTROL (rw) register accessor: an alias for `Reg<TAMP_TAMP2_CONTROL_SPEC>`"]
pub type TAMP_TAMP2_CONTROL = crate::Reg<tamp_tamp2_control::TAMP_TAMP2_CONTROL_SPEC>;
#[doc = "Tamper2 control"]
pub mod tamp_tamp2_control;
#[doc = "TAMP_TAMP2_POLY (rw) register accessor: an alias for `Reg<TAMP_TAMP2_POLY_SPEC>`"]
pub type TAMP_TAMP2_POLY = crate::Reg<tamp_tamp2_poly::TAMP_TAMP2_POLY_SPEC>;
#[doc = "Tamper2 Polynomial of LFSR"]
pub mod tamp_tamp2_poly;
#[doc = "TAMP_TAMP2_LFSR (w) register accessor: an alias for `Reg<TAMP_TAMP2_LFSR_SPEC>`"]
pub type TAMP_TAMP2_LFSR = crate::Reg<tamp_tamp2_lfsr::TAMP_TAMP2_LFSR_SPEC>;
#[doc = "Tamper2 LFSR shift register"]
pub mod tamp_tamp2_lfsr;
#[doc = "TAMP_TAMP3_CONTROL (rw) register accessor: an alias for `Reg<TAMP_TAMP3_CONTROL_SPEC>`"]
pub type TAMP_TAMP3_CONTROL = crate::Reg<tamp_tamp3_control::TAMP_TAMP3_CONTROL_SPEC>;
#[doc = "Tamper3 control"]
pub mod tamp_tamp3_control;
#[doc = "TAMP_TAMP3_POLY (rw) register accessor: an alias for `Reg<TAMP_TAMP3_POLY_SPEC>`"]
pub type TAMP_TAMP3_POLY = crate::Reg<tamp_tamp3_poly::TAMP_TAMP3_POLY_SPEC>;
#[doc = "Tamper3 Polynomial of LFSR"]
pub mod tamp_tamp3_poly;
#[doc = "TAMP_TAMP3_LFSR (w) register accessor: an alias for `Reg<TAMP_TAMP3_LFSR_SPEC>`"]
pub type TAMP_TAMP3_LFSR = crate::Reg<tamp_tamp3_lfsr::TAMP_TAMP3_LFSR_SPEC>;
#[doc = "Tamper3 LFSR shift register"]
pub mod tamp_tamp3_lfsr;
#[doc = "TAMP_TAMP4_CONTROL (rw) register accessor: an alias for `Reg<TAMP_TAMP4_CONTROL_SPEC>`"]
pub type TAMP_TAMP4_CONTROL = crate::Reg<tamp_tamp4_control::TAMP_TAMP4_CONTROL_SPEC>;
#[doc = "Tamper4 control"]
pub mod tamp_tamp4_control;
#[doc = "TAMP_TAMP4_POLY (rw) register accessor: an alias for `Reg<TAMP_TAMP4_POLY_SPEC>`"]
pub type TAMP_TAMP4_POLY = crate::Reg<tamp_tamp4_poly::TAMP_TAMP4_POLY_SPEC>;
#[doc = "Tamper4 Polynomial of LFSR"]
pub mod tamp_tamp4_poly;
#[doc = "TAMP_TAMP4_LFSR (w) register accessor: an alias for `Reg<TAMP_TAMP4_LFSR_SPEC>`"]
pub type TAMP_TAMP4_LFSR = crate::Reg<tamp_tamp4_lfsr::TAMP_TAMP4_LFSR_SPEC>;
#[doc = "Tamper4 LFSR shift register"]
pub mod tamp_tamp4_lfsr;
#[doc = "TAMP_TAMP5_CONTROL (rw) register accessor: an alias for `Reg<TAMP_TAMP5_CONTROL_SPEC>`"]
pub type TAMP_TAMP5_CONTROL = crate::Reg<tamp_tamp5_control::TAMP_TAMP5_CONTROL_SPEC>;
#[doc = "Tamper5 control"]
pub mod tamp_tamp5_control;
#[doc = "TAMP_TAMP5_POLY (rw) register accessor: an alias for `Reg<TAMP_TAMP5_POLY_SPEC>`"]
pub type TAMP_TAMP5_POLY = crate::Reg<tamp_tamp5_poly::TAMP_TAMP5_POLY_SPEC>;
#[doc = "Tamper5 Polynomial of LFSR"]
pub mod tamp_tamp5_poly;
#[doc = "TAMP_TAMP5_LFSR (w) register accessor: an alias for `Reg<TAMP_TAMP5_LFSR_SPEC>`"]
pub type TAMP_TAMP5_LFSR = crate::Reg<tamp_tamp5_lfsr::TAMP_TAMP5_LFSR_SPEC>;
#[doc = "Tamper5 LFSR shift register"]
pub mod tamp_tamp5_lfsr;
#[doc = "TAMP_FLAG (rw) register accessor: an alias for `Reg<TAMP_FLAG_SPEC>`"]
pub type TAMP_FLAG = crate::Reg<tamp_flag::TAMP_FLAG_SPEC>;
#[doc = "Tamper flag"]
pub mod tamp_flag;
#[doc = "IRQ_EN (rw) register accessor: an alias for `Reg<IRQ_EN_SPEC>`"]
pub type IRQ_EN = crate::Reg<irq_en::IRQ_EN_SPEC>;
#[doc = "Tamper interrupt enable"]
pub mod irq_en;
