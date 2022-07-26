#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No description avaiable"]
    pub config_trg0a: crate::Reg<config_trg0a::CONFIG_TRG0A_SPEC>,
    #[doc = "0x04 - No description avaiable"]
    pub config_trg0b: crate::Reg<config_trg0b::CONFIG_TRG0B_SPEC>,
    #[doc = "0x08 - No description avaiable"]
    pub config_trg0c: crate::Reg<config_trg0c::CONFIG_TRG0C_SPEC>,
    #[doc = "0x0c - No description avaiable"]
    pub config_trg1a: crate::Reg<config_trg1a::CONFIG_TRG1A_SPEC>,
    #[doc = "0x10 - No description avaiable"]
    pub config_trg1b: crate::Reg<config_trg1b::CONFIG_TRG1B_SPEC>,
    #[doc = "0x14 - No description avaiable"]
    pub config_trg1c: crate::Reg<config_trg1c::CONFIG_TRG1C_SPEC>,
    #[doc = "0x18 - No description avaiable"]
    pub config_trg2a: crate::Reg<config_trg2a::CONFIG_TRG2A_SPEC>,
    #[doc = "0x1c - No description avaiable"]
    pub config_trg2b: crate::Reg<config_trg2b::CONFIG_TRG2B_SPEC>,
    #[doc = "0x20 - No description avaiable"]
    pub config_trg2c: crate::Reg<config_trg2c::CONFIG_TRG2C_SPEC>,
    #[doc = "0x24 - No description avaiable"]
    pub config_trg3a: crate::Reg<config_trg3a::CONFIG_TRG3A_SPEC>,
    #[doc = "0x28 - No description avaiable"]
    pub config_trg3b: crate::Reg<config_trg3b::CONFIG_TRG3B_SPEC>,
    #[doc = "0x2c - No description avaiable"]
    pub config_trg3c: crate::Reg<config_trg3c::CONFIG_TRG3C_SPEC>,
    #[doc = "0x30 - No description avaiable"]
    pub trg_dma_addr: crate::Reg<trg_dma_addr::TRG_DMA_ADDR_SPEC>,
    _reserved13: [u8; 0x03cc],
    #[doc = "0x400 - No description avaiable"]
    pub bus_result_chn0: crate::Reg<bus_result_chn0::BUS_RESULT_CHN0_SPEC>,
    #[doc = "0x404 - No description avaiable"]
    pub bus_result_chn1: crate::Reg<bus_result_chn1::BUS_RESULT_CHN1_SPEC>,
    #[doc = "0x408 - No description avaiable"]
    pub bus_result_chn2: crate::Reg<bus_result_chn2::BUS_RESULT_CHN2_SPEC>,
    #[doc = "0x40c - No description avaiable"]
    pub bus_result_chn3: crate::Reg<bus_result_chn3::BUS_RESULT_CHN3_SPEC>,
    #[doc = "0x410 - No description avaiable"]
    pub bus_result_chn4: crate::Reg<bus_result_chn4::BUS_RESULT_CHN4_SPEC>,
    #[doc = "0x414 - No description avaiable"]
    pub bus_result_chn5: crate::Reg<bus_result_chn5::BUS_RESULT_CHN5_SPEC>,
    #[doc = "0x418 - No description avaiable"]
    pub bus_result_chn6: crate::Reg<bus_result_chn6::BUS_RESULT_CHN6_SPEC>,
    #[doc = "0x41c - No description avaiable"]
    pub bus_result_chn7: crate::Reg<bus_result_chn7::BUS_RESULT_CHN7_SPEC>,
    #[doc = "0x420 - No description avaiable"]
    pub bus_result_chn8: crate::Reg<bus_result_chn8::BUS_RESULT_CHN8_SPEC>,
    #[doc = "0x424 - No description avaiable"]
    pub bus_result_chn9: crate::Reg<bus_result_chn9::BUS_RESULT_CHN9_SPEC>,
    #[doc = "0x428 - No description avaiable"]
    pub bus_result_chn10: crate::Reg<bus_result_chn10::BUS_RESULT_CHN10_SPEC>,
    #[doc = "0x42c - No description avaiable"]
    pub bus_result_chn11: crate::Reg<bus_result_chn11::BUS_RESULT_CHN11_SPEC>,
    #[doc = "0x430 - No description avaiable"]
    pub bus_result_chn12: crate::Reg<bus_result_chn12::BUS_RESULT_CHN12_SPEC>,
    #[doc = "0x434 - No description avaiable"]
    pub bus_result_chn13: crate::Reg<bus_result_chn13::BUS_RESULT_CHN13_SPEC>,
    #[doc = "0x438 - No description avaiable"]
    pub bus_result_chn14: crate::Reg<bus_result_chn14::BUS_RESULT_CHN14_SPEC>,
    #[doc = "0x43c - No description avaiable"]
    pub bus_result_chn15: crate::Reg<bus_result_chn15::BUS_RESULT_CHN15_SPEC>,
    #[doc = "0x440 - No description avaiable"]
    pub bus_result_chn16: crate::Reg<bus_result_chn16::BUS_RESULT_CHN16_SPEC>,
    #[doc = "0x444 - No description avaiable"]
    pub bus_result_chn17: crate::Reg<bus_result_chn17::BUS_RESULT_CHN17_SPEC>,
    #[doc = "0x448 - No description avaiable"]
    pub bus_result_chn18: crate::Reg<bus_result_chn18::BUS_RESULT_CHN18_SPEC>,
    _reserved32: [u8; 0xb4],
    #[doc = "0x500 - No description avaiable"]
    pub buf_cfg0: crate::Reg<buf_cfg0::BUF_CFG0_SPEC>,
    _reserved33: [u8; 0x02fc],
    #[doc = "0x800 - No description avaiable"]
    pub seq_cfg0: crate::Reg<seq_cfg0::SEQ_CFG0_SPEC>,
    #[doc = "0x804 - No description avaiable"]
    pub seq_dma_addr: crate::Reg<seq_dma_addr::SEQ_DMA_ADDR_SPEC>,
    #[doc = "0x808 - No description avaiable"]
    pub seq_wr_addr: crate::Reg<seq_wr_addr::SEQ_WR_ADDR_SPEC>,
    #[doc = "0x80c - No description avaiable"]
    pub seq_dma_cfg: crate::Reg<seq_dma_cfg::SEQ_DMA_CFG_SPEC>,
    #[doc = "0x810 - No description avaiable"]
    pub seq_que_cfg0: crate::Reg<seq_que_cfg0::SEQ_QUE_CFG0_SPEC>,
    #[doc = "0x814 - No description avaiable"]
    pub seq_que_cfg1: crate::Reg<seq_que_cfg1::SEQ_QUE_CFG1_SPEC>,
    #[doc = "0x818 - No description avaiable"]
    pub seq_que_cfg2: crate::Reg<seq_que_cfg2::SEQ_QUE_CFG2_SPEC>,
    #[doc = "0x81c - No description avaiable"]
    pub seq_que_cfg3: crate::Reg<seq_que_cfg3::SEQ_QUE_CFG3_SPEC>,
    #[doc = "0x820 - No description avaiable"]
    pub seq_que_cfg4: crate::Reg<seq_que_cfg4::SEQ_QUE_CFG4_SPEC>,
    #[doc = "0x824 - No description avaiable"]
    pub seq_que_cfg5: crate::Reg<seq_que_cfg5::SEQ_QUE_CFG5_SPEC>,
    #[doc = "0x828 - No description avaiable"]
    pub seq_que_cfg6: crate::Reg<seq_que_cfg6::SEQ_QUE_CFG6_SPEC>,
    #[doc = "0x82c - No description avaiable"]
    pub seq_que_cfg7: crate::Reg<seq_que_cfg7::SEQ_QUE_CFG7_SPEC>,
    #[doc = "0x830 - No description avaiable"]
    pub seq_que_cfg8: crate::Reg<seq_que_cfg8::SEQ_QUE_CFG8_SPEC>,
    #[doc = "0x834 - No description avaiable"]
    pub seq_que_cfg9: crate::Reg<seq_que_cfg9::SEQ_QUE_CFG9_SPEC>,
    #[doc = "0x838 - No description avaiable"]
    pub seq_que_cfg10: crate::Reg<seq_que_cfg10::SEQ_QUE_CFG10_SPEC>,
    #[doc = "0x83c - No description avaiable"]
    pub seq_que_cfg11: crate::Reg<seq_que_cfg11::SEQ_QUE_CFG11_SPEC>,
    #[doc = "0x840 - No description avaiable"]
    pub seq_que_cfg12: crate::Reg<seq_que_cfg12::SEQ_QUE_CFG12_SPEC>,
    #[doc = "0x844 - No description avaiable"]
    pub seq_que_cfg13: crate::Reg<seq_que_cfg13::SEQ_QUE_CFG13_SPEC>,
    #[doc = "0x848 - No description avaiable"]
    pub seq_que_cfg14: crate::Reg<seq_que_cfg14::SEQ_QUE_CFG14_SPEC>,
    #[doc = "0x84c - No description avaiable"]
    pub seq_que_cfg15: crate::Reg<seq_que_cfg15::SEQ_QUE_CFG15_SPEC>,
    _reserved53: [u8; 0x03b0],
    #[doc = "0xc00 - No description avaiable"]
    pub prd_cfg_chn0_prd_cfg: crate::Reg<prd_cfg_chn0_prd_cfg::PRD_CFG_CHN0_PRD_CFG_SPEC>,
    #[doc = "0xc04 - No description avaiable"]
    pub prd_cfg_chn0_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn0_prd_thshd_cfg::PRD_CFG_CHN0_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xc08 - No description avaiable"]
    pub prd_cfg_chn0_prd_result: crate::Reg<prd_cfg_chn0_prd_result::PRD_CFG_CHN0_PRD_RESULT_SPEC>,
    _reserved56: [u8; 0x04],
    #[doc = "0xc10 - No description avaiable"]
    pub prd_cfg_chn1_prd_cfg: crate::Reg<prd_cfg_chn1_prd_cfg::PRD_CFG_CHN1_PRD_CFG_SPEC>,
    #[doc = "0xc14 - No description avaiable"]
    pub prd_cfg_chn1_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn1_prd_thshd_cfg::PRD_CFG_CHN1_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xc18 - No description avaiable"]
    pub prd_cfg_chn1_prd_result: crate::Reg<prd_cfg_chn1_prd_result::PRD_CFG_CHN1_PRD_RESULT_SPEC>,
    _reserved59: [u8; 0x04],
    #[doc = "0xc20 - No description avaiable"]
    pub prd_cfg_chn2_prd_cfg: crate::Reg<prd_cfg_chn2_prd_cfg::PRD_CFG_CHN2_PRD_CFG_SPEC>,
    #[doc = "0xc24 - No description avaiable"]
    pub prd_cfg_chn2_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn2_prd_thshd_cfg::PRD_CFG_CHN2_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xc28 - No description avaiable"]
    pub prd_cfg_chn2_prd_result: crate::Reg<prd_cfg_chn2_prd_result::PRD_CFG_CHN2_PRD_RESULT_SPEC>,
    _reserved62: [u8; 0x04],
    #[doc = "0xc30 - No description avaiable"]
    pub prd_cfg_chn3_prd_cfg: crate::Reg<prd_cfg_chn3_prd_cfg::PRD_CFG_CHN3_PRD_CFG_SPEC>,
    #[doc = "0xc34 - No description avaiable"]
    pub prd_cfg_chn3_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn3_prd_thshd_cfg::PRD_CFG_CHN3_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xc38 - No description avaiable"]
    pub prd_cfg_chn3_prd_result: crate::Reg<prd_cfg_chn3_prd_result::PRD_CFG_CHN3_PRD_RESULT_SPEC>,
    _reserved65: [u8; 0x04],
    #[doc = "0xc40 - No description avaiable"]
    pub prd_cfg_chn4_prd_cfg: crate::Reg<prd_cfg_chn4_prd_cfg::PRD_CFG_CHN4_PRD_CFG_SPEC>,
    #[doc = "0xc44 - No description avaiable"]
    pub prd_cfg_chn4_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn4_prd_thshd_cfg::PRD_CFG_CHN4_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xc48 - No description avaiable"]
    pub prd_cfg_chn4_prd_result: crate::Reg<prd_cfg_chn4_prd_result::PRD_CFG_CHN4_PRD_RESULT_SPEC>,
    _reserved68: [u8; 0x04],
    #[doc = "0xc50 - No description avaiable"]
    pub prd_cfg_chn5_prd_cfg: crate::Reg<prd_cfg_chn5_prd_cfg::PRD_CFG_CHN5_PRD_CFG_SPEC>,
    #[doc = "0xc54 - No description avaiable"]
    pub prd_cfg_chn5_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn5_prd_thshd_cfg::PRD_CFG_CHN5_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xc58 - No description avaiable"]
    pub prd_cfg_chn5_prd_result: crate::Reg<prd_cfg_chn5_prd_result::PRD_CFG_CHN5_PRD_RESULT_SPEC>,
    _reserved71: [u8; 0x04],
    #[doc = "0xc60 - No description avaiable"]
    pub prd_cfg_chn6_prd_cfg: crate::Reg<prd_cfg_chn6_prd_cfg::PRD_CFG_CHN6_PRD_CFG_SPEC>,
    #[doc = "0xc64 - No description avaiable"]
    pub prd_cfg_chn6_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn6_prd_thshd_cfg::PRD_CFG_CHN6_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xc68 - No description avaiable"]
    pub prd_cfg_chn6_prd_result: crate::Reg<prd_cfg_chn6_prd_result::PRD_CFG_CHN6_PRD_RESULT_SPEC>,
    _reserved74: [u8; 0x04],
    #[doc = "0xc70 - No description avaiable"]
    pub prd_cfg_chn7_prd_cfg: crate::Reg<prd_cfg_chn7_prd_cfg::PRD_CFG_CHN7_PRD_CFG_SPEC>,
    #[doc = "0xc74 - No description avaiable"]
    pub prd_cfg_chn7_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn7_prd_thshd_cfg::PRD_CFG_CHN7_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xc78 - No description avaiable"]
    pub prd_cfg_chn7_prd_result: crate::Reg<prd_cfg_chn7_prd_result::PRD_CFG_CHN7_PRD_RESULT_SPEC>,
    _reserved77: [u8; 0x04],
    #[doc = "0xc80 - No description avaiable"]
    pub prd_cfg_chn8_prd_cfg: crate::Reg<prd_cfg_chn8_prd_cfg::PRD_CFG_CHN8_PRD_CFG_SPEC>,
    #[doc = "0xc84 - No description avaiable"]
    pub prd_cfg_chn8_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn8_prd_thshd_cfg::PRD_CFG_CHN8_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xc88 - No description avaiable"]
    pub prd_cfg_chn8_prd_result: crate::Reg<prd_cfg_chn8_prd_result::PRD_CFG_CHN8_PRD_RESULT_SPEC>,
    _reserved80: [u8; 0x04],
    #[doc = "0xc90 - No description avaiable"]
    pub prd_cfg_chn9_prd_cfg: crate::Reg<prd_cfg_chn9_prd_cfg::PRD_CFG_CHN9_PRD_CFG_SPEC>,
    #[doc = "0xc94 - No description avaiable"]
    pub prd_cfg_chn9_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn9_prd_thshd_cfg::PRD_CFG_CHN9_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xc98 - No description avaiable"]
    pub prd_cfg_chn9_prd_result: crate::Reg<prd_cfg_chn9_prd_result::PRD_CFG_CHN9_PRD_RESULT_SPEC>,
    _reserved83: [u8; 0x04],
    #[doc = "0xca0 - No description avaiable"]
    pub prd_cfg_chn10_prd_cfg: crate::Reg<prd_cfg_chn10_prd_cfg::PRD_CFG_CHN10_PRD_CFG_SPEC>,
    #[doc = "0xca4 - No description avaiable"]
    pub prd_cfg_chn10_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn10_prd_thshd_cfg::PRD_CFG_CHN10_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xca8 - No description avaiable"]
    pub prd_cfg_chn10_prd_result:
        crate::Reg<prd_cfg_chn10_prd_result::PRD_CFG_CHN10_PRD_RESULT_SPEC>,
    _reserved86: [u8; 0x04],
    #[doc = "0xcb0 - No description avaiable"]
    pub prd_cfg_chn11_prd_cfg: crate::Reg<prd_cfg_chn11_prd_cfg::PRD_CFG_CHN11_PRD_CFG_SPEC>,
    #[doc = "0xcb4 - No description avaiable"]
    pub prd_cfg_chn11_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn11_prd_thshd_cfg::PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xcb8 - No description avaiable"]
    pub prd_cfg_chn11_prd_result:
        crate::Reg<prd_cfg_chn11_prd_result::PRD_CFG_CHN11_PRD_RESULT_SPEC>,
    _reserved89: [u8; 0x04],
    #[doc = "0xcc0 - No description avaiable"]
    pub prd_cfg_chn12_prd_cfg: crate::Reg<prd_cfg_chn12_prd_cfg::PRD_CFG_CHN12_PRD_CFG_SPEC>,
    #[doc = "0xcc4 - No description avaiable"]
    pub prd_cfg_chn12_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn12_prd_thshd_cfg::PRD_CFG_CHN12_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xcc8 - No description avaiable"]
    pub prd_cfg_chn12_prd_result:
        crate::Reg<prd_cfg_chn12_prd_result::PRD_CFG_CHN12_PRD_RESULT_SPEC>,
    _reserved92: [u8; 0x04],
    #[doc = "0xcd0 - No description avaiable"]
    pub prd_cfg_chn13_prd_cfg: crate::Reg<prd_cfg_chn13_prd_cfg::PRD_CFG_CHN13_PRD_CFG_SPEC>,
    #[doc = "0xcd4 - No description avaiable"]
    pub prd_cfg_chn13_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn13_prd_thshd_cfg::PRD_CFG_CHN13_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xcd8 - No description avaiable"]
    pub prd_cfg_chn13_prd_result:
        crate::Reg<prd_cfg_chn13_prd_result::PRD_CFG_CHN13_PRD_RESULT_SPEC>,
    _reserved95: [u8; 0x04],
    #[doc = "0xce0 - No description avaiable"]
    pub prd_cfg_chn14_prd_cfg: crate::Reg<prd_cfg_chn14_prd_cfg::PRD_CFG_CHN14_PRD_CFG_SPEC>,
    #[doc = "0xce4 - No description avaiable"]
    pub prd_cfg_chn14_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn14_prd_thshd_cfg::PRD_CFG_CHN14_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xce8 - No description avaiable"]
    pub prd_cfg_chn14_prd_result:
        crate::Reg<prd_cfg_chn14_prd_result::PRD_CFG_CHN14_PRD_RESULT_SPEC>,
    _reserved98: [u8; 0x04],
    #[doc = "0xcf0 - No description avaiable"]
    pub prd_cfg_chn15_prd_cfg: crate::Reg<prd_cfg_chn15_prd_cfg::PRD_CFG_CHN15_PRD_CFG_SPEC>,
    #[doc = "0xcf4 - No description avaiable"]
    pub prd_cfg_chn15_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn15_prd_thshd_cfg::PRD_CFG_CHN15_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xcf8 - No description avaiable"]
    pub prd_cfg_chn15_prd_result:
        crate::Reg<prd_cfg_chn15_prd_result::PRD_CFG_CHN15_PRD_RESULT_SPEC>,
    _reserved101: [u8; 0x04],
    #[doc = "0xd00 - No description avaiable"]
    pub prd_cfg_chn16_prd_cfg: crate::Reg<prd_cfg_chn16_prd_cfg::PRD_CFG_CHN16_PRD_CFG_SPEC>,
    #[doc = "0xd04 - No description avaiable"]
    pub prd_cfg_chn16_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn16_prd_thshd_cfg::PRD_CFG_CHN16_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xd08 - No description avaiable"]
    pub prd_cfg_chn16_prd_result:
        crate::Reg<prd_cfg_chn16_prd_result::PRD_CFG_CHN16_PRD_RESULT_SPEC>,
    _reserved104: [u8; 0x04],
    #[doc = "0xd10 - No description avaiable"]
    pub prd_cfg_chn17_prd_cfg: crate::Reg<prd_cfg_chn17_prd_cfg::PRD_CFG_CHN17_PRD_CFG_SPEC>,
    #[doc = "0xd14 - No description avaiable"]
    pub prd_cfg_chn17_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn17_prd_thshd_cfg::PRD_CFG_CHN17_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xd18 - No description avaiable"]
    pub prd_cfg_chn17_prd_result:
        crate::Reg<prd_cfg_chn17_prd_result::PRD_CFG_CHN17_PRD_RESULT_SPEC>,
    _reserved107: [u8; 0x04],
    #[doc = "0xd20 - No description avaiable"]
    pub prd_cfg_chn18_prd_cfg: crate::Reg<prd_cfg_chn18_prd_cfg::PRD_CFG_CHN18_PRD_CFG_SPEC>,
    #[doc = "0xd24 - No description avaiable"]
    pub prd_cfg_chn18_prd_thshd_cfg:
        crate::Reg<prd_cfg_chn18_prd_thshd_cfg::PRD_CFG_CHN18_PRD_THSHD_CFG_SPEC>,
    #[doc = "0xd28 - No description avaiable"]
    pub prd_cfg_chn18_prd_result:
        crate::Reg<prd_cfg_chn18_prd_result::PRD_CFG_CHN18_PRD_RESULT_SPEC>,
    _reserved110: [u8; 0x02d4],
    #[doc = "0x1000 - No description avaiable"]
    pub sample_cfg_chn0: crate::Reg<sample_cfg_chn0::SAMPLE_CFG_CHN0_SPEC>,
    #[doc = "0x1004 - No description avaiable"]
    pub sample_cfg_chn1: crate::Reg<sample_cfg_chn1::SAMPLE_CFG_CHN1_SPEC>,
    #[doc = "0x1008 - No description avaiable"]
    pub sample_cfg_chn2: crate::Reg<sample_cfg_chn2::SAMPLE_CFG_CHN2_SPEC>,
    #[doc = "0x100c - No description avaiable"]
    pub sample_cfg_chn3: crate::Reg<sample_cfg_chn3::SAMPLE_CFG_CHN3_SPEC>,
    #[doc = "0x1010 - No description avaiable"]
    pub sample_cfg_chn4: crate::Reg<sample_cfg_chn4::SAMPLE_CFG_CHN4_SPEC>,
    #[doc = "0x1014 - No description avaiable"]
    pub sample_cfg_chn5: crate::Reg<sample_cfg_chn5::SAMPLE_CFG_CHN5_SPEC>,
    #[doc = "0x1018 - No description avaiable"]
    pub sample_cfg_chn6: crate::Reg<sample_cfg_chn6::SAMPLE_CFG_CHN6_SPEC>,
    #[doc = "0x101c - No description avaiable"]
    pub sample_cfg_chn7: crate::Reg<sample_cfg_chn7::SAMPLE_CFG_CHN7_SPEC>,
    #[doc = "0x1020 - No description avaiable"]
    pub sample_cfg_chn8: crate::Reg<sample_cfg_chn8::SAMPLE_CFG_CHN8_SPEC>,
    #[doc = "0x1024 - No description avaiable"]
    pub sample_cfg_chn9: crate::Reg<sample_cfg_chn9::SAMPLE_CFG_CHN9_SPEC>,
    #[doc = "0x1028 - No description avaiable"]
    pub sample_cfg_chn10: crate::Reg<sample_cfg_chn10::SAMPLE_CFG_CHN10_SPEC>,
    #[doc = "0x102c - No description avaiable"]
    pub sample_cfg_chn11: crate::Reg<sample_cfg_chn11::SAMPLE_CFG_CHN11_SPEC>,
    #[doc = "0x1030 - No description avaiable"]
    pub sample_cfg_chn12: crate::Reg<sample_cfg_chn12::SAMPLE_CFG_CHN12_SPEC>,
    #[doc = "0x1034 - No description avaiable"]
    pub sample_cfg_chn13: crate::Reg<sample_cfg_chn13::SAMPLE_CFG_CHN13_SPEC>,
    #[doc = "0x1038 - No description avaiable"]
    pub sample_cfg_chn14: crate::Reg<sample_cfg_chn14::SAMPLE_CFG_CHN14_SPEC>,
    #[doc = "0x103c - No description avaiable"]
    pub sample_cfg_chn15: crate::Reg<sample_cfg_chn15::SAMPLE_CFG_CHN15_SPEC>,
    #[doc = "0x1040 - No description avaiable"]
    pub sample_cfg_chn16: crate::Reg<sample_cfg_chn16::SAMPLE_CFG_CHN16_SPEC>,
    #[doc = "0x1044 - No description avaiable"]
    pub sample_cfg_chn17: crate::Reg<sample_cfg_chn17::SAMPLE_CFG_CHN17_SPEC>,
    #[doc = "0x1048 - No description avaiable"]
    pub sample_cfg_chn18: crate::Reg<sample_cfg_chn18::SAMPLE_CFG_CHN18_SPEC>,
    _reserved129: [u8; 0xb8],
    #[doc = "0x1104 - No description avaiable"]
    pub conv_cfg1: crate::Reg<conv_cfg1::CONV_CFG1_SPEC>,
    #[doc = "0x1108 - No description avaiable"]
    pub adc_cfg0: crate::Reg<adc_cfg0::ADC_CFG0_SPEC>,
    _reserved131: [u8; 0x04],
    #[doc = "0x1110 - No description avaiable"]
    pub int_sts: crate::Reg<int_sts::INT_STS_SPEC>,
    #[doc = "0x1114 - No description avaiable"]
    pub int_en: crate::Reg<int_en::INT_EN_SPEC>,
    _reserved133: [u8; 0xe8],
    #[doc = "0x1200 - No description avaiable"]
    pub ana_ctrl0: crate::Reg<ana_ctrl0::ANA_CTRL0_SPEC>,
    #[doc = "0x1204 - No description avaiable"]
    pub ana_ctrl1: crate::Reg<ana_ctrl1::ANA_CTRL1_SPEC>,
    _reserved135: [u8; 0x08],
    #[doc = "0x1210 - No description avaiable"]
    pub ana_status: crate::Reg<ana_status::ANA_STATUS_SPEC>,
}
#[doc = "CONFIG_TRG0A register accessor: an alias for `Reg<CONFIG_TRG0A_SPEC>`"]
pub type CONFIG_TRG0A = crate::Reg<config_trg0a::CONFIG_TRG0A_SPEC>;
#[doc = "No description avaiable"]
pub mod config_trg0a;
#[doc = "CONFIG_TRG0B register accessor: an alias for `Reg<CONFIG_TRG0B_SPEC>`"]
pub type CONFIG_TRG0B = crate::Reg<config_trg0b::CONFIG_TRG0B_SPEC>;
#[doc = "No description avaiable"]
pub mod config_trg0b;
#[doc = "CONFIG_TRG0C register accessor: an alias for `Reg<CONFIG_TRG0C_SPEC>`"]
pub type CONFIG_TRG0C = crate::Reg<config_trg0c::CONFIG_TRG0C_SPEC>;
#[doc = "No description avaiable"]
pub mod config_trg0c;
#[doc = "CONFIG_TRG1A register accessor: an alias for `Reg<CONFIG_TRG1A_SPEC>`"]
pub type CONFIG_TRG1A = crate::Reg<config_trg1a::CONFIG_TRG1A_SPEC>;
#[doc = "No description avaiable"]
pub mod config_trg1a;
#[doc = "CONFIG_TRG1B register accessor: an alias for `Reg<CONFIG_TRG1B_SPEC>`"]
pub type CONFIG_TRG1B = crate::Reg<config_trg1b::CONFIG_TRG1B_SPEC>;
#[doc = "No description avaiable"]
pub mod config_trg1b;
#[doc = "CONFIG_TRG1C register accessor: an alias for `Reg<CONFIG_TRG1C_SPEC>`"]
pub type CONFIG_TRG1C = crate::Reg<config_trg1c::CONFIG_TRG1C_SPEC>;
#[doc = "No description avaiable"]
pub mod config_trg1c;
#[doc = "CONFIG_TRG2A register accessor: an alias for `Reg<CONFIG_TRG2A_SPEC>`"]
pub type CONFIG_TRG2A = crate::Reg<config_trg2a::CONFIG_TRG2A_SPEC>;
#[doc = "No description avaiable"]
pub mod config_trg2a;
#[doc = "CONFIG_TRG2B register accessor: an alias for `Reg<CONFIG_TRG2B_SPEC>`"]
pub type CONFIG_TRG2B = crate::Reg<config_trg2b::CONFIG_TRG2B_SPEC>;
#[doc = "No description avaiable"]
pub mod config_trg2b;
#[doc = "CONFIG_TRG2C register accessor: an alias for `Reg<CONFIG_TRG2C_SPEC>`"]
pub type CONFIG_TRG2C = crate::Reg<config_trg2c::CONFIG_TRG2C_SPEC>;
#[doc = "No description avaiable"]
pub mod config_trg2c;
#[doc = "CONFIG_TRG3A register accessor: an alias for `Reg<CONFIG_TRG3A_SPEC>`"]
pub type CONFIG_TRG3A = crate::Reg<config_trg3a::CONFIG_TRG3A_SPEC>;
#[doc = "No description avaiable"]
pub mod config_trg3a;
#[doc = "CONFIG_TRG3B register accessor: an alias for `Reg<CONFIG_TRG3B_SPEC>`"]
pub type CONFIG_TRG3B = crate::Reg<config_trg3b::CONFIG_TRG3B_SPEC>;
#[doc = "No description avaiable"]
pub mod config_trg3b;
#[doc = "CONFIG_TRG3C register accessor: an alias for `Reg<CONFIG_TRG3C_SPEC>`"]
pub type CONFIG_TRG3C = crate::Reg<config_trg3c::CONFIG_TRG3C_SPEC>;
#[doc = "No description avaiable"]
pub mod config_trg3c;
#[doc = "TRG_DMA_ADDR register accessor: an alias for `Reg<TRG_DMA_ADDR_SPEC>`"]
pub type TRG_DMA_ADDR = crate::Reg<trg_dma_addr::TRG_DMA_ADDR_SPEC>;
#[doc = "No description avaiable"]
pub mod trg_dma_addr;
#[doc = "BUS_RESULT_CHN0 register accessor: an alias for `Reg<BUS_RESULT_CHN0_SPEC>`"]
pub type BUS_RESULT_CHN0 = crate::Reg<bus_result_chn0::BUS_RESULT_CHN0_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn0;
#[doc = "BUS_RESULT_CHN1 register accessor: an alias for `Reg<BUS_RESULT_CHN1_SPEC>`"]
pub type BUS_RESULT_CHN1 = crate::Reg<bus_result_chn1::BUS_RESULT_CHN1_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn1;
#[doc = "BUS_RESULT_CHN2 register accessor: an alias for `Reg<BUS_RESULT_CHN2_SPEC>`"]
pub type BUS_RESULT_CHN2 = crate::Reg<bus_result_chn2::BUS_RESULT_CHN2_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn2;
#[doc = "BUS_RESULT_CHN3 register accessor: an alias for `Reg<BUS_RESULT_CHN3_SPEC>`"]
pub type BUS_RESULT_CHN3 = crate::Reg<bus_result_chn3::BUS_RESULT_CHN3_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn3;
#[doc = "BUS_RESULT_CHN4 register accessor: an alias for `Reg<BUS_RESULT_CHN4_SPEC>`"]
pub type BUS_RESULT_CHN4 = crate::Reg<bus_result_chn4::BUS_RESULT_CHN4_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn4;
#[doc = "BUS_RESULT_CHN5 register accessor: an alias for `Reg<BUS_RESULT_CHN5_SPEC>`"]
pub type BUS_RESULT_CHN5 = crate::Reg<bus_result_chn5::BUS_RESULT_CHN5_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn5;
#[doc = "BUS_RESULT_CHN6 register accessor: an alias for `Reg<BUS_RESULT_CHN6_SPEC>`"]
pub type BUS_RESULT_CHN6 = crate::Reg<bus_result_chn6::BUS_RESULT_CHN6_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn6;
#[doc = "BUS_RESULT_CHN7 register accessor: an alias for `Reg<BUS_RESULT_CHN7_SPEC>`"]
pub type BUS_RESULT_CHN7 = crate::Reg<bus_result_chn7::BUS_RESULT_CHN7_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn7;
#[doc = "BUS_RESULT_CHN8 register accessor: an alias for `Reg<BUS_RESULT_CHN8_SPEC>`"]
pub type BUS_RESULT_CHN8 = crate::Reg<bus_result_chn8::BUS_RESULT_CHN8_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn8;
#[doc = "BUS_RESULT_CHN9 register accessor: an alias for `Reg<BUS_RESULT_CHN9_SPEC>`"]
pub type BUS_RESULT_CHN9 = crate::Reg<bus_result_chn9::BUS_RESULT_CHN9_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn9;
#[doc = "BUS_RESULT_CHN10 register accessor: an alias for `Reg<BUS_RESULT_CHN10_SPEC>`"]
pub type BUS_RESULT_CHN10 = crate::Reg<bus_result_chn10::BUS_RESULT_CHN10_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn10;
#[doc = "BUS_RESULT_CHN11 register accessor: an alias for `Reg<BUS_RESULT_CHN11_SPEC>`"]
pub type BUS_RESULT_CHN11 = crate::Reg<bus_result_chn11::BUS_RESULT_CHN11_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn11;
#[doc = "BUS_RESULT_CHN12 register accessor: an alias for `Reg<BUS_RESULT_CHN12_SPEC>`"]
pub type BUS_RESULT_CHN12 = crate::Reg<bus_result_chn12::BUS_RESULT_CHN12_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn12;
#[doc = "BUS_RESULT_CHN13 register accessor: an alias for `Reg<BUS_RESULT_CHN13_SPEC>`"]
pub type BUS_RESULT_CHN13 = crate::Reg<bus_result_chn13::BUS_RESULT_CHN13_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn13;
#[doc = "BUS_RESULT_CHN14 register accessor: an alias for `Reg<BUS_RESULT_CHN14_SPEC>`"]
pub type BUS_RESULT_CHN14 = crate::Reg<bus_result_chn14::BUS_RESULT_CHN14_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn14;
#[doc = "BUS_RESULT_CHN15 register accessor: an alias for `Reg<BUS_RESULT_CHN15_SPEC>`"]
pub type BUS_RESULT_CHN15 = crate::Reg<bus_result_chn15::BUS_RESULT_CHN15_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn15;
#[doc = "BUS_RESULT_CHN16 register accessor: an alias for `Reg<BUS_RESULT_CHN16_SPEC>`"]
pub type BUS_RESULT_CHN16 = crate::Reg<bus_result_chn16::BUS_RESULT_CHN16_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn16;
#[doc = "BUS_RESULT_CHN17 register accessor: an alias for `Reg<BUS_RESULT_CHN17_SPEC>`"]
pub type BUS_RESULT_CHN17 = crate::Reg<bus_result_chn17::BUS_RESULT_CHN17_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn17;
#[doc = "BUS_RESULT_CHN18 register accessor: an alias for `Reg<BUS_RESULT_CHN18_SPEC>`"]
pub type BUS_RESULT_CHN18 = crate::Reg<bus_result_chn18::BUS_RESULT_CHN18_SPEC>;
#[doc = "No description avaiable"]
pub mod bus_result_chn18;
#[doc = "BUF_CFG0 register accessor: an alias for `Reg<BUF_CFG0_SPEC>`"]
pub type BUF_CFG0 = crate::Reg<buf_cfg0::BUF_CFG0_SPEC>;
#[doc = "No description avaiable"]
pub mod buf_cfg0;
#[doc = "SEQ_CFG0 register accessor: an alias for `Reg<SEQ_CFG0_SPEC>`"]
pub type SEQ_CFG0 = crate::Reg<seq_cfg0::SEQ_CFG0_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_cfg0;
#[doc = "SEQ_DMA_ADDR register accessor: an alias for `Reg<SEQ_DMA_ADDR_SPEC>`"]
pub type SEQ_DMA_ADDR = crate::Reg<seq_dma_addr::SEQ_DMA_ADDR_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_dma_addr;
#[doc = "SEQ_WR_ADDR register accessor: an alias for `Reg<SEQ_WR_ADDR_SPEC>`"]
pub type SEQ_WR_ADDR = crate::Reg<seq_wr_addr::SEQ_WR_ADDR_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_wr_addr;
#[doc = "SEQ_DMA_CFG register accessor: an alias for `Reg<SEQ_DMA_CFG_SPEC>`"]
pub type SEQ_DMA_CFG = crate::Reg<seq_dma_cfg::SEQ_DMA_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_dma_cfg;
#[doc = "SEQ_QUE_CFG0 register accessor: an alias for `Reg<SEQ_QUE_CFG0_SPEC>`"]
pub type SEQ_QUE_CFG0 = crate::Reg<seq_que_cfg0::SEQ_QUE_CFG0_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg0;
#[doc = "SEQ_QUE_CFG1 register accessor: an alias for `Reg<SEQ_QUE_CFG1_SPEC>`"]
pub type SEQ_QUE_CFG1 = crate::Reg<seq_que_cfg1::SEQ_QUE_CFG1_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg1;
#[doc = "SEQ_QUE_CFG2 register accessor: an alias for `Reg<SEQ_QUE_CFG2_SPEC>`"]
pub type SEQ_QUE_CFG2 = crate::Reg<seq_que_cfg2::SEQ_QUE_CFG2_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg2;
#[doc = "SEQ_QUE_CFG3 register accessor: an alias for `Reg<SEQ_QUE_CFG3_SPEC>`"]
pub type SEQ_QUE_CFG3 = crate::Reg<seq_que_cfg3::SEQ_QUE_CFG3_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg3;
#[doc = "SEQ_QUE_CFG4 register accessor: an alias for `Reg<SEQ_QUE_CFG4_SPEC>`"]
pub type SEQ_QUE_CFG4 = crate::Reg<seq_que_cfg4::SEQ_QUE_CFG4_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg4;
#[doc = "SEQ_QUE_CFG5 register accessor: an alias for `Reg<SEQ_QUE_CFG5_SPEC>`"]
pub type SEQ_QUE_CFG5 = crate::Reg<seq_que_cfg5::SEQ_QUE_CFG5_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg5;
#[doc = "SEQ_QUE_CFG6 register accessor: an alias for `Reg<SEQ_QUE_CFG6_SPEC>`"]
pub type SEQ_QUE_CFG6 = crate::Reg<seq_que_cfg6::SEQ_QUE_CFG6_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg6;
#[doc = "SEQ_QUE_CFG7 register accessor: an alias for `Reg<SEQ_QUE_CFG7_SPEC>`"]
pub type SEQ_QUE_CFG7 = crate::Reg<seq_que_cfg7::SEQ_QUE_CFG7_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg7;
#[doc = "SEQ_QUE_CFG8 register accessor: an alias for `Reg<SEQ_QUE_CFG8_SPEC>`"]
pub type SEQ_QUE_CFG8 = crate::Reg<seq_que_cfg8::SEQ_QUE_CFG8_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg8;
#[doc = "SEQ_QUE_CFG9 register accessor: an alias for `Reg<SEQ_QUE_CFG9_SPEC>`"]
pub type SEQ_QUE_CFG9 = crate::Reg<seq_que_cfg9::SEQ_QUE_CFG9_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg9;
#[doc = "SEQ_QUE_CFG10 register accessor: an alias for `Reg<SEQ_QUE_CFG10_SPEC>`"]
pub type SEQ_QUE_CFG10 = crate::Reg<seq_que_cfg10::SEQ_QUE_CFG10_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg10;
#[doc = "SEQ_QUE_CFG11 register accessor: an alias for `Reg<SEQ_QUE_CFG11_SPEC>`"]
pub type SEQ_QUE_CFG11 = crate::Reg<seq_que_cfg11::SEQ_QUE_CFG11_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg11;
#[doc = "SEQ_QUE_CFG12 register accessor: an alias for `Reg<SEQ_QUE_CFG12_SPEC>`"]
pub type SEQ_QUE_CFG12 = crate::Reg<seq_que_cfg12::SEQ_QUE_CFG12_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg12;
#[doc = "SEQ_QUE_CFG13 register accessor: an alias for `Reg<SEQ_QUE_CFG13_SPEC>`"]
pub type SEQ_QUE_CFG13 = crate::Reg<seq_que_cfg13::SEQ_QUE_CFG13_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg13;
#[doc = "SEQ_QUE_CFG14 register accessor: an alias for `Reg<SEQ_QUE_CFG14_SPEC>`"]
pub type SEQ_QUE_CFG14 = crate::Reg<seq_que_cfg14::SEQ_QUE_CFG14_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg14;
#[doc = "SEQ_QUE_CFG15 register accessor: an alias for `Reg<SEQ_QUE_CFG15_SPEC>`"]
pub type SEQ_QUE_CFG15 = crate::Reg<seq_que_cfg15::SEQ_QUE_CFG15_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_que_cfg15;
#[doc = "PRD_CFG_CHN0_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN0_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN0_PRD_CFG = crate::Reg<prd_cfg_chn0_prd_cfg::PRD_CFG_CHN0_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn0_prd_cfg;
#[doc = "PRD_CFG_CHN0_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN0_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN0_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn0_prd_thshd_cfg::PRD_CFG_CHN0_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn0_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN0_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN0_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN0_PRD_RESULT =
    crate::Reg<prd_cfg_chn0_prd_result::PRD_CFG_CHN0_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn0_prd_result;
#[doc = "PRD_CFG_CHN1_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN1_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN1_PRD_CFG = crate::Reg<prd_cfg_chn1_prd_cfg::PRD_CFG_CHN1_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn1_prd_cfg;
#[doc = "PRD_CFG_CHN1_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN1_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN1_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn1_prd_thshd_cfg::PRD_CFG_CHN1_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn1_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN1_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN1_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN1_PRD_RESULT =
    crate::Reg<prd_cfg_chn1_prd_result::PRD_CFG_CHN1_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn1_prd_result;
#[doc = "PRD_CFG_CHN2_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN2_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN2_PRD_CFG = crate::Reg<prd_cfg_chn2_prd_cfg::PRD_CFG_CHN2_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn2_prd_cfg;
#[doc = "PRD_CFG_CHN2_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN2_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN2_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn2_prd_thshd_cfg::PRD_CFG_CHN2_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn2_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN2_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN2_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN2_PRD_RESULT =
    crate::Reg<prd_cfg_chn2_prd_result::PRD_CFG_CHN2_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn2_prd_result;
#[doc = "PRD_CFG_CHN3_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN3_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN3_PRD_CFG = crate::Reg<prd_cfg_chn3_prd_cfg::PRD_CFG_CHN3_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn3_prd_cfg;
#[doc = "PRD_CFG_CHN3_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN3_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN3_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn3_prd_thshd_cfg::PRD_CFG_CHN3_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn3_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN3_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN3_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN3_PRD_RESULT =
    crate::Reg<prd_cfg_chn3_prd_result::PRD_CFG_CHN3_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn3_prd_result;
#[doc = "PRD_CFG_CHN4_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN4_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN4_PRD_CFG = crate::Reg<prd_cfg_chn4_prd_cfg::PRD_CFG_CHN4_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn4_prd_cfg;
#[doc = "PRD_CFG_CHN4_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN4_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN4_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn4_prd_thshd_cfg::PRD_CFG_CHN4_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn4_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN4_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN4_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN4_PRD_RESULT =
    crate::Reg<prd_cfg_chn4_prd_result::PRD_CFG_CHN4_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn4_prd_result;
#[doc = "PRD_CFG_CHN5_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN5_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN5_PRD_CFG = crate::Reg<prd_cfg_chn5_prd_cfg::PRD_CFG_CHN5_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn5_prd_cfg;
#[doc = "PRD_CFG_CHN5_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN5_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN5_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn5_prd_thshd_cfg::PRD_CFG_CHN5_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn5_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN5_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN5_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN5_PRD_RESULT =
    crate::Reg<prd_cfg_chn5_prd_result::PRD_CFG_CHN5_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn5_prd_result;
#[doc = "PRD_CFG_CHN6_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN6_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN6_PRD_CFG = crate::Reg<prd_cfg_chn6_prd_cfg::PRD_CFG_CHN6_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn6_prd_cfg;
#[doc = "PRD_CFG_CHN6_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN6_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN6_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn6_prd_thshd_cfg::PRD_CFG_CHN6_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn6_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN6_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN6_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN6_PRD_RESULT =
    crate::Reg<prd_cfg_chn6_prd_result::PRD_CFG_CHN6_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn6_prd_result;
#[doc = "PRD_CFG_CHN7_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN7_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN7_PRD_CFG = crate::Reg<prd_cfg_chn7_prd_cfg::PRD_CFG_CHN7_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn7_prd_cfg;
#[doc = "PRD_CFG_CHN7_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN7_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN7_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn7_prd_thshd_cfg::PRD_CFG_CHN7_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn7_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN7_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN7_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN7_PRD_RESULT =
    crate::Reg<prd_cfg_chn7_prd_result::PRD_CFG_CHN7_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn7_prd_result;
#[doc = "PRD_CFG_CHN8_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN8_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN8_PRD_CFG = crate::Reg<prd_cfg_chn8_prd_cfg::PRD_CFG_CHN8_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn8_prd_cfg;
#[doc = "PRD_CFG_CHN8_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN8_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN8_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn8_prd_thshd_cfg::PRD_CFG_CHN8_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn8_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN8_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN8_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN8_PRD_RESULT =
    crate::Reg<prd_cfg_chn8_prd_result::PRD_CFG_CHN8_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn8_prd_result;
#[doc = "PRD_CFG_CHN9_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN9_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN9_PRD_CFG = crate::Reg<prd_cfg_chn9_prd_cfg::PRD_CFG_CHN9_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn9_prd_cfg;
#[doc = "PRD_CFG_CHN9_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN9_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN9_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn9_prd_thshd_cfg::PRD_CFG_CHN9_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn9_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN9_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN9_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN9_PRD_RESULT =
    crate::Reg<prd_cfg_chn9_prd_result::PRD_CFG_CHN9_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn9_prd_result;
#[doc = "PRD_CFG_CHN10_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN10_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN10_PRD_CFG = crate::Reg<prd_cfg_chn10_prd_cfg::PRD_CFG_CHN10_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn10_prd_cfg;
#[doc = "PRD_CFG_CHN10_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN10_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN10_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn10_prd_thshd_cfg::PRD_CFG_CHN10_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn10_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN10_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN10_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN10_PRD_RESULT =
    crate::Reg<prd_cfg_chn10_prd_result::PRD_CFG_CHN10_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn10_prd_result;
#[doc = "PRD_CFG_CHN11_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN11_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN11_PRD_CFG = crate::Reg<prd_cfg_chn11_prd_cfg::PRD_CFG_CHN11_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn11_prd_cfg;
#[doc = "PRD_CFG_CHN11_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN11_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn11_prd_thshd_cfg::PRD_CFG_CHN11_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn11_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN11_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN11_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN11_PRD_RESULT =
    crate::Reg<prd_cfg_chn11_prd_result::PRD_CFG_CHN11_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn11_prd_result;
#[doc = "PRD_CFG_CHN12_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN12_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN12_PRD_CFG = crate::Reg<prd_cfg_chn12_prd_cfg::PRD_CFG_CHN12_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn12_prd_cfg;
#[doc = "PRD_CFG_CHN12_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN12_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN12_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn12_prd_thshd_cfg::PRD_CFG_CHN12_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn12_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN12_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN12_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN12_PRD_RESULT =
    crate::Reg<prd_cfg_chn12_prd_result::PRD_CFG_CHN12_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn12_prd_result;
#[doc = "PRD_CFG_CHN13_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN13_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN13_PRD_CFG = crate::Reg<prd_cfg_chn13_prd_cfg::PRD_CFG_CHN13_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn13_prd_cfg;
#[doc = "PRD_CFG_CHN13_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN13_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN13_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn13_prd_thshd_cfg::PRD_CFG_CHN13_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn13_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN13_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN13_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN13_PRD_RESULT =
    crate::Reg<prd_cfg_chn13_prd_result::PRD_CFG_CHN13_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn13_prd_result;
#[doc = "PRD_CFG_CHN14_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN14_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN14_PRD_CFG = crate::Reg<prd_cfg_chn14_prd_cfg::PRD_CFG_CHN14_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn14_prd_cfg;
#[doc = "PRD_CFG_CHN14_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN14_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN14_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn14_prd_thshd_cfg::PRD_CFG_CHN14_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn14_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN14_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN14_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN14_PRD_RESULT =
    crate::Reg<prd_cfg_chn14_prd_result::PRD_CFG_CHN14_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn14_prd_result;
#[doc = "PRD_CFG_CHN15_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN15_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN15_PRD_CFG = crate::Reg<prd_cfg_chn15_prd_cfg::PRD_CFG_CHN15_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn15_prd_cfg;
#[doc = "PRD_CFG_CHN15_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN15_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN15_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn15_prd_thshd_cfg::PRD_CFG_CHN15_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn15_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN15_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN15_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN15_PRD_RESULT =
    crate::Reg<prd_cfg_chn15_prd_result::PRD_CFG_CHN15_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn15_prd_result;
#[doc = "PRD_CFG_CHN16_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN16_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN16_PRD_CFG = crate::Reg<prd_cfg_chn16_prd_cfg::PRD_CFG_CHN16_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn16_prd_cfg;
#[doc = "PRD_CFG_CHN16_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN16_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN16_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn16_prd_thshd_cfg::PRD_CFG_CHN16_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn16_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN16_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN16_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN16_PRD_RESULT =
    crate::Reg<prd_cfg_chn16_prd_result::PRD_CFG_CHN16_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn16_prd_result;
#[doc = "PRD_CFG_CHN17_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN17_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN17_PRD_CFG = crate::Reg<prd_cfg_chn17_prd_cfg::PRD_CFG_CHN17_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn17_prd_cfg;
#[doc = "PRD_CFG_CHN17_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN17_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN17_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn17_prd_thshd_cfg::PRD_CFG_CHN17_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn17_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN17_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN17_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN17_PRD_RESULT =
    crate::Reg<prd_cfg_chn17_prd_result::PRD_CFG_CHN17_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn17_prd_result;
#[doc = "PRD_CFG_CHN18_PRD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN18_PRD_CFG_SPEC>`"]
pub type PRD_CFG_CHN18_PRD_CFG = crate::Reg<prd_cfg_chn18_prd_cfg::PRD_CFG_CHN18_PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn18_prd_cfg;
#[doc = "PRD_CFG_CHN18_PRD_THSHD_CFG register accessor: an alias for `Reg<PRD_CFG_CHN18_PRD_THSHD_CFG_SPEC>`"]
pub type PRD_CFG_CHN18_PRD_THSHD_CFG =
    crate::Reg<prd_cfg_chn18_prd_thshd_cfg::PRD_CFG_CHN18_PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn18_prd_thshd_cfg;
#[doc = "PRD_CFG_CHN18_PRD_RESULT register accessor: an alias for `Reg<PRD_CFG_CHN18_PRD_RESULT_SPEC>`"]
pub type PRD_CFG_CHN18_PRD_RESULT =
    crate::Reg<prd_cfg_chn18_prd_result::PRD_CFG_CHN18_PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg_chn18_prd_result;
#[doc = "SAMPLE_CFG_CHN0 register accessor: an alias for `Reg<SAMPLE_CFG_CHN0_SPEC>`"]
pub type SAMPLE_CFG_CHN0 = crate::Reg<sample_cfg_chn0::SAMPLE_CFG_CHN0_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn0;
#[doc = "SAMPLE_CFG_CHN1 register accessor: an alias for `Reg<SAMPLE_CFG_CHN1_SPEC>`"]
pub type SAMPLE_CFG_CHN1 = crate::Reg<sample_cfg_chn1::SAMPLE_CFG_CHN1_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn1;
#[doc = "SAMPLE_CFG_CHN2 register accessor: an alias for `Reg<SAMPLE_CFG_CHN2_SPEC>`"]
pub type SAMPLE_CFG_CHN2 = crate::Reg<sample_cfg_chn2::SAMPLE_CFG_CHN2_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn2;
#[doc = "SAMPLE_CFG_CHN3 register accessor: an alias for `Reg<SAMPLE_CFG_CHN3_SPEC>`"]
pub type SAMPLE_CFG_CHN3 = crate::Reg<sample_cfg_chn3::SAMPLE_CFG_CHN3_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn3;
#[doc = "SAMPLE_CFG_CHN4 register accessor: an alias for `Reg<SAMPLE_CFG_CHN4_SPEC>`"]
pub type SAMPLE_CFG_CHN4 = crate::Reg<sample_cfg_chn4::SAMPLE_CFG_CHN4_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn4;
#[doc = "SAMPLE_CFG_CHN5 register accessor: an alias for `Reg<SAMPLE_CFG_CHN5_SPEC>`"]
pub type SAMPLE_CFG_CHN5 = crate::Reg<sample_cfg_chn5::SAMPLE_CFG_CHN5_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn5;
#[doc = "SAMPLE_CFG_CHN6 register accessor: an alias for `Reg<SAMPLE_CFG_CHN6_SPEC>`"]
pub type SAMPLE_CFG_CHN6 = crate::Reg<sample_cfg_chn6::SAMPLE_CFG_CHN6_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn6;
#[doc = "SAMPLE_CFG_CHN7 register accessor: an alias for `Reg<SAMPLE_CFG_CHN7_SPEC>`"]
pub type SAMPLE_CFG_CHN7 = crate::Reg<sample_cfg_chn7::SAMPLE_CFG_CHN7_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn7;
#[doc = "SAMPLE_CFG_CHN8 register accessor: an alias for `Reg<SAMPLE_CFG_CHN8_SPEC>`"]
pub type SAMPLE_CFG_CHN8 = crate::Reg<sample_cfg_chn8::SAMPLE_CFG_CHN8_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn8;
#[doc = "SAMPLE_CFG_CHN9 register accessor: an alias for `Reg<SAMPLE_CFG_CHN9_SPEC>`"]
pub type SAMPLE_CFG_CHN9 = crate::Reg<sample_cfg_chn9::SAMPLE_CFG_CHN9_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn9;
#[doc = "SAMPLE_CFG_CHN10 register accessor: an alias for `Reg<SAMPLE_CFG_CHN10_SPEC>`"]
pub type SAMPLE_CFG_CHN10 = crate::Reg<sample_cfg_chn10::SAMPLE_CFG_CHN10_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn10;
#[doc = "SAMPLE_CFG_CHN11 register accessor: an alias for `Reg<SAMPLE_CFG_CHN11_SPEC>`"]
pub type SAMPLE_CFG_CHN11 = crate::Reg<sample_cfg_chn11::SAMPLE_CFG_CHN11_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn11;
#[doc = "SAMPLE_CFG_CHN12 register accessor: an alias for `Reg<SAMPLE_CFG_CHN12_SPEC>`"]
pub type SAMPLE_CFG_CHN12 = crate::Reg<sample_cfg_chn12::SAMPLE_CFG_CHN12_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn12;
#[doc = "SAMPLE_CFG_CHN13 register accessor: an alias for `Reg<SAMPLE_CFG_CHN13_SPEC>`"]
pub type SAMPLE_CFG_CHN13 = crate::Reg<sample_cfg_chn13::SAMPLE_CFG_CHN13_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn13;
#[doc = "SAMPLE_CFG_CHN14 register accessor: an alias for `Reg<SAMPLE_CFG_CHN14_SPEC>`"]
pub type SAMPLE_CFG_CHN14 = crate::Reg<sample_cfg_chn14::SAMPLE_CFG_CHN14_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn14;
#[doc = "SAMPLE_CFG_CHN15 register accessor: an alias for `Reg<SAMPLE_CFG_CHN15_SPEC>`"]
pub type SAMPLE_CFG_CHN15 = crate::Reg<sample_cfg_chn15::SAMPLE_CFG_CHN15_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn15;
#[doc = "SAMPLE_CFG_CHN16 register accessor: an alias for `Reg<SAMPLE_CFG_CHN16_SPEC>`"]
pub type SAMPLE_CFG_CHN16 = crate::Reg<sample_cfg_chn16::SAMPLE_CFG_CHN16_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn16;
#[doc = "SAMPLE_CFG_CHN17 register accessor: an alias for `Reg<SAMPLE_CFG_CHN17_SPEC>`"]
pub type SAMPLE_CFG_CHN17 = crate::Reg<sample_cfg_chn17::SAMPLE_CFG_CHN17_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn17;
#[doc = "SAMPLE_CFG_CHN18 register accessor: an alias for `Reg<SAMPLE_CFG_CHN18_SPEC>`"]
pub type SAMPLE_CFG_CHN18 = crate::Reg<sample_cfg_chn18::SAMPLE_CFG_CHN18_SPEC>;
#[doc = "No description avaiable"]
pub mod sample_cfg_chn18;
#[doc = "CONV_CFG1 register accessor: an alias for `Reg<CONV_CFG1_SPEC>`"]
pub type CONV_CFG1 = crate::Reg<conv_cfg1::CONV_CFG1_SPEC>;
#[doc = "No description avaiable"]
pub mod conv_cfg1;
#[doc = "ADC_CFG0 register accessor: an alias for `Reg<ADC_CFG0_SPEC>`"]
pub type ADC_CFG0 = crate::Reg<adc_cfg0::ADC_CFG0_SPEC>;
#[doc = "No description avaiable"]
pub mod adc_cfg0;
#[doc = "INT_STS register accessor: an alias for `Reg<INT_STS_SPEC>`"]
pub type INT_STS = crate::Reg<int_sts::INT_STS_SPEC>;
#[doc = "No description avaiable"]
pub mod int_sts;
#[doc = "INT_EN register accessor: an alias for `Reg<INT_EN_SPEC>`"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "No description avaiable"]
pub mod int_en;
#[doc = "ANA_CTRL0 register accessor: an alias for `Reg<ANA_CTRL0_SPEC>`"]
pub type ANA_CTRL0 = crate::Reg<ana_ctrl0::ANA_CTRL0_SPEC>;
#[doc = "No description avaiable"]
pub mod ana_ctrl0;
#[doc = "ANA_CTRL1 register accessor: an alias for `Reg<ANA_CTRL1_SPEC>`"]
pub type ANA_CTRL1 = crate::Reg<ana_ctrl1::ANA_CTRL1_SPEC>;
#[doc = "No description avaiable"]
pub mod ana_ctrl1;
#[doc = "ANA_STATUS register accessor: an alias for `Reg<ANA_STATUS_SPEC>`"]
pub type ANA_STATUS = crate::Reg<ana_status::ANA_STATUS_SPEC>;
#[doc = "No description avaiable"]
pub mod ana_status;
