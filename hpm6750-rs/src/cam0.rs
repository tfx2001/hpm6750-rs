#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x04 - Interrupt Enable Register"]
    pub int_en: crate::Reg<int_en::INT_EN_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Control 2 Register"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    _reserved3: [u8; 0x10],
    #[doc = "0x24 - Status Register"]
    pub sta: crate::Reg<sta::STA_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x30 - Pixel DMA Frame Buffer 1 Address"]
    pub dmasa_fb1: crate::Reg<dmasa_fb1::DMASA_FB1_SPEC>,
    #[doc = "0x34 - Pixel DMA Frame Buffer 2 Address"]
    pub dmasa_fb2: crate::Reg<dmasa_fb2::DMASA_FB2_SPEC>,
    #[doc = "0x38 - Buffer Parameters Register"]
    pub buf_para: crate::Reg<buf_para::BUF_PARA_SPEC>,
    #[doc = "0x3c - Ideal Image Size Register"]
    pub ideal_wn_size: crate::Reg<ideal_wn_size::IDEAL_WN_SIZE_SPEC>,
    _reserved8: [u8; 0x0c],
    #[doc = "0x4c - Control CR18 Register"]
    pub cr18: crate::Reg<cr18::CR18_SPEC>,
    #[doc = "0x50 - Pixel UV DMA Frame Buffer 1 Address"]
    pub dmasa_uv1: crate::Reg<dmasa_uv1::DMASA_UV1_SPEC>,
    #[doc = "0x54 - Pixel UV DMA Frame Buffer 2 Address"]
    pub dmasa_uv2: crate::Reg<dmasa_uv2::DMASA_UV2_SPEC>,
    #[doc = "0x58 - Control CR20 Register"]
    pub cr20: crate::Reg<cr20::CR20_SPEC>,
    #[doc = "0x5c - Max Window Size Register"]
    pub max_wn_cycle: crate::Reg<max_wn_cycle::MAX_WN_CYCLE_SPEC>,
    _reserved13: [u8; 0x10],
    #[doc = "0x70 - Color Space Conversion Config Register 0"]
    pub csc_coef0: crate::Reg<csc_coef0::CSC_COEF0_SPEC>,
    #[doc = "0x74 - Color Space Conversion Config Register 1"]
    pub csc_coef1: crate::Reg<csc_coef1::CSC_COEF1_SPEC>,
    #[doc = "0x78 - Color Space Conversion Config Register 2"]
    pub csc_coef2: crate::Reg<csc_coef2::CSC_COEF2_SPEC>,
    #[doc = "0x7c - Low Color Key Register"]
    pub clrkey_low: crate::Reg<clrkey_low::CLRKEY_LOW_SPEC>,
    #[doc = "0x80 - High Color Key Register"]
    pub clrkey_high: crate::Reg<clrkey_high::CLRKEY_HIGH_SPEC>,
    _reserved18: [u8; 0x0c],
    #[doc = "0x90 - Histogram Registers"]
    pub histogram_fifo_data0: crate::Reg<histogram_fifo_data0::HISTOGRAM_FIFO_DATA0_SPEC>,
    #[doc = "0x94 - Histogram Registers"]
    pub histogram_fifo_data1: crate::Reg<histogram_fifo_data1::HISTOGRAM_FIFO_DATA1_SPEC>,
    #[doc = "0x98 - Histogram Registers"]
    pub histogram_fifo_data2: crate::Reg<histogram_fifo_data2::HISTOGRAM_FIFO_DATA2_SPEC>,
    #[doc = "0x9c - Histogram Registers"]
    pub histogram_fifo_data3: crate::Reg<histogram_fifo_data3::HISTOGRAM_FIFO_DATA3_SPEC>,
    #[doc = "0xa0 - Histogram Registers"]
    pub histogram_fifo_data4: crate::Reg<histogram_fifo_data4::HISTOGRAM_FIFO_DATA4_SPEC>,
    #[doc = "0xa4 - Histogram Registers"]
    pub histogram_fifo_data5: crate::Reg<histogram_fifo_data5::HISTOGRAM_FIFO_DATA5_SPEC>,
    #[doc = "0xa8 - Histogram Registers"]
    pub histogram_fifo_data6: crate::Reg<histogram_fifo_data6::HISTOGRAM_FIFO_DATA6_SPEC>,
    #[doc = "0xac - Histogram Registers"]
    pub histogram_fifo_data7: crate::Reg<histogram_fifo_data7::HISTOGRAM_FIFO_DATA7_SPEC>,
    #[doc = "0xb0 - Histogram Registers"]
    pub histogram_fifo_data8: crate::Reg<histogram_fifo_data8::HISTOGRAM_FIFO_DATA8_SPEC>,
    #[doc = "0xb4 - Histogram Registers"]
    pub histogram_fifo_data9: crate::Reg<histogram_fifo_data9::HISTOGRAM_FIFO_DATA9_SPEC>,
    #[doc = "0xb8 - Histogram Registers"]
    pub histogram_fifo_data10: crate::Reg<histogram_fifo_data10::HISTOGRAM_FIFO_DATA10_SPEC>,
    #[doc = "0xbc - Histogram Registers"]
    pub histogram_fifo_data11: crate::Reg<histogram_fifo_data11::HISTOGRAM_FIFO_DATA11_SPEC>,
    #[doc = "0xc0 - Histogram Registers"]
    pub histogram_fifo_data12: crate::Reg<histogram_fifo_data12::HISTOGRAM_FIFO_DATA12_SPEC>,
    #[doc = "0xc4 - Histogram Registers"]
    pub histogram_fifo_data13: crate::Reg<histogram_fifo_data13::HISTOGRAM_FIFO_DATA13_SPEC>,
    #[doc = "0xc8 - Histogram Registers"]
    pub histogram_fifo_data14: crate::Reg<histogram_fifo_data14::HISTOGRAM_FIFO_DATA14_SPEC>,
    #[doc = "0xcc - Histogram Registers"]
    pub histogram_fifo_data15: crate::Reg<histogram_fifo_data15::HISTOGRAM_FIFO_DATA15_SPEC>,
    #[doc = "0xd0 - Histogram Registers"]
    pub histogram_fifo_data16: crate::Reg<histogram_fifo_data16::HISTOGRAM_FIFO_DATA16_SPEC>,
    #[doc = "0xd4 - Histogram Registers"]
    pub histogram_fifo_data17: crate::Reg<histogram_fifo_data17::HISTOGRAM_FIFO_DATA17_SPEC>,
    #[doc = "0xd8 - Histogram Registers"]
    pub histogram_fifo_data18: crate::Reg<histogram_fifo_data18::HISTOGRAM_FIFO_DATA18_SPEC>,
    #[doc = "0xdc - Histogram Registers"]
    pub histogram_fifo_data19: crate::Reg<histogram_fifo_data19::HISTOGRAM_FIFO_DATA19_SPEC>,
    #[doc = "0xe0 - Histogram Registers"]
    pub histogram_fifo_data20: crate::Reg<histogram_fifo_data20::HISTOGRAM_FIFO_DATA20_SPEC>,
    #[doc = "0xe4 - Histogram Registers"]
    pub histogram_fifo_data21: crate::Reg<histogram_fifo_data21::HISTOGRAM_FIFO_DATA21_SPEC>,
    #[doc = "0xe8 - Histogram Registers"]
    pub histogram_fifo_data22: crate::Reg<histogram_fifo_data22::HISTOGRAM_FIFO_DATA22_SPEC>,
    #[doc = "0xec - Histogram Registers"]
    pub histogram_fifo_data23: crate::Reg<histogram_fifo_data23::HISTOGRAM_FIFO_DATA23_SPEC>,
    #[doc = "0xf0 - Histogram Registers"]
    pub histogram_fifo_data24: crate::Reg<histogram_fifo_data24::HISTOGRAM_FIFO_DATA24_SPEC>,
    #[doc = "0xf4 - Histogram Registers"]
    pub histogram_fifo_data25: crate::Reg<histogram_fifo_data25::HISTOGRAM_FIFO_DATA25_SPEC>,
    #[doc = "0xf8 - Histogram Registers"]
    pub histogram_fifo_data26: crate::Reg<histogram_fifo_data26::HISTOGRAM_FIFO_DATA26_SPEC>,
    #[doc = "0xfc - Histogram Registers"]
    pub histogram_fifo_data27: crate::Reg<histogram_fifo_data27::HISTOGRAM_FIFO_DATA27_SPEC>,
    #[doc = "0x100 - Histogram Registers"]
    pub histogram_fifo_data28: crate::Reg<histogram_fifo_data28::HISTOGRAM_FIFO_DATA28_SPEC>,
    #[doc = "0x104 - Histogram Registers"]
    pub histogram_fifo_data29: crate::Reg<histogram_fifo_data29::HISTOGRAM_FIFO_DATA29_SPEC>,
    #[doc = "0x108 - Histogram Registers"]
    pub histogram_fifo_data30: crate::Reg<histogram_fifo_data30::HISTOGRAM_FIFO_DATA30_SPEC>,
    #[doc = "0x10c - Histogram Registers"]
    pub histogram_fifo_data31: crate::Reg<histogram_fifo_data31::HISTOGRAM_FIFO_DATA31_SPEC>,
    #[doc = "0x110 - Histogram Registers"]
    pub histogram_fifo_data32: crate::Reg<histogram_fifo_data32::HISTOGRAM_FIFO_DATA32_SPEC>,
    #[doc = "0x114 - Histogram Registers"]
    pub histogram_fifo_data33: crate::Reg<histogram_fifo_data33::HISTOGRAM_FIFO_DATA33_SPEC>,
    #[doc = "0x118 - Histogram Registers"]
    pub histogram_fifo_data34: crate::Reg<histogram_fifo_data34::HISTOGRAM_FIFO_DATA34_SPEC>,
    #[doc = "0x11c - Histogram Registers"]
    pub histogram_fifo_data35: crate::Reg<histogram_fifo_data35::HISTOGRAM_FIFO_DATA35_SPEC>,
    #[doc = "0x120 - Histogram Registers"]
    pub histogram_fifo_data36: crate::Reg<histogram_fifo_data36::HISTOGRAM_FIFO_DATA36_SPEC>,
    #[doc = "0x124 - Histogram Registers"]
    pub histogram_fifo_data37: crate::Reg<histogram_fifo_data37::HISTOGRAM_FIFO_DATA37_SPEC>,
    #[doc = "0x128 - Histogram Registers"]
    pub histogram_fifo_data38: crate::Reg<histogram_fifo_data38::HISTOGRAM_FIFO_DATA38_SPEC>,
    #[doc = "0x12c - Histogram Registers"]
    pub histogram_fifo_data39: crate::Reg<histogram_fifo_data39::HISTOGRAM_FIFO_DATA39_SPEC>,
    #[doc = "0x130 - Histogram Registers"]
    pub histogram_fifo_data40: crate::Reg<histogram_fifo_data40::HISTOGRAM_FIFO_DATA40_SPEC>,
    #[doc = "0x134 - Histogram Registers"]
    pub histogram_fifo_data41: crate::Reg<histogram_fifo_data41::HISTOGRAM_FIFO_DATA41_SPEC>,
    #[doc = "0x138 - Histogram Registers"]
    pub histogram_fifo_data42: crate::Reg<histogram_fifo_data42::HISTOGRAM_FIFO_DATA42_SPEC>,
    #[doc = "0x13c - Histogram Registers"]
    pub histogram_fifo_data43: crate::Reg<histogram_fifo_data43::HISTOGRAM_FIFO_DATA43_SPEC>,
    #[doc = "0x140 - Histogram Registers"]
    pub histogram_fifo_data44: crate::Reg<histogram_fifo_data44::HISTOGRAM_FIFO_DATA44_SPEC>,
    #[doc = "0x144 - Histogram Registers"]
    pub histogram_fifo_data45: crate::Reg<histogram_fifo_data45::HISTOGRAM_FIFO_DATA45_SPEC>,
    #[doc = "0x148 - Histogram Registers"]
    pub histogram_fifo_data46: crate::Reg<histogram_fifo_data46::HISTOGRAM_FIFO_DATA46_SPEC>,
    #[doc = "0x14c - Histogram Registers"]
    pub histogram_fifo_data47: crate::Reg<histogram_fifo_data47::HISTOGRAM_FIFO_DATA47_SPEC>,
    #[doc = "0x150 - Histogram Registers"]
    pub histogram_fifo_data48: crate::Reg<histogram_fifo_data48::HISTOGRAM_FIFO_DATA48_SPEC>,
    #[doc = "0x154 - Histogram Registers"]
    pub histogram_fifo_data49: crate::Reg<histogram_fifo_data49::HISTOGRAM_FIFO_DATA49_SPEC>,
    #[doc = "0x158 - Histogram Registers"]
    pub histogram_fifo_data50: crate::Reg<histogram_fifo_data50::HISTOGRAM_FIFO_DATA50_SPEC>,
    #[doc = "0x15c - Histogram Registers"]
    pub histogram_fifo_data51: crate::Reg<histogram_fifo_data51::HISTOGRAM_FIFO_DATA51_SPEC>,
    #[doc = "0x160 - Histogram Registers"]
    pub histogram_fifo_data52: crate::Reg<histogram_fifo_data52::HISTOGRAM_FIFO_DATA52_SPEC>,
    #[doc = "0x164 - Histogram Registers"]
    pub histogram_fifo_data53: crate::Reg<histogram_fifo_data53::HISTOGRAM_FIFO_DATA53_SPEC>,
    #[doc = "0x168 - Histogram Registers"]
    pub histogram_fifo_data54: crate::Reg<histogram_fifo_data54::HISTOGRAM_FIFO_DATA54_SPEC>,
    #[doc = "0x16c - Histogram Registers"]
    pub histogram_fifo_data55: crate::Reg<histogram_fifo_data55::HISTOGRAM_FIFO_DATA55_SPEC>,
    #[doc = "0x170 - Histogram Registers"]
    pub histogram_fifo_data56: crate::Reg<histogram_fifo_data56::HISTOGRAM_FIFO_DATA56_SPEC>,
    #[doc = "0x174 - Histogram Registers"]
    pub histogram_fifo_data57: crate::Reg<histogram_fifo_data57::HISTOGRAM_FIFO_DATA57_SPEC>,
    #[doc = "0x178 - Histogram Registers"]
    pub histogram_fifo_data58: crate::Reg<histogram_fifo_data58::HISTOGRAM_FIFO_DATA58_SPEC>,
    #[doc = "0x17c - Histogram Registers"]
    pub histogram_fifo_data59: crate::Reg<histogram_fifo_data59::HISTOGRAM_FIFO_DATA59_SPEC>,
    #[doc = "0x180 - Histogram Registers"]
    pub histogram_fifo_data60: crate::Reg<histogram_fifo_data60::HISTOGRAM_FIFO_DATA60_SPEC>,
    #[doc = "0x184 - Histogram Registers"]
    pub histogram_fifo_data61: crate::Reg<histogram_fifo_data61::HISTOGRAM_FIFO_DATA61_SPEC>,
    #[doc = "0x188 - Histogram Registers"]
    pub histogram_fifo_data62: crate::Reg<histogram_fifo_data62::HISTOGRAM_FIFO_DATA62_SPEC>,
    #[doc = "0x18c - Histogram Registers"]
    pub histogram_fifo_data63: crate::Reg<histogram_fifo_data63::HISTOGRAM_FIFO_DATA63_SPEC>,
    #[doc = "0x190 - Histogram Registers"]
    pub histogram_fifo_data64: crate::Reg<histogram_fifo_data64::HISTOGRAM_FIFO_DATA64_SPEC>,
    #[doc = "0x194 - Histogram Registers"]
    pub histogram_fifo_data65: crate::Reg<histogram_fifo_data65::HISTOGRAM_FIFO_DATA65_SPEC>,
    #[doc = "0x198 - Histogram Registers"]
    pub histogram_fifo_data66: crate::Reg<histogram_fifo_data66::HISTOGRAM_FIFO_DATA66_SPEC>,
    #[doc = "0x19c - Histogram Registers"]
    pub histogram_fifo_data67: crate::Reg<histogram_fifo_data67::HISTOGRAM_FIFO_DATA67_SPEC>,
    #[doc = "0x1a0 - Histogram Registers"]
    pub histogram_fifo_data68: crate::Reg<histogram_fifo_data68::HISTOGRAM_FIFO_DATA68_SPEC>,
    #[doc = "0x1a4 - Histogram Registers"]
    pub histogram_fifo_data69: crate::Reg<histogram_fifo_data69::HISTOGRAM_FIFO_DATA69_SPEC>,
    #[doc = "0x1a8 - Histogram Registers"]
    pub histogram_fifo_data70: crate::Reg<histogram_fifo_data70::HISTOGRAM_FIFO_DATA70_SPEC>,
    #[doc = "0x1ac - Histogram Registers"]
    pub histogram_fifo_data71: crate::Reg<histogram_fifo_data71::HISTOGRAM_FIFO_DATA71_SPEC>,
    #[doc = "0x1b0 - Histogram Registers"]
    pub histogram_fifo_data72: crate::Reg<histogram_fifo_data72::HISTOGRAM_FIFO_DATA72_SPEC>,
    #[doc = "0x1b4 - Histogram Registers"]
    pub histogram_fifo_data73: crate::Reg<histogram_fifo_data73::HISTOGRAM_FIFO_DATA73_SPEC>,
    #[doc = "0x1b8 - Histogram Registers"]
    pub histogram_fifo_data74: crate::Reg<histogram_fifo_data74::HISTOGRAM_FIFO_DATA74_SPEC>,
    #[doc = "0x1bc - Histogram Registers"]
    pub histogram_fifo_data75: crate::Reg<histogram_fifo_data75::HISTOGRAM_FIFO_DATA75_SPEC>,
    #[doc = "0x1c0 - Histogram Registers"]
    pub histogram_fifo_data76: crate::Reg<histogram_fifo_data76::HISTOGRAM_FIFO_DATA76_SPEC>,
    #[doc = "0x1c4 - Histogram Registers"]
    pub histogram_fifo_data77: crate::Reg<histogram_fifo_data77::HISTOGRAM_FIFO_DATA77_SPEC>,
    #[doc = "0x1c8 - Histogram Registers"]
    pub histogram_fifo_data78: crate::Reg<histogram_fifo_data78::HISTOGRAM_FIFO_DATA78_SPEC>,
    #[doc = "0x1cc - Histogram Registers"]
    pub histogram_fifo_data79: crate::Reg<histogram_fifo_data79::HISTOGRAM_FIFO_DATA79_SPEC>,
    #[doc = "0x1d0 - Histogram Registers"]
    pub histogram_fifo_data80: crate::Reg<histogram_fifo_data80::HISTOGRAM_FIFO_DATA80_SPEC>,
    #[doc = "0x1d4 - Histogram Registers"]
    pub histogram_fifo_data81: crate::Reg<histogram_fifo_data81::HISTOGRAM_FIFO_DATA81_SPEC>,
    #[doc = "0x1d8 - Histogram Registers"]
    pub histogram_fifo_data82: crate::Reg<histogram_fifo_data82::HISTOGRAM_FIFO_DATA82_SPEC>,
    #[doc = "0x1dc - Histogram Registers"]
    pub histogram_fifo_data83: crate::Reg<histogram_fifo_data83::HISTOGRAM_FIFO_DATA83_SPEC>,
    #[doc = "0x1e0 - Histogram Registers"]
    pub histogram_fifo_data84: crate::Reg<histogram_fifo_data84::HISTOGRAM_FIFO_DATA84_SPEC>,
    #[doc = "0x1e4 - Histogram Registers"]
    pub histogram_fifo_data85: crate::Reg<histogram_fifo_data85::HISTOGRAM_FIFO_DATA85_SPEC>,
    #[doc = "0x1e8 - Histogram Registers"]
    pub histogram_fifo_data86: crate::Reg<histogram_fifo_data86::HISTOGRAM_FIFO_DATA86_SPEC>,
    #[doc = "0x1ec - Histogram Registers"]
    pub histogram_fifo_data87: crate::Reg<histogram_fifo_data87::HISTOGRAM_FIFO_DATA87_SPEC>,
    #[doc = "0x1f0 - Histogram Registers"]
    pub histogram_fifo_data88: crate::Reg<histogram_fifo_data88::HISTOGRAM_FIFO_DATA88_SPEC>,
    #[doc = "0x1f4 - Histogram Registers"]
    pub histogram_fifo_data89: crate::Reg<histogram_fifo_data89::HISTOGRAM_FIFO_DATA89_SPEC>,
    #[doc = "0x1f8 - Histogram Registers"]
    pub histogram_fifo_data90: crate::Reg<histogram_fifo_data90::HISTOGRAM_FIFO_DATA90_SPEC>,
    #[doc = "0x1fc - Histogram Registers"]
    pub histogram_fifo_data91: crate::Reg<histogram_fifo_data91::HISTOGRAM_FIFO_DATA91_SPEC>,
    #[doc = "0x200 - Histogram Registers"]
    pub histogram_fifo_data92: crate::Reg<histogram_fifo_data92::HISTOGRAM_FIFO_DATA92_SPEC>,
    #[doc = "0x204 - Histogram Registers"]
    pub histogram_fifo_data93: crate::Reg<histogram_fifo_data93::HISTOGRAM_FIFO_DATA93_SPEC>,
    #[doc = "0x208 - Histogram Registers"]
    pub histogram_fifo_data94: crate::Reg<histogram_fifo_data94::HISTOGRAM_FIFO_DATA94_SPEC>,
    #[doc = "0x20c - Histogram Registers"]
    pub histogram_fifo_data95: crate::Reg<histogram_fifo_data95::HISTOGRAM_FIFO_DATA95_SPEC>,
    #[doc = "0x210 - Histogram Registers"]
    pub histogram_fifo_data96: crate::Reg<histogram_fifo_data96::HISTOGRAM_FIFO_DATA96_SPEC>,
    #[doc = "0x214 - Histogram Registers"]
    pub histogram_fifo_data97: crate::Reg<histogram_fifo_data97::HISTOGRAM_FIFO_DATA97_SPEC>,
    #[doc = "0x218 - Histogram Registers"]
    pub histogram_fifo_data98: crate::Reg<histogram_fifo_data98::HISTOGRAM_FIFO_DATA98_SPEC>,
    #[doc = "0x21c - Histogram Registers"]
    pub histogram_fifo_data99: crate::Reg<histogram_fifo_data99::HISTOGRAM_FIFO_DATA99_SPEC>,
    #[doc = "0x220 - Histogram Registers"]
    pub histogram_fifo_data100: crate::Reg<histogram_fifo_data100::HISTOGRAM_FIFO_DATA100_SPEC>,
    #[doc = "0x224 - Histogram Registers"]
    pub histogram_fifo_data101: crate::Reg<histogram_fifo_data101::HISTOGRAM_FIFO_DATA101_SPEC>,
    #[doc = "0x228 - Histogram Registers"]
    pub histogram_fifo_data102: crate::Reg<histogram_fifo_data102::HISTOGRAM_FIFO_DATA102_SPEC>,
    #[doc = "0x22c - Histogram Registers"]
    pub histogram_fifo_data103: crate::Reg<histogram_fifo_data103::HISTOGRAM_FIFO_DATA103_SPEC>,
    #[doc = "0x230 - Histogram Registers"]
    pub histogram_fifo_data104: crate::Reg<histogram_fifo_data104::HISTOGRAM_FIFO_DATA104_SPEC>,
    #[doc = "0x234 - Histogram Registers"]
    pub histogram_fifo_data105: crate::Reg<histogram_fifo_data105::HISTOGRAM_FIFO_DATA105_SPEC>,
    #[doc = "0x238 - Histogram Registers"]
    pub histogram_fifo_data106: crate::Reg<histogram_fifo_data106::HISTOGRAM_FIFO_DATA106_SPEC>,
    #[doc = "0x23c - Histogram Registers"]
    pub histogram_fifo_data107: crate::Reg<histogram_fifo_data107::HISTOGRAM_FIFO_DATA107_SPEC>,
    #[doc = "0x240 - Histogram Registers"]
    pub histogram_fifo_data108: crate::Reg<histogram_fifo_data108::HISTOGRAM_FIFO_DATA108_SPEC>,
    #[doc = "0x244 - Histogram Registers"]
    pub histogram_fifo_data109: crate::Reg<histogram_fifo_data109::HISTOGRAM_FIFO_DATA109_SPEC>,
    #[doc = "0x248 - Histogram Registers"]
    pub histogram_fifo_data110: crate::Reg<histogram_fifo_data110::HISTOGRAM_FIFO_DATA110_SPEC>,
    #[doc = "0x24c - Histogram Registers"]
    pub histogram_fifo_data111: crate::Reg<histogram_fifo_data111::HISTOGRAM_FIFO_DATA111_SPEC>,
    #[doc = "0x250 - Histogram Registers"]
    pub histogram_fifo_data112: crate::Reg<histogram_fifo_data112::HISTOGRAM_FIFO_DATA112_SPEC>,
    #[doc = "0x254 - Histogram Registers"]
    pub histogram_fifo_data113: crate::Reg<histogram_fifo_data113::HISTOGRAM_FIFO_DATA113_SPEC>,
    #[doc = "0x258 - Histogram Registers"]
    pub histogram_fifo_data114: crate::Reg<histogram_fifo_data114::HISTOGRAM_FIFO_DATA114_SPEC>,
    #[doc = "0x25c - Histogram Registers"]
    pub histogram_fifo_data115: crate::Reg<histogram_fifo_data115::HISTOGRAM_FIFO_DATA115_SPEC>,
    #[doc = "0x260 - Histogram Registers"]
    pub histogram_fifo_data116: crate::Reg<histogram_fifo_data116::HISTOGRAM_FIFO_DATA116_SPEC>,
    #[doc = "0x264 - Histogram Registers"]
    pub histogram_fifo_data117: crate::Reg<histogram_fifo_data117::HISTOGRAM_FIFO_DATA117_SPEC>,
    #[doc = "0x268 - Histogram Registers"]
    pub histogram_fifo_data118: crate::Reg<histogram_fifo_data118::HISTOGRAM_FIFO_DATA118_SPEC>,
    #[doc = "0x26c - Histogram Registers"]
    pub histogram_fifo_data119: crate::Reg<histogram_fifo_data119::HISTOGRAM_FIFO_DATA119_SPEC>,
    #[doc = "0x270 - Histogram Registers"]
    pub histogram_fifo_data120: crate::Reg<histogram_fifo_data120::HISTOGRAM_FIFO_DATA120_SPEC>,
    #[doc = "0x274 - Histogram Registers"]
    pub histogram_fifo_data121: crate::Reg<histogram_fifo_data121::HISTOGRAM_FIFO_DATA121_SPEC>,
    #[doc = "0x278 - Histogram Registers"]
    pub histogram_fifo_data122: crate::Reg<histogram_fifo_data122::HISTOGRAM_FIFO_DATA122_SPEC>,
    #[doc = "0x27c - Histogram Registers"]
    pub histogram_fifo_data123: crate::Reg<histogram_fifo_data123::HISTOGRAM_FIFO_DATA123_SPEC>,
    #[doc = "0x280 - Histogram Registers"]
    pub histogram_fifo_data124: crate::Reg<histogram_fifo_data124::HISTOGRAM_FIFO_DATA124_SPEC>,
    #[doc = "0x284 - Histogram Registers"]
    pub histogram_fifo_data125: crate::Reg<histogram_fifo_data125::HISTOGRAM_FIFO_DATA125_SPEC>,
    #[doc = "0x288 - Histogram Registers"]
    pub histogram_fifo_data126: crate::Reg<histogram_fifo_data126::HISTOGRAM_FIFO_DATA126_SPEC>,
    #[doc = "0x28c - Histogram Registers"]
    pub histogram_fifo_data127: crate::Reg<histogram_fifo_data127::HISTOGRAM_FIFO_DATA127_SPEC>,
    #[doc = "0x290 - Histogram Registers"]
    pub histogram_fifo_data128: crate::Reg<histogram_fifo_data128::HISTOGRAM_FIFO_DATA128_SPEC>,
    #[doc = "0x294 - Histogram Registers"]
    pub histogram_fifo_data129: crate::Reg<histogram_fifo_data129::HISTOGRAM_FIFO_DATA129_SPEC>,
    #[doc = "0x298 - Histogram Registers"]
    pub histogram_fifo_data130: crate::Reg<histogram_fifo_data130::HISTOGRAM_FIFO_DATA130_SPEC>,
    #[doc = "0x29c - Histogram Registers"]
    pub histogram_fifo_data131: crate::Reg<histogram_fifo_data131::HISTOGRAM_FIFO_DATA131_SPEC>,
    #[doc = "0x2a0 - Histogram Registers"]
    pub histogram_fifo_data132: crate::Reg<histogram_fifo_data132::HISTOGRAM_FIFO_DATA132_SPEC>,
    #[doc = "0x2a4 - Histogram Registers"]
    pub histogram_fifo_data133: crate::Reg<histogram_fifo_data133::HISTOGRAM_FIFO_DATA133_SPEC>,
    #[doc = "0x2a8 - Histogram Registers"]
    pub histogram_fifo_data134: crate::Reg<histogram_fifo_data134::HISTOGRAM_FIFO_DATA134_SPEC>,
    #[doc = "0x2ac - Histogram Registers"]
    pub histogram_fifo_data135: crate::Reg<histogram_fifo_data135::HISTOGRAM_FIFO_DATA135_SPEC>,
    #[doc = "0x2b0 - Histogram Registers"]
    pub histogram_fifo_data136: crate::Reg<histogram_fifo_data136::HISTOGRAM_FIFO_DATA136_SPEC>,
    #[doc = "0x2b4 - Histogram Registers"]
    pub histogram_fifo_data137: crate::Reg<histogram_fifo_data137::HISTOGRAM_FIFO_DATA137_SPEC>,
    #[doc = "0x2b8 - Histogram Registers"]
    pub histogram_fifo_data138: crate::Reg<histogram_fifo_data138::HISTOGRAM_FIFO_DATA138_SPEC>,
    #[doc = "0x2bc - Histogram Registers"]
    pub histogram_fifo_data139: crate::Reg<histogram_fifo_data139::HISTOGRAM_FIFO_DATA139_SPEC>,
    #[doc = "0x2c0 - Histogram Registers"]
    pub histogram_fifo_data140: crate::Reg<histogram_fifo_data140::HISTOGRAM_FIFO_DATA140_SPEC>,
    #[doc = "0x2c4 - Histogram Registers"]
    pub histogram_fifo_data141: crate::Reg<histogram_fifo_data141::HISTOGRAM_FIFO_DATA141_SPEC>,
    #[doc = "0x2c8 - Histogram Registers"]
    pub histogram_fifo_data142: crate::Reg<histogram_fifo_data142::HISTOGRAM_FIFO_DATA142_SPEC>,
    #[doc = "0x2cc - Histogram Registers"]
    pub histogram_fifo_data143: crate::Reg<histogram_fifo_data143::HISTOGRAM_FIFO_DATA143_SPEC>,
    #[doc = "0x2d0 - Histogram Registers"]
    pub histogram_fifo_data144: crate::Reg<histogram_fifo_data144::HISTOGRAM_FIFO_DATA144_SPEC>,
    #[doc = "0x2d4 - Histogram Registers"]
    pub histogram_fifo_data145: crate::Reg<histogram_fifo_data145::HISTOGRAM_FIFO_DATA145_SPEC>,
    #[doc = "0x2d8 - Histogram Registers"]
    pub histogram_fifo_data146: crate::Reg<histogram_fifo_data146::HISTOGRAM_FIFO_DATA146_SPEC>,
    #[doc = "0x2dc - Histogram Registers"]
    pub histogram_fifo_data147: crate::Reg<histogram_fifo_data147::HISTOGRAM_FIFO_DATA147_SPEC>,
    #[doc = "0x2e0 - Histogram Registers"]
    pub histogram_fifo_data148: crate::Reg<histogram_fifo_data148::HISTOGRAM_FIFO_DATA148_SPEC>,
    #[doc = "0x2e4 - Histogram Registers"]
    pub histogram_fifo_data149: crate::Reg<histogram_fifo_data149::HISTOGRAM_FIFO_DATA149_SPEC>,
    #[doc = "0x2e8 - Histogram Registers"]
    pub histogram_fifo_data150: crate::Reg<histogram_fifo_data150::HISTOGRAM_FIFO_DATA150_SPEC>,
    #[doc = "0x2ec - Histogram Registers"]
    pub histogram_fifo_data151: crate::Reg<histogram_fifo_data151::HISTOGRAM_FIFO_DATA151_SPEC>,
    #[doc = "0x2f0 - Histogram Registers"]
    pub histogram_fifo_data152: crate::Reg<histogram_fifo_data152::HISTOGRAM_FIFO_DATA152_SPEC>,
    #[doc = "0x2f4 - Histogram Registers"]
    pub histogram_fifo_data153: crate::Reg<histogram_fifo_data153::HISTOGRAM_FIFO_DATA153_SPEC>,
    #[doc = "0x2f8 - Histogram Registers"]
    pub histogram_fifo_data154: crate::Reg<histogram_fifo_data154::HISTOGRAM_FIFO_DATA154_SPEC>,
    #[doc = "0x2fc - Histogram Registers"]
    pub histogram_fifo_data155: crate::Reg<histogram_fifo_data155::HISTOGRAM_FIFO_DATA155_SPEC>,
    #[doc = "0x300 - Histogram Registers"]
    pub histogram_fifo_data156: crate::Reg<histogram_fifo_data156::HISTOGRAM_FIFO_DATA156_SPEC>,
    #[doc = "0x304 - Histogram Registers"]
    pub histogram_fifo_data157: crate::Reg<histogram_fifo_data157::HISTOGRAM_FIFO_DATA157_SPEC>,
    #[doc = "0x308 - Histogram Registers"]
    pub histogram_fifo_data158: crate::Reg<histogram_fifo_data158::HISTOGRAM_FIFO_DATA158_SPEC>,
    #[doc = "0x30c - Histogram Registers"]
    pub histogram_fifo_data159: crate::Reg<histogram_fifo_data159::HISTOGRAM_FIFO_DATA159_SPEC>,
    #[doc = "0x310 - Histogram Registers"]
    pub histogram_fifo_data160: crate::Reg<histogram_fifo_data160::HISTOGRAM_FIFO_DATA160_SPEC>,
    #[doc = "0x314 - Histogram Registers"]
    pub histogram_fifo_data161: crate::Reg<histogram_fifo_data161::HISTOGRAM_FIFO_DATA161_SPEC>,
    #[doc = "0x318 - Histogram Registers"]
    pub histogram_fifo_data162: crate::Reg<histogram_fifo_data162::HISTOGRAM_FIFO_DATA162_SPEC>,
    #[doc = "0x31c - Histogram Registers"]
    pub histogram_fifo_data163: crate::Reg<histogram_fifo_data163::HISTOGRAM_FIFO_DATA163_SPEC>,
    #[doc = "0x320 - Histogram Registers"]
    pub histogram_fifo_data164: crate::Reg<histogram_fifo_data164::HISTOGRAM_FIFO_DATA164_SPEC>,
    #[doc = "0x324 - Histogram Registers"]
    pub histogram_fifo_data165: crate::Reg<histogram_fifo_data165::HISTOGRAM_FIFO_DATA165_SPEC>,
    #[doc = "0x328 - Histogram Registers"]
    pub histogram_fifo_data166: crate::Reg<histogram_fifo_data166::HISTOGRAM_FIFO_DATA166_SPEC>,
    #[doc = "0x32c - Histogram Registers"]
    pub histogram_fifo_data167: crate::Reg<histogram_fifo_data167::HISTOGRAM_FIFO_DATA167_SPEC>,
    #[doc = "0x330 - Histogram Registers"]
    pub histogram_fifo_data168: crate::Reg<histogram_fifo_data168::HISTOGRAM_FIFO_DATA168_SPEC>,
    #[doc = "0x334 - Histogram Registers"]
    pub histogram_fifo_data169: crate::Reg<histogram_fifo_data169::HISTOGRAM_FIFO_DATA169_SPEC>,
    #[doc = "0x338 - Histogram Registers"]
    pub histogram_fifo_data170: crate::Reg<histogram_fifo_data170::HISTOGRAM_FIFO_DATA170_SPEC>,
    #[doc = "0x33c - Histogram Registers"]
    pub histogram_fifo_data171: crate::Reg<histogram_fifo_data171::HISTOGRAM_FIFO_DATA171_SPEC>,
    #[doc = "0x340 - Histogram Registers"]
    pub histogram_fifo_data172: crate::Reg<histogram_fifo_data172::HISTOGRAM_FIFO_DATA172_SPEC>,
    #[doc = "0x344 - Histogram Registers"]
    pub histogram_fifo_data173: crate::Reg<histogram_fifo_data173::HISTOGRAM_FIFO_DATA173_SPEC>,
    #[doc = "0x348 - Histogram Registers"]
    pub histogram_fifo_data174: crate::Reg<histogram_fifo_data174::HISTOGRAM_FIFO_DATA174_SPEC>,
    #[doc = "0x34c - Histogram Registers"]
    pub histogram_fifo_data175: crate::Reg<histogram_fifo_data175::HISTOGRAM_FIFO_DATA175_SPEC>,
    #[doc = "0x350 - Histogram Registers"]
    pub histogram_fifo_data176: crate::Reg<histogram_fifo_data176::HISTOGRAM_FIFO_DATA176_SPEC>,
    #[doc = "0x354 - Histogram Registers"]
    pub histogram_fifo_data177: crate::Reg<histogram_fifo_data177::HISTOGRAM_FIFO_DATA177_SPEC>,
    #[doc = "0x358 - Histogram Registers"]
    pub histogram_fifo_data178: crate::Reg<histogram_fifo_data178::HISTOGRAM_FIFO_DATA178_SPEC>,
    #[doc = "0x35c - Histogram Registers"]
    pub histogram_fifo_data179: crate::Reg<histogram_fifo_data179::HISTOGRAM_FIFO_DATA179_SPEC>,
    #[doc = "0x360 - Histogram Registers"]
    pub histogram_fifo_data180: crate::Reg<histogram_fifo_data180::HISTOGRAM_FIFO_DATA180_SPEC>,
    #[doc = "0x364 - Histogram Registers"]
    pub histogram_fifo_data181: crate::Reg<histogram_fifo_data181::HISTOGRAM_FIFO_DATA181_SPEC>,
    #[doc = "0x368 - Histogram Registers"]
    pub histogram_fifo_data182: crate::Reg<histogram_fifo_data182::HISTOGRAM_FIFO_DATA182_SPEC>,
    #[doc = "0x36c - Histogram Registers"]
    pub histogram_fifo_data183: crate::Reg<histogram_fifo_data183::HISTOGRAM_FIFO_DATA183_SPEC>,
    #[doc = "0x370 - Histogram Registers"]
    pub histogram_fifo_data184: crate::Reg<histogram_fifo_data184::HISTOGRAM_FIFO_DATA184_SPEC>,
    #[doc = "0x374 - Histogram Registers"]
    pub histogram_fifo_data185: crate::Reg<histogram_fifo_data185::HISTOGRAM_FIFO_DATA185_SPEC>,
    #[doc = "0x378 - Histogram Registers"]
    pub histogram_fifo_data186: crate::Reg<histogram_fifo_data186::HISTOGRAM_FIFO_DATA186_SPEC>,
    #[doc = "0x37c - Histogram Registers"]
    pub histogram_fifo_data187: crate::Reg<histogram_fifo_data187::HISTOGRAM_FIFO_DATA187_SPEC>,
    #[doc = "0x380 - Histogram Registers"]
    pub histogram_fifo_data188: crate::Reg<histogram_fifo_data188::HISTOGRAM_FIFO_DATA188_SPEC>,
    #[doc = "0x384 - Histogram Registers"]
    pub histogram_fifo_data189: crate::Reg<histogram_fifo_data189::HISTOGRAM_FIFO_DATA189_SPEC>,
    #[doc = "0x388 - Histogram Registers"]
    pub histogram_fifo_data190: crate::Reg<histogram_fifo_data190::HISTOGRAM_FIFO_DATA190_SPEC>,
    #[doc = "0x38c - Histogram Registers"]
    pub histogram_fifo_data191: crate::Reg<histogram_fifo_data191::HISTOGRAM_FIFO_DATA191_SPEC>,
    #[doc = "0x390 - Histogram Registers"]
    pub histogram_fifo_data192: crate::Reg<histogram_fifo_data192::HISTOGRAM_FIFO_DATA192_SPEC>,
    #[doc = "0x394 - Histogram Registers"]
    pub histogram_fifo_data193: crate::Reg<histogram_fifo_data193::HISTOGRAM_FIFO_DATA193_SPEC>,
    #[doc = "0x398 - Histogram Registers"]
    pub histogram_fifo_data194: crate::Reg<histogram_fifo_data194::HISTOGRAM_FIFO_DATA194_SPEC>,
    #[doc = "0x39c - Histogram Registers"]
    pub histogram_fifo_data195: crate::Reg<histogram_fifo_data195::HISTOGRAM_FIFO_DATA195_SPEC>,
    #[doc = "0x3a0 - Histogram Registers"]
    pub histogram_fifo_data196: crate::Reg<histogram_fifo_data196::HISTOGRAM_FIFO_DATA196_SPEC>,
    #[doc = "0x3a4 - Histogram Registers"]
    pub histogram_fifo_data197: crate::Reg<histogram_fifo_data197::HISTOGRAM_FIFO_DATA197_SPEC>,
    #[doc = "0x3a8 - Histogram Registers"]
    pub histogram_fifo_data198: crate::Reg<histogram_fifo_data198::HISTOGRAM_FIFO_DATA198_SPEC>,
    #[doc = "0x3ac - Histogram Registers"]
    pub histogram_fifo_data199: crate::Reg<histogram_fifo_data199::HISTOGRAM_FIFO_DATA199_SPEC>,
    #[doc = "0x3b0 - Histogram Registers"]
    pub histogram_fifo_data200: crate::Reg<histogram_fifo_data200::HISTOGRAM_FIFO_DATA200_SPEC>,
    #[doc = "0x3b4 - Histogram Registers"]
    pub histogram_fifo_data201: crate::Reg<histogram_fifo_data201::HISTOGRAM_FIFO_DATA201_SPEC>,
    #[doc = "0x3b8 - Histogram Registers"]
    pub histogram_fifo_data202: crate::Reg<histogram_fifo_data202::HISTOGRAM_FIFO_DATA202_SPEC>,
    #[doc = "0x3bc - Histogram Registers"]
    pub histogram_fifo_data203: crate::Reg<histogram_fifo_data203::HISTOGRAM_FIFO_DATA203_SPEC>,
    #[doc = "0x3c0 - Histogram Registers"]
    pub histogram_fifo_data204: crate::Reg<histogram_fifo_data204::HISTOGRAM_FIFO_DATA204_SPEC>,
    #[doc = "0x3c4 - Histogram Registers"]
    pub histogram_fifo_data205: crate::Reg<histogram_fifo_data205::HISTOGRAM_FIFO_DATA205_SPEC>,
    #[doc = "0x3c8 - Histogram Registers"]
    pub histogram_fifo_data206: crate::Reg<histogram_fifo_data206::HISTOGRAM_FIFO_DATA206_SPEC>,
    #[doc = "0x3cc - Histogram Registers"]
    pub histogram_fifo_data207: crate::Reg<histogram_fifo_data207::HISTOGRAM_FIFO_DATA207_SPEC>,
    #[doc = "0x3d0 - Histogram Registers"]
    pub histogram_fifo_data208: crate::Reg<histogram_fifo_data208::HISTOGRAM_FIFO_DATA208_SPEC>,
    #[doc = "0x3d4 - Histogram Registers"]
    pub histogram_fifo_data209: crate::Reg<histogram_fifo_data209::HISTOGRAM_FIFO_DATA209_SPEC>,
    #[doc = "0x3d8 - Histogram Registers"]
    pub histogram_fifo_data210: crate::Reg<histogram_fifo_data210::HISTOGRAM_FIFO_DATA210_SPEC>,
    #[doc = "0x3dc - Histogram Registers"]
    pub histogram_fifo_data211: crate::Reg<histogram_fifo_data211::HISTOGRAM_FIFO_DATA211_SPEC>,
    #[doc = "0x3e0 - Histogram Registers"]
    pub histogram_fifo_data212: crate::Reg<histogram_fifo_data212::HISTOGRAM_FIFO_DATA212_SPEC>,
    #[doc = "0x3e4 - Histogram Registers"]
    pub histogram_fifo_data213: crate::Reg<histogram_fifo_data213::HISTOGRAM_FIFO_DATA213_SPEC>,
    #[doc = "0x3e8 - Histogram Registers"]
    pub histogram_fifo_data214: crate::Reg<histogram_fifo_data214::HISTOGRAM_FIFO_DATA214_SPEC>,
    #[doc = "0x3ec - Histogram Registers"]
    pub histogram_fifo_data215: crate::Reg<histogram_fifo_data215::HISTOGRAM_FIFO_DATA215_SPEC>,
    #[doc = "0x3f0 - Histogram Registers"]
    pub histogram_fifo_data216: crate::Reg<histogram_fifo_data216::HISTOGRAM_FIFO_DATA216_SPEC>,
    #[doc = "0x3f4 - Histogram Registers"]
    pub histogram_fifo_data217: crate::Reg<histogram_fifo_data217::HISTOGRAM_FIFO_DATA217_SPEC>,
    #[doc = "0x3f8 - Histogram Registers"]
    pub histogram_fifo_data218: crate::Reg<histogram_fifo_data218::HISTOGRAM_FIFO_DATA218_SPEC>,
    #[doc = "0x3fc - Histogram Registers"]
    pub histogram_fifo_data219: crate::Reg<histogram_fifo_data219::HISTOGRAM_FIFO_DATA219_SPEC>,
    #[doc = "0x400 - Histogram Registers"]
    pub histogram_fifo_data220: crate::Reg<histogram_fifo_data220::HISTOGRAM_FIFO_DATA220_SPEC>,
    #[doc = "0x404 - Histogram Registers"]
    pub histogram_fifo_data221: crate::Reg<histogram_fifo_data221::HISTOGRAM_FIFO_DATA221_SPEC>,
    #[doc = "0x408 - Histogram Registers"]
    pub histogram_fifo_data222: crate::Reg<histogram_fifo_data222::HISTOGRAM_FIFO_DATA222_SPEC>,
    #[doc = "0x40c - Histogram Registers"]
    pub histogram_fifo_data223: crate::Reg<histogram_fifo_data223::HISTOGRAM_FIFO_DATA223_SPEC>,
    #[doc = "0x410 - Histogram Registers"]
    pub histogram_fifo_data224: crate::Reg<histogram_fifo_data224::HISTOGRAM_FIFO_DATA224_SPEC>,
    #[doc = "0x414 - Histogram Registers"]
    pub histogram_fifo_data225: crate::Reg<histogram_fifo_data225::HISTOGRAM_FIFO_DATA225_SPEC>,
    #[doc = "0x418 - Histogram Registers"]
    pub histogram_fifo_data226: crate::Reg<histogram_fifo_data226::HISTOGRAM_FIFO_DATA226_SPEC>,
    #[doc = "0x41c - Histogram Registers"]
    pub histogram_fifo_data227: crate::Reg<histogram_fifo_data227::HISTOGRAM_FIFO_DATA227_SPEC>,
    #[doc = "0x420 - Histogram Registers"]
    pub histogram_fifo_data228: crate::Reg<histogram_fifo_data228::HISTOGRAM_FIFO_DATA228_SPEC>,
    #[doc = "0x424 - Histogram Registers"]
    pub histogram_fifo_data229: crate::Reg<histogram_fifo_data229::HISTOGRAM_FIFO_DATA229_SPEC>,
    #[doc = "0x428 - Histogram Registers"]
    pub histogram_fifo_data230: crate::Reg<histogram_fifo_data230::HISTOGRAM_FIFO_DATA230_SPEC>,
    #[doc = "0x42c - Histogram Registers"]
    pub histogram_fifo_data231: crate::Reg<histogram_fifo_data231::HISTOGRAM_FIFO_DATA231_SPEC>,
    #[doc = "0x430 - Histogram Registers"]
    pub histogram_fifo_data232: crate::Reg<histogram_fifo_data232::HISTOGRAM_FIFO_DATA232_SPEC>,
    #[doc = "0x434 - Histogram Registers"]
    pub histogram_fifo_data233: crate::Reg<histogram_fifo_data233::HISTOGRAM_FIFO_DATA233_SPEC>,
    #[doc = "0x438 - Histogram Registers"]
    pub histogram_fifo_data234: crate::Reg<histogram_fifo_data234::HISTOGRAM_FIFO_DATA234_SPEC>,
    #[doc = "0x43c - Histogram Registers"]
    pub histogram_fifo_data235: crate::Reg<histogram_fifo_data235::HISTOGRAM_FIFO_DATA235_SPEC>,
    #[doc = "0x440 - Histogram Registers"]
    pub histogram_fifo_data236: crate::Reg<histogram_fifo_data236::HISTOGRAM_FIFO_DATA236_SPEC>,
    #[doc = "0x444 - Histogram Registers"]
    pub histogram_fifo_data237: crate::Reg<histogram_fifo_data237::HISTOGRAM_FIFO_DATA237_SPEC>,
    #[doc = "0x448 - Histogram Registers"]
    pub histogram_fifo_data238: crate::Reg<histogram_fifo_data238::HISTOGRAM_FIFO_DATA238_SPEC>,
    #[doc = "0x44c - Histogram Registers"]
    pub histogram_fifo_data239: crate::Reg<histogram_fifo_data239::HISTOGRAM_FIFO_DATA239_SPEC>,
    #[doc = "0x450 - Histogram Registers"]
    pub histogram_fifo_data240: crate::Reg<histogram_fifo_data240::HISTOGRAM_FIFO_DATA240_SPEC>,
    #[doc = "0x454 - Histogram Registers"]
    pub histogram_fifo_data241: crate::Reg<histogram_fifo_data241::HISTOGRAM_FIFO_DATA241_SPEC>,
    #[doc = "0x458 - Histogram Registers"]
    pub histogram_fifo_data242: crate::Reg<histogram_fifo_data242::HISTOGRAM_FIFO_DATA242_SPEC>,
    #[doc = "0x45c - Histogram Registers"]
    pub histogram_fifo_data243: crate::Reg<histogram_fifo_data243::HISTOGRAM_FIFO_DATA243_SPEC>,
    #[doc = "0x460 - Histogram Registers"]
    pub histogram_fifo_data244: crate::Reg<histogram_fifo_data244::HISTOGRAM_FIFO_DATA244_SPEC>,
    #[doc = "0x464 - Histogram Registers"]
    pub histogram_fifo_data245: crate::Reg<histogram_fifo_data245::HISTOGRAM_FIFO_DATA245_SPEC>,
    #[doc = "0x468 - Histogram Registers"]
    pub histogram_fifo_data246: crate::Reg<histogram_fifo_data246::HISTOGRAM_FIFO_DATA246_SPEC>,
    #[doc = "0x46c - Histogram Registers"]
    pub histogram_fifo_data247: crate::Reg<histogram_fifo_data247::HISTOGRAM_FIFO_DATA247_SPEC>,
    #[doc = "0x470 - Histogram Registers"]
    pub histogram_fifo_data248: crate::Reg<histogram_fifo_data248::HISTOGRAM_FIFO_DATA248_SPEC>,
    #[doc = "0x474 - Histogram Registers"]
    pub histogram_fifo_data249: crate::Reg<histogram_fifo_data249::HISTOGRAM_FIFO_DATA249_SPEC>,
    #[doc = "0x478 - Histogram Registers"]
    pub histogram_fifo_data250: crate::Reg<histogram_fifo_data250::HISTOGRAM_FIFO_DATA250_SPEC>,
    #[doc = "0x47c - Histogram Registers"]
    pub histogram_fifo_data251: crate::Reg<histogram_fifo_data251::HISTOGRAM_FIFO_DATA251_SPEC>,
    #[doc = "0x480 - Histogram Registers"]
    pub histogram_fifo_data252: crate::Reg<histogram_fifo_data252::HISTOGRAM_FIFO_DATA252_SPEC>,
    #[doc = "0x484 - Histogram Registers"]
    pub histogram_fifo_data253: crate::Reg<histogram_fifo_data253::HISTOGRAM_FIFO_DATA253_SPEC>,
    #[doc = "0x488 - Histogram Registers"]
    pub histogram_fifo_data254: crate::Reg<histogram_fifo_data254::HISTOGRAM_FIFO_DATA254_SPEC>,
    #[doc = "0x48c - Histogram Registers"]
    pub histogram_fifo_data255: crate::Reg<histogram_fifo_data255::HISTOGRAM_FIFO_DATA255_SPEC>,
}
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Control Register"]
pub mod cr1;
#[doc = "INT_EN register accessor: an alias for `Reg<INT_EN_SPEC>`"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod int_en;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Control 2 Register"]
pub mod cr2;
#[doc = "STA register accessor: an alias for `Reg<STA_SPEC>`"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "Status Register"]
pub mod sta;
#[doc = "DMASA_FB1 register accessor: an alias for `Reg<DMASA_FB1_SPEC>`"]
pub type DMASA_FB1 = crate::Reg<dmasa_fb1::DMASA_FB1_SPEC>;
#[doc = "Pixel DMA Frame Buffer 1 Address"]
pub mod dmasa_fb1;
#[doc = "DMASA_FB2 register accessor: an alias for `Reg<DMASA_FB2_SPEC>`"]
pub type DMASA_FB2 = crate::Reg<dmasa_fb2::DMASA_FB2_SPEC>;
#[doc = "Pixel DMA Frame Buffer 2 Address"]
pub mod dmasa_fb2;
#[doc = "BUF_PARA register accessor: an alias for `Reg<BUF_PARA_SPEC>`"]
pub type BUF_PARA = crate::Reg<buf_para::BUF_PARA_SPEC>;
#[doc = "Buffer Parameters Register"]
pub mod buf_para;
#[doc = "IDEAL_WN_SIZE register accessor: an alias for `Reg<IDEAL_WN_SIZE_SPEC>`"]
pub type IDEAL_WN_SIZE = crate::Reg<ideal_wn_size::IDEAL_WN_SIZE_SPEC>;
#[doc = "Ideal Image Size Register"]
pub mod ideal_wn_size;
#[doc = "CR18 register accessor: an alias for `Reg<CR18_SPEC>`"]
pub type CR18 = crate::Reg<cr18::CR18_SPEC>;
#[doc = "Control CR18 Register"]
pub mod cr18;
#[doc = "DMASA_UV1 register accessor: an alias for `Reg<DMASA_UV1_SPEC>`"]
pub type DMASA_UV1 = crate::Reg<dmasa_uv1::DMASA_UV1_SPEC>;
#[doc = "Pixel UV DMA Frame Buffer 1 Address"]
pub mod dmasa_uv1;
#[doc = "DMASA_UV2 register accessor: an alias for `Reg<DMASA_UV2_SPEC>`"]
pub type DMASA_UV2 = crate::Reg<dmasa_uv2::DMASA_UV2_SPEC>;
#[doc = "Pixel UV DMA Frame Buffer 2 Address"]
pub mod dmasa_uv2;
#[doc = "CR20 register accessor: an alias for `Reg<CR20_SPEC>`"]
pub type CR20 = crate::Reg<cr20::CR20_SPEC>;
#[doc = "Control CR20 Register"]
pub mod cr20;
#[doc = "MAX_WN_CYCLE register accessor: an alias for `Reg<MAX_WN_CYCLE_SPEC>`"]
pub type MAX_WN_CYCLE = crate::Reg<max_wn_cycle::MAX_WN_CYCLE_SPEC>;
#[doc = "Max Window Size Register"]
pub mod max_wn_cycle;
#[doc = "CSC_COEF0 register accessor: an alias for `Reg<CSC_COEF0_SPEC>`"]
pub type CSC_COEF0 = crate::Reg<csc_coef0::CSC_COEF0_SPEC>;
#[doc = "Color Space Conversion Config Register 0"]
pub mod csc_coef0;
#[doc = "CSC_COEF1 register accessor: an alias for `Reg<CSC_COEF1_SPEC>`"]
pub type CSC_COEF1 = crate::Reg<csc_coef1::CSC_COEF1_SPEC>;
#[doc = "Color Space Conversion Config Register 1"]
pub mod csc_coef1;
#[doc = "CSC_COEF2 register accessor: an alias for `Reg<CSC_COEF2_SPEC>`"]
pub type CSC_COEF2 = crate::Reg<csc_coef2::CSC_COEF2_SPEC>;
#[doc = "Color Space Conversion Config Register 2"]
pub mod csc_coef2;
#[doc = "CLRKEY_LOW register accessor: an alias for `Reg<CLRKEY_LOW_SPEC>`"]
pub type CLRKEY_LOW = crate::Reg<clrkey_low::CLRKEY_LOW_SPEC>;
#[doc = "Low Color Key Register"]
pub mod clrkey_low;
#[doc = "CLRKEY_HIGH register accessor: an alias for `Reg<CLRKEY_HIGH_SPEC>`"]
pub type CLRKEY_HIGH = crate::Reg<clrkey_high::CLRKEY_HIGH_SPEC>;
#[doc = "High Color Key Register"]
pub mod clrkey_high;
#[doc = "HISTOGRAM_FIFO_DATA0 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA0_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA0 = crate::Reg<histogram_fifo_data0::HISTOGRAM_FIFO_DATA0_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data0;
#[doc = "HISTOGRAM_FIFO_DATA1 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA1_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA1 = crate::Reg<histogram_fifo_data1::HISTOGRAM_FIFO_DATA1_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data1;
#[doc = "HISTOGRAM_FIFO_DATA2 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA2_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA2 = crate::Reg<histogram_fifo_data2::HISTOGRAM_FIFO_DATA2_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data2;
#[doc = "HISTOGRAM_FIFO_DATA3 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA3_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA3 = crate::Reg<histogram_fifo_data3::HISTOGRAM_FIFO_DATA3_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data3;
#[doc = "HISTOGRAM_FIFO_DATA4 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA4_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA4 = crate::Reg<histogram_fifo_data4::HISTOGRAM_FIFO_DATA4_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data4;
#[doc = "HISTOGRAM_FIFO_DATA5 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA5_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA5 = crate::Reg<histogram_fifo_data5::HISTOGRAM_FIFO_DATA5_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data5;
#[doc = "HISTOGRAM_FIFO_DATA6 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA6_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA6 = crate::Reg<histogram_fifo_data6::HISTOGRAM_FIFO_DATA6_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data6;
#[doc = "HISTOGRAM_FIFO_DATA7 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA7_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA7 = crate::Reg<histogram_fifo_data7::HISTOGRAM_FIFO_DATA7_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data7;
#[doc = "HISTOGRAM_FIFO_DATA8 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA8_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA8 = crate::Reg<histogram_fifo_data8::HISTOGRAM_FIFO_DATA8_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data8;
#[doc = "HISTOGRAM_FIFO_DATA9 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA9_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA9 = crate::Reg<histogram_fifo_data9::HISTOGRAM_FIFO_DATA9_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data9;
#[doc = "HISTOGRAM_FIFO_DATA10 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA10_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA10 = crate::Reg<histogram_fifo_data10::HISTOGRAM_FIFO_DATA10_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data10;
#[doc = "HISTOGRAM_FIFO_DATA11 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA11_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA11 = crate::Reg<histogram_fifo_data11::HISTOGRAM_FIFO_DATA11_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data11;
#[doc = "HISTOGRAM_FIFO_DATA12 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA12_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA12 = crate::Reg<histogram_fifo_data12::HISTOGRAM_FIFO_DATA12_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data12;
#[doc = "HISTOGRAM_FIFO_DATA13 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA13_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA13 = crate::Reg<histogram_fifo_data13::HISTOGRAM_FIFO_DATA13_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data13;
#[doc = "HISTOGRAM_FIFO_DATA14 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA14_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA14 = crate::Reg<histogram_fifo_data14::HISTOGRAM_FIFO_DATA14_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data14;
#[doc = "HISTOGRAM_FIFO_DATA15 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA15_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA15 = crate::Reg<histogram_fifo_data15::HISTOGRAM_FIFO_DATA15_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data15;
#[doc = "HISTOGRAM_FIFO_DATA16 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA16_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA16 = crate::Reg<histogram_fifo_data16::HISTOGRAM_FIFO_DATA16_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data16;
#[doc = "HISTOGRAM_FIFO_DATA17 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA17_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA17 = crate::Reg<histogram_fifo_data17::HISTOGRAM_FIFO_DATA17_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data17;
#[doc = "HISTOGRAM_FIFO_DATA18 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA18_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA18 = crate::Reg<histogram_fifo_data18::HISTOGRAM_FIFO_DATA18_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data18;
#[doc = "HISTOGRAM_FIFO_DATA19 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA19_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA19 = crate::Reg<histogram_fifo_data19::HISTOGRAM_FIFO_DATA19_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data19;
#[doc = "HISTOGRAM_FIFO_DATA20 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA20_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA20 = crate::Reg<histogram_fifo_data20::HISTOGRAM_FIFO_DATA20_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data20;
#[doc = "HISTOGRAM_FIFO_DATA21 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA21_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA21 = crate::Reg<histogram_fifo_data21::HISTOGRAM_FIFO_DATA21_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data21;
#[doc = "HISTOGRAM_FIFO_DATA22 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA22_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA22 = crate::Reg<histogram_fifo_data22::HISTOGRAM_FIFO_DATA22_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data22;
#[doc = "HISTOGRAM_FIFO_DATA23 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA23_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA23 = crate::Reg<histogram_fifo_data23::HISTOGRAM_FIFO_DATA23_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data23;
#[doc = "HISTOGRAM_FIFO_DATA24 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA24_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA24 = crate::Reg<histogram_fifo_data24::HISTOGRAM_FIFO_DATA24_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data24;
#[doc = "HISTOGRAM_FIFO_DATA25 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA25_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA25 = crate::Reg<histogram_fifo_data25::HISTOGRAM_FIFO_DATA25_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data25;
#[doc = "HISTOGRAM_FIFO_DATA26 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA26_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA26 = crate::Reg<histogram_fifo_data26::HISTOGRAM_FIFO_DATA26_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data26;
#[doc = "HISTOGRAM_FIFO_DATA27 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA27_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA27 = crate::Reg<histogram_fifo_data27::HISTOGRAM_FIFO_DATA27_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data27;
#[doc = "HISTOGRAM_FIFO_DATA28 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA28_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA28 = crate::Reg<histogram_fifo_data28::HISTOGRAM_FIFO_DATA28_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data28;
#[doc = "HISTOGRAM_FIFO_DATA29 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA29_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA29 = crate::Reg<histogram_fifo_data29::HISTOGRAM_FIFO_DATA29_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data29;
#[doc = "HISTOGRAM_FIFO_DATA30 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA30_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA30 = crate::Reg<histogram_fifo_data30::HISTOGRAM_FIFO_DATA30_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data30;
#[doc = "HISTOGRAM_FIFO_DATA31 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA31_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA31 = crate::Reg<histogram_fifo_data31::HISTOGRAM_FIFO_DATA31_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data31;
#[doc = "HISTOGRAM_FIFO_DATA32 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA32_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA32 = crate::Reg<histogram_fifo_data32::HISTOGRAM_FIFO_DATA32_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data32;
#[doc = "HISTOGRAM_FIFO_DATA33 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA33_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA33 = crate::Reg<histogram_fifo_data33::HISTOGRAM_FIFO_DATA33_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data33;
#[doc = "HISTOGRAM_FIFO_DATA34 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA34_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA34 = crate::Reg<histogram_fifo_data34::HISTOGRAM_FIFO_DATA34_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data34;
#[doc = "HISTOGRAM_FIFO_DATA35 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA35_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA35 = crate::Reg<histogram_fifo_data35::HISTOGRAM_FIFO_DATA35_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data35;
#[doc = "HISTOGRAM_FIFO_DATA36 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA36_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA36 = crate::Reg<histogram_fifo_data36::HISTOGRAM_FIFO_DATA36_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data36;
#[doc = "HISTOGRAM_FIFO_DATA37 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA37_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA37 = crate::Reg<histogram_fifo_data37::HISTOGRAM_FIFO_DATA37_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data37;
#[doc = "HISTOGRAM_FIFO_DATA38 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA38_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA38 = crate::Reg<histogram_fifo_data38::HISTOGRAM_FIFO_DATA38_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data38;
#[doc = "HISTOGRAM_FIFO_DATA39 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA39_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA39 = crate::Reg<histogram_fifo_data39::HISTOGRAM_FIFO_DATA39_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data39;
#[doc = "HISTOGRAM_FIFO_DATA40 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA40_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA40 = crate::Reg<histogram_fifo_data40::HISTOGRAM_FIFO_DATA40_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data40;
#[doc = "HISTOGRAM_FIFO_DATA41 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA41_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA41 = crate::Reg<histogram_fifo_data41::HISTOGRAM_FIFO_DATA41_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data41;
#[doc = "HISTOGRAM_FIFO_DATA42 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA42_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA42 = crate::Reg<histogram_fifo_data42::HISTOGRAM_FIFO_DATA42_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data42;
#[doc = "HISTOGRAM_FIFO_DATA43 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA43_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA43 = crate::Reg<histogram_fifo_data43::HISTOGRAM_FIFO_DATA43_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data43;
#[doc = "HISTOGRAM_FIFO_DATA44 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA44_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA44 = crate::Reg<histogram_fifo_data44::HISTOGRAM_FIFO_DATA44_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data44;
#[doc = "HISTOGRAM_FIFO_DATA45 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA45_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA45 = crate::Reg<histogram_fifo_data45::HISTOGRAM_FIFO_DATA45_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data45;
#[doc = "HISTOGRAM_FIFO_DATA46 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA46_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA46 = crate::Reg<histogram_fifo_data46::HISTOGRAM_FIFO_DATA46_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data46;
#[doc = "HISTOGRAM_FIFO_DATA47 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA47_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA47 = crate::Reg<histogram_fifo_data47::HISTOGRAM_FIFO_DATA47_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data47;
#[doc = "HISTOGRAM_FIFO_DATA48 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA48_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA48 = crate::Reg<histogram_fifo_data48::HISTOGRAM_FIFO_DATA48_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data48;
#[doc = "HISTOGRAM_FIFO_DATA49 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA49_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA49 = crate::Reg<histogram_fifo_data49::HISTOGRAM_FIFO_DATA49_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data49;
#[doc = "HISTOGRAM_FIFO_DATA50 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA50_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA50 = crate::Reg<histogram_fifo_data50::HISTOGRAM_FIFO_DATA50_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data50;
#[doc = "HISTOGRAM_FIFO_DATA51 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA51_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA51 = crate::Reg<histogram_fifo_data51::HISTOGRAM_FIFO_DATA51_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data51;
#[doc = "HISTOGRAM_FIFO_DATA52 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA52_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA52 = crate::Reg<histogram_fifo_data52::HISTOGRAM_FIFO_DATA52_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data52;
#[doc = "HISTOGRAM_FIFO_DATA53 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA53_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA53 = crate::Reg<histogram_fifo_data53::HISTOGRAM_FIFO_DATA53_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data53;
#[doc = "HISTOGRAM_FIFO_DATA54 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA54_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA54 = crate::Reg<histogram_fifo_data54::HISTOGRAM_FIFO_DATA54_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data54;
#[doc = "HISTOGRAM_FIFO_DATA55 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA55_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA55 = crate::Reg<histogram_fifo_data55::HISTOGRAM_FIFO_DATA55_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data55;
#[doc = "HISTOGRAM_FIFO_DATA56 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA56_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA56 = crate::Reg<histogram_fifo_data56::HISTOGRAM_FIFO_DATA56_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data56;
#[doc = "HISTOGRAM_FIFO_DATA57 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA57_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA57 = crate::Reg<histogram_fifo_data57::HISTOGRAM_FIFO_DATA57_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data57;
#[doc = "HISTOGRAM_FIFO_DATA58 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA58_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA58 = crate::Reg<histogram_fifo_data58::HISTOGRAM_FIFO_DATA58_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data58;
#[doc = "HISTOGRAM_FIFO_DATA59 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA59_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA59 = crate::Reg<histogram_fifo_data59::HISTOGRAM_FIFO_DATA59_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data59;
#[doc = "HISTOGRAM_FIFO_DATA60 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA60_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA60 = crate::Reg<histogram_fifo_data60::HISTOGRAM_FIFO_DATA60_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data60;
#[doc = "HISTOGRAM_FIFO_DATA61 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA61_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA61 = crate::Reg<histogram_fifo_data61::HISTOGRAM_FIFO_DATA61_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data61;
#[doc = "HISTOGRAM_FIFO_DATA62 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA62_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA62 = crate::Reg<histogram_fifo_data62::HISTOGRAM_FIFO_DATA62_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data62;
#[doc = "HISTOGRAM_FIFO_DATA63 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA63_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA63 = crate::Reg<histogram_fifo_data63::HISTOGRAM_FIFO_DATA63_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data63;
#[doc = "HISTOGRAM_FIFO_DATA64 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA64_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA64 = crate::Reg<histogram_fifo_data64::HISTOGRAM_FIFO_DATA64_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data64;
#[doc = "HISTOGRAM_FIFO_DATA65 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA65_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA65 = crate::Reg<histogram_fifo_data65::HISTOGRAM_FIFO_DATA65_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data65;
#[doc = "HISTOGRAM_FIFO_DATA66 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA66_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA66 = crate::Reg<histogram_fifo_data66::HISTOGRAM_FIFO_DATA66_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data66;
#[doc = "HISTOGRAM_FIFO_DATA67 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA67_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA67 = crate::Reg<histogram_fifo_data67::HISTOGRAM_FIFO_DATA67_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data67;
#[doc = "HISTOGRAM_FIFO_DATA68 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA68_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA68 = crate::Reg<histogram_fifo_data68::HISTOGRAM_FIFO_DATA68_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data68;
#[doc = "HISTOGRAM_FIFO_DATA69 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA69_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA69 = crate::Reg<histogram_fifo_data69::HISTOGRAM_FIFO_DATA69_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data69;
#[doc = "HISTOGRAM_FIFO_DATA70 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA70_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA70 = crate::Reg<histogram_fifo_data70::HISTOGRAM_FIFO_DATA70_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data70;
#[doc = "HISTOGRAM_FIFO_DATA71 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA71_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA71 = crate::Reg<histogram_fifo_data71::HISTOGRAM_FIFO_DATA71_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data71;
#[doc = "HISTOGRAM_FIFO_DATA72 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA72_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA72 = crate::Reg<histogram_fifo_data72::HISTOGRAM_FIFO_DATA72_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data72;
#[doc = "HISTOGRAM_FIFO_DATA73 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA73_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA73 = crate::Reg<histogram_fifo_data73::HISTOGRAM_FIFO_DATA73_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data73;
#[doc = "HISTOGRAM_FIFO_DATA74 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA74_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA74 = crate::Reg<histogram_fifo_data74::HISTOGRAM_FIFO_DATA74_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data74;
#[doc = "HISTOGRAM_FIFO_DATA75 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA75_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA75 = crate::Reg<histogram_fifo_data75::HISTOGRAM_FIFO_DATA75_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data75;
#[doc = "HISTOGRAM_FIFO_DATA76 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA76_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA76 = crate::Reg<histogram_fifo_data76::HISTOGRAM_FIFO_DATA76_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data76;
#[doc = "HISTOGRAM_FIFO_DATA77 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA77_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA77 = crate::Reg<histogram_fifo_data77::HISTOGRAM_FIFO_DATA77_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data77;
#[doc = "HISTOGRAM_FIFO_DATA78 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA78_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA78 = crate::Reg<histogram_fifo_data78::HISTOGRAM_FIFO_DATA78_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data78;
#[doc = "HISTOGRAM_FIFO_DATA79 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA79_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA79 = crate::Reg<histogram_fifo_data79::HISTOGRAM_FIFO_DATA79_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data79;
#[doc = "HISTOGRAM_FIFO_DATA80 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA80_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA80 = crate::Reg<histogram_fifo_data80::HISTOGRAM_FIFO_DATA80_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data80;
#[doc = "HISTOGRAM_FIFO_DATA81 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA81_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA81 = crate::Reg<histogram_fifo_data81::HISTOGRAM_FIFO_DATA81_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data81;
#[doc = "HISTOGRAM_FIFO_DATA82 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA82_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA82 = crate::Reg<histogram_fifo_data82::HISTOGRAM_FIFO_DATA82_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data82;
#[doc = "HISTOGRAM_FIFO_DATA83 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA83_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA83 = crate::Reg<histogram_fifo_data83::HISTOGRAM_FIFO_DATA83_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data83;
#[doc = "HISTOGRAM_FIFO_DATA84 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA84_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA84 = crate::Reg<histogram_fifo_data84::HISTOGRAM_FIFO_DATA84_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data84;
#[doc = "HISTOGRAM_FIFO_DATA85 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA85_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA85 = crate::Reg<histogram_fifo_data85::HISTOGRAM_FIFO_DATA85_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data85;
#[doc = "HISTOGRAM_FIFO_DATA86 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA86_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA86 = crate::Reg<histogram_fifo_data86::HISTOGRAM_FIFO_DATA86_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data86;
#[doc = "HISTOGRAM_FIFO_DATA87 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA87_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA87 = crate::Reg<histogram_fifo_data87::HISTOGRAM_FIFO_DATA87_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data87;
#[doc = "HISTOGRAM_FIFO_DATA88 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA88_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA88 = crate::Reg<histogram_fifo_data88::HISTOGRAM_FIFO_DATA88_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data88;
#[doc = "HISTOGRAM_FIFO_DATA89 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA89_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA89 = crate::Reg<histogram_fifo_data89::HISTOGRAM_FIFO_DATA89_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data89;
#[doc = "HISTOGRAM_FIFO_DATA90 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA90_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA90 = crate::Reg<histogram_fifo_data90::HISTOGRAM_FIFO_DATA90_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data90;
#[doc = "HISTOGRAM_FIFO_DATA91 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA91_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA91 = crate::Reg<histogram_fifo_data91::HISTOGRAM_FIFO_DATA91_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data91;
#[doc = "HISTOGRAM_FIFO_DATA92 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA92_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA92 = crate::Reg<histogram_fifo_data92::HISTOGRAM_FIFO_DATA92_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data92;
#[doc = "HISTOGRAM_FIFO_DATA93 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA93_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA93 = crate::Reg<histogram_fifo_data93::HISTOGRAM_FIFO_DATA93_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data93;
#[doc = "HISTOGRAM_FIFO_DATA94 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA94_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA94 = crate::Reg<histogram_fifo_data94::HISTOGRAM_FIFO_DATA94_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data94;
#[doc = "HISTOGRAM_FIFO_DATA95 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA95_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA95 = crate::Reg<histogram_fifo_data95::HISTOGRAM_FIFO_DATA95_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data95;
#[doc = "HISTOGRAM_FIFO_DATA96 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA96_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA96 = crate::Reg<histogram_fifo_data96::HISTOGRAM_FIFO_DATA96_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data96;
#[doc = "HISTOGRAM_FIFO_DATA97 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA97_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA97 = crate::Reg<histogram_fifo_data97::HISTOGRAM_FIFO_DATA97_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data97;
#[doc = "HISTOGRAM_FIFO_DATA98 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA98_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA98 = crate::Reg<histogram_fifo_data98::HISTOGRAM_FIFO_DATA98_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data98;
#[doc = "HISTOGRAM_FIFO_DATA99 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA99_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA99 = crate::Reg<histogram_fifo_data99::HISTOGRAM_FIFO_DATA99_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data99;
#[doc = "HISTOGRAM_FIFO_DATA100 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA100_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA100 = crate::Reg<histogram_fifo_data100::HISTOGRAM_FIFO_DATA100_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data100;
#[doc = "HISTOGRAM_FIFO_DATA101 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA101_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA101 = crate::Reg<histogram_fifo_data101::HISTOGRAM_FIFO_DATA101_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data101;
#[doc = "HISTOGRAM_FIFO_DATA102 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA102_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA102 = crate::Reg<histogram_fifo_data102::HISTOGRAM_FIFO_DATA102_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data102;
#[doc = "HISTOGRAM_FIFO_DATA103 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA103_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA103 = crate::Reg<histogram_fifo_data103::HISTOGRAM_FIFO_DATA103_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data103;
#[doc = "HISTOGRAM_FIFO_DATA104 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA104_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA104 = crate::Reg<histogram_fifo_data104::HISTOGRAM_FIFO_DATA104_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data104;
#[doc = "HISTOGRAM_FIFO_DATA105 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA105_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA105 = crate::Reg<histogram_fifo_data105::HISTOGRAM_FIFO_DATA105_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data105;
#[doc = "HISTOGRAM_FIFO_DATA106 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA106_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA106 = crate::Reg<histogram_fifo_data106::HISTOGRAM_FIFO_DATA106_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data106;
#[doc = "HISTOGRAM_FIFO_DATA107 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA107_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA107 = crate::Reg<histogram_fifo_data107::HISTOGRAM_FIFO_DATA107_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data107;
#[doc = "HISTOGRAM_FIFO_DATA108 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA108_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA108 = crate::Reg<histogram_fifo_data108::HISTOGRAM_FIFO_DATA108_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data108;
#[doc = "HISTOGRAM_FIFO_DATA109 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA109_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA109 = crate::Reg<histogram_fifo_data109::HISTOGRAM_FIFO_DATA109_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data109;
#[doc = "HISTOGRAM_FIFO_DATA110 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA110_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA110 = crate::Reg<histogram_fifo_data110::HISTOGRAM_FIFO_DATA110_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data110;
#[doc = "HISTOGRAM_FIFO_DATA111 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA111_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA111 = crate::Reg<histogram_fifo_data111::HISTOGRAM_FIFO_DATA111_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data111;
#[doc = "HISTOGRAM_FIFO_DATA112 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA112_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA112 = crate::Reg<histogram_fifo_data112::HISTOGRAM_FIFO_DATA112_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data112;
#[doc = "HISTOGRAM_FIFO_DATA113 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA113_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA113 = crate::Reg<histogram_fifo_data113::HISTOGRAM_FIFO_DATA113_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data113;
#[doc = "HISTOGRAM_FIFO_DATA114 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA114_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA114 = crate::Reg<histogram_fifo_data114::HISTOGRAM_FIFO_DATA114_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data114;
#[doc = "HISTOGRAM_FIFO_DATA115 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA115_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA115 = crate::Reg<histogram_fifo_data115::HISTOGRAM_FIFO_DATA115_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data115;
#[doc = "HISTOGRAM_FIFO_DATA116 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA116_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA116 = crate::Reg<histogram_fifo_data116::HISTOGRAM_FIFO_DATA116_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data116;
#[doc = "HISTOGRAM_FIFO_DATA117 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA117_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA117 = crate::Reg<histogram_fifo_data117::HISTOGRAM_FIFO_DATA117_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data117;
#[doc = "HISTOGRAM_FIFO_DATA118 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA118_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA118 = crate::Reg<histogram_fifo_data118::HISTOGRAM_FIFO_DATA118_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data118;
#[doc = "HISTOGRAM_FIFO_DATA119 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA119_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA119 = crate::Reg<histogram_fifo_data119::HISTOGRAM_FIFO_DATA119_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data119;
#[doc = "HISTOGRAM_FIFO_DATA120 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA120_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA120 = crate::Reg<histogram_fifo_data120::HISTOGRAM_FIFO_DATA120_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data120;
#[doc = "HISTOGRAM_FIFO_DATA121 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA121_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA121 = crate::Reg<histogram_fifo_data121::HISTOGRAM_FIFO_DATA121_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data121;
#[doc = "HISTOGRAM_FIFO_DATA122 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA122_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA122 = crate::Reg<histogram_fifo_data122::HISTOGRAM_FIFO_DATA122_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data122;
#[doc = "HISTOGRAM_FIFO_DATA123 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA123_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA123 = crate::Reg<histogram_fifo_data123::HISTOGRAM_FIFO_DATA123_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data123;
#[doc = "HISTOGRAM_FIFO_DATA124 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA124_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA124 = crate::Reg<histogram_fifo_data124::HISTOGRAM_FIFO_DATA124_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data124;
#[doc = "HISTOGRAM_FIFO_DATA125 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA125_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA125 = crate::Reg<histogram_fifo_data125::HISTOGRAM_FIFO_DATA125_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data125;
#[doc = "HISTOGRAM_FIFO_DATA126 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA126_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA126 = crate::Reg<histogram_fifo_data126::HISTOGRAM_FIFO_DATA126_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data126;
#[doc = "HISTOGRAM_FIFO_DATA127 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA127_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA127 = crate::Reg<histogram_fifo_data127::HISTOGRAM_FIFO_DATA127_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data127;
#[doc = "HISTOGRAM_FIFO_DATA128 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA128_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA128 = crate::Reg<histogram_fifo_data128::HISTOGRAM_FIFO_DATA128_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data128;
#[doc = "HISTOGRAM_FIFO_DATA129 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA129_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA129 = crate::Reg<histogram_fifo_data129::HISTOGRAM_FIFO_DATA129_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data129;
#[doc = "HISTOGRAM_FIFO_DATA130 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA130_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA130 = crate::Reg<histogram_fifo_data130::HISTOGRAM_FIFO_DATA130_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data130;
#[doc = "HISTOGRAM_FIFO_DATA131 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA131_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA131 = crate::Reg<histogram_fifo_data131::HISTOGRAM_FIFO_DATA131_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data131;
#[doc = "HISTOGRAM_FIFO_DATA132 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA132_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA132 = crate::Reg<histogram_fifo_data132::HISTOGRAM_FIFO_DATA132_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data132;
#[doc = "HISTOGRAM_FIFO_DATA133 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA133_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA133 = crate::Reg<histogram_fifo_data133::HISTOGRAM_FIFO_DATA133_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data133;
#[doc = "HISTOGRAM_FIFO_DATA134 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA134_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA134 = crate::Reg<histogram_fifo_data134::HISTOGRAM_FIFO_DATA134_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data134;
#[doc = "HISTOGRAM_FIFO_DATA135 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA135_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA135 = crate::Reg<histogram_fifo_data135::HISTOGRAM_FIFO_DATA135_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data135;
#[doc = "HISTOGRAM_FIFO_DATA136 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA136_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA136 = crate::Reg<histogram_fifo_data136::HISTOGRAM_FIFO_DATA136_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data136;
#[doc = "HISTOGRAM_FIFO_DATA137 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA137_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA137 = crate::Reg<histogram_fifo_data137::HISTOGRAM_FIFO_DATA137_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data137;
#[doc = "HISTOGRAM_FIFO_DATA138 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA138_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA138 = crate::Reg<histogram_fifo_data138::HISTOGRAM_FIFO_DATA138_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data138;
#[doc = "HISTOGRAM_FIFO_DATA139 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA139_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA139 = crate::Reg<histogram_fifo_data139::HISTOGRAM_FIFO_DATA139_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data139;
#[doc = "HISTOGRAM_FIFO_DATA140 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA140_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA140 = crate::Reg<histogram_fifo_data140::HISTOGRAM_FIFO_DATA140_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data140;
#[doc = "HISTOGRAM_FIFO_DATA141 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA141_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA141 = crate::Reg<histogram_fifo_data141::HISTOGRAM_FIFO_DATA141_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data141;
#[doc = "HISTOGRAM_FIFO_DATA142 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA142_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA142 = crate::Reg<histogram_fifo_data142::HISTOGRAM_FIFO_DATA142_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data142;
#[doc = "HISTOGRAM_FIFO_DATA143 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA143_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA143 = crate::Reg<histogram_fifo_data143::HISTOGRAM_FIFO_DATA143_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data143;
#[doc = "HISTOGRAM_FIFO_DATA144 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA144_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA144 = crate::Reg<histogram_fifo_data144::HISTOGRAM_FIFO_DATA144_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data144;
#[doc = "HISTOGRAM_FIFO_DATA145 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA145_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA145 = crate::Reg<histogram_fifo_data145::HISTOGRAM_FIFO_DATA145_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data145;
#[doc = "HISTOGRAM_FIFO_DATA146 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA146_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA146 = crate::Reg<histogram_fifo_data146::HISTOGRAM_FIFO_DATA146_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data146;
#[doc = "HISTOGRAM_FIFO_DATA147 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA147_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA147 = crate::Reg<histogram_fifo_data147::HISTOGRAM_FIFO_DATA147_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data147;
#[doc = "HISTOGRAM_FIFO_DATA148 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA148_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA148 = crate::Reg<histogram_fifo_data148::HISTOGRAM_FIFO_DATA148_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data148;
#[doc = "HISTOGRAM_FIFO_DATA149 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA149_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA149 = crate::Reg<histogram_fifo_data149::HISTOGRAM_FIFO_DATA149_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data149;
#[doc = "HISTOGRAM_FIFO_DATA150 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA150_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA150 = crate::Reg<histogram_fifo_data150::HISTOGRAM_FIFO_DATA150_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data150;
#[doc = "HISTOGRAM_FIFO_DATA151 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA151_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA151 = crate::Reg<histogram_fifo_data151::HISTOGRAM_FIFO_DATA151_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data151;
#[doc = "HISTOGRAM_FIFO_DATA152 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA152_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA152 = crate::Reg<histogram_fifo_data152::HISTOGRAM_FIFO_DATA152_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data152;
#[doc = "HISTOGRAM_FIFO_DATA153 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA153_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA153 = crate::Reg<histogram_fifo_data153::HISTOGRAM_FIFO_DATA153_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data153;
#[doc = "HISTOGRAM_FIFO_DATA154 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA154_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA154 = crate::Reg<histogram_fifo_data154::HISTOGRAM_FIFO_DATA154_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data154;
#[doc = "HISTOGRAM_FIFO_DATA155 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA155_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA155 = crate::Reg<histogram_fifo_data155::HISTOGRAM_FIFO_DATA155_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data155;
#[doc = "HISTOGRAM_FIFO_DATA156 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA156_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA156 = crate::Reg<histogram_fifo_data156::HISTOGRAM_FIFO_DATA156_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data156;
#[doc = "HISTOGRAM_FIFO_DATA157 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA157_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA157 = crate::Reg<histogram_fifo_data157::HISTOGRAM_FIFO_DATA157_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data157;
#[doc = "HISTOGRAM_FIFO_DATA158 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA158_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA158 = crate::Reg<histogram_fifo_data158::HISTOGRAM_FIFO_DATA158_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data158;
#[doc = "HISTOGRAM_FIFO_DATA159 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA159_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA159 = crate::Reg<histogram_fifo_data159::HISTOGRAM_FIFO_DATA159_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data159;
#[doc = "HISTOGRAM_FIFO_DATA160 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA160_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA160 = crate::Reg<histogram_fifo_data160::HISTOGRAM_FIFO_DATA160_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data160;
#[doc = "HISTOGRAM_FIFO_DATA161 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA161_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA161 = crate::Reg<histogram_fifo_data161::HISTOGRAM_FIFO_DATA161_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data161;
#[doc = "HISTOGRAM_FIFO_DATA162 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA162_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA162 = crate::Reg<histogram_fifo_data162::HISTOGRAM_FIFO_DATA162_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data162;
#[doc = "HISTOGRAM_FIFO_DATA163 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA163_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA163 = crate::Reg<histogram_fifo_data163::HISTOGRAM_FIFO_DATA163_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data163;
#[doc = "HISTOGRAM_FIFO_DATA164 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA164_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA164 = crate::Reg<histogram_fifo_data164::HISTOGRAM_FIFO_DATA164_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data164;
#[doc = "HISTOGRAM_FIFO_DATA165 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA165_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA165 = crate::Reg<histogram_fifo_data165::HISTOGRAM_FIFO_DATA165_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data165;
#[doc = "HISTOGRAM_FIFO_DATA166 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA166_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA166 = crate::Reg<histogram_fifo_data166::HISTOGRAM_FIFO_DATA166_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data166;
#[doc = "HISTOGRAM_FIFO_DATA167 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA167_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA167 = crate::Reg<histogram_fifo_data167::HISTOGRAM_FIFO_DATA167_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data167;
#[doc = "HISTOGRAM_FIFO_DATA168 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA168_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA168 = crate::Reg<histogram_fifo_data168::HISTOGRAM_FIFO_DATA168_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data168;
#[doc = "HISTOGRAM_FIFO_DATA169 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA169_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA169 = crate::Reg<histogram_fifo_data169::HISTOGRAM_FIFO_DATA169_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data169;
#[doc = "HISTOGRAM_FIFO_DATA170 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA170_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA170 = crate::Reg<histogram_fifo_data170::HISTOGRAM_FIFO_DATA170_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data170;
#[doc = "HISTOGRAM_FIFO_DATA171 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA171_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA171 = crate::Reg<histogram_fifo_data171::HISTOGRAM_FIFO_DATA171_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data171;
#[doc = "HISTOGRAM_FIFO_DATA172 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA172_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA172 = crate::Reg<histogram_fifo_data172::HISTOGRAM_FIFO_DATA172_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data172;
#[doc = "HISTOGRAM_FIFO_DATA173 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA173_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA173 = crate::Reg<histogram_fifo_data173::HISTOGRAM_FIFO_DATA173_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data173;
#[doc = "HISTOGRAM_FIFO_DATA174 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA174_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA174 = crate::Reg<histogram_fifo_data174::HISTOGRAM_FIFO_DATA174_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data174;
#[doc = "HISTOGRAM_FIFO_DATA175 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA175_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA175 = crate::Reg<histogram_fifo_data175::HISTOGRAM_FIFO_DATA175_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data175;
#[doc = "HISTOGRAM_FIFO_DATA176 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA176_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA176 = crate::Reg<histogram_fifo_data176::HISTOGRAM_FIFO_DATA176_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data176;
#[doc = "HISTOGRAM_FIFO_DATA177 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA177_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA177 = crate::Reg<histogram_fifo_data177::HISTOGRAM_FIFO_DATA177_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data177;
#[doc = "HISTOGRAM_FIFO_DATA178 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA178_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA178 = crate::Reg<histogram_fifo_data178::HISTOGRAM_FIFO_DATA178_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data178;
#[doc = "HISTOGRAM_FIFO_DATA179 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA179_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA179 = crate::Reg<histogram_fifo_data179::HISTOGRAM_FIFO_DATA179_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data179;
#[doc = "HISTOGRAM_FIFO_DATA180 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA180_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA180 = crate::Reg<histogram_fifo_data180::HISTOGRAM_FIFO_DATA180_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data180;
#[doc = "HISTOGRAM_FIFO_DATA181 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA181_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA181 = crate::Reg<histogram_fifo_data181::HISTOGRAM_FIFO_DATA181_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data181;
#[doc = "HISTOGRAM_FIFO_DATA182 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA182_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA182 = crate::Reg<histogram_fifo_data182::HISTOGRAM_FIFO_DATA182_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data182;
#[doc = "HISTOGRAM_FIFO_DATA183 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA183_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA183 = crate::Reg<histogram_fifo_data183::HISTOGRAM_FIFO_DATA183_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data183;
#[doc = "HISTOGRAM_FIFO_DATA184 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA184_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA184 = crate::Reg<histogram_fifo_data184::HISTOGRAM_FIFO_DATA184_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data184;
#[doc = "HISTOGRAM_FIFO_DATA185 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA185_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA185 = crate::Reg<histogram_fifo_data185::HISTOGRAM_FIFO_DATA185_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data185;
#[doc = "HISTOGRAM_FIFO_DATA186 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA186_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA186 = crate::Reg<histogram_fifo_data186::HISTOGRAM_FIFO_DATA186_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data186;
#[doc = "HISTOGRAM_FIFO_DATA187 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA187_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA187 = crate::Reg<histogram_fifo_data187::HISTOGRAM_FIFO_DATA187_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data187;
#[doc = "HISTOGRAM_FIFO_DATA188 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA188_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA188 = crate::Reg<histogram_fifo_data188::HISTOGRAM_FIFO_DATA188_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data188;
#[doc = "HISTOGRAM_FIFO_DATA189 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA189_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA189 = crate::Reg<histogram_fifo_data189::HISTOGRAM_FIFO_DATA189_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data189;
#[doc = "HISTOGRAM_FIFO_DATA190 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA190_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA190 = crate::Reg<histogram_fifo_data190::HISTOGRAM_FIFO_DATA190_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data190;
#[doc = "HISTOGRAM_FIFO_DATA191 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA191_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA191 = crate::Reg<histogram_fifo_data191::HISTOGRAM_FIFO_DATA191_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data191;
#[doc = "HISTOGRAM_FIFO_DATA192 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA192_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA192 = crate::Reg<histogram_fifo_data192::HISTOGRAM_FIFO_DATA192_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data192;
#[doc = "HISTOGRAM_FIFO_DATA193 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA193_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA193 = crate::Reg<histogram_fifo_data193::HISTOGRAM_FIFO_DATA193_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data193;
#[doc = "HISTOGRAM_FIFO_DATA194 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA194_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA194 = crate::Reg<histogram_fifo_data194::HISTOGRAM_FIFO_DATA194_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data194;
#[doc = "HISTOGRAM_FIFO_DATA195 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA195_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA195 = crate::Reg<histogram_fifo_data195::HISTOGRAM_FIFO_DATA195_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data195;
#[doc = "HISTOGRAM_FIFO_DATA196 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA196_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA196 = crate::Reg<histogram_fifo_data196::HISTOGRAM_FIFO_DATA196_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data196;
#[doc = "HISTOGRAM_FIFO_DATA197 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA197_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA197 = crate::Reg<histogram_fifo_data197::HISTOGRAM_FIFO_DATA197_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data197;
#[doc = "HISTOGRAM_FIFO_DATA198 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA198_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA198 = crate::Reg<histogram_fifo_data198::HISTOGRAM_FIFO_DATA198_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data198;
#[doc = "HISTOGRAM_FIFO_DATA199 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA199_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA199 = crate::Reg<histogram_fifo_data199::HISTOGRAM_FIFO_DATA199_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data199;
#[doc = "HISTOGRAM_FIFO_DATA200 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA200_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA200 = crate::Reg<histogram_fifo_data200::HISTOGRAM_FIFO_DATA200_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data200;
#[doc = "HISTOGRAM_FIFO_DATA201 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA201_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA201 = crate::Reg<histogram_fifo_data201::HISTOGRAM_FIFO_DATA201_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data201;
#[doc = "HISTOGRAM_FIFO_DATA202 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA202_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA202 = crate::Reg<histogram_fifo_data202::HISTOGRAM_FIFO_DATA202_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data202;
#[doc = "HISTOGRAM_FIFO_DATA203 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA203_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA203 = crate::Reg<histogram_fifo_data203::HISTOGRAM_FIFO_DATA203_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data203;
#[doc = "HISTOGRAM_FIFO_DATA204 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA204_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA204 = crate::Reg<histogram_fifo_data204::HISTOGRAM_FIFO_DATA204_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data204;
#[doc = "HISTOGRAM_FIFO_DATA205 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA205_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA205 = crate::Reg<histogram_fifo_data205::HISTOGRAM_FIFO_DATA205_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data205;
#[doc = "HISTOGRAM_FIFO_DATA206 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA206_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA206 = crate::Reg<histogram_fifo_data206::HISTOGRAM_FIFO_DATA206_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data206;
#[doc = "HISTOGRAM_FIFO_DATA207 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA207_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA207 = crate::Reg<histogram_fifo_data207::HISTOGRAM_FIFO_DATA207_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data207;
#[doc = "HISTOGRAM_FIFO_DATA208 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA208_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA208 = crate::Reg<histogram_fifo_data208::HISTOGRAM_FIFO_DATA208_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data208;
#[doc = "HISTOGRAM_FIFO_DATA209 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA209_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA209 = crate::Reg<histogram_fifo_data209::HISTOGRAM_FIFO_DATA209_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data209;
#[doc = "HISTOGRAM_FIFO_DATA210 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA210_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA210 = crate::Reg<histogram_fifo_data210::HISTOGRAM_FIFO_DATA210_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data210;
#[doc = "HISTOGRAM_FIFO_DATA211 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA211_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA211 = crate::Reg<histogram_fifo_data211::HISTOGRAM_FIFO_DATA211_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data211;
#[doc = "HISTOGRAM_FIFO_DATA212 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA212_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA212 = crate::Reg<histogram_fifo_data212::HISTOGRAM_FIFO_DATA212_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data212;
#[doc = "HISTOGRAM_FIFO_DATA213 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA213_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA213 = crate::Reg<histogram_fifo_data213::HISTOGRAM_FIFO_DATA213_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data213;
#[doc = "HISTOGRAM_FIFO_DATA214 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA214_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA214 = crate::Reg<histogram_fifo_data214::HISTOGRAM_FIFO_DATA214_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data214;
#[doc = "HISTOGRAM_FIFO_DATA215 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA215_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA215 = crate::Reg<histogram_fifo_data215::HISTOGRAM_FIFO_DATA215_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data215;
#[doc = "HISTOGRAM_FIFO_DATA216 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA216_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA216 = crate::Reg<histogram_fifo_data216::HISTOGRAM_FIFO_DATA216_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data216;
#[doc = "HISTOGRAM_FIFO_DATA217 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA217_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA217 = crate::Reg<histogram_fifo_data217::HISTOGRAM_FIFO_DATA217_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data217;
#[doc = "HISTOGRAM_FIFO_DATA218 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA218_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA218 = crate::Reg<histogram_fifo_data218::HISTOGRAM_FIFO_DATA218_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data218;
#[doc = "HISTOGRAM_FIFO_DATA219 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA219_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA219 = crate::Reg<histogram_fifo_data219::HISTOGRAM_FIFO_DATA219_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data219;
#[doc = "HISTOGRAM_FIFO_DATA220 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA220_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA220 = crate::Reg<histogram_fifo_data220::HISTOGRAM_FIFO_DATA220_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data220;
#[doc = "HISTOGRAM_FIFO_DATA221 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA221_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA221 = crate::Reg<histogram_fifo_data221::HISTOGRAM_FIFO_DATA221_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data221;
#[doc = "HISTOGRAM_FIFO_DATA222 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA222_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA222 = crate::Reg<histogram_fifo_data222::HISTOGRAM_FIFO_DATA222_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data222;
#[doc = "HISTOGRAM_FIFO_DATA223 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA223_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA223 = crate::Reg<histogram_fifo_data223::HISTOGRAM_FIFO_DATA223_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data223;
#[doc = "HISTOGRAM_FIFO_DATA224 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA224_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA224 = crate::Reg<histogram_fifo_data224::HISTOGRAM_FIFO_DATA224_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data224;
#[doc = "HISTOGRAM_FIFO_DATA225 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA225_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA225 = crate::Reg<histogram_fifo_data225::HISTOGRAM_FIFO_DATA225_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data225;
#[doc = "HISTOGRAM_FIFO_DATA226 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA226_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA226 = crate::Reg<histogram_fifo_data226::HISTOGRAM_FIFO_DATA226_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data226;
#[doc = "HISTOGRAM_FIFO_DATA227 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA227_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA227 = crate::Reg<histogram_fifo_data227::HISTOGRAM_FIFO_DATA227_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data227;
#[doc = "HISTOGRAM_FIFO_DATA228 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA228_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA228 = crate::Reg<histogram_fifo_data228::HISTOGRAM_FIFO_DATA228_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data228;
#[doc = "HISTOGRAM_FIFO_DATA229 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA229_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA229 = crate::Reg<histogram_fifo_data229::HISTOGRAM_FIFO_DATA229_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data229;
#[doc = "HISTOGRAM_FIFO_DATA230 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA230_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA230 = crate::Reg<histogram_fifo_data230::HISTOGRAM_FIFO_DATA230_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data230;
#[doc = "HISTOGRAM_FIFO_DATA231 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA231_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA231 = crate::Reg<histogram_fifo_data231::HISTOGRAM_FIFO_DATA231_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data231;
#[doc = "HISTOGRAM_FIFO_DATA232 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA232_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA232 = crate::Reg<histogram_fifo_data232::HISTOGRAM_FIFO_DATA232_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data232;
#[doc = "HISTOGRAM_FIFO_DATA233 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA233_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA233 = crate::Reg<histogram_fifo_data233::HISTOGRAM_FIFO_DATA233_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data233;
#[doc = "HISTOGRAM_FIFO_DATA234 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA234_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA234 = crate::Reg<histogram_fifo_data234::HISTOGRAM_FIFO_DATA234_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data234;
#[doc = "HISTOGRAM_FIFO_DATA235 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA235_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA235 = crate::Reg<histogram_fifo_data235::HISTOGRAM_FIFO_DATA235_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data235;
#[doc = "HISTOGRAM_FIFO_DATA236 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA236_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA236 = crate::Reg<histogram_fifo_data236::HISTOGRAM_FIFO_DATA236_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data236;
#[doc = "HISTOGRAM_FIFO_DATA237 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA237_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA237 = crate::Reg<histogram_fifo_data237::HISTOGRAM_FIFO_DATA237_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data237;
#[doc = "HISTOGRAM_FIFO_DATA238 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA238_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA238 = crate::Reg<histogram_fifo_data238::HISTOGRAM_FIFO_DATA238_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data238;
#[doc = "HISTOGRAM_FIFO_DATA239 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA239_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA239 = crate::Reg<histogram_fifo_data239::HISTOGRAM_FIFO_DATA239_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data239;
#[doc = "HISTOGRAM_FIFO_DATA240 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA240_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA240 = crate::Reg<histogram_fifo_data240::HISTOGRAM_FIFO_DATA240_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data240;
#[doc = "HISTOGRAM_FIFO_DATA241 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA241_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA241 = crate::Reg<histogram_fifo_data241::HISTOGRAM_FIFO_DATA241_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data241;
#[doc = "HISTOGRAM_FIFO_DATA242 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA242_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA242 = crate::Reg<histogram_fifo_data242::HISTOGRAM_FIFO_DATA242_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data242;
#[doc = "HISTOGRAM_FIFO_DATA243 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA243_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA243 = crate::Reg<histogram_fifo_data243::HISTOGRAM_FIFO_DATA243_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data243;
#[doc = "HISTOGRAM_FIFO_DATA244 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA244_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA244 = crate::Reg<histogram_fifo_data244::HISTOGRAM_FIFO_DATA244_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data244;
#[doc = "HISTOGRAM_FIFO_DATA245 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA245_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA245 = crate::Reg<histogram_fifo_data245::HISTOGRAM_FIFO_DATA245_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data245;
#[doc = "HISTOGRAM_FIFO_DATA246 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA246_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA246 = crate::Reg<histogram_fifo_data246::HISTOGRAM_FIFO_DATA246_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data246;
#[doc = "HISTOGRAM_FIFO_DATA247 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA247_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA247 = crate::Reg<histogram_fifo_data247::HISTOGRAM_FIFO_DATA247_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data247;
#[doc = "HISTOGRAM_FIFO_DATA248 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA248_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA248 = crate::Reg<histogram_fifo_data248::HISTOGRAM_FIFO_DATA248_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data248;
#[doc = "HISTOGRAM_FIFO_DATA249 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA249_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA249 = crate::Reg<histogram_fifo_data249::HISTOGRAM_FIFO_DATA249_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data249;
#[doc = "HISTOGRAM_FIFO_DATA250 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA250_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA250 = crate::Reg<histogram_fifo_data250::HISTOGRAM_FIFO_DATA250_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data250;
#[doc = "HISTOGRAM_FIFO_DATA251 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA251_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA251 = crate::Reg<histogram_fifo_data251::HISTOGRAM_FIFO_DATA251_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data251;
#[doc = "HISTOGRAM_FIFO_DATA252 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA252_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA252 = crate::Reg<histogram_fifo_data252::HISTOGRAM_FIFO_DATA252_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data252;
#[doc = "HISTOGRAM_FIFO_DATA253 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA253_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA253 = crate::Reg<histogram_fifo_data253::HISTOGRAM_FIFO_DATA253_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data253;
#[doc = "HISTOGRAM_FIFO_DATA254 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA254_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA254 = crate::Reg<histogram_fifo_data254::HISTOGRAM_FIFO_DATA254_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data254;
#[doc = "HISTOGRAM_FIFO_DATA255 register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA255_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA255 = crate::Reg<histogram_fifo_data255::HISTOGRAM_FIFO_DATA255_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data255;
