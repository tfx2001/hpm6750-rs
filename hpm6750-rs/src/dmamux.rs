#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HDMA MUX0 Configuration"]
    pub muxcfg_hdma_mux0: crate::Reg<muxcfg_hdma_mux0::MUXCFG_HDMA_MUX0_SPEC>,
    #[doc = "0x04 - HDMA MUX1 Configuration"]
    pub muxcfg_hdma_mux1: crate::Reg<muxcfg_hdma_mux1::MUXCFG_HDMA_MUX1_SPEC>,
    #[doc = "0x08 - HDMA MUX2 Configuration"]
    pub muxcfg_hdma_mux2: crate::Reg<muxcfg_hdma_mux2::MUXCFG_HDMA_MUX2_SPEC>,
    #[doc = "0x0c - HDMA MUX3 Configuration"]
    pub muxcfg_hdma_mux3: crate::Reg<muxcfg_hdma_mux3::MUXCFG_HDMA_MUX3_SPEC>,
    #[doc = "0x10 - HDMA MUX4 Configuration"]
    pub muxcfg_hdma_mux4: crate::Reg<muxcfg_hdma_mux4::MUXCFG_HDMA_MUX4_SPEC>,
    #[doc = "0x14 - HDMA MUX5 Configuration"]
    pub muxcfg_hdma_mux5: crate::Reg<muxcfg_hdma_mux5::MUXCFG_HDMA_MUX5_SPEC>,
    #[doc = "0x18 - HDMA MUX6 Configuration"]
    pub muxcfg_hdma_mux6: crate::Reg<muxcfg_hdma_mux6::MUXCFG_HDMA_MUX6_SPEC>,
    #[doc = "0x1c - HDMA MUX7 Configuration"]
    pub muxcfg_hdma_mux7: crate::Reg<muxcfg_hdma_mux7::MUXCFG_HDMA_MUX7_SPEC>,
    #[doc = "0x20 - XDMA MUX0 Configuration"]
    pub muxcfg_xdma_mux0: crate::Reg<muxcfg_xdma_mux0::MUXCFG_XDMA_MUX0_SPEC>,
    #[doc = "0x24 - XDMA MUX1 Configuration"]
    pub muxcfg_xdma_mux1: crate::Reg<muxcfg_xdma_mux1::MUXCFG_XDMA_MUX1_SPEC>,
    #[doc = "0x28 - XDMA MUX2 Configuration"]
    pub muxcfg_xdma_mux2: crate::Reg<muxcfg_xdma_mux2::MUXCFG_XDMA_MUX2_SPEC>,
    #[doc = "0x2c - XDMA MUX3 Configuration"]
    pub muxcfg_xdma_mux3: crate::Reg<muxcfg_xdma_mux3::MUXCFG_XDMA_MUX3_SPEC>,
    #[doc = "0x30 - XDMA MUX4 Configuration"]
    pub muxcfg_xdma_mux4: crate::Reg<muxcfg_xdma_mux4::MUXCFG_XDMA_MUX4_SPEC>,
    #[doc = "0x34 - XDMA MUX5 Configuration"]
    pub muxcfg_xdma_mux5: crate::Reg<muxcfg_xdma_mux5::MUXCFG_XDMA_MUX5_SPEC>,
    #[doc = "0x38 - XDMA MUX6 Configuration"]
    pub muxcfg_xdma_mux6: crate::Reg<muxcfg_xdma_mux6::MUXCFG_XDMA_MUX6_SPEC>,
    #[doc = "0x3c - XDMA MUX7 Configuration"]
    pub muxcfg_xdma_mux7: crate::Reg<muxcfg_xdma_mux7::MUXCFG_XDMA_MUX7_SPEC>,
}
#[doc = "MUXCFG_HDMA_MUX0 register accessor: an alias for `Reg<MUXCFG_HDMA_MUX0_SPEC>`"]
pub type MUXCFG_HDMA_MUX0 = crate::Reg<muxcfg_hdma_mux0::MUXCFG_HDMA_MUX0_SPEC>;
#[doc = "HDMA MUX0 Configuration"]
pub mod muxcfg_hdma_mux0;
#[doc = "MUXCFG_HDMA_MUX1 register accessor: an alias for `Reg<MUXCFG_HDMA_MUX1_SPEC>`"]
pub type MUXCFG_HDMA_MUX1 = crate::Reg<muxcfg_hdma_mux1::MUXCFG_HDMA_MUX1_SPEC>;
#[doc = "HDMA MUX1 Configuration"]
pub mod muxcfg_hdma_mux1;
#[doc = "MUXCFG_HDMA_MUX2 register accessor: an alias for `Reg<MUXCFG_HDMA_MUX2_SPEC>`"]
pub type MUXCFG_HDMA_MUX2 = crate::Reg<muxcfg_hdma_mux2::MUXCFG_HDMA_MUX2_SPEC>;
#[doc = "HDMA MUX2 Configuration"]
pub mod muxcfg_hdma_mux2;
#[doc = "MUXCFG_HDMA_MUX3 register accessor: an alias for `Reg<MUXCFG_HDMA_MUX3_SPEC>`"]
pub type MUXCFG_HDMA_MUX3 = crate::Reg<muxcfg_hdma_mux3::MUXCFG_HDMA_MUX3_SPEC>;
#[doc = "HDMA MUX3 Configuration"]
pub mod muxcfg_hdma_mux3;
#[doc = "MUXCFG_HDMA_MUX4 register accessor: an alias for `Reg<MUXCFG_HDMA_MUX4_SPEC>`"]
pub type MUXCFG_HDMA_MUX4 = crate::Reg<muxcfg_hdma_mux4::MUXCFG_HDMA_MUX4_SPEC>;
#[doc = "HDMA MUX4 Configuration"]
pub mod muxcfg_hdma_mux4;
#[doc = "MUXCFG_HDMA_MUX5 register accessor: an alias for `Reg<MUXCFG_HDMA_MUX5_SPEC>`"]
pub type MUXCFG_HDMA_MUX5 = crate::Reg<muxcfg_hdma_mux5::MUXCFG_HDMA_MUX5_SPEC>;
#[doc = "HDMA MUX5 Configuration"]
pub mod muxcfg_hdma_mux5;
#[doc = "MUXCFG_HDMA_MUX6 register accessor: an alias for `Reg<MUXCFG_HDMA_MUX6_SPEC>`"]
pub type MUXCFG_HDMA_MUX6 = crate::Reg<muxcfg_hdma_mux6::MUXCFG_HDMA_MUX6_SPEC>;
#[doc = "HDMA MUX6 Configuration"]
pub mod muxcfg_hdma_mux6;
#[doc = "MUXCFG_HDMA_MUX7 register accessor: an alias for `Reg<MUXCFG_HDMA_MUX7_SPEC>`"]
pub type MUXCFG_HDMA_MUX7 = crate::Reg<muxcfg_hdma_mux7::MUXCFG_HDMA_MUX7_SPEC>;
#[doc = "HDMA MUX7 Configuration"]
pub mod muxcfg_hdma_mux7;
#[doc = "MUXCFG_XDMA_MUX0 register accessor: an alias for `Reg<MUXCFG_XDMA_MUX0_SPEC>`"]
pub type MUXCFG_XDMA_MUX0 = crate::Reg<muxcfg_xdma_mux0::MUXCFG_XDMA_MUX0_SPEC>;
#[doc = "XDMA MUX0 Configuration"]
pub mod muxcfg_xdma_mux0;
#[doc = "MUXCFG_XDMA_MUX1 register accessor: an alias for `Reg<MUXCFG_XDMA_MUX1_SPEC>`"]
pub type MUXCFG_XDMA_MUX1 = crate::Reg<muxcfg_xdma_mux1::MUXCFG_XDMA_MUX1_SPEC>;
#[doc = "XDMA MUX1 Configuration"]
pub mod muxcfg_xdma_mux1;
#[doc = "MUXCFG_XDMA_MUX2 register accessor: an alias for `Reg<MUXCFG_XDMA_MUX2_SPEC>`"]
pub type MUXCFG_XDMA_MUX2 = crate::Reg<muxcfg_xdma_mux2::MUXCFG_XDMA_MUX2_SPEC>;
#[doc = "XDMA MUX2 Configuration"]
pub mod muxcfg_xdma_mux2;
#[doc = "MUXCFG_XDMA_MUX3 register accessor: an alias for `Reg<MUXCFG_XDMA_MUX3_SPEC>`"]
pub type MUXCFG_XDMA_MUX3 = crate::Reg<muxcfg_xdma_mux3::MUXCFG_XDMA_MUX3_SPEC>;
#[doc = "XDMA MUX3 Configuration"]
pub mod muxcfg_xdma_mux3;
#[doc = "MUXCFG_XDMA_MUX4 register accessor: an alias for `Reg<MUXCFG_XDMA_MUX4_SPEC>`"]
pub type MUXCFG_XDMA_MUX4 = crate::Reg<muxcfg_xdma_mux4::MUXCFG_XDMA_MUX4_SPEC>;
#[doc = "XDMA MUX4 Configuration"]
pub mod muxcfg_xdma_mux4;
#[doc = "MUXCFG_XDMA_MUX5 register accessor: an alias for `Reg<MUXCFG_XDMA_MUX5_SPEC>`"]
pub type MUXCFG_XDMA_MUX5 = crate::Reg<muxcfg_xdma_mux5::MUXCFG_XDMA_MUX5_SPEC>;
#[doc = "XDMA MUX5 Configuration"]
pub mod muxcfg_xdma_mux5;
#[doc = "MUXCFG_XDMA_MUX6 register accessor: an alias for `Reg<MUXCFG_XDMA_MUX6_SPEC>`"]
pub type MUXCFG_XDMA_MUX6 = crate::Reg<muxcfg_xdma_mux6::MUXCFG_XDMA_MUX6_SPEC>;
#[doc = "XDMA MUX6 Configuration"]
pub mod muxcfg_xdma_mux6;
#[doc = "MUXCFG_XDMA_MUX7 register accessor: an alias for `Reg<MUXCFG_XDMA_MUX7_SPEC>`"]
pub type MUXCFG_XDMA_MUX7 = crate::Reg<muxcfg_xdma_mux7::MUXCFG_XDMA_MUX7_SPEC>;
#[doc = "XDMA MUX7 Configuration"]
pub mod muxcfg_xdma_mux7;
