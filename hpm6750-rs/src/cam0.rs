#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr1: CR1,
    #[doc = "0x04 - Interrupt Enable Register"]
    pub int_en: INT_EN,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Control 2 Register"]
    pub cr2: CR2,
    _reserved3: [u8; 0x10],
    #[doc = "0x24 - Status Register"]
    pub sta: STA,
    _reserved4: [u8; 0x08],
    #[doc = "0x30 - Pixel DMA Frame Buffer 1 Address"]
    pub dmasa_fb1: DMASA_FB1,
    #[doc = "0x34 - Pixel DMA Frame Buffer 2 Address"]
    pub dmasa_fb2: DMASA_FB2,
    #[doc = "0x38 - Buffer Parameters Register"]
    pub buf_para: BUF_PARA,
    #[doc = "0x3c - Ideal Image Size Register"]
    pub ideal_wn_size: IDEAL_WN_SIZE,
    _reserved8: [u8; 0x0c],
    #[doc = "0x4c - Control CR18 Register"]
    pub cr18: CR18,
    #[doc = "0x50 - Pixel UV DMA Frame Buffer 1 Address"]
    pub dmasa_uv1: DMASA_UV1,
    #[doc = "0x54 - Pixel UV DMA Frame Buffer 2 Address"]
    pub dmasa_uv2: DMASA_UV2,
    #[doc = "0x58 - Control CR20 Register"]
    pub cr20: CR20,
    #[doc = "0x5c - Max Window Size Register"]
    pub max_wn_cycle: MAX_WN_CYCLE,
    _reserved13: [u8; 0x10],
    #[doc = "0x70 - Color Space Conversion Config Register 0"]
    pub csc_coef0: CSC_COEF0,
    #[doc = "0x74 - Color Space Conversion Config Register 1"]
    pub csc_coef1: CSC_COEF1,
    #[doc = "0x78 - Color Space Conversion Config Register 2"]
    pub csc_coef2: CSC_COEF2,
    #[doc = "0x7c - Low Color Key Register"]
    pub clrkey_low: CLRKEY_LOW,
    #[doc = "0x80 - High Color Key Register"]
    pub clrkey_high: CLRKEY_HIGH,
    _reserved18: [u8; 0x0c],
    #[doc = "0x90 - Histogram Registers"]
    pub histogram_fifo_data0: HISTOGRAM_FIFO_DATA0,
    #[doc = "0x94 - Histogram Registers"]
    pub histogram_fifo_data1: HISTOGRAM_FIFO_DATA1,
    #[doc = "0x98 - Histogram Registers"]
    pub histogram_fifo_data2: HISTOGRAM_FIFO_DATA2,
    #[doc = "0x9c - Histogram Registers"]
    pub histogram_fifo_data3: HISTOGRAM_FIFO_DATA3,
    #[doc = "0xa0 - Histogram Registers"]
    pub histogram_fifo_data4: HISTOGRAM_FIFO_DATA4,
    #[doc = "0xa4 - Histogram Registers"]
    pub histogram_fifo_data5: HISTOGRAM_FIFO_DATA5,
    #[doc = "0xa8 - Histogram Registers"]
    pub histogram_fifo_data6: HISTOGRAM_FIFO_DATA6,
    #[doc = "0xac - Histogram Registers"]
    pub histogram_fifo_data7: HISTOGRAM_FIFO_DATA7,
    #[doc = "0xb0 - Histogram Registers"]
    pub histogram_fifo_data8: HISTOGRAM_FIFO_DATA8,
    #[doc = "0xb4 - Histogram Registers"]
    pub histogram_fifo_data9: HISTOGRAM_FIFO_DATA9,
    #[doc = "0xb8 - Histogram Registers"]
    pub histogram_fifo_data10: HISTOGRAM_FIFO_DATA10,
    #[doc = "0xbc - Histogram Registers"]
    pub histogram_fifo_data11: HISTOGRAM_FIFO_DATA11,
    #[doc = "0xc0 - Histogram Registers"]
    pub histogram_fifo_data12: HISTOGRAM_FIFO_DATA12,
    #[doc = "0xc4 - Histogram Registers"]
    pub histogram_fifo_data13: HISTOGRAM_FIFO_DATA13,
    #[doc = "0xc8 - Histogram Registers"]
    pub histogram_fifo_data14: HISTOGRAM_FIFO_DATA14,
    #[doc = "0xcc - Histogram Registers"]
    pub histogram_fifo_data15: HISTOGRAM_FIFO_DATA15,
    #[doc = "0xd0 - Histogram Registers"]
    pub histogram_fifo_data16: HISTOGRAM_FIFO_DATA16,
    #[doc = "0xd4 - Histogram Registers"]
    pub histogram_fifo_data17: HISTOGRAM_FIFO_DATA17,
    #[doc = "0xd8 - Histogram Registers"]
    pub histogram_fifo_data18: HISTOGRAM_FIFO_DATA18,
    #[doc = "0xdc - Histogram Registers"]
    pub histogram_fifo_data19: HISTOGRAM_FIFO_DATA19,
    #[doc = "0xe0 - Histogram Registers"]
    pub histogram_fifo_data20: HISTOGRAM_FIFO_DATA20,
    #[doc = "0xe4 - Histogram Registers"]
    pub histogram_fifo_data21: HISTOGRAM_FIFO_DATA21,
    #[doc = "0xe8 - Histogram Registers"]
    pub histogram_fifo_data22: HISTOGRAM_FIFO_DATA22,
    #[doc = "0xec - Histogram Registers"]
    pub histogram_fifo_data23: HISTOGRAM_FIFO_DATA23,
    #[doc = "0xf0 - Histogram Registers"]
    pub histogram_fifo_data24: HISTOGRAM_FIFO_DATA24,
    #[doc = "0xf4 - Histogram Registers"]
    pub histogram_fifo_data25: HISTOGRAM_FIFO_DATA25,
    #[doc = "0xf8 - Histogram Registers"]
    pub histogram_fifo_data26: HISTOGRAM_FIFO_DATA26,
    #[doc = "0xfc - Histogram Registers"]
    pub histogram_fifo_data27: HISTOGRAM_FIFO_DATA27,
    #[doc = "0x100 - Histogram Registers"]
    pub histogram_fifo_data28: HISTOGRAM_FIFO_DATA28,
    #[doc = "0x104 - Histogram Registers"]
    pub histogram_fifo_data29: HISTOGRAM_FIFO_DATA29,
    #[doc = "0x108 - Histogram Registers"]
    pub histogram_fifo_data30: HISTOGRAM_FIFO_DATA30,
    #[doc = "0x10c - Histogram Registers"]
    pub histogram_fifo_data31: HISTOGRAM_FIFO_DATA31,
    #[doc = "0x110 - Histogram Registers"]
    pub histogram_fifo_data32: HISTOGRAM_FIFO_DATA32,
    #[doc = "0x114 - Histogram Registers"]
    pub histogram_fifo_data33: HISTOGRAM_FIFO_DATA33,
    #[doc = "0x118 - Histogram Registers"]
    pub histogram_fifo_data34: HISTOGRAM_FIFO_DATA34,
    #[doc = "0x11c - Histogram Registers"]
    pub histogram_fifo_data35: HISTOGRAM_FIFO_DATA35,
    #[doc = "0x120 - Histogram Registers"]
    pub histogram_fifo_data36: HISTOGRAM_FIFO_DATA36,
    #[doc = "0x124 - Histogram Registers"]
    pub histogram_fifo_data37: HISTOGRAM_FIFO_DATA37,
    #[doc = "0x128 - Histogram Registers"]
    pub histogram_fifo_data38: HISTOGRAM_FIFO_DATA38,
    #[doc = "0x12c - Histogram Registers"]
    pub histogram_fifo_data39: HISTOGRAM_FIFO_DATA39,
    #[doc = "0x130 - Histogram Registers"]
    pub histogram_fifo_data40: HISTOGRAM_FIFO_DATA40,
    #[doc = "0x134 - Histogram Registers"]
    pub histogram_fifo_data41: HISTOGRAM_FIFO_DATA41,
    #[doc = "0x138 - Histogram Registers"]
    pub histogram_fifo_data42: HISTOGRAM_FIFO_DATA42,
    #[doc = "0x13c - Histogram Registers"]
    pub histogram_fifo_data43: HISTOGRAM_FIFO_DATA43,
    #[doc = "0x140 - Histogram Registers"]
    pub histogram_fifo_data44: HISTOGRAM_FIFO_DATA44,
    #[doc = "0x144 - Histogram Registers"]
    pub histogram_fifo_data45: HISTOGRAM_FIFO_DATA45,
    #[doc = "0x148 - Histogram Registers"]
    pub histogram_fifo_data46: HISTOGRAM_FIFO_DATA46,
    #[doc = "0x14c - Histogram Registers"]
    pub histogram_fifo_data47: HISTOGRAM_FIFO_DATA47,
    #[doc = "0x150 - Histogram Registers"]
    pub histogram_fifo_data48: HISTOGRAM_FIFO_DATA48,
    #[doc = "0x154 - Histogram Registers"]
    pub histogram_fifo_data49: HISTOGRAM_FIFO_DATA49,
    #[doc = "0x158 - Histogram Registers"]
    pub histogram_fifo_data50: HISTOGRAM_FIFO_DATA50,
    #[doc = "0x15c - Histogram Registers"]
    pub histogram_fifo_data51: HISTOGRAM_FIFO_DATA51,
    #[doc = "0x160 - Histogram Registers"]
    pub histogram_fifo_data52: HISTOGRAM_FIFO_DATA52,
    #[doc = "0x164 - Histogram Registers"]
    pub histogram_fifo_data53: HISTOGRAM_FIFO_DATA53,
    #[doc = "0x168 - Histogram Registers"]
    pub histogram_fifo_data54: HISTOGRAM_FIFO_DATA54,
    #[doc = "0x16c - Histogram Registers"]
    pub histogram_fifo_data55: HISTOGRAM_FIFO_DATA55,
    #[doc = "0x170 - Histogram Registers"]
    pub histogram_fifo_data56: HISTOGRAM_FIFO_DATA56,
    #[doc = "0x174 - Histogram Registers"]
    pub histogram_fifo_data57: HISTOGRAM_FIFO_DATA57,
    #[doc = "0x178 - Histogram Registers"]
    pub histogram_fifo_data58: HISTOGRAM_FIFO_DATA58,
    #[doc = "0x17c - Histogram Registers"]
    pub histogram_fifo_data59: HISTOGRAM_FIFO_DATA59,
    #[doc = "0x180 - Histogram Registers"]
    pub histogram_fifo_data60: HISTOGRAM_FIFO_DATA60,
    #[doc = "0x184 - Histogram Registers"]
    pub histogram_fifo_data61: HISTOGRAM_FIFO_DATA61,
    #[doc = "0x188 - Histogram Registers"]
    pub histogram_fifo_data62: HISTOGRAM_FIFO_DATA62,
    #[doc = "0x18c - Histogram Registers"]
    pub histogram_fifo_data63: HISTOGRAM_FIFO_DATA63,
    #[doc = "0x190 - Histogram Registers"]
    pub histogram_fifo_data64: HISTOGRAM_FIFO_DATA64,
    #[doc = "0x194 - Histogram Registers"]
    pub histogram_fifo_data65: HISTOGRAM_FIFO_DATA65,
    #[doc = "0x198 - Histogram Registers"]
    pub histogram_fifo_data66: HISTOGRAM_FIFO_DATA66,
    #[doc = "0x19c - Histogram Registers"]
    pub histogram_fifo_data67: HISTOGRAM_FIFO_DATA67,
    #[doc = "0x1a0 - Histogram Registers"]
    pub histogram_fifo_data68: HISTOGRAM_FIFO_DATA68,
    #[doc = "0x1a4 - Histogram Registers"]
    pub histogram_fifo_data69: HISTOGRAM_FIFO_DATA69,
    #[doc = "0x1a8 - Histogram Registers"]
    pub histogram_fifo_data70: HISTOGRAM_FIFO_DATA70,
    #[doc = "0x1ac - Histogram Registers"]
    pub histogram_fifo_data71: HISTOGRAM_FIFO_DATA71,
    #[doc = "0x1b0 - Histogram Registers"]
    pub histogram_fifo_data72: HISTOGRAM_FIFO_DATA72,
    #[doc = "0x1b4 - Histogram Registers"]
    pub histogram_fifo_data73: HISTOGRAM_FIFO_DATA73,
    #[doc = "0x1b8 - Histogram Registers"]
    pub histogram_fifo_data74: HISTOGRAM_FIFO_DATA74,
    #[doc = "0x1bc - Histogram Registers"]
    pub histogram_fifo_data75: HISTOGRAM_FIFO_DATA75,
    #[doc = "0x1c0 - Histogram Registers"]
    pub histogram_fifo_data76: HISTOGRAM_FIFO_DATA76,
    #[doc = "0x1c4 - Histogram Registers"]
    pub histogram_fifo_data77: HISTOGRAM_FIFO_DATA77,
    #[doc = "0x1c8 - Histogram Registers"]
    pub histogram_fifo_data78: HISTOGRAM_FIFO_DATA78,
    #[doc = "0x1cc - Histogram Registers"]
    pub histogram_fifo_data79: HISTOGRAM_FIFO_DATA79,
    #[doc = "0x1d0 - Histogram Registers"]
    pub histogram_fifo_data80: HISTOGRAM_FIFO_DATA80,
    #[doc = "0x1d4 - Histogram Registers"]
    pub histogram_fifo_data81: HISTOGRAM_FIFO_DATA81,
    #[doc = "0x1d8 - Histogram Registers"]
    pub histogram_fifo_data82: HISTOGRAM_FIFO_DATA82,
    #[doc = "0x1dc - Histogram Registers"]
    pub histogram_fifo_data83: HISTOGRAM_FIFO_DATA83,
    #[doc = "0x1e0 - Histogram Registers"]
    pub histogram_fifo_data84: HISTOGRAM_FIFO_DATA84,
    #[doc = "0x1e4 - Histogram Registers"]
    pub histogram_fifo_data85: HISTOGRAM_FIFO_DATA85,
    #[doc = "0x1e8 - Histogram Registers"]
    pub histogram_fifo_data86: HISTOGRAM_FIFO_DATA86,
    #[doc = "0x1ec - Histogram Registers"]
    pub histogram_fifo_data87: HISTOGRAM_FIFO_DATA87,
    #[doc = "0x1f0 - Histogram Registers"]
    pub histogram_fifo_data88: HISTOGRAM_FIFO_DATA88,
    #[doc = "0x1f4 - Histogram Registers"]
    pub histogram_fifo_data89: HISTOGRAM_FIFO_DATA89,
    #[doc = "0x1f8 - Histogram Registers"]
    pub histogram_fifo_data90: HISTOGRAM_FIFO_DATA90,
    #[doc = "0x1fc - Histogram Registers"]
    pub histogram_fifo_data91: HISTOGRAM_FIFO_DATA91,
    #[doc = "0x200 - Histogram Registers"]
    pub histogram_fifo_data92: HISTOGRAM_FIFO_DATA92,
    #[doc = "0x204 - Histogram Registers"]
    pub histogram_fifo_data93: HISTOGRAM_FIFO_DATA93,
    #[doc = "0x208 - Histogram Registers"]
    pub histogram_fifo_data94: HISTOGRAM_FIFO_DATA94,
    #[doc = "0x20c - Histogram Registers"]
    pub histogram_fifo_data95: HISTOGRAM_FIFO_DATA95,
    #[doc = "0x210 - Histogram Registers"]
    pub histogram_fifo_data96: HISTOGRAM_FIFO_DATA96,
    #[doc = "0x214 - Histogram Registers"]
    pub histogram_fifo_data97: HISTOGRAM_FIFO_DATA97,
    #[doc = "0x218 - Histogram Registers"]
    pub histogram_fifo_data98: HISTOGRAM_FIFO_DATA98,
    #[doc = "0x21c - Histogram Registers"]
    pub histogram_fifo_data99: HISTOGRAM_FIFO_DATA99,
    #[doc = "0x220 - Histogram Registers"]
    pub histogram_fifo_data100: HISTOGRAM_FIFO_DATA100,
    #[doc = "0x224 - Histogram Registers"]
    pub histogram_fifo_data101: HISTOGRAM_FIFO_DATA101,
    #[doc = "0x228 - Histogram Registers"]
    pub histogram_fifo_data102: HISTOGRAM_FIFO_DATA102,
    #[doc = "0x22c - Histogram Registers"]
    pub histogram_fifo_data103: HISTOGRAM_FIFO_DATA103,
    #[doc = "0x230 - Histogram Registers"]
    pub histogram_fifo_data104: HISTOGRAM_FIFO_DATA104,
    #[doc = "0x234 - Histogram Registers"]
    pub histogram_fifo_data105: HISTOGRAM_FIFO_DATA105,
    #[doc = "0x238 - Histogram Registers"]
    pub histogram_fifo_data106: HISTOGRAM_FIFO_DATA106,
    #[doc = "0x23c - Histogram Registers"]
    pub histogram_fifo_data107: HISTOGRAM_FIFO_DATA107,
    #[doc = "0x240 - Histogram Registers"]
    pub histogram_fifo_data108: HISTOGRAM_FIFO_DATA108,
    #[doc = "0x244 - Histogram Registers"]
    pub histogram_fifo_data109: HISTOGRAM_FIFO_DATA109,
    #[doc = "0x248 - Histogram Registers"]
    pub histogram_fifo_data110: HISTOGRAM_FIFO_DATA110,
    #[doc = "0x24c - Histogram Registers"]
    pub histogram_fifo_data111: HISTOGRAM_FIFO_DATA111,
    #[doc = "0x250 - Histogram Registers"]
    pub histogram_fifo_data112: HISTOGRAM_FIFO_DATA112,
    #[doc = "0x254 - Histogram Registers"]
    pub histogram_fifo_data113: HISTOGRAM_FIFO_DATA113,
    #[doc = "0x258 - Histogram Registers"]
    pub histogram_fifo_data114: HISTOGRAM_FIFO_DATA114,
    #[doc = "0x25c - Histogram Registers"]
    pub histogram_fifo_data115: HISTOGRAM_FIFO_DATA115,
    #[doc = "0x260 - Histogram Registers"]
    pub histogram_fifo_data116: HISTOGRAM_FIFO_DATA116,
    #[doc = "0x264 - Histogram Registers"]
    pub histogram_fifo_data117: HISTOGRAM_FIFO_DATA117,
    #[doc = "0x268 - Histogram Registers"]
    pub histogram_fifo_data118: HISTOGRAM_FIFO_DATA118,
    #[doc = "0x26c - Histogram Registers"]
    pub histogram_fifo_data119: HISTOGRAM_FIFO_DATA119,
    #[doc = "0x270 - Histogram Registers"]
    pub histogram_fifo_data120: HISTOGRAM_FIFO_DATA120,
    #[doc = "0x274 - Histogram Registers"]
    pub histogram_fifo_data121: HISTOGRAM_FIFO_DATA121,
    #[doc = "0x278 - Histogram Registers"]
    pub histogram_fifo_data122: HISTOGRAM_FIFO_DATA122,
    #[doc = "0x27c - Histogram Registers"]
    pub histogram_fifo_data123: HISTOGRAM_FIFO_DATA123,
    #[doc = "0x280 - Histogram Registers"]
    pub histogram_fifo_data124: HISTOGRAM_FIFO_DATA124,
    #[doc = "0x284 - Histogram Registers"]
    pub histogram_fifo_data125: HISTOGRAM_FIFO_DATA125,
    #[doc = "0x288 - Histogram Registers"]
    pub histogram_fifo_data126: HISTOGRAM_FIFO_DATA126,
    #[doc = "0x28c - Histogram Registers"]
    pub histogram_fifo_data127: HISTOGRAM_FIFO_DATA127,
    #[doc = "0x290 - Histogram Registers"]
    pub histogram_fifo_data128: HISTOGRAM_FIFO_DATA128,
    #[doc = "0x294 - Histogram Registers"]
    pub histogram_fifo_data129: HISTOGRAM_FIFO_DATA129,
    #[doc = "0x298 - Histogram Registers"]
    pub histogram_fifo_data130: HISTOGRAM_FIFO_DATA130,
    #[doc = "0x29c - Histogram Registers"]
    pub histogram_fifo_data131: HISTOGRAM_FIFO_DATA131,
    #[doc = "0x2a0 - Histogram Registers"]
    pub histogram_fifo_data132: HISTOGRAM_FIFO_DATA132,
    #[doc = "0x2a4 - Histogram Registers"]
    pub histogram_fifo_data133: HISTOGRAM_FIFO_DATA133,
    #[doc = "0x2a8 - Histogram Registers"]
    pub histogram_fifo_data134: HISTOGRAM_FIFO_DATA134,
    #[doc = "0x2ac - Histogram Registers"]
    pub histogram_fifo_data135: HISTOGRAM_FIFO_DATA135,
    #[doc = "0x2b0 - Histogram Registers"]
    pub histogram_fifo_data136: HISTOGRAM_FIFO_DATA136,
    #[doc = "0x2b4 - Histogram Registers"]
    pub histogram_fifo_data137: HISTOGRAM_FIFO_DATA137,
    #[doc = "0x2b8 - Histogram Registers"]
    pub histogram_fifo_data138: HISTOGRAM_FIFO_DATA138,
    #[doc = "0x2bc - Histogram Registers"]
    pub histogram_fifo_data139: HISTOGRAM_FIFO_DATA139,
    #[doc = "0x2c0 - Histogram Registers"]
    pub histogram_fifo_data140: HISTOGRAM_FIFO_DATA140,
    #[doc = "0x2c4 - Histogram Registers"]
    pub histogram_fifo_data141: HISTOGRAM_FIFO_DATA141,
    #[doc = "0x2c8 - Histogram Registers"]
    pub histogram_fifo_data142: HISTOGRAM_FIFO_DATA142,
    #[doc = "0x2cc - Histogram Registers"]
    pub histogram_fifo_data143: HISTOGRAM_FIFO_DATA143,
    #[doc = "0x2d0 - Histogram Registers"]
    pub histogram_fifo_data144: HISTOGRAM_FIFO_DATA144,
    #[doc = "0x2d4 - Histogram Registers"]
    pub histogram_fifo_data145: HISTOGRAM_FIFO_DATA145,
    #[doc = "0x2d8 - Histogram Registers"]
    pub histogram_fifo_data146: HISTOGRAM_FIFO_DATA146,
    #[doc = "0x2dc - Histogram Registers"]
    pub histogram_fifo_data147: HISTOGRAM_FIFO_DATA147,
    #[doc = "0x2e0 - Histogram Registers"]
    pub histogram_fifo_data148: HISTOGRAM_FIFO_DATA148,
    #[doc = "0x2e4 - Histogram Registers"]
    pub histogram_fifo_data149: HISTOGRAM_FIFO_DATA149,
    #[doc = "0x2e8 - Histogram Registers"]
    pub histogram_fifo_data150: HISTOGRAM_FIFO_DATA150,
    #[doc = "0x2ec - Histogram Registers"]
    pub histogram_fifo_data151: HISTOGRAM_FIFO_DATA151,
    #[doc = "0x2f0 - Histogram Registers"]
    pub histogram_fifo_data152: HISTOGRAM_FIFO_DATA152,
    #[doc = "0x2f4 - Histogram Registers"]
    pub histogram_fifo_data153: HISTOGRAM_FIFO_DATA153,
    #[doc = "0x2f8 - Histogram Registers"]
    pub histogram_fifo_data154: HISTOGRAM_FIFO_DATA154,
    #[doc = "0x2fc - Histogram Registers"]
    pub histogram_fifo_data155: HISTOGRAM_FIFO_DATA155,
    #[doc = "0x300 - Histogram Registers"]
    pub histogram_fifo_data156: HISTOGRAM_FIFO_DATA156,
    #[doc = "0x304 - Histogram Registers"]
    pub histogram_fifo_data157: HISTOGRAM_FIFO_DATA157,
    #[doc = "0x308 - Histogram Registers"]
    pub histogram_fifo_data158: HISTOGRAM_FIFO_DATA158,
    #[doc = "0x30c - Histogram Registers"]
    pub histogram_fifo_data159: HISTOGRAM_FIFO_DATA159,
    #[doc = "0x310 - Histogram Registers"]
    pub histogram_fifo_data160: HISTOGRAM_FIFO_DATA160,
    #[doc = "0x314 - Histogram Registers"]
    pub histogram_fifo_data161: HISTOGRAM_FIFO_DATA161,
    #[doc = "0x318 - Histogram Registers"]
    pub histogram_fifo_data162: HISTOGRAM_FIFO_DATA162,
    #[doc = "0x31c - Histogram Registers"]
    pub histogram_fifo_data163: HISTOGRAM_FIFO_DATA163,
    #[doc = "0x320 - Histogram Registers"]
    pub histogram_fifo_data164: HISTOGRAM_FIFO_DATA164,
    #[doc = "0x324 - Histogram Registers"]
    pub histogram_fifo_data165: HISTOGRAM_FIFO_DATA165,
    #[doc = "0x328 - Histogram Registers"]
    pub histogram_fifo_data166: HISTOGRAM_FIFO_DATA166,
    #[doc = "0x32c - Histogram Registers"]
    pub histogram_fifo_data167: HISTOGRAM_FIFO_DATA167,
    #[doc = "0x330 - Histogram Registers"]
    pub histogram_fifo_data168: HISTOGRAM_FIFO_DATA168,
    #[doc = "0x334 - Histogram Registers"]
    pub histogram_fifo_data169: HISTOGRAM_FIFO_DATA169,
    #[doc = "0x338 - Histogram Registers"]
    pub histogram_fifo_data170: HISTOGRAM_FIFO_DATA170,
    #[doc = "0x33c - Histogram Registers"]
    pub histogram_fifo_data171: HISTOGRAM_FIFO_DATA171,
    #[doc = "0x340 - Histogram Registers"]
    pub histogram_fifo_data172: HISTOGRAM_FIFO_DATA172,
    #[doc = "0x344 - Histogram Registers"]
    pub histogram_fifo_data173: HISTOGRAM_FIFO_DATA173,
    #[doc = "0x348 - Histogram Registers"]
    pub histogram_fifo_data174: HISTOGRAM_FIFO_DATA174,
    #[doc = "0x34c - Histogram Registers"]
    pub histogram_fifo_data175: HISTOGRAM_FIFO_DATA175,
    #[doc = "0x350 - Histogram Registers"]
    pub histogram_fifo_data176: HISTOGRAM_FIFO_DATA176,
    #[doc = "0x354 - Histogram Registers"]
    pub histogram_fifo_data177: HISTOGRAM_FIFO_DATA177,
    #[doc = "0x358 - Histogram Registers"]
    pub histogram_fifo_data178: HISTOGRAM_FIFO_DATA178,
    #[doc = "0x35c - Histogram Registers"]
    pub histogram_fifo_data179: HISTOGRAM_FIFO_DATA179,
    #[doc = "0x360 - Histogram Registers"]
    pub histogram_fifo_data180: HISTOGRAM_FIFO_DATA180,
    #[doc = "0x364 - Histogram Registers"]
    pub histogram_fifo_data181: HISTOGRAM_FIFO_DATA181,
    #[doc = "0x368 - Histogram Registers"]
    pub histogram_fifo_data182: HISTOGRAM_FIFO_DATA182,
    #[doc = "0x36c - Histogram Registers"]
    pub histogram_fifo_data183: HISTOGRAM_FIFO_DATA183,
    #[doc = "0x370 - Histogram Registers"]
    pub histogram_fifo_data184: HISTOGRAM_FIFO_DATA184,
    #[doc = "0x374 - Histogram Registers"]
    pub histogram_fifo_data185: HISTOGRAM_FIFO_DATA185,
    #[doc = "0x378 - Histogram Registers"]
    pub histogram_fifo_data186: HISTOGRAM_FIFO_DATA186,
    #[doc = "0x37c - Histogram Registers"]
    pub histogram_fifo_data187: HISTOGRAM_FIFO_DATA187,
    #[doc = "0x380 - Histogram Registers"]
    pub histogram_fifo_data188: HISTOGRAM_FIFO_DATA188,
    #[doc = "0x384 - Histogram Registers"]
    pub histogram_fifo_data189: HISTOGRAM_FIFO_DATA189,
    #[doc = "0x388 - Histogram Registers"]
    pub histogram_fifo_data190: HISTOGRAM_FIFO_DATA190,
    #[doc = "0x38c - Histogram Registers"]
    pub histogram_fifo_data191: HISTOGRAM_FIFO_DATA191,
    #[doc = "0x390 - Histogram Registers"]
    pub histogram_fifo_data192: HISTOGRAM_FIFO_DATA192,
    #[doc = "0x394 - Histogram Registers"]
    pub histogram_fifo_data193: HISTOGRAM_FIFO_DATA193,
    #[doc = "0x398 - Histogram Registers"]
    pub histogram_fifo_data194: HISTOGRAM_FIFO_DATA194,
    #[doc = "0x39c - Histogram Registers"]
    pub histogram_fifo_data195: HISTOGRAM_FIFO_DATA195,
    #[doc = "0x3a0 - Histogram Registers"]
    pub histogram_fifo_data196: HISTOGRAM_FIFO_DATA196,
    #[doc = "0x3a4 - Histogram Registers"]
    pub histogram_fifo_data197: HISTOGRAM_FIFO_DATA197,
    #[doc = "0x3a8 - Histogram Registers"]
    pub histogram_fifo_data198: HISTOGRAM_FIFO_DATA198,
    #[doc = "0x3ac - Histogram Registers"]
    pub histogram_fifo_data199: HISTOGRAM_FIFO_DATA199,
    #[doc = "0x3b0 - Histogram Registers"]
    pub histogram_fifo_data200: HISTOGRAM_FIFO_DATA200,
    #[doc = "0x3b4 - Histogram Registers"]
    pub histogram_fifo_data201: HISTOGRAM_FIFO_DATA201,
    #[doc = "0x3b8 - Histogram Registers"]
    pub histogram_fifo_data202: HISTOGRAM_FIFO_DATA202,
    #[doc = "0x3bc - Histogram Registers"]
    pub histogram_fifo_data203: HISTOGRAM_FIFO_DATA203,
    #[doc = "0x3c0 - Histogram Registers"]
    pub histogram_fifo_data204: HISTOGRAM_FIFO_DATA204,
    #[doc = "0x3c4 - Histogram Registers"]
    pub histogram_fifo_data205: HISTOGRAM_FIFO_DATA205,
    #[doc = "0x3c8 - Histogram Registers"]
    pub histogram_fifo_data206: HISTOGRAM_FIFO_DATA206,
    #[doc = "0x3cc - Histogram Registers"]
    pub histogram_fifo_data207: HISTOGRAM_FIFO_DATA207,
    #[doc = "0x3d0 - Histogram Registers"]
    pub histogram_fifo_data208: HISTOGRAM_FIFO_DATA208,
    #[doc = "0x3d4 - Histogram Registers"]
    pub histogram_fifo_data209: HISTOGRAM_FIFO_DATA209,
    #[doc = "0x3d8 - Histogram Registers"]
    pub histogram_fifo_data210: HISTOGRAM_FIFO_DATA210,
    #[doc = "0x3dc - Histogram Registers"]
    pub histogram_fifo_data211: HISTOGRAM_FIFO_DATA211,
    #[doc = "0x3e0 - Histogram Registers"]
    pub histogram_fifo_data212: HISTOGRAM_FIFO_DATA212,
    #[doc = "0x3e4 - Histogram Registers"]
    pub histogram_fifo_data213: HISTOGRAM_FIFO_DATA213,
    #[doc = "0x3e8 - Histogram Registers"]
    pub histogram_fifo_data214: HISTOGRAM_FIFO_DATA214,
    #[doc = "0x3ec - Histogram Registers"]
    pub histogram_fifo_data215: HISTOGRAM_FIFO_DATA215,
    #[doc = "0x3f0 - Histogram Registers"]
    pub histogram_fifo_data216: HISTOGRAM_FIFO_DATA216,
    #[doc = "0x3f4 - Histogram Registers"]
    pub histogram_fifo_data217: HISTOGRAM_FIFO_DATA217,
    #[doc = "0x3f8 - Histogram Registers"]
    pub histogram_fifo_data218: HISTOGRAM_FIFO_DATA218,
    #[doc = "0x3fc - Histogram Registers"]
    pub histogram_fifo_data219: HISTOGRAM_FIFO_DATA219,
    #[doc = "0x400 - Histogram Registers"]
    pub histogram_fifo_data220: HISTOGRAM_FIFO_DATA220,
    #[doc = "0x404 - Histogram Registers"]
    pub histogram_fifo_data221: HISTOGRAM_FIFO_DATA221,
    #[doc = "0x408 - Histogram Registers"]
    pub histogram_fifo_data222: HISTOGRAM_FIFO_DATA222,
    #[doc = "0x40c - Histogram Registers"]
    pub histogram_fifo_data223: HISTOGRAM_FIFO_DATA223,
    #[doc = "0x410 - Histogram Registers"]
    pub histogram_fifo_data224: HISTOGRAM_FIFO_DATA224,
    #[doc = "0x414 - Histogram Registers"]
    pub histogram_fifo_data225: HISTOGRAM_FIFO_DATA225,
    #[doc = "0x418 - Histogram Registers"]
    pub histogram_fifo_data226: HISTOGRAM_FIFO_DATA226,
    #[doc = "0x41c - Histogram Registers"]
    pub histogram_fifo_data227: HISTOGRAM_FIFO_DATA227,
    #[doc = "0x420 - Histogram Registers"]
    pub histogram_fifo_data228: HISTOGRAM_FIFO_DATA228,
    #[doc = "0x424 - Histogram Registers"]
    pub histogram_fifo_data229: HISTOGRAM_FIFO_DATA229,
    #[doc = "0x428 - Histogram Registers"]
    pub histogram_fifo_data230: HISTOGRAM_FIFO_DATA230,
    #[doc = "0x42c - Histogram Registers"]
    pub histogram_fifo_data231: HISTOGRAM_FIFO_DATA231,
    #[doc = "0x430 - Histogram Registers"]
    pub histogram_fifo_data232: HISTOGRAM_FIFO_DATA232,
    #[doc = "0x434 - Histogram Registers"]
    pub histogram_fifo_data233: HISTOGRAM_FIFO_DATA233,
    #[doc = "0x438 - Histogram Registers"]
    pub histogram_fifo_data234: HISTOGRAM_FIFO_DATA234,
    #[doc = "0x43c - Histogram Registers"]
    pub histogram_fifo_data235: HISTOGRAM_FIFO_DATA235,
    #[doc = "0x440 - Histogram Registers"]
    pub histogram_fifo_data236: HISTOGRAM_FIFO_DATA236,
    #[doc = "0x444 - Histogram Registers"]
    pub histogram_fifo_data237: HISTOGRAM_FIFO_DATA237,
    #[doc = "0x448 - Histogram Registers"]
    pub histogram_fifo_data238: HISTOGRAM_FIFO_DATA238,
    #[doc = "0x44c - Histogram Registers"]
    pub histogram_fifo_data239: HISTOGRAM_FIFO_DATA239,
    #[doc = "0x450 - Histogram Registers"]
    pub histogram_fifo_data240: HISTOGRAM_FIFO_DATA240,
    #[doc = "0x454 - Histogram Registers"]
    pub histogram_fifo_data241: HISTOGRAM_FIFO_DATA241,
    #[doc = "0x458 - Histogram Registers"]
    pub histogram_fifo_data242: HISTOGRAM_FIFO_DATA242,
    #[doc = "0x45c - Histogram Registers"]
    pub histogram_fifo_data243: HISTOGRAM_FIFO_DATA243,
    #[doc = "0x460 - Histogram Registers"]
    pub histogram_fifo_data244: HISTOGRAM_FIFO_DATA244,
    #[doc = "0x464 - Histogram Registers"]
    pub histogram_fifo_data245: HISTOGRAM_FIFO_DATA245,
    #[doc = "0x468 - Histogram Registers"]
    pub histogram_fifo_data246: HISTOGRAM_FIFO_DATA246,
    #[doc = "0x46c - Histogram Registers"]
    pub histogram_fifo_data247: HISTOGRAM_FIFO_DATA247,
    #[doc = "0x470 - Histogram Registers"]
    pub histogram_fifo_data248: HISTOGRAM_FIFO_DATA248,
    #[doc = "0x474 - Histogram Registers"]
    pub histogram_fifo_data249: HISTOGRAM_FIFO_DATA249,
    #[doc = "0x478 - Histogram Registers"]
    pub histogram_fifo_data250: HISTOGRAM_FIFO_DATA250,
    #[doc = "0x47c - Histogram Registers"]
    pub histogram_fifo_data251: HISTOGRAM_FIFO_DATA251,
    #[doc = "0x480 - Histogram Registers"]
    pub histogram_fifo_data252: HISTOGRAM_FIFO_DATA252,
    #[doc = "0x484 - Histogram Registers"]
    pub histogram_fifo_data253: HISTOGRAM_FIFO_DATA253,
    #[doc = "0x488 - Histogram Registers"]
    pub histogram_fifo_data254: HISTOGRAM_FIFO_DATA254,
    #[doc = "0x48c - Histogram Registers"]
    pub histogram_fifo_data255: HISTOGRAM_FIFO_DATA255,
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Control Register"]
pub mod cr1;
#[doc = "INT_EN (rw) register accessor: an alias for `Reg<INT_EN_SPEC>`"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod int_en;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Control 2 Register"]
pub mod cr2;
#[doc = "STA (rw) register accessor: an alias for `Reg<STA_SPEC>`"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "Status Register"]
pub mod sta;
#[doc = "DMASA_FB1 (rw) register accessor: an alias for `Reg<DMASA_FB1_SPEC>`"]
pub type DMASA_FB1 = crate::Reg<dmasa_fb1::DMASA_FB1_SPEC>;
#[doc = "Pixel DMA Frame Buffer 1 Address"]
pub mod dmasa_fb1;
#[doc = "DMASA_FB2 (rw) register accessor: an alias for `Reg<DMASA_FB2_SPEC>`"]
pub type DMASA_FB2 = crate::Reg<dmasa_fb2::DMASA_FB2_SPEC>;
#[doc = "Pixel DMA Frame Buffer 2 Address"]
pub mod dmasa_fb2;
#[doc = "BUF_PARA (rw) register accessor: an alias for `Reg<BUF_PARA_SPEC>`"]
pub type BUF_PARA = crate::Reg<buf_para::BUF_PARA_SPEC>;
#[doc = "Buffer Parameters Register"]
pub mod buf_para;
#[doc = "IDEAL_WN_SIZE (rw) register accessor: an alias for `Reg<IDEAL_WN_SIZE_SPEC>`"]
pub type IDEAL_WN_SIZE = crate::Reg<ideal_wn_size::IDEAL_WN_SIZE_SPEC>;
#[doc = "Ideal Image Size Register"]
pub mod ideal_wn_size;
#[doc = "CR18 (rw) register accessor: an alias for `Reg<CR18_SPEC>`"]
pub type CR18 = crate::Reg<cr18::CR18_SPEC>;
#[doc = "Control CR18 Register"]
pub mod cr18;
#[doc = "DMASA_UV1 (rw) register accessor: an alias for `Reg<DMASA_UV1_SPEC>`"]
pub type DMASA_UV1 = crate::Reg<dmasa_uv1::DMASA_UV1_SPEC>;
#[doc = "Pixel UV DMA Frame Buffer 1 Address"]
pub mod dmasa_uv1;
#[doc = "DMASA_UV2 (rw) register accessor: an alias for `Reg<DMASA_UV2_SPEC>`"]
pub type DMASA_UV2 = crate::Reg<dmasa_uv2::DMASA_UV2_SPEC>;
#[doc = "Pixel UV DMA Frame Buffer 2 Address"]
pub mod dmasa_uv2;
#[doc = "CR20 (rw) register accessor: an alias for `Reg<CR20_SPEC>`"]
pub type CR20 = crate::Reg<cr20::CR20_SPEC>;
#[doc = "Control CR20 Register"]
pub mod cr20;
#[doc = "MAX_WN_CYCLE (rw) register accessor: an alias for `Reg<MAX_WN_CYCLE_SPEC>`"]
pub type MAX_WN_CYCLE = crate::Reg<max_wn_cycle::MAX_WN_CYCLE_SPEC>;
#[doc = "Max Window Size Register"]
pub mod max_wn_cycle;
#[doc = "CSC_COEF0 (rw) register accessor: an alias for `Reg<CSC_COEF0_SPEC>`"]
pub type CSC_COEF0 = crate::Reg<csc_coef0::CSC_COEF0_SPEC>;
#[doc = "Color Space Conversion Config Register 0"]
pub mod csc_coef0;
#[doc = "CSC_COEF1 (rw) register accessor: an alias for `Reg<CSC_COEF1_SPEC>`"]
pub type CSC_COEF1 = crate::Reg<csc_coef1::CSC_COEF1_SPEC>;
#[doc = "Color Space Conversion Config Register 1"]
pub mod csc_coef1;
#[doc = "CSC_COEF2 (rw) register accessor: an alias for `Reg<CSC_COEF2_SPEC>`"]
pub type CSC_COEF2 = crate::Reg<csc_coef2::CSC_COEF2_SPEC>;
#[doc = "Color Space Conversion Config Register 2"]
pub mod csc_coef2;
#[doc = "CLRKEY_LOW (rw) register accessor: an alias for `Reg<CLRKEY_LOW_SPEC>`"]
pub type CLRKEY_LOW = crate::Reg<clrkey_low::CLRKEY_LOW_SPEC>;
#[doc = "Low Color Key Register"]
pub mod clrkey_low;
#[doc = "CLRKEY_HIGH (rw) register accessor: an alias for `Reg<CLRKEY_HIGH_SPEC>`"]
pub type CLRKEY_HIGH = crate::Reg<clrkey_high::CLRKEY_HIGH_SPEC>;
#[doc = "High Color Key Register"]
pub mod clrkey_high;
#[doc = "HISTOGRAM_FIFO_DATA0 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA0_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA0 = crate::Reg<histogram_fifo_data0::HISTOGRAM_FIFO_DATA0_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data0;
#[doc = "HISTOGRAM_FIFO_DATA1 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA1_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA1 = crate::Reg<histogram_fifo_data1::HISTOGRAM_FIFO_DATA1_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data1;
#[doc = "HISTOGRAM_FIFO_DATA2 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA2_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA2 = crate::Reg<histogram_fifo_data2::HISTOGRAM_FIFO_DATA2_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data2;
#[doc = "HISTOGRAM_FIFO_DATA3 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA3_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA3 = crate::Reg<histogram_fifo_data3::HISTOGRAM_FIFO_DATA3_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data3;
#[doc = "HISTOGRAM_FIFO_DATA4 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA4_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA4 = crate::Reg<histogram_fifo_data4::HISTOGRAM_FIFO_DATA4_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data4;
#[doc = "HISTOGRAM_FIFO_DATA5 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA5_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA5 = crate::Reg<histogram_fifo_data5::HISTOGRAM_FIFO_DATA5_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data5;
#[doc = "HISTOGRAM_FIFO_DATA6 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA6_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA6 = crate::Reg<histogram_fifo_data6::HISTOGRAM_FIFO_DATA6_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data6;
#[doc = "HISTOGRAM_FIFO_DATA7 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA7_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA7 = crate::Reg<histogram_fifo_data7::HISTOGRAM_FIFO_DATA7_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data7;
#[doc = "HISTOGRAM_FIFO_DATA8 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA8_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA8 = crate::Reg<histogram_fifo_data8::HISTOGRAM_FIFO_DATA8_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data8;
#[doc = "HISTOGRAM_FIFO_DATA9 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA9_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA9 = crate::Reg<histogram_fifo_data9::HISTOGRAM_FIFO_DATA9_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data9;
#[doc = "HISTOGRAM_FIFO_DATA10 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA10_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA10 = crate::Reg<histogram_fifo_data10::HISTOGRAM_FIFO_DATA10_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data10;
#[doc = "HISTOGRAM_FIFO_DATA11 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA11_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA11 = crate::Reg<histogram_fifo_data11::HISTOGRAM_FIFO_DATA11_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data11;
#[doc = "HISTOGRAM_FIFO_DATA12 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA12_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA12 = crate::Reg<histogram_fifo_data12::HISTOGRAM_FIFO_DATA12_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data12;
#[doc = "HISTOGRAM_FIFO_DATA13 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA13_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA13 = crate::Reg<histogram_fifo_data13::HISTOGRAM_FIFO_DATA13_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data13;
#[doc = "HISTOGRAM_FIFO_DATA14 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA14_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA14 = crate::Reg<histogram_fifo_data14::HISTOGRAM_FIFO_DATA14_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data14;
#[doc = "HISTOGRAM_FIFO_DATA15 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA15_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA15 = crate::Reg<histogram_fifo_data15::HISTOGRAM_FIFO_DATA15_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data15;
#[doc = "HISTOGRAM_FIFO_DATA16 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA16_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA16 = crate::Reg<histogram_fifo_data16::HISTOGRAM_FIFO_DATA16_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data16;
#[doc = "HISTOGRAM_FIFO_DATA17 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA17_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA17 = crate::Reg<histogram_fifo_data17::HISTOGRAM_FIFO_DATA17_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data17;
#[doc = "HISTOGRAM_FIFO_DATA18 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA18_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA18 = crate::Reg<histogram_fifo_data18::HISTOGRAM_FIFO_DATA18_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data18;
#[doc = "HISTOGRAM_FIFO_DATA19 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA19_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA19 = crate::Reg<histogram_fifo_data19::HISTOGRAM_FIFO_DATA19_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data19;
#[doc = "HISTOGRAM_FIFO_DATA20 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA20_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA20 = crate::Reg<histogram_fifo_data20::HISTOGRAM_FIFO_DATA20_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data20;
#[doc = "HISTOGRAM_FIFO_DATA21 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA21_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA21 = crate::Reg<histogram_fifo_data21::HISTOGRAM_FIFO_DATA21_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data21;
#[doc = "HISTOGRAM_FIFO_DATA22 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA22_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA22 = crate::Reg<histogram_fifo_data22::HISTOGRAM_FIFO_DATA22_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data22;
#[doc = "HISTOGRAM_FIFO_DATA23 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA23_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA23 = crate::Reg<histogram_fifo_data23::HISTOGRAM_FIFO_DATA23_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data23;
#[doc = "HISTOGRAM_FIFO_DATA24 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA24_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA24 = crate::Reg<histogram_fifo_data24::HISTOGRAM_FIFO_DATA24_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data24;
#[doc = "HISTOGRAM_FIFO_DATA25 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA25_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA25 = crate::Reg<histogram_fifo_data25::HISTOGRAM_FIFO_DATA25_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data25;
#[doc = "HISTOGRAM_FIFO_DATA26 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA26_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA26 = crate::Reg<histogram_fifo_data26::HISTOGRAM_FIFO_DATA26_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data26;
#[doc = "HISTOGRAM_FIFO_DATA27 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA27_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA27 = crate::Reg<histogram_fifo_data27::HISTOGRAM_FIFO_DATA27_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data27;
#[doc = "HISTOGRAM_FIFO_DATA28 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA28_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA28 = crate::Reg<histogram_fifo_data28::HISTOGRAM_FIFO_DATA28_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data28;
#[doc = "HISTOGRAM_FIFO_DATA29 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA29_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA29 = crate::Reg<histogram_fifo_data29::HISTOGRAM_FIFO_DATA29_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data29;
#[doc = "HISTOGRAM_FIFO_DATA30 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA30_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA30 = crate::Reg<histogram_fifo_data30::HISTOGRAM_FIFO_DATA30_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data30;
#[doc = "HISTOGRAM_FIFO_DATA31 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA31_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA31 = crate::Reg<histogram_fifo_data31::HISTOGRAM_FIFO_DATA31_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data31;
#[doc = "HISTOGRAM_FIFO_DATA32 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA32_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA32 = crate::Reg<histogram_fifo_data32::HISTOGRAM_FIFO_DATA32_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data32;
#[doc = "HISTOGRAM_FIFO_DATA33 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA33_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA33 = crate::Reg<histogram_fifo_data33::HISTOGRAM_FIFO_DATA33_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data33;
#[doc = "HISTOGRAM_FIFO_DATA34 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA34_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA34 = crate::Reg<histogram_fifo_data34::HISTOGRAM_FIFO_DATA34_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data34;
#[doc = "HISTOGRAM_FIFO_DATA35 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA35_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA35 = crate::Reg<histogram_fifo_data35::HISTOGRAM_FIFO_DATA35_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data35;
#[doc = "HISTOGRAM_FIFO_DATA36 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA36_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA36 = crate::Reg<histogram_fifo_data36::HISTOGRAM_FIFO_DATA36_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data36;
#[doc = "HISTOGRAM_FIFO_DATA37 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA37_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA37 = crate::Reg<histogram_fifo_data37::HISTOGRAM_FIFO_DATA37_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data37;
#[doc = "HISTOGRAM_FIFO_DATA38 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA38_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA38 = crate::Reg<histogram_fifo_data38::HISTOGRAM_FIFO_DATA38_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data38;
#[doc = "HISTOGRAM_FIFO_DATA39 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA39_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA39 = crate::Reg<histogram_fifo_data39::HISTOGRAM_FIFO_DATA39_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data39;
#[doc = "HISTOGRAM_FIFO_DATA40 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA40_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA40 = crate::Reg<histogram_fifo_data40::HISTOGRAM_FIFO_DATA40_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data40;
#[doc = "HISTOGRAM_FIFO_DATA41 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA41_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA41 = crate::Reg<histogram_fifo_data41::HISTOGRAM_FIFO_DATA41_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data41;
#[doc = "HISTOGRAM_FIFO_DATA42 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA42_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA42 = crate::Reg<histogram_fifo_data42::HISTOGRAM_FIFO_DATA42_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data42;
#[doc = "HISTOGRAM_FIFO_DATA43 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA43_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA43 = crate::Reg<histogram_fifo_data43::HISTOGRAM_FIFO_DATA43_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data43;
#[doc = "HISTOGRAM_FIFO_DATA44 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA44_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA44 = crate::Reg<histogram_fifo_data44::HISTOGRAM_FIFO_DATA44_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data44;
#[doc = "HISTOGRAM_FIFO_DATA45 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA45_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA45 = crate::Reg<histogram_fifo_data45::HISTOGRAM_FIFO_DATA45_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data45;
#[doc = "HISTOGRAM_FIFO_DATA46 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA46_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA46 = crate::Reg<histogram_fifo_data46::HISTOGRAM_FIFO_DATA46_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data46;
#[doc = "HISTOGRAM_FIFO_DATA47 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA47_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA47 = crate::Reg<histogram_fifo_data47::HISTOGRAM_FIFO_DATA47_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data47;
#[doc = "HISTOGRAM_FIFO_DATA48 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA48_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA48 = crate::Reg<histogram_fifo_data48::HISTOGRAM_FIFO_DATA48_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data48;
#[doc = "HISTOGRAM_FIFO_DATA49 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA49_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA49 = crate::Reg<histogram_fifo_data49::HISTOGRAM_FIFO_DATA49_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data49;
#[doc = "HISTOGRAM_FIFO_DATA50 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA50_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA50 = crate::Reg<histogram_fifo_data50::HISTOGRAM_FIFO_DATA50_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data50;
#[doc = "HISTOGRAM_FIFO_DATA51 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA51_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA51 = crate::Reg<histogram_fifo_data51::HISTOGRAM_FIFO_DATA51_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data51;
#[doc = "HISTOGRAM_FIFO_DATA52 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA52_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA52 = crate::Reg<histogram_fifo_data52::HISTOGRAM_FIFO_DATA52_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data52;
#[doc = "HISTOGRAM_FIFO_DATA53 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA53_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA53 = crate::Reg<histogram_fifo_data53::HISTOGRAM_FIFO_DATA53_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data53;
#[doc = "HISTOGRAM_FIFO_DATA54 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA54_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA54 = crate::Reg<histogram_fifo_data54::HISTOGRAM_FIFO_DATA54_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data54;
#[doc = "HISTOGRAM_FIFO_DATA55 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA55_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA55 = crate::Reg<histogram_fifo_data55::HISTOGRAM_FIFO_DATA55_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data55;
#[doc = "HISTOGRAM_FIFO_DATA56 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA56_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA56 = crate::Reg<histogram_fifo_data56::HISTOGRAM_FIFO_DATA56_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data56;
#[doc = "HISTOGRAM_FIFO_DATA57 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA57_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA57 = crate::Reg<histogram_fifo_data57::HISTOGRAM_FIFO_DATA57_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data57;
#[doc = "HISTOGRAM_FIFO_DATA58 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA58_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA58 = crate::Reg<histogram_fifo_data58::HISTOGRAM_FIFO_DATA58_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data58;
#[doc = "HISTOGRAM_FIFO_DATA59 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA59_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA59 = crate::Reg<histogram_fifo_data59::HISTOGRAM_FIFO_DATA59_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data59;
#[doc = "HISTOGRAM_FIFO_DATA60 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA60_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA60 = crate::Reg<histogram_fifo_data60::HISTOGRAM_FIFO_DATA60_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data60;
#[doc = "HISTOGRAM_FIFO_DATA61 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA61_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA61 = crate::Reg<histogram_fifo_data61::HISTOGRAM_FIFO_DATA61_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data61;
#[doc = "HISTOGRAM_FIFO_DATA62 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA62_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA62 = crate::Reg<histogram_fifo_data62::HISTOGRAM_FIFO_DATA62_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data62;
#[doc = "HISTOGRAM_FIFO_DATA63 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA63_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA63 = crate::Reg<histogram_fifo_data63::HISTOGRAM_FIFO_DATA63_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data63;
#[doc = "HISTOGRAM_FIFO_DATA64 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA64_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA64 = crate::Reg<histogram_fifo_data64::HISTOGRAM_FIFO_DATA64_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data64;
#[doc = "HISTOGRAM_FIFO_DATA65 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA65_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA65 = crate::Reg<histogram_fifo_data65::HISTOGRAM_FIFO_DATA65_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data65;
#[doc = "HISTOGRAM_FIFO_DATA66 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA66_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA66 = crate::Reg<histogram_fifo_data66::HISTOGRAM_FIFO_DATA66_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data66;
#[doc = "HISTOGRAM_FIFO_DATA67 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA67_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA67 = crate::Reg<histogram_fifo_data67::HISTOGRAM_FIFO_DATA67_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data67;
#[doc = "HISTOGRAM_FIFO_DATA68 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA68_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA68 = crate::Reg<histogram_fifo_data68::HISTOGRAM_FIFO_DATA68_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data68;
#[doc = "HISTOGRAM_FIFO_DATA69 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA69_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA69 = crate::Reg<histogram_fifo_data69::HISTOGRAM_FIFO_DATA69_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data69;
#[doc = "HISTOGRAM_FIFO_DATA70 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA70_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA70 = crate::Reg<histogram_fifo_data70::HISTOGRAM_FIFO_DATA70_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data70;
#[doc = "HISTOGRAM_FIFO_DATA71 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA71_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA71 = crate::Reg<histogram_fifo_data71::HISTOGRAM_FIFO_DATA71_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data71;
#[doc = "HISTOGRAM_FIFO_DATA72 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA72_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA72 = crate::Reg<histogram_fifo_data72::HISTOGRAM_FIFO_DATA72_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data72;
#[doc = "HISTOGRAM_FIFO_DATA73 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA73_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA73 = crate::Reg<histogram_fifo_data73::HISTOGRAM_FIFO_DATA73_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data73;
#[doc = "HISTOGRAM_FIFO_DATA74 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA74_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA74 = crate::Reg<histogram_fifo_data74::HISTOGRAM_FIFO_DATA74_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data74;
#[doc = "HISTOGRAM_FIFO_DATA75 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA75_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA75 = crate::Reg<histogram_fifo_data75::HISTOGRAM_FIFO_DATA75_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data75;
#[doc = "HISTOGRAM_FIFO_DATA76 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA76_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA76 = crate::Reg<histogram_fifo_data76::HISTOGRAM_FIFO_DATA76_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data76;
#[doc = "HISTOGRAM_FIFO_DATA77 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA77_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA77 = crate::Reg<histogram_fifo_data77::HISTOGRAM_FIFO_DATA77_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data77;
#[doc = "HISTOGRAM_FIFO_DATA78 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA78_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA78 = crate::Reg<histogram_fifo_data78::HISTOGRAM_FIFO_DATA78_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data78;
#[doc = "HISTOGRAM_FIFO_DATA79 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA79_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA79 = crate::Reg<histogram_fifo_data79::HISTOGRAM_FIFO_DATA79_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data79;
#[doc = "HISTOGRAM_FIFO_DATA80 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA80_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA80 = crate::Reg<histogram_fifo_data80::HISTOGRAM_FIFO_DATA80_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data80;
#[doc = "HISTOGRAM_FIFO_DATA81 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA81_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA81 = crate::Reg<histogram_fifo_data81::HISTOGRAM_FIFO_DATA81_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data81;
#[doc = "HISTOGRAM_FIFO_DATA82 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA82_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA82 = crate::Reg<histogram_fifo_data82::HISTOGRAM_FIFO_DATA82_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data82;
#[doc = "HISTOGRAM_FIFO_DATA83 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA83_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA83 = crate::Reg<histogram_fifo_data83::HISTOGRAM_FIFO_DATA83_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data83;
#[doc = "HISTOGRAM_FIFO_DATA84 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA84_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA84 = crate::Reg<histogram_fifo_data84::HISTOGRAM_FIFO_DATA84_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data84;
#[doc = "HISTOGRAM_FIFO_DATA85 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA85_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA85 = crate::Reg<histogram_fifo_data85::HISTOGRAM_FIFO_DATA85_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data85;
#[doc = "HISTOGRAM_FIFO_DATA86 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA86_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA86 = crate::Reg<histogram_fifo_data86::HISTOGRAM_FIFO_DATA86_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data86;
#[doc = "HISTOGRAM_FIFO_DATA87 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA87_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA87 = crate::Reg<histogram_fifo_data87::HISTOGRAM_FIFO_DATA87_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data87;
#[doc = "HISTOGRAM_FIFO_DATA88 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA88_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA88 = crate::Reg<histogram_fifo_data88::HISTOGRAM_FIFO_DATA88_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data88;
#[doc = "HISTOGRAM_FIFO_DATA89 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA89_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA89 = crate::Reg<histogram_fifo_data89::HISTOGRAM_FIFO_DATA89_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data89;
#[doc = "HISTOGRAM_FIFO_DATA90 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA90_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA90 = crate::Reg<histogram_fifo_data90::HISTOGRAM_FIFO_DATA90_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data90;
#[doc = "HISTOGRAM_FIFO_DATA91 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA91_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA91 = crate::Reg<histogram_fifo_data91::HISTOGRAM_FIFO_DATA91_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data91;
#[doc = "HISTOGRAM_FIFO_DATA92 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA92_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA92 = crate::Reg<histogram_fifo_data92::HISTOGRAM_FIFO_DATA92_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data92;
#[doc = "HISTOGRAM_FIFO_DATA93 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA93_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA93 = crate::Reg<histogram_fifo_data93::HISTOGRAM_FIFO_DATA93_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data93;
#[doc = "HISTOGRAM_FIFO_DATA94 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA94_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA94 = crate::Reg<histogram_fifo_data94::HISTOGRAM_FIFO_DATA94_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data94;
#[doc = "HISTOGRAM_FIFO_DATA95 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA95_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA95 = crate::Reg<histogram_fifo_data95::HISTOGRAM_FIFO_DATA95_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data95;
#[doc = "HISTOGRAM_FIFO_DATA96 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA96_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA96 = crate::Reg<histogram_fifo_data96::HISTOGRAM_FIFO_DATA96_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data96;
#[doc = "HISTOGRAM_FIFO_DATA97 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA97_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA97 = crate::Reg<histogram_fifo_data97::HISTOGRAM_FIFO_DATA97_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data97;
#[doc = "HISTOGRAM_FIFO_DATA98 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA98_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA98 = crate::Reg<histogram_fifo_data98::HISTOGRAM_FIFO_DATA98_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data98;
#[doc = "HISTOGRAM_FIFO_DATA99 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA99_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA99 = crate::Reg<histogram_fifo_data99::HISTOGRAM_FIFO_DATA99_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data99;
#[doc = "HISTOGRAM_FIFO_DATA100 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA100_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA100 = crate::Reg<histogram_fifo_data100::HISTOGRAM_FIFO_DATA100_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data100;
#[doc = "HISTOGRAM_FIFO_DATA101 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA101_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA101 = crate::Reg<histogram_fifo_data101::HISTOGRAM_FIFO_DATA101_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data101;
#[doc = "HISTOGRAM_FIFO_DATA102 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA102_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA102 = crate::Reg<histogram_fifo_data102::HISTOGRAM_FIFO_DATA102_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data102;
#[doc = "HISTOGRAM_FIFO_DATA103 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA103_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA103 = crate::Reg<histogram_fifo_data103::HISTOGRAM_FIFO_DATA103_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data103;
#[doc = "HISTOGRAM_FIFO_DATA104 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA104_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA104 = crate::Reg<histogram_fifo_data104::HISTOGRAM_FIFO_DATA104_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data104;
#[doc = "HISTOGRAM_FIFO_DATA105 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA105_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA105 = crate::Reg<histogram_fifo_data105::HISTOGRAM_FIFO_DATA105_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data105;
#[doc = "HISTOGRAM_FIFO_DATA106 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA106_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA106 = crate::Reg<histogram_fifo_data106::HISTOGRAM_FIFO_DATA106_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data106;
#[doc = "HISTOGRAM_FIFO_DATA107 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA107_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA107 = crate::Reg<histogram_fifo_data107::HISTOGRAM_FIFO_DATA107_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data107;
#[doc = "HISTOGRAM_FIFO_DATA108 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA108_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA108 = crate::Reg<histogram_fifo_data108::HISTOGRAM_FIFO_DATA108_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data108;
#[doc = "HISTOGRAM_FIFO_DATA109 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA109_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA109 = crate::Reg<histogram_fifo_data109::HISTOGRAM_FIFO_DATA109_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data109;
#[doc = "HISTOGRAM_FIFO_DATA110 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA110_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA110 = crate::Reg<histogram_fifo_data110::HISTOGRAM_FIFO_DATA110_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data110;
#[doc = "HISTOGRAM_FIFO_DATA111 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA111_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA111 = crate::Reg<histogram_fifo_data111::HISTOGRAM_FIFO_DATA111_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data111;
#[doc = "HISTOGRAM_FIFO_DATA112 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA112_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA112 = crate::Reg<histogram_fifo_data112::HISTOGRAM_FIFO_DATA112_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data112;
#[doc = "HISTOGRAM_FIFO_DATA113 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA113_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA113 = crate::Reg<histogram_fifo_data113::HISTOGRAM_FIFO_DATA113_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data113;
#[doc = "HISTOGRAM_FIFO_DATA114 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA114_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA114 = crate::Reg<histogram_fifo_data114::HISTOGRAM_FIFO_DATA114_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data114;
#[doc = "HISTOGRAM_FIFO_DATA115 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA115_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA115 = crate::Reg<histogram_fifo_data115::HISTOGRAM_FIFO_DATA115_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data115;
#[doc = "HISTOGRAM_FIFO_DATA116 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA116_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA116 = crate::Reg<histogram_fifo_data116::HISTOGRAM_FIFO_DATA116_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data116;
#[doc = "HISTOGRAM_FIFO_DATA117 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA117_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA117 = crate::Reg<histogram_fifo_data117::HISTOGRAM_FIFO_DATA117_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data117;
#[doc = "HISTOGRAM_FIFO_DATA118 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA118_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA118 = crate::Reg<histogram_fifo_data118::HISTOGRAM_FIFO_DATA118_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data118;
#[doc = "HISTOGRAM_FIFO_DATA119 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA119_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA119 = crate::Reg<histogram_fifo_data119::HISTOGRAM_FIFO_DATA119_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data119;
#[doc = "HISTOGRAM_FIFO_DATA120 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA120_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA120 = crate::Reg<histogram_fifo_data120::HISTOGRAM_FIFO_DATA120_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data120;
#[doc = "HISTOGRAM_FIFO_DATA121 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA121_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA121 = crate::Reg<histogram_fifo_data121::HISTOGRAM_FIFO_DATA121_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data121;
#[doc = "HISTOGRAM_FIFO_DATA122 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA122_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA122 = crate::Reg<histogram_fifo_data122::HISTOGRAM_FIFO_DATA122_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data122;
#[doc = "HISTOGRAM_FIFO_DATA123 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA123_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA123 = crate::Reg<histogram_fifo_data123::HISTOGRAM_FIFO_DATA123_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data123;
#[doc = "HISTOGRAM_FIFO_DATA124 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA124_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA124 = crate::Reg<histogram_fifo_data124::HISTOGRAM_FIFO_DATA124_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data124;
#[doc = "HISTOGRAM_FIFO_DATA125 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA125_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA125 = crate::Reg<histogram_fifo_data125::HISTOGRAM_FIFO_DATA125_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data125;
#[doc = "HISTOGRAM_FIFO_DATA126 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA126_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA126 = crate::Reg<histogram_fifo_data126::HISTOGRAM_FIFO_DATA126_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data126;
#[doc = "HISTOGRAM_FIFO_DATA127 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA127_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA127 = crate::Reg<histogram_fifo_data127::HISTOGRAM_FIFO_DATA127_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data127;
#[doc = "HISTOGRAM_FIFO_DATA128 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA128_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA128 = crate::Reg<histogram_fifo_data128::HISTOGRAM_FIFO_DATA128_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data128;
#[doc = "HISTOGRAM_FIFO_DATA129 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA129_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA129 = crate::Reg<histogram_fifo_data129::HISTOGRAM_FIFO_DATA129_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data129;
#[doc = "HISTOGRAM_FIFO_DATA130 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA130_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA130 = crate::Reg<histogram_fifo_data130::HISTOGRAM_FIFO_DATA130_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data130;
#[doc = "HISTOGRAM_FIFO_DATA131 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA131_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA131 = crate::Reg<histogram_fifo_data131::HISTOGRAM_FIFO_DATA131_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data131;
#[doc = "HISTOGRAM_FIFO_DATA132 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA132_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA132 = crate::Reg<histogram_fifo_data132::HISTOGRAM_FIFO_DATA132_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data132;
#[doc = "HISTOGRAM_FIFO_DATA133 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA133_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA133 = crate::Reg<histogram_fifo_data133::HISTOGRAM_FIFO_DATA133_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data133;
#[doc = "HISTOGRAM_FIFO_DATA134 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA134_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA134 = crate::Reg<histogram_fifo_data134::HISTOGRAM_FIFO_DATA134_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data134;
#[doc = "HISTOGRAM_FIFO_DATA135 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA135_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA135 = crate::Reg<histogram_fifo_data135::HISTOGRAM_FIFO_DATA135_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data135;
#[doc = "HISTOGRAM_FIFO_DATA136 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA136_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA136 = crate::Reg<histogram_fifo_data136::HISTOGRAM_FIFO_DATA136_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data136;
#[doc = "HISTOGRAM_FIFO_DATA137 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA137_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA137 = crate::Reg<histogram_fifo_data137::HISTOGRAM_FIFO_DATA137_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data137;
#[doc = "HISTOGRAM_FIFO_DATA138 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA138_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA138 = crate::Reg<histogram_fifo_data138::HISTOGRAM_FIFO_DATA138_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data138;
#[doc = "HISTOGRAM_FIFO_DATA139 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA139_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA139 = crate::Reg<histogram_fifo_data139::HISTOGRAM_FIFO_DATA139_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data139;
#[doc = "HISTOGRAM_FIFO_DATA140 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA140_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA140 = crate::Reg<histogram_fifo_data140::HISTOGRAM_FIFO_DATA140_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data140;
#[doc = "HISTOGRAM_FIFO_DATA141 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA141_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA141 = crate::Reg<histogram_fifo_data141::HISTOGRAM_FIFO_DATA141_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data141;
#[doc = "HISTOGRAM_FIFO_DATA142 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA142_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA142 = crate::Reg<histogram_fifo_data142::HISTOGRAM_FIFO_DATA142_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data142;
#[doc = "HISTOGRAM_FIFO_DATA143 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA143_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA143 = crate::Reg<histogram_fifo_data143::HISTOGRAM_FIFO_DATA143_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data143;
#[doc = "HISTOGRAM_FIFO_DATA144 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA144_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA144 = crate::Reg<histogram_fifo_data144::HISTOGRAM_FIFO_DATA144_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data144;
#[doc = "HISTOGRAM_FIFO_DATA145 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA145_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA145 = crate::Reg<histogram_fifo_data145::HISTOGRAM_FIFO_DATA145_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data145;
#[doc = "HISTOGRAM_FIFO_DATA146 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA146_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA146 = crate::Reg<histogram_fifo_data146::HISTOGRAM_FIFO_DATA146_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data146;
#[doc = "HISTOGRAM_FIFO_DATA147 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA147_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA147 = crate::Reg<histogram_fifo_data147::HISTOGRAM_FIFO_DATA147_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data147;
#[doc = "HISTOGRAM_FIFO_DATA148 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA148_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA148 = crate::Reg<histogram_fifo_data148::HISTOGRAM_FIFO_DATA148_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data148;
#[doc = "HISTOGRAM_FIFO_DATA149 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA149_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA149 = crate::Reg<histogram_fifo_data149::HISTOGRAM_FIFO_DATA149_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data149;
#[doc = "HISTOGRAM_FIFO_DATA150 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA150_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA150 = crate::Reg<histogram_fifo_data150::HISTOGRAM_FIFO_DATA150_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data150;
#[doc = "HISTOGRAM_FIFO_DATA151 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA151_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA151 = crate::Reg<histogram_fifo_data151::HISTOGRAM_FIFO_DATA151_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data151;
#[doc = "HISTOGRAM_FIFO_DATA152 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA152_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA152 = crate::Reg<histogram_fifo_data152::HISTOGRAM_FIFO_DATA152_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data152;
#[doc = "HISTOGRAM_FIFO_DATA153 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA153_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA153 = crate::Reg<histogram_fifo_data153::HISTOGRAM_FIFO_DATA153_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data153;
#[doc = "HISTOGRAM_FIFO_DATA154 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA154_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA154 = crate::Reg<histogram_fifo_data154::HISTOGRAM_FIFO_DATA154_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data154;
#[doc = "HISTOGRAM_FIFO_DATA155 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA155_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA155 = crate::Reg<histogram_fifo_data155::HISTOGRAM_FIFO_DATA155_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data155;
#[doc = "HISTOGRAM_FIFO_DATA156 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA156_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA156 = crate::Reg<histogram_fifo_data156::HISTOGRAM_FIFO_DATA156_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data156;
#[doc = "HISTOGRAM_FIFO_DATA157 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA157_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA157 = crate::Reg<histogram_fifo_data157::HISTOGRAM_FIFO_DATA157_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data157;
#[doc = "HISTOGRAM_FIFO_DATA158 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA158_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA158 = crate::Reg<histogram_fifo_data158::HISTOGRAM_FIFO_DATA158_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data158;
#[doc = "HISTOGRAM_FIFO_DATA159 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA159_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA159 = crate::Reg<histogram_fifo_data159::HISTOGRAM_FIFO_DATA159_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data159;
#[doc = "HISTOGRAM_FIFO_DATA160 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA160_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA160 = crate::Reg<histogram_fifo_data160::HISTOGRAM_FIFO_DATA160_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data160;
#[doc = "HISTOGRAM_FIFO_DATA161 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA161_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA161 = crate::Reg<histogram_fifo_data161::HISTOGRAM_FIFO_DATA161_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data161;
#[doc = "HISTOGRAM_FIFO_DATA162 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA162_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA162 = crate::Reg<histogram_fifo_data162::HISTOGRAM_FIFO_DATA162_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data162;
#[doc = "HISTOGRAM_FIFO_DATA163 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA163_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA163 = crate::Reg<histogram_fifo_data163::HISTOGRAM_FIFO_DATA163_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data163;
#[doc = "HISTOGRAM_FIFO_DATA164 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA164_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA164 = crate::Reg<histogram_fifo_data164::HISTOGRAM_FIFO_DATA164_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data164;
#[doc = "HISTOGRAM_FIFO_DATA165 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA165_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA165 = crate::Reg<histogram_fifo_data165::HISTOGRAM_FIFO_DATA165_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data165;
#[doc = "HISTOGRAM_FIFO_DATA166 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA166_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA166 = crate::Reg<histogram_fifo_data166::HISTOGRAM_FIFO_DATA166_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data166;
#[doc = "HISTOGRAM_FIFO_DATA167 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA167_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA167 = crate::Reg<histogram_fifo_data167::HISTOGRAM_FIFO_DATA167_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data167;
#[doc = "HISTOGRAM_FIFO_DATA168 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA168_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA168 = crate::Reg<histogram_fifo_data168::HISTOGRAM_FIFO_DATA168_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data168;
#[doc = "HISTOGRAM_FIFO_DATA169 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA169_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA169 = crate::Reg<histogram_fifo_data169::HISTOGRAM_FIFO_DATA169_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data169;
#[doc = "HISTOGRAM_FIFO_DATA170 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA170_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA170 = crate::Reg<histogram_fifo_data170::HISTOGRAM_FIFO_DATA170_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data170;
#[doc = "HISTOGRAM_FIFO_DATA171 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA171_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA171 = crate::Reg<histogram_fifo_data171::HISTOGRAM_FIFO_DATA171_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data171;
#[doc = "HISTOGRAM_FIFO_DATA172 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA172_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA172 = crate::Reg<histogram_fifo_data172::HISTOGRAM_FIFO_DATA172_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data172;
#[doc = "HISTOGRAM_FIFO_DATA173 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA173_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA173 = crate::Reg<histogram_fifo_data173::HISTOGRAM_FIFO_DATA173_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data173;
#[doc = "HISTOGRAM_FIFO_DATA174 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA174_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA174 = crate::Reg<histogram_fifo_data174::HISTOGRAM_FIFO_DATA174_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data174;
#[doc = "HISTOGRAM_FIFO_DATA175 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA175_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA175 = crate::Reg<histogram_fifo_data175::HISTOGRAM_FIFO_DATA175_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data175;
#[doc = "HISTOGRAM_FIFO_DATA176 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA176_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA176 = crate::Reg<histogram_fifo_data176::HISTOGRAM_FIFO_DATA176_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data176;
#[doc = "HISTOGRAM_FIFO_DATA177 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA177_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA177 = crate::Reg<histogram_fifo_data177::HISTOGRAM_FIFO_DATA177_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data177;
#[doc = "HISTOGRAM_FIFO_DATA178 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA178_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA178 = crate::Reg<histogram_fifo_data178::HISTOGRAM_FIFO_DATA178_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data178;
#[doc = "HISTOGRAM_FIFO_DATA179 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA179_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA179 = crate::Reg<histogram_fifo_data179::HISTOGRAM_FIFO_DATA179_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data179;
#[doc = "HISTOGRAM_FIFO_DATA180 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA180_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA180 = crate::Reg<histogram_fifo_data180::HISTOGRAM_FIFO_DATA180_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data180;
#[doc = "HISTOGRAM_FIFO_DATA181 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA181_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA181 = crate::Reg<histogram_fifo_data181::HISTOGRAM_FIFO_DATA181_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data181;
#[doc = "HISTOGRAM_FIFO_DATA182 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA182_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA182 = crate::Reg<histogram_fifo_data182::HISTOGRAM_FIFO_DATA182_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data182;
#[doc = "HISTOGRAM_FIFO_DATA183 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA183_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA183 = crate::Reg<histogram_fifo_data183::HISTOGRAM_FIFO_DATA183_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data183;
#[doc = "HISTOGRAM_FIFO_DATA184 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA184_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA184 = crate::Reg<histogram_fifo_data184::HISTOGRAM_FIFO_DATA184_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data184;
#[doc = "HISTOGRAM_FIFO_DATA185 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA185_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA185 = crate::Reg<histogram_fifo_data185::HISTOGRAM_FIFO_DATA185_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data185;
#[doc = "HISTOGRAM_FIFO_DATA186 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA186_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA186 = crate::Reg<histogram_fifo_data186::HISTOGRAM_FIFO_DATA186_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data186;
#[doc = "HISTOGRAM_FIFO_DATA187 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA187_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA187 = crate::Reg<histogram_fifo_data187::HISTOGRAM_FIFO_DATA187_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data187;
#[doc = "HISTOGRAM_FIFO_DATA188 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA188_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA188 = crate::Reg<histogram_fifo_data188::HISTOGRAM_FIFO_DATA188_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data188;
#[doc = "HISTOGRAM_FIFO_DATA189 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA189_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA189 = crate::Reg<histogram_fifo_data189::HISTOGRAM_FIFO_DATA189_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data189;
#[doc = "HISTOGRAM_FIFO_DATA190 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA190_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA190 = crate::Reg<histogram_fifo_data190::HISTOGRAM_FIFO_DATA190_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data190;
#[doc = "HISTOGRAM_FIFO_DATA191 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA191_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA191 = crate::Reg<histogram_fifo_data191::HISTOGRAM_FIFO_DATA191_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data191;
#[doc = "HISTOGRAM_FIFO_DATA192 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA192_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA192 = crate::Reg<histogram_fifo_data192::HISTOGRAM_FIFO_DATA192_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data192;
#[doc = "HISTOGRAM_FIFO_DATA193 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA193_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA193 = crate::Reg<histogram_fifo_data193::HISTOGRAM_FIFO_DATA193_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data193;
#[doc = "HISTOGRAM_FIFO_DATA194 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA194_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA194 = crate::Reg<histogram_fifo_data194::HISTOGRAM_FIFO_DATA194_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data194;
#[doc = "HISTOGRAM_FIFO_DATA195 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA195_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA195 = crate::Reg<histogram_fifo_data195::HISTOGRAM_FIFO_DATA195_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data195;
#[doc = "HISTOGRAM_FIFO_DATA196 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA196_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA196 = crate::Reg<histogram_fifo_data196::HISTOGRAM_FIFO_DATA196_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data196;
#[doc = "HISTOGRAM_FIFO_DATA197 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA197_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA197 = crate::Reg<histogram_fifo_data197::HISTOGRAM_FIFO_DATA197_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data197;
#[doc = "HISTOGRAM_FIFO_DATA198 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA198_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA198 = crate::Reg<histogram_fifo_data198::HISTOGRAM_FIFO_DATA198_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data198;
#[doc = "HISTOGRAM_FIFO_DATA199 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA199_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA199 = crate::Reg<histogram_fifo_data199::HISTOGRAM_FIFO_DATA199_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data199;
#[doc = "HISTOGRAM_FIFO_DATA200 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA200_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA200 = crate::Reg<histogram_fifo_data200::HISTOGRAM_FIFO_DATA200_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data200;
#[doc = "HISTOGRAM_FIFO_DATA201 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA201_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA201 = crate::Reg<histogram_fifo_data201::HISTOGRAM_FIFO_DATA201_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data201;
#[doc = "HISTOGRAM_FIFO_DATA202 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA202_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA202 = crate::Reg<histogram_fifo_data202::HISTOGRAM_FIFO_DATA202_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data202;
#[doc = "HISTOGRAM_FIFO_DATA203 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA203_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA203 = crate::Reg<histogram_fifo_data203::HISTOGRAM_FIFO_DATA203_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data203;
#[doc = "HISTOGRAM_FIFO_DATA204 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA204_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA204 = crate::Reg<histogram_fifo_data204::HISTOGRAM_FIFO_DATA204_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data204;
#[doc = "HISTOGRAM_FIFO_DATA205 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA205_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA205 = crate::Reg<histogram_fifo_data205::HISTOGRAM_FIFO_DATA205_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data205;
#[doc = "HISTOGRAM_FIFO_DATA206 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA206_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA206 = crate::Reg<histogram_fifo_data206::HISTOGRAM_FIFO_DATA206_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data206;
#[doc = "HISTOGRAM_FIFO_DATA207 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA207_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA207 = crate::Reg<histogram_fifo_data207::HISTOGRAM_FIFO_DATA207_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data207;
#[doc = "HISTOGRAM_FIFO_DATA208 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA208_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA208 = crate::Reg<histogram_fifo_data208::HISTOGRAM_FIFO_DATA208_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data208;
#[doc = "HISTOGRAM_FIFO_DATA209 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA209_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA209 = crate::Reg<histogram_fifo_data209::HISTOGRAM_FIFO_DATA209_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data209;
#[doc = "HISTOGRAM_FIFO_DATA210 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA210_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA210 = crate::Reg<histogram_fifo_data210::HISTOGRAM_FIFO_DATA210_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data210;
#[doc = "HISTOGRAM_FIFO_DATA211 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA211_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA211 = crate::Reg<histogram_fifo_data211::HISTOGRAM_FIFO_DATA211_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data211;
#[doc = "HISTOGRAM_FIFO_DATA212 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA212_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA212 = crate::Reg<histogram_fifo_data212::HISTOGRAM_FIFO_DATA212_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data212;
#[doc = "HISTOGRAM_FIFO_DATA213 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA213_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA213 = crate::Reg<histogram_fifo_data213::HISTOGRAM_FIFO_DATA213_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data213;
#[doc = "HISTOGRAM_FIFO_DATA214 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA214_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA214 = crate::Reg<histogram_fifo_data214::HISTOGRAM_FIFO_DATA214_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data214;
#[doc = "HISTOGRAM_FIFO_DATA215 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA215_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA215 = crate::Reg<histogram_fifo_data215::HISTOGRAM_FIFO_DATA215_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data215;
#[doc = "HISTOGRAM_FIFO_DATA216 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA216_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA216 = crate::Reg<histogram_fifo_data216::HISTOGRAM_FIFO_DATA216_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data216;
#[doc = "HISTOGRAM_FIFO_DATA217 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA217_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA217 = crate::Reg<histogram_fifo_data217::HISTOGRAM_FIFO_DATA217_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data217;
#[doc = "HISTOGRAM_FIFO_DATA218 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA218_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA218 = crate::Reg<histogram_fifo_data218::HISTOGRAM_FIFO_DATA218_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data218;
#[doc = "HISTOGRAM_FIFO_DATA219 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA219_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA219 = crate::Reg<histogram_fifo_data219::HISTOGRAM_FIFO_DATA219_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data219;
#[doc = "HISTOGRAM_FIFO_DATA220 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA220_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA220 = crate::Reg<histogram_fifo_data220::HISTOGRAM_FIFO_DATA220_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data220;
#[doc = "HISTOGRAM_FIFO_DATA221 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA221_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA221 = crate::Reg<histogram_fifo_data221::HISTOGRAM_FIFO_DATA221_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data221;
#[doc = "HISTOGRAM_FIFO_DATA222 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA222_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA222 = crate::Reg<histogram_fifo_data222::HISTOGRAM_FIFO_DATA222_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data222;
#[doc = "HISTOGRAM_FIFO_DATA223 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA223_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA223 = crate::Reg<histogram_fifo_data223::HISTOGRAM_FIFO_DATA223_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data223;
#[doc = "HISTOGRAM_FIFO_DATA224 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA224_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA224 = crate::Reg<histogram_fifo_data224::HISTOGRAM_FIFO_DATA224_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data224;
#[doc = "HISTOGRAM_FIFO_DATA225 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA225_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA225 = crate::Reg<histogram_fifo_data225::HISTOGRAM_FIFO_DATA225_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data225;
#[doc = "HISTOGRAM_FIFO_DATA226 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA226_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA226 = crate::Reg<histogram_fifo_data226::HISTOGRAM_FIFO_DATA226_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data226;
#[doc = "HISTOGRAM_FIFO_DATA227 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA227_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA227 = crate::Reg<histogram_fifo_data227::HISTOGRAM_FIFO_DATA227_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data227;
#[doc = "HISTOGRAM_FIFO_DATA228 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA228_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA228 = crate::Reg<histogram_fifo_data228::HISTOGRAM_FIFO_DATA228_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data228;
#[doc = "HISTOGRAM_FIFO_DATA229 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA229_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA229 = crate::Reg<histogram_fifo_data229::HISTOGRAM_FIFO_DATA229_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data229;
#[doc = "HISTOGRAM_FIFO_DATA230 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA230_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA230 = crate::Reg<histogram_fifo_data230::HISTOGRAM_FIFO_DATA230_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data230;
#[doc = "HISTOGRAM_FIFO_DATA231 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA231_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA231 = crate::Reg<histogram_fifo_data231::HISTOGRAM_FIFO_DATA231_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data231;
#[doc = "HISTOGRAM_FIFO_DATA232 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA232_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA232 = crate::Reg<histogram_fifo_data232::HISTOGRAM_FIFO_DATA232_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data232;
#[doc = "HISTOGRAM_FIFO_DATA233 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA233_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA233 = crate::Reg<histogram_fifo_data233::HISTOGRAM_FIFO_DATA233_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data233;
#[doc = "HISTOGRAM_FIFO_DATA234 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA234_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA234 = crate::Reg<histogram_fifo_data234::HISTOGRAM_FIFO_DATA234_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data234;
#[doc = "HISTOGRAM_FIFO_DATA235 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA235_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA235 = crate::Reg<histogram_fifo_data235::HISTOGRAM_FIFO_DATA235_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data235;
#[doc = "HISTOGRAM_FIFO_DATA236 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA236_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA236 = crate::Reg<histogram_fifo_data236::HISTOGRAM_FIFO_DATA236_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data236;
#[doc = "HISTOGRAM_FIFO_DATA237 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA237_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA237 = crate::Reg<histogram_fifo_data237::HISTOGRAM_FIFO_DATA237_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data237;
#[doc = "HISTOGRAM_FIFO_DATA238 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA238_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA238 = crate::Reg<histogram_fifo_data238::HISTOGRAM_FIFO_DATA238_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data238;
#[doc = "HISTOGRAM_FIFO_DATA239 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA239_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA239 = crate::Reg<histogram_fifo_data239::HISTOGRAM_FIFO_DATA239_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data239;
#[doc = "HISTOGRAM_FIFO_DATA240 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA240_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA240 = crate::Reg<histogram_fifo_data240::HISTOGRAM_FIFO_DATA240_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data240;
#[doc = "HISTOGRAM_FIFO_DATA241 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA241_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA241 = crate::Reg<histogram_fifo_data241::HISTOGRAM_FIFO_DATA241_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data241;
#[doc = "HISTOGRAM_FIFO_DATA242 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA242_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA242 = crate::Reg<histogram_fifo_data242::HISTOGRAM_FIFO_DATA242_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data242;
#[doc = "HISTOGRAM_FIFO_DATA243 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA243_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA243 = crate::Reg<histogram_fifo_data243::HISTOGRAM_FIFO_DATA243_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data243;
#[doc = "HISTOGRAM_FIFO_DATA244 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA244_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA244 = crate::Reg<histogram_fifo_data244::HISTOGRAM_FIFO_DATA244_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data244;
#[doc = "HISTOGRAM_FIFO_DATA245 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA245_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA245 = crate::Reg<histogram_fifo_data245::HISTOGRAM_FIFO_DATA245_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data245;
#[doc = "HISTOGRAM_FIFO_DATA246 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA246_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA246 = crate::Reg<histogram_fifo_data246::HISTOGRAM_FIFO_DATA246_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data246;
#[doc = "HISTOGRAM_FIFO_DATA247 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA247_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA247 = crate::Reg<histogram_fifo_data247::HISTOGRAM_FIFO_DATA247_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data247;
#[doc = "HISTOGRAM_FIFO_DATA248 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA248_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA248 = crate::Reg<histogram_fifo_data248::HISTOGRAM_FIFO_DATA248_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data248;
#[doc = "HISTOGRAM_FIFO_DATA249 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA249_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA249 = crate::Reg<histogram_fifo_data249::HISTOGRAM_FIFO_DATA249_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data249;
#[doc = "HISTOGRAM_FIFO_DATA250 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA250_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA250 = crate::Reg<histogram_fifo_data250::HISTOGRAM_FIFO_DATA250_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data250;
#[doc = "HISTOGRAM_FIFO_DATA251 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA251_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA251 = crate::Reg<histogram_fifo_data251::HISTOGRAM_FIFO_DATA251_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data251;
#[doc = "HISTOGRAM_FIFO_DATA252 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA252_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA252 = crate::Reg<histogram_fifo_data252::HISTOGRAM_FIFO_DATA252_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data252;
#[doc = "HISTOGRAM_FIFO_DATA253 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA253_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA253 = crate::Reg<histogram_fifo_data253::HISTOGRAM_FIFO_DATA253_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data253;
#[doc = "HISTOGRAM_FIFO_DATA254 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA254_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA254 = crate::Reg<histogram_fifo_data254::HISTOGRAM_FIFO_DATA254_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data254;
#[doc = "HISTOGRAM_FIFO_DATA255 (r) register accessor: an alias for `Reg<HISTOGRAM_FIFO_DATA255_SPEC>`"]
pub type HISTOGRAM_FIFO_DATA255 = crate::Reg<histogram_fifo_data255::HISTOGRAM_FIFO_DATA255_SPEC>;
#[doc = "Histogram Registers"]
pub mod histogram_fifo_data255;
