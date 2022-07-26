#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub channel_ch0_cr: crate::Reg<channel_ch0_cr::CHANNEL_CH0_CR_SPEC>,
    #[doc = "0x04 - Comparator register 0"]
    pub channel_ch0_cmp0: crate::Reg<channel_ch0_cmp0::CHANNEL_CH0_CMP0_SPEC>,
    #[doc = "0x08 - Comparator register 1"]
    pub channel_ch0_cmp1: crate::Reg<channel_ch0_cmp1::CHANNEL_CH0_CMP1_SPEC>,
    #[doc = "0x0c - Reload register"]
    pub channel_ch0_rld: crate::Reg<channel_ch0_rld::CHANNEL_CH0_RLD_SPEC>,
    #[doc = "0x10 - Counter update value register"]
    pub channel_ch0_cntuptval: crate::Reg<channel_ch0_cntuptval::CHANNEL_CH0_CNTUPTVAL_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - Capture rising edge register"]
    pub channel_ch0_cappos: crate::Reg<channel_ch0_cappos::CHANNEL_CH0_CAPPOS_SPEC>,
    #[doc = "0x24 - Capture falling edge register"]
    pub channel_ch0_capneg: crate::Reg<channel_ch0_capneg::CHANNEL_CH0_CAPNEG_SPEC>,
    #[doc = "0x28 - PWM period measure register"]
    pub channel_ch0_capprd: crate::Reg<channel_ch0_capprd::CHANNEL_CH0_CAPPRD_SPEC>,
    #[doc = "0x2c - PWM duty cycle measure register"]
    pub channel_ch0_capdty: crate::Reg<channel_ch0_capdty::CHANNEL_CH0_CAPDTY_SPEC>,
    #[doc = "0x30 - Counter"]
    pub channel_ch0_cnt: crate::Reg<channel_ch0_cnt::CHANNEL_CH0_CNT_SPEC>,
    _reserved10: [u8; 0x0c],
    #[doc = "0x40 - Control Register"]
    pub channel_ch1_cr: crate::Reg<channel_ch1_cr::CHANNEL_CH1_CR_SPEC>,
    #[doc = "0x44 - Comparator register 0"]
    pub channel_ch1_cmp0: crate::Reg<channel_ch1_cmp0::CHANNEL_CH1_CMP0_SPEC>,
    #[doc = "0x48 - Comparator register 1"]
    pub channel_ch1_cmp1: crate::Reg<channel_ch1_cmp1::CHANNEL_CH1_CMP1_SPEC>,
    #[doc = "0x4c - Reload register"]
    pub channel_ch1_rld: crate::Reg<channel_ch1_rld::CHANNEL_CH1_RLD_SPEC>,
    #[doc = "0x50 - Counter update value register"]
    pub channel_ch1_cntuptval: crate::Reg<channel_ch1_cntuptval::CHANNEL_CH1_CNTUPTVAL_SPEC>,
    _reserved15: [u8; 0x0c],
    #[doc = "0x60 - Capture rising edge register"]
    pub channel_ch1_cappos: crate::Reg<channel_ch1_cappos::CHANNEL_CH1_CAPPOS_SPEC>,
    #[doc = "0x64 - Capture falling edge register"]
    pub channel_ch1_capneg: crate::Reg<channel_ch1_capneg::CHANNEL_CH1_CAPNEG_SPEC>,
    #[doc = "0x68 - PWM period measure register"]
    pub channel_ch1_capprd: crate::Reg<channel_ch1_capprd::CHANNEL_CH1_CAPPRD_SPEC>,
    #[doc = "0x6c - PWM duty cycle measure register"]
    pub channel_ch1_capdty: crate::Reg<channel_ch1_capdty::CHANNEL_CH1_CAPDTY_SPEC>,
    #[doc = "0x70 - Counter"]
    pub channel_ch1_cnt: crate::Reg<channel_ch1_cnt::CHANNEL_CH1_CNT_SPEC>,
    _reserved20: [u8; 0x0c],
    #[doc = "0x80 - Control Register"]
    pub channel_ch2_cr: crate::Reg<channel_ch2_cr::CHANNEL_CH2_CR_SPEC>,
    #[doc = "0x84 - Comparator register 0"]
    pub channel_ch2_cmp0: crate::Reg<channel_ch2_cmp0::CHANNEL_CH2_CMP0_SPEC>,
    #[doc = "0x88 - Comparator register 1"]
    pub channel_ch2_cmp1: crate::Reg<channel_ch2_cmp1::CHANNEL_CH2_CMP1_SPEC>,
    #[doc = "0x8c - Reload register"]
    pub channel_ch2_rld: crate::Reg<channel_ch2_rld::CHANNEL_CH2_RLD_SPEC>,
    #[doc = "0x90 - Counter update value register"]
    pub channel_ch2_cntuptval: crate::Reg<channel_ch2_cntuptval::CHANNEL_CH2_CNTUPTVAL_SPEC>,
    _reserved25: [u8; 0x0c],
    #[doc = "0xa0 - Capture rising edge register"]
    pub channel_ch2_cappos: crate::Reg<channel_ch2_cappos::CHANNEL_CH2_CAPPOS_SPEC>,
    #[doc = "0xa4 - Capture falling edge register"]
    pub channel_ch2_capneg: crate::Reg<channel_ch2_capneg::CHANNEL_CH2_CAPNEG_SPEC>,
    #[doc = "0xa8 - PWM period measure register"]
    pub channel_ch2_capprd: crate::Reg<channel_ch2_capprd::CHANNEL_CH2_CAPPRD_SPEC>,
    #[doc = "0xac - PWM duty cycle measure register"]
    pub channel_ch2_capdty: crate::Reg<channel_ch2_capdty::CHANNEL_CH2_CAPDTY_SPEC>,
    #[doc = "0xb0 - Counter"]
    pub channel_ch2_cnt: crate::Reg<channel_ch2_cnt::CHANNEL_CH2_CNT_SPEC>,
    _reserved30: [u8; 0x0c],
    #[doc = "0xc0 - Control Register"]
    pub channel_ch3_cr: crate::Reg<channel_ch3_cr::CHANNEL_CH3_CR_SPEC>,
    #[doc = "0xc4 - Comparator register 0"]
    pub channel_ch3_cmp0: crate::Reg<channel_ch3_cmp0::CHANNEL_CH3_CMP0_SPEC>,
    #[doc = "0xc8 - Comparator register 1"]
    pub channel_ch3_cmp1: crate::Reg<channel_ch3_cmp1::CHANNEL_CH3_CMP1_SPEC>,
    #[doc = "0xcc - Reload register"]
    pub channel_ch3_rld: crate::Reg<channel_ch3_rld::CHANNEL_CH3_RLD_SPEC>,
    #[doc = "0xd0 - Counter update value register"]
    pub channel_ch3_cntuptval: crate::Reg<channel_ch3_cntuptval::CHANNEL_CH3_CNTUPTVAL_SPEC>,
    _reserved35: [u8; 0x0c],
    #[doc = "0xe0 - Capture rising edge register"]
    pub channel_ch3_cappos: crate::Reg<channel_ch3_cappos::CHANNEL_CH3_CAPPOS_SPEC>,
    #[doc = "0xe4 - Capture falling edge register"]
    pub channel_ch3_capneg: crate::Reg<channel_ch3_capneg::CHANNEL_CH3_CAPNEG_SPEC>,
    #[doc = "0xe8 - PWM period measure register"]
    pub channel_ch3_capprd: crate::Reg<channel_ch3_capprd::CHANNEL_CH3_CAPPRD_SPEC>,
    #[doc = "0xec - PWM duty cycle measure register"]
    pub channel_ch3_capdty: crate::Reg<channel_ch3_capdty::CHANNEL_CH3_CAPDTY_SPEC>,
    #[doc = "0xf0 - Counter"]
    pub channel_ch3_cnt: crate::Reg<channel_ch3_cnt::CHANNEL_CH3_CNT_SPEC>,
    _reserved40: [u8; 0x010c],
    #[doc = "0x200 - Status register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x204 - Interrupt request enable register"]
    pub irqen: crate::Reg<irqen::IRQEN_SPEC>,
    #[doc = "0x208 - Global control register"]
    pub gcr: crate::Reg<gcr::GCR_SPEC>,
}
#[doc = "CHANNEL_CH0_CR register accessor: an alias for `Reg<CHANNEL_CH0_CR_SPEC>`"]
pub type CHANNEL_CH0_CR = crate::Reg<channel_ch0_cr::CHANNEL_CH0_CR_SPEC>;
#[doc = "Control Register"]
pub mod channel_ch0_cr;
#[doc = "CHANNEL_CH0_CMP0 register accessor: an alias for `Reg<CHANNEL_CH0_CMP0_SPEC>`"]
pub type CHANNEL_CH0_CMP0 = crate::Reg<channel_ch0_cmp0::CHANNEL_CH0_CMP0_SPEC>;
#[doc = "Comparator register 0"]
pub mod channel_ch0_cmp0;
#[doc = "CHANNEL_CH0_CMP1 register accessor: an alias for `Reg<CHANNEL_CH0_CMP1_SPEC>`"]
pub type CHANNEL_CH0_CMP1 = crate::Reg<channel_ch0_cmp1::CHANNEL_CH0_CMP1_SPEC>;
#[doc = "Comparator register 1"]
pub mod channel_ch0_cmp1;
#[doc = "CHANNEL_CH0_RLD register accessor: an alias for `Reg<CHANNEL_CH0_RLD_SPEC>`"]
pub type CHANNEL_CH0_RLD = crate::Reg<channel_ch0_rld::CHANNEL_CH0_RLD_SPEC>;
#[doc = "Reload register"]
pub mod channel_ch0_rld;
#[doc = "CHANNEL_CH0_CNTUPTVAL register accessor: an alias for `Reg<CHANNEL_CH0_CNTUPTVAL_SPEC>`"]
pub type CHANNEL_CH0_CNTUPTVAL = crate::Reg<channel_ch0_cntuptval::CHANNEL_CH0_CNTUPTVAL_SPEC>;
#[doc = "Counter update value register"]
pub mod channel_ch0_cntuptval;
#[doc = "CHANNEL_CH0_CAPPOS register accessor: an alias for `Reg<CHANNEL_CH0_CAPPOS_SPEC>`"]
pub type CHANNEL_CH0_CAPPOS = crate::Reg<channel_ch0_cappos::CHANNEL_CH0_CAPPOS_SPEC>;
#[doc = "Capture rising edge register"]
pub mod channel_ch0_cappos;
#[doc = "CHANNEL_CH0_CAPNEG register accessor: an alias for `Reg<CHANNEL_CH0_CAPNEG_SPEC>`"]
pub type CHANNEL_CH0_CAPNEG = crate::Reg<channel_ch0_capneg::CHANNEL_CH0_CAPNEG_SPEC>;
#[doc = "Capture falling edge register"]
pub mod channel_ch0_capneg;
#[doc = "CHANNEL_CH0_CAPPRD register accessor: an alias for `Reg<CHANNEL_CH0_CAPPRD_SPEC>`"]
pub type CHANNEL_CH0_CAPPRD = crate::Reg<channel_ch0_capprd::CHANNEL_CH0_CAPPRD_SPEC>;
#[doc = "PWM period measure register"]
pub mod channel_ch0_capprd;
#[doc = "CHANNEL_CH0_CAPDTY register accessor: an alias for `Reg<CHANNEL_CH0_CAPDTY_SPEC>`"]
pub type CHANNEL_CH0_CAPDTY = crate::Reg<channel_ch0_capdty::CHANNEL_CH0_CAPDTY_SPEC>;
#[doc = "PWM duty cycle measure register"]
pub mod channel_ch0_capdty;
#[doc = "CHANNEL_CH0_CNT register accessor: an alias for `Reg<CHANNEL_CH0_CNT_SPEC>`"]
pub type CHANNEL_CH0_CNT = crate::Reg<channel_ch0_cnt::CHANNEL_CH0_CNT_SPEC>;
#[doc = "Counter"]
pub mod channel_ch0_cnt;
#[doc = "CHANNEL_CH1_CR register accessor: an alias for `Reg<CHANNEL_CH1_CR_SPEC>`"]
pub type CHANNEL_CH1_CR = crate::Reg<channel_ch1_cr::CHANNEL_CH1_CR_SPEC>;
#[doc = "Control Register"]
pub mod channel_ch1_cr;
#[doc = "CHANNEL_CH1_CMP0 register accessor: an alias for `Reg<CHANNEL_CH1_CMP0_SPEC>`"]
pub type CHANNEL_CH1_CMP0 = crate::Reg<channel_ch1_cmp0::CHANNEL_CH1_CMP0_SPEC>;
#[doc = "Comparator register 0"]
pub mod channel_ch1_cmp0;
#[doc = "CHANNEL_CH1_CMP1 register accessor: an alias for `Reg<CHANNEL_CH1_CMP1_SPEC>`"]
pub type CHANNEL_CH1_CMP1 = crate::Reg<channel_ch1_cmp1::CHANNEL_CH1_CMP1_SPEC>;
#[doc = "Comparator register 1"]
pub mod channel_ch1_cmp1;
#[doc = "CHANNEL_CH1_RLD register accessor: an alias for `Reg<CHANNEL_CH1_RLD_SPEC>`"]
pub type CHANNEL_CH1_RLD = crate::Reg<channel_ch1_rld::CHANNEL_CH1_RLD_SPEC>;
#[doc = "Reload register"]
pub mod channel_ch1_rld;
#[doc = "CHANNEL_CH1_CNTUPTVAL register accessor: an alias for `Reg<CHANNEL_CH1_CNTUPTVAL_SPEC>`"]
pub type CHANNEL_CH1_CNTUPTVAL = crate::Reg<channel_ch1_cntuptval::CHANNEL_CH1_CNTUPTVAL_SPEC>;
#[doc = "Counter update value register"]
pub mod channel_ch1_cntuptval;
#[doc = "CHANNEL_CH1_CAPPOS register accessor: an alias for `Reg<CHANNEL_CH1_CAPPOS_SPEC>`"]
pub type CHANNEL_CH1_CAPPOS = crate::Reg<channel_ch1_cappos::CHANNEL_CH1_CAPPOS_SPEC>;
#[doc = "Capture rising edge register"]
pub mod channel_ch1_cappos;
#[doc = "CHANNEL_CH1_CAPNEG register accessor: an alias for `Reg<CHANNEL_CH1_CAPNEG_SPEC>`"]
pub type CHANNEL_CH1_CAPNEG = crate::Reg<channel_ch1_capneg::CHANNEL_CH1_CAPNEG_SPEC>;
#[doc = "Capture falling edge register"]
pub mod channel_ch1_capneg;
#[doc = "CHANNEL_CH1_CAPPRD register accessor: an alias for `Reg<CHANNEL_CH1_CAPPRD_SPEC>`"]
pub type CHANNEL_CH1_CAPPRD = crate::Reg<channel_ch1_capprd::CHANNEL_CH1_CAPPRD_SPEC>;
#[doc = "PWM period measure register"]
pub mod channel_ch1_capprd;
#[doc = "CHANNEL_CH1_CAPDTY register accessor: an alias for `Reg<CHANNEL_CH1_CAPDTY_SPEC>`"]
pub type CHANNEL_CH1_CAPDTY = crate::Reg<channel_ch1_capdty::CHANNEL_CH1_CAPDTY_SPEC>;
#[doc = "PWM duty cycle measure register"]
pub mod channel_ch1_capdty;
#[doc = "CHANNEL_CH1_CNT register accessor: an alias for `Reg<CHANNEL_CH1_CNT_SPEC>`"]
pub type CHANNEL_CH1_CNT = crate::Reg<channel_ch1_cnt::CHANNEL_CH1_CNT_SPEC>;
#[doc = "Counter"]
pub mod channel_ch1_cnt;
#[doc = "CHANNEL_CH2_CR register accessor: an alias for `Reg<CHANNEL_CH2_CR_SPEC>`"]
pub type CHANNEL_CH2_CR = crate::Reg<channel_ch2_cr::CHANNEL_CH2_CR_SPEC>;
#[doc = "Control Register"]
pub mod channel_ch2_cr;
#[doc = "CHANNEL_CH2_CMP0 register accessor: an alias for `Reg<CHANNEL_CH2_CMP0_SPEC>`"]
pub type CHANNEL_CH2_CMP0 = crate::Reg<channel_ch2_cmp0::CHANNEL_CH2_CMP0_SPEC>;
#[doc = "Comparator register 0"]
pub mod channel_ch2_cmp0;
#[doc = "CHANNEL_CH2_CMP1 register accessor: an alias for `Reg<CHANNEL_CH2_CMP1_SPEC>`"]
pub type CHANNEL_CH2_CMP1 = crate::Reg<channel_ch2_cmp1::CHANNEL_CH2_CMP1_SPEC>;
#[doc = "Comparator register 1"]
pub mod channel_ch2_cmp1;
#[doc = "CHANNEL_CH2_RLD register accessor: an alias for `Reg<CHANNEL_CH2_RLD_SPEC>`"]
pub type CHANNEL_CH2_RLD = crate::Reg<channel_ch2_rld::CHANNEL_CH2_RLD_SPEC>;
#[doc = "Reload register"]
pub mod channel_ch2_rld;
#[doc = "CHANNEL_CH2_CNTUPTVAL register accessor: an alias for `Reg<CHANNEL_CH2_CNTUPTVAL_SPEC>`"]
pub type CHANNEL_CH2_CNTUPTVAL = crate::Reg<channel_ch2_cntuptval::CHANNEL_CH2_CNTUPTVAL_SPEC>;
#[doc = "Counter update value register"]
pub mod channel_ch2_cntuptval;
#[doc = "CHANNEL_CH2_CAPPOS register accessor: an alias for `Reg<CHANNEL_CH2_CAPPOS_SPEC>`"]
pub type CHANNEL_CH2_CAPPOS = crate::Reg<channel_ch2_cappos::CHANNEL_CH2_CAPPOS_SPEC>;
#[doc = "Capture rising edge register"]
pub mod channel_ch2_cappos;
#[doc = "CHANNEL_CH2_CAPNEG register accessor: an alias for `Reg<CHANNEL_CH2_CAPNEG_SPEC>`"]
pub type CHANNEL_CH2_CAPNEG = crate::Reg<channel_ch2_capneg::CHANNEL_CH2_CAPNEG_SPEC>;
#[doc = "Capture falling edge register"]
pub mod channel_ch2_capneg;
#[doc = "CHANNEL_CH2_CAPPRD register accessor: an alias for `Reg<CHANNEL_CH2_CAPPRD_SPEC>`"]
pub type CHANNEL_CH2_CAPPRD = crate::Reg<channel_ch2_capprd::CHANNEL_CH2_CAPPRD_SPEC>;
#[doc = "PWM period measure register"]
pub mod channel_ch2_capprd;
#[doc = "CHANNEL_CH2_CAPDTY register accessor: an alias for `Reg<CHANNEL_CH2_CAPDTY_SPEC>`"]
pub type CHANNEL_CH2_CAPDTY = crate::Reg<channel_ch2_capdty::CHANNEL_CH2_CAPDTY_SPEC>;
#[doc = "PWM duty cycle measure register"]
pub mod channel_ch2_capdty;
#[doc = "CHANNEL_CH2_CNT register accessor: an alias for `Reg<CHANNEL_CH2_CNT_SPEC>`"]
pub type CHANNEL_CH2_CNT = crate::Reg<channel_ch2_cnt::CHANNEL_CH2_CNT_SPEC>;
#[doc = "Counter"]
pub mod channel_ch2_cnt;
#[doc = "CHANNEL_CH3_CR register accessor: an alias for `Reg<CHANNEL_CH3_CR_SPEC>`"]
pub type CHANNEL_CH3_CR = crate::Reg<channel_ch3_cr::CHANNEL_CH3_CR_SPEC>;
#[doc = "Control Register"]
pub mod channel_ch3_cr;
#[doc = "CHANNEL_CH3_CMP0 register accessor: an alias for `Reg<CHANNEL_CH3_CMP0_SPEC>`"]
pub type CHANNEL_CH3_CMP0 = crate::Reg<channel_ch3_cmp0::CHANNEL_CH3_CMP0_SPEC>;
#[doc = "Comparator register 0"]
pub mod channel_ch3_cmp0;
#[doc = "CHANNEL_CH3_CMP1 register accessor: an alias for `Reg<CHANNEL_CH3_CMP1_SPEC>`"]
pub type CHANNEL_CH3_CMP1 = crate::Reg<channel_ch3_cmp1::CHANNEL_CH3_CMP1_SPEC>;
#[doc = "Comparator register 1"]
pub mod channel_ch3_cmp1;
#[doc = "CHANNEL_CH3_RLD register accessor: an alias for `Reg<CHANNEL_CH3_RLD_SPEC>`"]
pub type CHANNEL_CH3_RLD = crate::Reg<channel_ch3_rld::CHANNEL_CH3_RLD_SPEC>;
#[doc = "Reload register"]
pub mod channel_ch3_rld;
#[doc = "CHANNEL_CH3_CNTUPTVAL register accessor: an alias for `Reg<CHANNEL_CH3_CNTUPTVAL_SPEC>`"]
pub type CHANNEL_CH3_CNTUPTVAL = crate::Reg<channel_ch3_cntuptval::CHANNEL_CH3_CNTUPTVAL_SPEC>;
#[doc = "Counter update value register"]
pub mod channel_ch3_cntuptval;
#[doc = "CHANNEL_CH3_CAPPOS register accessor: an alias for `Reg<CHANNEL_CH3_CAPPOS_SPEC>`"]
pub type CHANNEL_CH3_CAPPOS = crate::Reg<channel_ch3_cappos::CHANNEL_CH3_CAPPOS_SPEC>;
#[doc = "Capture rising edge register"]
pub mod channel_ch3_cappos;
#[doc = "CHANNEL_CH3_CAPNEG register accessor: an alias for `Reg<CHANNEL_CH3_CAPNEG_SPEC>`"]
pub type CHANNEL_CH3_CAPNEG = crate::Reg<channel_ch3_capneg::CHANNEL_CH3_CAPNEG_SPEC>;
#[doc = "Capture falling edge register"]
pub mod channel_ch3_capneg;
#[doc = "CHANNEL_CH3_CAPPRD register accessor: an alias for `Reg<CHANNEL_CH3_CAPPRD_SPEC>`"]
pub type CHANNEL_CH3_CAPPRD = crate::Reg<channel_ch3_capprd::CHANNEL_CH3_CAPPRD_SPEC>;
#[doc = "PWM period measure register"]
pub mod channel_ch3_capprd;
#[doc = "CHANNEL_CH3_CAPDTY register accessor: an alias for `Reg<CHANNEL_CH3_CAPDTY_SPEC>`"]
pub type CHANNEL_CH3_CAPDTY = crate::Reg<channel_ch3_capdty::CHANNEL_CH3_CAPDTY_SPEC>;
#[doc = "PWM duty cycle measure register"]
pub mod channel_ch3_capdty;
#[doc = "CHANNEL_CH3_CNT register accessor: an alias for `Reg<CHANNEL_CH3_CNT_SPEC>`"]
pub type CHANNEL_CH3_CNT = crate::Reg<channel_ch3_cnt::CHANNEL_CH3_CNT_SPEC>;
#[doc = "Counter"]
pub mod channel_ch3_cnt;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "IRQEN register accessor: an alias for `Reg<IRQEN_SPEC>`"]
pub type IRQEN = crate::Reg<irqen::IRQEN_SPEC>;
#[doc = "Interrupt request enable register"]
pub mod irqen;
#[doc = "GCR register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "Global control register"]
pub mod gcr;
