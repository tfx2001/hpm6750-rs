#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - DMAC Configuration Register"]
    pub dmacfg: crate::Reg<dmacfg::DMACFG_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x20 - DMAC Control Register"]
    pub dmactrl: crate::Reg<dmactrl::DMACTRL_SPEC>,
    #[doc = "0x24 - Channel Abort Register"]
    pub chabort: crate::Reg<chabort::CHABORT_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x30 - Interrupt Status Register"]
    pub intstatus: crate::Reg<intstatus::INTSTATUS_SPEC>,
    #[doc = "0x34 - Channel Enable Register"]
    pub chen: crate::Reg<chen::CHEN_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x40 - Channel n Control Register"]
    pub chctrl_ch0_ctrl: crate::Reg<chctrl_ch0_ctrl::CHCTRL_CH0_CTRL_SPEC>,
    #[doc = "0x44 - Channel n Transfer Size Register"]
    pub chctrl_ch0_transize: crate::Reg<chctrl_ch0_transize::CHCTRL_CH0_TRANSIZE_SPEC>,
    #[doc = "0x48 - Channel n Source Address Low Part Register"]
    pub chctrl_ch0_srcaddr: crate::Reg<chctrl_ch0_srcaddr::CHCTRL_CH0_SRCADDR_SPEC>,
    #[doc = "0x4c - Channel n Source Address High Part Register"]
    pub chctrl_ch0_srcaddrh: crate::Reg<chctrl_ch0_srcaddrh::CHCTRL_CH0_SRCADDRH_SPEC>,
    #[doc = "0x50 - Channel n Destination Address Low Part Register"]
    pub chctrl_ch0_dstaddr: crate::Reg<chctrl_ch0_dstaddr::CHCTRL_CH0_DSTADDR_SPEC>,
    #[doc = "0x54 - Channel n Destination Address High Part Register"]
    pub chctrl_ch0_dstaddrh: crate::Reg<chctrl_ch0_dstaddrh::CHCTRL_CH0_DSTADDRH_SPEC>,
    #[doc = "0x58 - Channel n Linked List Pointer Low Part Register"]
    pub chctrl_ch0_llpointer: crate::Reg<chctrl_ch0_llpointer::CHCTRL_CH0_LLPOINTER_SPEC>,
    #[doc = "0x5c - Channel n Linked List Pointer High Part Register"]
    pub chctrl_ch0_llpointerh: crate::Reg<chctrl_ch0_llpointerh::CHCTRL_CH0_LLPOINTERH_SPEC>,
    #[doc = "0x60 - Channel n Control Register"]
    pub chctrl_ch1_ctrl: crate::Reg<chctrl_ch1_ctrl::CHCTRL_CH1_CTRL_SPEC>,
    #[doc = "0x64 - Channel n Transfer Size Register"]
    pub chctrl_ch1_transize: crate::Reg<chctrl_ch1_transize::CHCTRL_CH1_TRANSIZE_SPEC>,
    #[doc = "0x68 - Channel n Source Address Low Part Register"]
    pub chctrl_ch1_srcaddr: crate::Reg<chctrl_ch1_srcaddr::CHCTRL_CH1_SRCADDR_SPEC>,
    #[doc = "0x6c - Channel n Source Address High Part Register"]
    pub chctrl_ch1_srcaddrh: crate::Reg<chctrl_ch1_srcaddrh::CHCTRL_CH1_SRCADDRH_SPEC>,
    #[doc = "0x70 - Channel n Destination Address Low Part Register"]
    pub chctrl_ch1_dstaddr: crate::Reg<chctrl_ch1_dstaddr::CHCTRL_CH1_DSTADDR_SPEC>,
    #[doc = "0x74 - Channel n Destination Address High Part Register"]
    pub chctrl_ch1_dstaddrh: crate::Reg<chctrl_ch1_dstaddrh::CHCTRL_CH1_DSTADDRH_SPEC>,
    #[doc = "0x78 - Channel n Linked List Pointer Low Part Register"]
    pub chctrl_ch1_llpointer: crate::Reg<chctrl_ch1_llpointer::CHCTRL_CH1_LLPOINTER_SPEC>,
    #[doc = "0x7c - Channel n Linked List Pointer High Part Register"]
    pub chctrl_ch1_llpointerh: crate::Reg<chctrl_ch1_llpointerh::CHCTRL_CH1_LLPOINTERH_SPEC>,
    #[doc = "0x80 - Channel n Control Register"]
    pub chctrl_ch2_ctrl: crate::Reg<chctrl_ch2_ctrl::CHCTRL_CH2_CTRL_SPEC>,
    #[doc = "0x84 - Channel n Transfer Size Register"]
    pub chctrl_ch2_transize: crate::Reg<chctrl_ch2_transize::CHCTRL_CH2_TRANSIZE_SPEC>,
    #[doc = "0x88 - Channel n Source Address Low Part Register"]
    pub chctrl_ch2_srcaddr: crate::Reg<chctrl_ch2_srcaddr::CHCTRL_CH2_SRCADDR_SPEC>,
    #[doc = "0x8c - Channel n Source Address High Part Register"]
    pub chctrl_ch2_srcaddrh: crate::Reg<chctrl_ch2_srcaddrh::CHCTRL_CH2_SRCADDRH_SPEC>,
    #[doc = "0x90 - Channel n Destination Address Low Part Register"]
    pub chctrl_ch2_dstaddr: crate::Reg<chctrl_ch2_dstaddr::CHCTRL_CH2_DSTADDR_SPEC>,
    #[doc = "0x94 - Channel n Destination Address High Part Register"]
    pub chctrl_ch2_dstaddrh: crate::Reg<chctrl_ch2_dstaddrh::CHCTRL_CH2_DSTADDRH_SPEC>,
    #[doc = "0x98 - Channel n Linked List Pointer Low Part Register"]
    pub chctrl_ch2_llpointer: crate::Reg<chctrl_ch2_llpointer::CHCTRL_CH2_LLPOINTER_SPEC>,
    #[doc = "0x9c - Channel n Linked List Pointer High Part Register"]
    pub chctrl_ch2_llpointerh: crate::Reg<chctrl_ch2_llpointerh::CHCTRL_CH2_LLPOINTERH_SPEC>,
    #[doc = "0xa0 - Channel n Control Register"]
    pub chctrl_ch3_ctrl: crate::Reg<chctrl_ch3_ctrl::CHCTRL_CH3_CTRL_SPEC>,
    #[doc = "0xa4 - Channel n Transfer Size Register"]
    pub chctrl_ch3_transize: crate::Reg<chctrl_ch3_transize::CHCTRL_CH3_TRANSIZE_SPEC>,
    #[doc = "0xa8 - Channel n Source Address Low Part Register"]
    pub chctrl_ch3_srcaddr: crate::Reg<chctrl_ch3_srcaddr::CHCTRL_CH3_SRCADDR_SPEC>,
    #[doc = "0xac - Channel n Source Address High Part Register"]
    pub chctrl_ch3_srcaddrh: crate::Reg<chctrl_ch3_srcaddrh::CHCTRL_CH3_SRCADDRH_SPEC>,
    #[doc = "0xb0 - Channel n Destination Address Low Part Register"]
    pub chctrl_ch3_dstaddr: crate::Reg<chctrl_ch3_dstaddr::CHCTRL_CH3_DSTADDR_SPEC>,
    #[doc = "0xb4 - Channel n Destination Address High Part Register"]
    pub chctrl_ch3_dstaddrh: crate::Reg<chctrl_ch3_dstaddrh::CHCTRL_CH3_DSTADDRH_SPEC>,
    #[doc = "0xb8 - Channel n Linked List Pointer Low Part Register"]
    pub chctrl_ch3_llpointer: crate::Reg<chctrl_ch3_llpointer::CHCTRL_CH3_LLPOINTER_SPEC>,
    #[doc = "0xbc - Channel n Linked List Pointer High Part Register"]
    pub chctrl_ch3_llpointerh: crate::Reg<chctrl_ch3_llpointerh::CHCTRL_CH3_LLPOINTERH_SPEC>,
    #[doc = "0xc0 - Channel n Control Register"]
    pub chctrl_ch4_ctrl: crate::Reg<chctrl_ch4_ctrl::CHCTRL_CH4_CTRL_SPEC>,
    #[doc = "0xc4 - Channel n Transfer Size Register"]
    pub chctrl_ch4_transize: crate::Reg<chctrl_ch4_transize::CHCTRL_CH4_TRANSIZE_SPEC>,
    #[doc = "0xc8 - Channel n Source Address Low Part Register"]
    pub chctrl_ch4_srcaddr: crate::Reg<chctrl_ch4_srcaddr::CHCTRL_CH4_SRCADDR_SPEC>,
    #[doc = "0xcc - Channel n Source Address High Part Register"]
    pub chctrl_ch4_srcaddrh: crate::Reg<chctrl_ch4_srcaddrh::CHCTRL_CH4_SRCADDRH_SPEC>,
    #[doc = "0xd0 - Channel n Destination Address Low Part Register"]
    pub chctrl_ch4_dstaddr: crate::Reg<chctrl_ch4_dstaddr::CHCTRL_CH4_DSTADDR_SPEC>,
    #[doc = "0xd4 - Channel n Destination Address High Part Register"]
    pub chctrl_ch4_dstaddrh: crate::Reg<chctrl_ch4_dstaddrh::CHCTRL_CH4_DSTADDRH_SPEC>,
    #[doc = "0xd8 - Channel n Linked List Pointer Low Part Register"]
    pub chctrl_ch4_llpointer: crate::Reg<chctrl_ch4_llpointer::CHCTRL_CH4_LLPOINTER_SPEC>,
    #[doc = "0xdc - Channel n Linked List Pointer High Part Register"]
    pub chctrl_ch4_llpointerh: crate::Reg<chctrl_ch4_llpointerh::CHCTRL_CH4_LLPOINTERH_SPEC>,
    #[doc = "0xe0 - Channel n Control Register"]
    pub chctrl_ch5_ctrl: crate::Reg<chctrl_ch5_ctrl::CHCTRL_CH5_CTRL_SPEC>,
    #[doc = "0xe4 - Channel n Transfer Size Register"]
    pub chctrl_ch5_transize: crate::Reg<chctrl_ch5_transize::CHCTRL_CH5_TRANSIZE_SPEC>,
    #[doc = "0xe8 - Channel n Source Address Low Part Register"]
    pub chctrl_ch5_srcaddr: crate::Reg<chctrl_ch5_srcaddr::CHCTRL_CH5_SRCADDR_SPEC>,
    #[doc = "0xec - Channel n Source Address High Part Register"]
    pub chctrl_ch5_srcaddrh: crate::Reg<chctrl_ch5_srcaddrh::CHCTRL_CH5_SRCADDRH_SPEC>,
    #[doc = "0xf0 - Channel n Destination Address Low Part Register"]
    pub chctrl_ch5_dstaddr: crate::Reg<chctrl_ch5_dstaddr::CHCTRL_CH5_DSTADDR_SPEC>,
    #[doc = "0xf4 - Channel n Destination Address High Part Register"]
    pub chctrl_ch5_dstaddrh: crate::Reg<chctrl_ch5_dstaddrh::CHCTRL_CH5_DSTADDRH_SPEC>,
    #[doc = "0xf8 - Channel n Linked List Pointer Low Part Register"]
    pub chctrl_ch5_llpointer: crate::Reg<chctrl_ch5_llpointer::CHCTRL_CH5_LLPOINTER_SPEC>,
    #[doc = "0xfc - Channel n Linked List Pointer High Part Register"]
    pub chctrl_ch5_llpointerh: crate::Reg<chctrl_ch5_llpointerh::CHCTRL_CH5_LLPOINTERH_SPEC>,
    #[doc = "0x100 - Channel n Control Register"]
    pub chctrl_ch6_ctrl: crate::Reg<chctrl_ch6_ctrl::CHCTRL_CH6_CTRL_SPEC>,
    #[doc = "0x104 - Channel n Transfer Size Register"]
    pub chctrl_ch6_transize: crate::Reg<chctrl_ch6_transize::CHCTRL_CH6_TRANSIZE_SPEC>,
    #[doc = "0x108 - Channel n Source Address Low Part Register"]
    pub chctrl_ch6_srcaddr: crate::Reg<chctrl_ch6_srcaddr::CHCTRL_CH6_SRCADDR_SPEC>,
    #[doc = "0x10c - Channel n Source Address High Part Register"]
    pub chctrl_ch6_srcaddrh: crate::Reg<chctrl_ch6_srcaddrh::CHCTRL_CH6_SRCADDRH_SPEC>,
    #[doc = "0x110 - Channel n Destination Address Low Part Register"]
    pub chctrl_ch6_dstaddr: crate::Reg<chctrl_ch6_dstaddr::CHCTRL_CH6_DSTADDR_SPEC>,
    #[doc = "0x114 - Channel n Destination Address High Part Register"]
    pub chctrl_ch6_dstaddrh: crate::Reg<chctrl_ch6_dstaddrh::CHCTRL_CH6_DSTADDRH_SPEC>,
    #[doc = "0x118 - Channel n Linked List Pointer Low Part Register"]
    pub chctrl_ch6_llpointer: crate::Reg<chctrl_ch6_llpointer::CHCTRL_CH6_LLPOINTER_SPEC>,
    #[doc = "0x11c - Channel n Linked List Pointer High Part Register"]
    pub chctrl_ch6_llpointerh: crate::Reg<chctrl_ch6_llpointerh::CHCTRL_CH6_LLPOINTERH_SPEC>,
    #[doc = "0x120 - Channel n Control Register"]
    pub chctrl_ch7_ctrl: crate::Reg<chctrl_ch7_ctrl::CHCTRL_CH7_CTRL_SPEC>,
    #[doc = "0x124 - Channel n Transfer Size Register"]
    pub chctrl_ch7_transize: crate::Reg<chctrl_ch7_transize::CHCTRL_CH7_TRANSIZE_SPEC>,
    #[doc = "0x128 - Channel n Source Address Low Part Register"]
    pub chctrl_ch7_srcaddr: crate::Reg<chctrl_ch7_srcaddr::CHCTRL_CH7_SRCADDR_SPEC>,
    #[doc = "0x12c - Channel n Source Address High Part Register"]
    pub chctrl_ch7_srcaddrh: crate::Reg<chctrl_ch7_srcaddrh::CHCTRL_CH7_SRCADDRH_SPEC>,
    #[doc = "0x130 - Channel n Destination Address Low Part Register"]
    pub chctrl_ch7_dstaddr: crate::Reg<chctrl_ch7_dstaddr::CHCTRL_CH7_DSTADDR_SPEC>,
    #[doc = "0x134 - Channel n Destination Address High Part Register"]
    pub chctrl_ch7_dstaddrh: crate::Reg<chctrl_ch7_dstaddrh::CHCTRL_CH7_DSTADDRH_SPEC>,
    #[doc = "0x138 - Channel n Linked List Pointer Low Part Register"]
    pub chctrl_ch7_llpointer: crate::Reg<chctrl_ch7_llpointer::CHCTRL_CH7_LLPOINTER_SPEC>,
    #[doc = "0x13c - Channel n Linked List Pointer High Part Register"]
    pub chctrl_ch7_llpointerh: crate::Reg<chctrl_ch7_llpointerh::CHCTRL_CH7_LLPOINTERH_SPEC>,
}
#[doc = "DMACFG register accessor: an alias for `Reg<DMACFG_SPEC>`"]
pub type DMACFG = crate::Reg<dmacfg::DMACFG_SPEC>;
#[doc = "DMAC Configuration Register"]
pub mod dmacfg;
#[doc = "DMACTRL register accessor: an alias for `Reg<DMACTRL_SPEC>`"]
pub type DMACTRL = crate::Reg<dmactrl::DMACTRL_SPEC>;
#[doc = "DMAC Control Register"]
pub mod dmactrl;
#[doc = "CHABORT register accessor: an alias for `Reg<CHABORT_SPEC>`"]
pub type CHABORT = crate::Reg<chabort::CHABORT_SPEC>;
#[doc = "Channel Abort Register"]
pub mod chabort;
#[doc = "INTSTATUS register accessor: an alias for `Reg<INTSTATUS_SPEC>`"]
pub type INTSTATUS = crate::Reg<intstatus::INTSTATUS_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod intstatus;
#[doc = "CHEN register accessor: an alias for `Reg<CHEN_SPEC>`"]
pub type CHEN = crate::Reg<chen::CHEN_SPEC>;
#[doc = "Channel Enable Register"]
pub mod chen;
#[doc = "CHCTRL_CH0_CTRL register accessor: an alias for `Reg<CHCTRL_CH0_CTRL_SPEC>`"]
pub type CHCTRL_CH0_CTRL = crate::Reg<chctrl_ch0_ctrl::CHCTRL_CH0_CTRL_SPEC>;
#[doc = "Channel n Control Register"]
pub mod chctrl_ch0_ctrl;
#[doc = "CHCTRL_CH0_TRANSIZE register accessor: an alias for `Reg<CHCTRL_CH0_TRANSIZE_SPEC>`"]
pub type CHCTRL_CH0_TRANSIZE = crate::Reg<chctrl_ch0_transize::CHCTRL_CH0_TRANSIZE_SPEC>;
#[doc = "Channel n Transfer Size Register"]
pub mod chctrl_ch0_transize;
#[doc = "CHCTRL_CH0_SRCADDR register accessor: an alias for `Reg<CHCTRL_CH0_SRCADDR_SPEC>`"]
pub type CHCTRL_CH0_SRCADDR = crate::Reg<chctrl_ch0_srcaddr::CHCTRL_CH0_SRCADDR_SPEC>;
#[doc = "Channel n Source Address Low Part Register"]
pub mod chctrl_ch0_srcaddr;
#[doc = "CHCTRL_CH0_SRCADDRH register accessor: an alias for `Reg<CHCTRL_CH0_SRCADDRH_SPEC>`"]
pub type CHCTRL_CH0_SRCADDRH = crate::Reg<chctrl_ch0_srcaddrh::CHCTRL_CH0_SRCADDRH_SPEC>;
#[doc = "Channel n Source Address High Part Register"]
pub mod chctrl_ch0_srcaddrh;
#[doc = "CHCTRL_CH0_DSTADDR register accessor: an alias for `Reg<CHCTRL_CH0_DSTADDR_SPEC>`"]
pub type CHCTRL_CH0_DSTADDR = crate::Reg<chctrl_ch0_dstaddr::CHCTRL_CH0_DSTADDR_SPEC>;
#[doc = "Channel n Destination Address Low Part Register"]
pub mod chctrl_ch0_dstaddr;
#[doc = "CHCTRL_CH0_DSTADDRH register accessor: an alias for `Reg<CHCTRL_CH0_DSTADDRH_SPEC>`"]
pub type CHCTRL_CH0_DSTADDRH = crate::Reg<chctrl_ch0_dstaddrh::CHCTRL_CH0_DSTADDRH_SPEC>;
#[doc = "Channel n Destination Address High Part Register"]
pub mod chctrl_ch0_dstaddrh;
#[doc = "CHCTRL_CH0_LLPOINTER register accessor: an alias for `Reg<CHCTRL_CH0_LLPOINTER_SPEC>`"]
pub type CHCTRL_CH0_LLPOINTER = crate::Reg<chctrl_ch0_llpointer::CHCTRL_CH0_LLPOINTER_SPEC>;
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod chctrl_ch0_llpointer;
#[doc = "CHCTRL_CH0_LLPOINTERH register accessor: an alias for `Reg<CHCTRL_CH0_LLPOINTERH_SPEC>`"]
pub type CHCTRL_CH0_LLPOINTERH = crate::Reg<chctrl_ch0_llpointerh::CHCTRL_CH0_LLPOINTERH_SPEC>;
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod chctrl_ch0_llpointerh;
#[doc = "CHCTRL_CH1_CTRL register accessor: an alias for `Reg<CHCTRL_CH1_CTRL_SPEC>`"]
pub type CHCTRL_CH1_CTRL = crate::Reg<chctrl_ch1_ctrl::CHCTRL_CH1_CTRL_SPEC>;
#[doc = "Channel n Control Register"]
pub mod chctrl_ch1_ctrl;
#[doc = "CHCTRL_CH1_TRANSIZE register accessor: an alias for `Reg<CHCTRL_CH1_TRANSIZE_SPEC>`"]
pub type CHCTRL_CH1_TRANSIZE = crate::Reg<chctrl_ch1_transize::CHCTRL_CH1_TRANSIZE_SPEC>;
#[doc = "Channel n Transfer Size Register"]
pub mod chctrl_ch1_transize;
#[doc = "CHCTRL_CH1_SRCADDR register accessor: an alias for `Reg<CHCTRL_CH1_SRCADDR_SPEC>`"]
pub type CHCTRL_CH1_SRCADDR = crate::Reg<chctrl_ch1_srcaddr::CHCTRL_CH1_SRCADDR_SPEC>;
#[doc = "Channel n Source Address Low Part Register"]
pub mod chctrl_ch1_srcaddr;
#[doc = "CHCTRL_CH1_SRCADDRH register accessor: an alias for `Reg<CHCTRL_CH1_SRCADDRH_SPEC>`"]
pub type CHCTRL_CH1_SRCADDRH = crate::Reg<chctrl_ch1_srcaddrh::CHCTRL_CH1_SRCADDRH_SPEC>;
#[doc = "Channel n Source Address High Part Register"]
pub mod chctrl_ch1_srcaddrh;
#[doc = "CHCTRL_CH1_DSTADDR register accessor: an alias for `Reg<CHCTRL_CH1_DSTADDR_SPEC>`"]
pub type CHCTRL_CH1_DSTADDR = crate::Reg<chctrl_ch1_dstaddr::CHCTRL_CH1_DSTADDR_SPEC>;
#[doc = "Channel n Destination Address Low Part Register"]
pub mod chctrl_ch1_dstaddr;
#[doc = "CHCTRL_CH1_DSTADDRH register accessor: an alias for `Reg<CHCTRL_CH1_DSTADDRH_SPEC>`"]
pub type CHCTRL_CH1_DSTADDRH = crate::Reg<chctrl_ch1_dstaddrh::CHCTRL_CH1_DSTADDRH_SPEC>;
#[doc = "Channel n Destination Address High Part Register"]
pub mod chctrl_ch1_dstaddrh;
#[doc = "CHCTRL_CH1_LLPOINTER register accessor: an alias for `Reg<CHCTRL_CH1_LLPOINTER_SPEC>`"]
pub type CHCTRL_CH1_LLPOINTER = crate::Reg<chctrl_ch1_llpointer::CHCTRL_CH1_LLPOINTER_SPEC>;
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod chctrl_ch1_llpointer;
#[doc = "CHCTRL_CH1_LLPOINTERH register accessor: an alias for `Reg<CHCTRL_CH1_LLPOINTERH_SPEC>`"]
pub type CHCTRL_CH1_LLPOINTERH = crate::Reg<chctrl_ch1_llpointerh::CHCTRL_CH1_LLPOINTERH_SPEC>;
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod chctrl_ch1_llpointerh;
#[doc = "CHCTRL_CH2_CTRL register accessor: an alias for `Reg<CHCTRL_CH2_CTRL_SPEC>`"]
pub type CHCTRL_CH2_CTRL = crate::Reg<chctrl_ch2_ctrl::CHCTRL_CH2_CTRL_SPEC>;
#[doc = "Channel n Control Register"]
pub mod chctrl_ch2_ctrl;
#[doc = "CHCTRL_CH2_TRANSIZE register accessor: an alias for `Reg<CHCTRL_CH2_TRANSIZE_SPEC>`"]
pub type CHCTRL_CH2_TRANSIZE = crate::Reg<chctrl_ch2_transize::CHCTRL_CH2_TRANSIZE_SPEC>;
#[doc = "Channel n Transfer Size Register"]
pub mod chctrl_ch2_transize;
#[doc = "CHCTRL_CH2_SRCADDR register accessor: an alias for `Reg<CHCTRL_CH2_SRCADDR_SPEC>`"]
pub type CHCTRL_CH2_SRCADDR = crate::Reg<chctrl_ch2_srcaddr::CHCTRL_CH2_SRCADDR_SPEC>;
#[doc = "Channel n Source Address Low Part Register"]
pub mod chctrl_ch2_srcaddr;
#[doc = "CHCTRL_CH2_SRCADDRH register accessor: an alias for `Reg<CHCTRL_CH2_SRCADDRH_SPEC>`"]
pub type CHCTRL_CH2_SRCADDRH = crate::Reg<chctrl_ch2_srcaddrh::CHCTRL_CH2_SRCADDRH_SPEC>;
#[doc = "Channel n Source Address High Part Register"]
pub mod chctrl_ch2_srcaddrh;
#[doc = "CHCTRL_CH2_DSTADDR register accessor: an alias for `Reg<CHCTRL_CH2_DSTADDR_SPEC>`"]
pub type CHCTRL_CH2_DSTADDR = crate::Reg<chctrl_ch2_dstaddr::CHCTRL_CH2_DSTADDR_SPEC>;
#[doc = "Channel n Destination Address Low Part Register"]
pub mod chctrl_ch2_dstaddr;
#[doc = "CHCTRL_CH2_DSTADDRH register accessor: an alias for `Reg<CHCTRL_CH2_DSTADDRH_SPEC>`"]
pub type CHCTRL_CH2_DSTADDRH = crate::Reg<chctrl_ch2_dstaddrh::CHCTRL_CH2_DSTADDRH_SPEC>;
#[doc = "Channel n Destination Address High Part Register"]
pub mod chctrl_ch2_dstaddrh;
#[doc = "CHCTRL_CH2_LLPOINTER register accessor: an alias for `Reg<CHCTRL_CH2_LLPOINTER_SPEC>`"]
pub type CHCTRL_CH2_LLPOINTER = crate::Reg<chctrl_ch2_llpointer::CHCTRL_CH2_LLPOINTER_SPEC>;
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod chctrl_ch2_llpointer;
#[doc = "CHCTRL_CH2_LLPOINTERH register accessor: an alias for `Reg<CHCTRL_CH2_LLPOINTERH_SPEC>`"]
pub type CHCTRL_CH2_LLPOINTERH = crate::Reg<chctrl_ch2_llpointerh::CHCTRL_CH2_LLPOINTERH_SPEC>;
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod chctrl_ch2_llpointerh;
#[doc = "CHCTRL_CH3_CTRL register accessor: an alias for `Reg<CHCTRL_CH3_CTRL_SPEC>`"]
pub type CHCTRL_CH3_CTRL = crate::Reg<chctrl_ch3_ctrl::CHCTRL_CH3_CTRL_SPEC>;
#[doc = "Channel n Control Register"]
pub mod chctrl_ch3_ctrl;
#[doc = "CHCTRL_CH3_TRANSIZE register accessor: an alias for `Reg<CHCTRL_CH3_TRANSIZE_SPEC>`"]
pub type CHCTRL_CH3_TRANSIZE = crate::Reg<chctrl_ch3_transize::CHCTRL_CH3_TRANSIZE_SPEC>;
#[doc = "Channel n Transfer Size Register"]
pub mod chctrl_ch3_transize;
#[doc = "CHCTRL_CH3_SRCADDR register accessor: an alias for `Reg<CHCTRL_CH3_SRCADDR_SPEC>`"]
pub type CHCTRL_CH3_SRCADDR = crate::Reg<chctrl_ch3_srcaddr::CHCTRL_CH3_SRCADDR_SPEC>;
#[doc = "Channel n Source Address Low Part Register"]
pub mod chctrl_ch3_srcaddr;
#[doc = "CHCTRL_CH3_SRCADDRH register accessor: an alias for `Reg<CHCTRL_CH3_SRCADDRH_SPEC>`"]
pub type CHCTRL_CH3_SRCADDRH = crate::Reg<chctrl_ch3_srcaddrh::CHCTRL_CH3_SRCADDRH_SPEC>;
#[doc = "Channel n Source Address High Part Register"]
pub mod chctrl_ch3_srcaddrh;
#[doc = "CHCTRL_CH3_DSTADDR register accessor: an alias for `Reg<CHCTRL_CH3_DSTADDR_SPEC>`"]
pub type CHCTRL_CH3_DSTADDR = crate::Reg<chctrl_ch3_dstaddr::CHCTRL_CH3_DSTADDR_SPEC>;
#[doc = "Channel n Destination Address Low Part Register"]
pub mod chctrl_ch3_dstaddr;
#[doc = "CHCTRL_CH3_DSTADDRH register accessor: an alias for `Reg<CHCTRL_CH3_DSTADDRH_SPEC>`"]
pub type CHCTRL_CH3_DSTADDRH = crate::Reg<chctrl_ch3_dstaddrh::CHCTRL_CH3_DSTADDRH_SPEC>;
#[doc = "Channel n Destination Address High Part Register"]
pub mod chctrl_ch3_dstaddrh;
#[doc = "CHCTRL_CH3_LLPOINTER register accessor: an alias for `Reg<CHCTRL_CH3_LLPOINTER_SPEC>`"]
pub type CHCTRL_CH3_LLPOINTER = crate::Reg<chctrl_ch3_llpointer::CHCTRL_CH3_LLPOINTER_SPEC>;
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod chctrl_ch3_llpointer;
#[doc = "CHCTRL_CH3_LLPOINTERH register accessor: an alias for `Reg<CHCTRL_CH3_LLPOINTERH_SPEC>`"]
pub type CHCTRL_CH3_LLPOINTERH = crate::Reg<chctrl_ch3_llpointerh::CHCTRL_CH3_LLPOINTERH_SPEC>;
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod chctrl_ch3_llpointerh;
#[doc = "CHCTRL_CH4_CTRL register accessor: an alias for `Reg<CHCTRL_CH4_CTRL_SPEC>`"]
pub type CHCTRL_CH4_CTRL = crate::Reg<chctrl_ch4_ctrl::CHCTRL_CH4_CTRL_SPEC>;
#[doc = "Channel n Control Register"]
pub mod chctrl_ch4_ctrl;
#[doc = "CHCTRL_CH4_TRANSIZE register accessor: an alias for `Reg<CHCTRL_CH4_TRANSIZE_SPEC>`"]
pub type CHCTRL_CH4_TRANSIZE = crate::Reg<chctrl_ch4_transize::CHCTRL_CH4_TRANSIZE_SPEC>;
#[doc = "Channel n Transfer Size Register"]
pub mod chctrl_ch4_transize;
#[doc = "CHCTRL_CH4_SRCADDR register accessor: an alias for `Reg<CHCTRL_CH4_SRCADDR_SPEC>`"]
pub type CHCTRL_CH4_SRCADDR = crate::Reg<chctrl_ch4_srcaddr::CHCTRL_CH4_SRCADDR_SPEC>;
#[doc = "Channel n Source Address Low Part Register"]
pub mod chctrl_ch4_srcaddr;
#[doc = "CHCTRL_CH4_SRCADDRH register accessor: an alias for `Reg<CHCTRL_CH4_SRCADDRH_SPEC>`"]
pub type CHCTRL_CH4_SRCADDRH = crate::Reg<chctrl_ch4_srcaddrh::CHCTRL_CH4_SRCADDRH_SPEC>;
#[doc = "Channel n Source Address High Part Register"]
pub mod chctrl_ch4_srcaddrh;
#[doc = "CHCTRL_CH4_DSTADDR register accessor: an alias for `Reg<CHCTRL_CH4_DSTADDR_SPEC>`"]
pub type CHCTRL_CH4_DSTADDR = crate::Reg<chctrl_ch4_dstaddr::CHCTRL_CH4_DSTADDR_SPEC>;
#[doc = "Channel n Destination Address Low Part Register"]
pub mod chctrl_ch4_dstaddr;
#[doc = "CHCTRL_CH4_DSTADDRH register accessor: an alias for `Reg<CHCTRL_CH4_DSTADDRH_SPEC>`"]
pub type CHCTRL_CH4_DSTADDRH = crate::Reg<chctrl_ch4_dstaddrh::CHCTRL_CH4_DSTADDRH_SPEC>;
#[doc = "Channel n Destination Address High Part Register"]
pub mod chctrl_ch4_dstaddrh;
#[doc = "CHCTRL_CH4_LLPOINTER register accessor: an alias for `Reg<CHCTRL_CH4_LLPOINTER_SPEC>`"]
pub type CHCTRL_CH4_LLPOINTER = crate::Reg<chctrl_ch4_llpointer::CHCTRL_CH4_LLPOINTER_SPEC>;
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod chctrl_ch4_llpointer;
#[doc = "CHCTRL_CH4_LLPOINTERH register accessor: an alias for `Reg<CHCTRL_CH4_LLPOINTERH_SPEC>`"]
pub type CHCTRL_CH4_LLPOINTERH = crate::Reg<chctrl_ch4_llpointerh::CHCTRL_CH4_LLPOINTERH_SPEC>;
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod chctrl_ch4_llpointerh;
#[doc = "CHCTRL_CH5_CTRL register accessor: an alias for `Reg<CHCTRL_CH5_CTRL_SPEC>`"]
pub type CHCTRL_CH5_CTRL = crate::Reg<chctrl_ch5_ctrl::CHCTRL_CH5_CTRL_SPEC>;
#[doc = "Channel n Control Register"]
pub mod chctrl_ch5_ctrl;
#[doc = "CHCTRL_CH5_TRANSIZE register accessor: an alias for `Reg<CHCTRL_CH5_TRANSIZE_SPEC>`"]
pub type CHCTRL_CH5_TRANSIZE = crate::Reg<chctrl_ch5_transize::CHCTRL_CH5_TRANSIZE_SPEC>;
#[doc = "Channel n Transfer Size Register"]
pub mod chctrl_ch5_transize;
#[doc = "CHCTRL_CH5_SRCADDR register accessor: an alias for `Reg<CHCTRL_CH5_SRCADDR_SPEC>`"]
pub type CHCTRL_CH5_SRCADDR = crate::Reg<chctrl_ch5_srcaddr::CHCTRL_CH5_SRCADDR_SPEC>;
#[doc = "Channel n Source Address Low Part Register"]
pub mod chctrl_ch5_srcaddr;
#[doc = "CHCTRL_CH5_SRCADDRH register accessor: an alias for `Reg<CHCTRL_CH5_SRCADDRH_SPEC>`"]
pub type CHCTRL_CH5_SRCADDRH = crate::Reg<chctrl_ch5_srcaddrh::CHCTRL_CH5_SRCADDRH_SPEC>;
#[doc = "Channel n Source Address High Part Register"]
pub mod chctrl_ch5_srcaddrh;
#[doc = "CHCTRL_CH5_DSTADDR register accessor: an alias for `Reg<CHCTRL_CH5_DSTADDR_SPEC>`"]
pub type CHCTRL_CH5_DSTADDR = crate::Reg<chctrl_ch5_dstaddr::CHCTRL_CH5_DSTADDR_SPEC>;
#[doc = "Channel n Destination Address Low Part Register"]
pub mod chctrl_ch5_dstaddr;
#[doc = "CHCTRL_CH5_DSTADDRH register accessor: an alias for `Reg<CHCTRL_CH5_DSTADDRH_SPEC>`"]
pub type CHCTRL_CH5_DSTADDRH = crate::Reg<chctrl_ch5_dstaddrh::CHCTRL_CH5_DSTADDRH_SPEC>;
#[doc = "Channel n Destination Address High Part Register"]
pub mod chctrl_ch5_dstaddrh;
#[doc = "CHCTRL_CH5_LLPOINTER register accessor: an alias for `Reg<CHCTRL_CH5_LLPOINTER_SPEC>`"]
pub type CHCTRL_CH5_LLPOINTER = crate::Reg<chctrl_ch5_llpointer::CHCTRL_CH5_LLPOINTER_SPEC>;
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod chctrl_ch5_llpointer;
#[doc = "CHCTRL_CH5_LLPOINTERH register accessor: an alias for `Reg<CHCTRL_CH5_LLPOINTERH_SPEC>`"]
pub type CHCTRL_CH5_LLPOINTERH = crate::Reg<chctrl_ch5_llpointerh::CHCTRL_CH5_LLPOINTERH_SPEC>;
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod chctrl_ch5_llpointerh;
#[doc = "CHCTRL_CH6_CTRL register accessor: an alias for `Reg<CHCTRL_CH6_CTRL_SPEC>`"]
pub type CHCTRL_CH6_CTRL = crate::Reg<chctrl_ch6_ctrl::CHCTRL_CH6_CTRL_SPEC>;
#[doc = "Channel n Control Register"]
pub mod chctrl_ch6_ctrl;
#[doc = "CHCTRL_CH6_TRANSIZE register accessor: an alias for `Reg<CHCTRL_CH6_TRANSIZE_SPEC>`"]
pub type CHCTRL_CH6_TRANSIZE = crate::Reg<chctrl_ch6_transize::CHCTRL_CH6_TRANSIZE_SPEC>;
#[doc = "Channel n Transfer Size Register"]
pub mod chctrl_ch6_transize;
#[doc = "CHCTRL_CH6_SRCADDR register accessor: an alias for `Reg<CHCTRL_CH6_SRCADDR_SPEC>`"]
pub type CHCTRL_CH6_SRCADDR = crate::Reg<chctrl_ch6_srcaddr::CHCTRL_CH6_SRCADDR_SPEC>;
#[doc = "Channel n Source Address Low Part Register"]
pub mod chctrl_ch6_srcaddr;
#[doc = "CHCTRL_CH6_SRCADDRH register accessor: an alias for `Reg<CHCTRL_CH6_SRCADDRH_SPEC>`"]
pub type CHCTRL_CH6_SRCADDRH = crate::Reg<chctrl_ch6_srcaddrh::CHCTRL_CH6_SRCADDRH_SPEC>;
#[doc = "Channel n Source Address High Part Register"]
pub mod chctrl_ch6_srcaddrh;
#[doc = "CHCTRL_CH6_DSTADDR register accessor: an alias for `Reg<CHCTRL_CH6_DSTADDR_SPEC>`"]
pub type CHCTRL_CH6_DSTADDR = crate::Reg<chctrl_ch6_dstaddr::CHCTRL_CH6_DSTADDR_SPEC>;
#[doc = "Channel n Destination Address Low Part Register"]
pub mod chctrl_ch6_dstaddr;
#[doc = "CHCTRL_CH6_DSTADDRH register accessor: an alias for `Reg<CHCTRL_CH6_DSTADDRH_SPEC>`"]
pub type CHCTRL_CH6_DSTADDRH = crate::Reg<chctrl_ch6_dstaddrh::CHCTRL_CH6_DSTADDRH_SPEC>;
#[doc = "Channel n Destination Address High Part Register"]
pub mod chctrl_ch6_dstaddrh;
#[doc = "CHCTRL_CH6_LLPOINTER register accessor: an alias for `Reg<CHCTRL_CH6_LLPOINTER_SPEC>`"]
pub type CHCTRL_CH6_LLPOINTER = crate::Reg<chctrl_ch6_llpointer::CHCTRL_CH6_LLPOINTER_SPEC>;
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod chctrl_ch6_llpointer;
#[doc = "CHCTRL_CH6_LLPOINTERH register accessor: an alias for `Reg<CHCTRL_CH6_LLPOINTERH_SPEC>`"]
pub type CHCTRL_CH6_LLPOINTERH = crate::Reg<chctrl_ch6_llpointerh::CHCTRL_CH6_LLPOINTERH_SPEC>;
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod chctrl_ch6_llpointerh;
#[doc = "CHCTRL_CH7_CTRL register accessor: an alias for `Reg<CHCTRL_CH7_CTRL_SPEC>`"]
pub type CHCTRL_CH7_CTRL = crate::Reg<chctrl_ch7_ctrl::CHCTRL_CH7_CTRL_SPEC>;
#[doc = "Channel n Control Register"]
pub mod chctrl_ch7_ctrl;
#[doc = "CHCTRL_CH7_TRANSIZE register accessor: an alias for `Reg<CHCTRL_CH7_TRANSIZE_SPEC>`"]
pub type CHCTRL_CH7_TRANSIZE = crate::Reg<chctrl_ch7_transize::CHCTRL_CH7_TRANSIZE_SPEC>;
#[doc = "Channel n Transfer Size Register"]
pub mod chctrl_ch7_transize;
#[doc = "CHCTRL_CH7_SRCADDR register accessor: an alias for `Reg<CHCTRL_CH7_SRCADDR_SPEC>`"]
pub type CHCTRL_CH7_SRCADDR = crate::Reg<chctrl_ch7_srcaddr::CHCTRL_CH7_SRCADDR_SPEC>;
#[doc = "Channel n Source Address Low Part Register"]
pub mod chctrl_ch7_srcaddr;
#[doc = "CHCTRL_CH7_SRCADDRH register accessor: an alias for `Reg<CHCTRL_CH7_SRCADDRH_SPEC>`"]
pub type CHCTRL_CH7_SRCADDRH = crate::Reg<chctrl_ch7_srcaddrh::CHCTRL_CH7_SRCADDRH_SPEC>;
#[doc = "Channel n Source Address High Part Register"]
pub mod chctrl_ch7_srcaddrh;
#[doc = "CHCTRL_CH7_DSTADDR register accessor: an alias for `Reg<CHCTRL_CH7_DSTADDR_SPEC>`"]
pub type CHCTRL_CH7_DSTADDR = crate::Reg<chctrl_ch7_dstaddr::CHCTRL_CH7_DSTADDR_SPEC>;
#[doc = "Channel n Destination Address Low Part Register"]
pub mod chctrl_ch7_dstaddr;
#[doc = "CHCTRL_CH7_DSTADDRH register accessor: an alias for `Reg<CHCTRL_CH7_DSTADDRH_SPEC>`"]
pub type CHCTRL_CH7_DSTADDRH = crate::Reg<chctrl_ch7_dstaddrh::CHCTRL_CH7_DSTADDRH_SPEC>;
#[doc = "Channel n Destination Address High Part Register"]
pub mod chctrl_ch7_dstaddrh;
#[doc = "CHCTRL_CH7_LLPOINTER register accessor: an alias for `Reg<CHCTRL_CH7_LLPOINTER_SPEC>`"]
pub type CHCTRL_CH7_LLPOINTER = crate::Reg<chctrl_ch7_llpointer::CHCTRL_CH7_LLPOINTER_SPEC>;
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod chctrl_ch7_llpointer;
#[doc = "CHCTRL_CH7_LLPOINTERH register accessor: an alias for `Reg<CHCTRL_CH7_LLPOINTERH_SPEC>`"]
pub type CHCTRL_CH7_LLPOINTERH = crate::Reg<chctrl_ch7_llpointerh::CHCTRL_CH7_LLPOINTERH_SPEC>;
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod chctrl_ch7_llpointerh;
