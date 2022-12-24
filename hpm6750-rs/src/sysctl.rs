#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Resource control register for cpu0"]
    pub resource_cpu0_core: RESOURCE_CPU0_CORE,
    #[doc = "0x04 - Resource control register for cpx0"]
    pub resource_cpu0_subsys: RESOURCE_CPU0_SUBSYS,
    _reserved2: [u8; 0x18],
    #[doc = "0x20 - Resource control register for cpu1"]
    pub resource_cpu1_core: RESOURCE_CPU1_CORE,
    #[doc = "0x24 - Resource control register for cpx1"]
    pub resource_cpx1_subsys: RESOURCE_CPX1_SUBSYS,
    _reserved4: [u8; 0x2c],
    #[doc = "0x54 - Resource control register for pow_con"]
    pub resource_pow_con: RESOURCE_POW_CON,
    #[doc = "0x58 - Resource control register for pow_vis"]
    pub resource_pow_vis: RESOURCE_POW_VIS,
    #[doc = "0x5c - Resource control register for pow_cpu0"]
    pub resource_pow_cpu0: RESOURCE_POW_CPU0,
    #[doc = "0x60 - Resource control register for pow_cpu1"]
    pub resource_pow_cpu1: RESOURCE_POW_CPU1,
    #[doc = "0x64 - Resource control register for rst_soc"]
    pub resource_rst_soc: RESOURCE_RST_SOC,
    #[doc = "0x68 - Resource control register for rst_con"]
    pub resource_rst_con: RESOURCE_RST_CON,
    #[doc = "0x6c - Resource control register for rst_vis"]
    pub resource_rst_vis: RESOURCE_RST_VIS,
    #[doc = "0x70 - Resource control register for rst_cpu0"]
    pub resource_rst_cpu0: RESOURCE_RST_CPU0,
    #[doc = "0x74 - Resource control register for rst_cpu1"]
    pub resource_rst_cpu1: RESOURCE_RST_CPU1,
    _reserved13: [u8; 0x08],
    #[doc = "0x80 - Resource control register for xtal"]
    pub resource_clk_src_xtal: RESOURCE_CLK_SRC_XTAL,
    #[doc = "0x84 - Resource control register for pll0"]
    pub resource_clk_src_pll0: RESOURCE_CLK_SRC_PLL0,
    #[doc = "0x88 - Resource control register for clk0_pll0"]
    pub resource_clk_src_pll0clk0: RESOURCE_CLK_SRC_PLL0CLK0,
    #[doc = "0x8c - Resource control register for pll1"]
    pub resource_clk_src_pll1: RESOURCE_CLK_SRC_PLL1,
    #[doc = "0x90 - Resource control register for clk0_pll1"]
    pub resource_clk_src_pll1clk0: RESOURCE_CLK_SRC_PLL1CLK0,
    #[doc = "0x94 - Resource control register for clk1_pll1"]
    pub resource_clk_src_pll1clk1: RESOURCE_CLK_SRC_PLL1CLK1,
    #[doc = "0x98 - Resource control register for pll2"]
    pub resource_clk_src_pll2: RESOURCE_CLK_SRC_PLL2,
    #[doc = "0x9c - Resource control register for clk0_pll2"]
    pub resource_clk_src_pll2clk0: RESOURCE_CLK_SRC_PLL2CLK0,
    #[doc = "0xa0 - Resource control register for clk1_pll2"]
    pub resource_clk_src_pll2clk1: RESOURCE_CLK_SRC_PLL2CLK1,
    #[doc = "0xa4 - Resource control register for pll3"]
    pub resource_clk_src_pll3: RESOURCE_CLK_SRC_PLL3,
    #[doc = "0xa8 - Resource control register for clk0_pll3"]
    pub resource_clk_src_pll3clk0: RESOURCE_CLK_SRC_PLL3CLK0,
    #[doc = "0xac - Resource control register for pll4"]
    pub resource_clk_src_pll4: RESOURCE_CLK_SRC_PLL4,
    #[doc = "0xb0 - Resource control register for clk0_pll4"]
    pub resource_clk_src_pll4clk0: RESOURCE_CLK_SRC_PLL4CLK0,
    _reserved26: [u8; 0x4c],
    #[doc = "0x100 - Resource control register for clk_top_cpu0"]
    pub resource_clk_top_cpu0: RESOURCE_CLK_TOP_CPU0,
    #[doc = "0x104 - Resource control register for clk_top_mct0"]
    pub resource_clk_top_mchtmr0: RESOURCE_CLK_TOP_MCHTMR0,
    #[doc = "0x108 - Resource control register for clk_top_cpu1"]
    pub resource_clk_top_cpu1: RESOURCE_CLK_TOP_CPU1,
    #[doc = "0x10c - Resource control register for clk_top_mct1"]
    pub resource_clk_top_mchtmr1: RESOURCE_CLK_TOP_MCHTMR1,
    #[doc = "0x110 - Resource control register for clk_top_axi0"]
    pub resource_clk_top_axi: RESOURCE_CLK_TOP_AXI,
    #[doc = "0x114 - Resource control register for clk_top_axi1"]
    pub resource_clk_top_conn: RESOURCE_CLK_TOP_CONN,
    #[doc = "0x118 - Resource control register for clk_top_axi2"]
    pub resource_clk_top_vis: RESOURCE_CLK_TOP_VIS,
    #[doc = "0x11c - Resource control register for clk_top_ahb0"]
    pub resource_clk_top_ahb: RESOURCE_CLK_TOP_AHB,
    #[doc = "0x120 - Resource control register for clk_top_dram"]
    pub resource_clk_top_dram: RESOURCE_CLK_TOP_DRAM,
    #[doc = "0x124 - Resource control register for clk_top_xpi0"]
    pub resource_clk_top_xpi0: RESOURCE_CLK_TOP_XPI0,
    #[doc = "0x128 - Resource control register for clk_top_xpi1"]
    pub resource_clk_top_xpi1: RESOURCE_CLK_TOP_XPI1,
    #[doc = "0x12c - Resource control register for clk_top_tmr0"]
    pub resource_clk_top_gptmr0: RESOURCE_CLK_TOP_GPTMR0,
    #[doc = "0x130 - Resource control register for clk_top_tmr1"]
    pub resource_clk_top_gptmr1: RESOURCE_CLK_TOP_GPTMR1,
    #[doc = "0x134 - Resource control register for clk_top_tmr2"]
    pub resource_clk_top_gptmr2: RESOURCE_CLK_TOP_GPTMR2,
    #[doc = "0x138 - Resource control register for clk_top_tmr3"]
    pub resource_clk_top_gptmr3: RESOURCE_CLK_TOP_GPTMR3,
    #[doc = "0x13c - Resource control register for clk_top_tmr4"]
    pub resource_clk_top_gptmr4: RESOURCE_CLK_TOP_GPTMR4,
    #[doc = "0x140 - Resource control register for clk_top_tmr5"]
    pub resource_clk_top_gptmr5: RESOURCE_CLK_TOP_GPTMR5,
    #[doc = "0x144 - Resource control register for clk_top_tmr6"]
    pub resource_clk_top_gptmr6: RESOURCE_CLK_TOP_GPTMR6,
    #[doc = "0x148 - Resource control register for clk_top_tmr7"]
    pub resource_clk_top_gptmr7: RESOURCE_CLK_TOP_GPTMR7,
    #[doc = "0x14c - Resource control register for clk_top_urt0"]
    pub resource_clk_top_uart0: RESOURCE_CLK_TOP_UART0,
    #[doc = "0x150 - Resource control register for clk_top_urt1"]
    pub resource_clk_top_uart1: RESOURCE_CLK_TOP_UART1,
    #[doc = "0x154 - Resource control register for clk_top_urt2"]
    pub resource_clk_top_uart2: RESOURCE_CLK_TOP_UART2,
    #[doc = "0x158 - Resource control register for clk_top_urt3"]
    pub resource_clk_top_uart3: RESOURCE_CLK_TOP_UART3,
    #[doc = "0x15c - Resource control register for clk_top_urt4"]
    pub resource_clk_top_uart4: RESOURCE_CLK_TOP_UART4,
    #[doc = "0x160 - Resource control register for clk_top_urt5"]
    pub resource_clk_top_uart5: RESOURCE_CLK_TOP_UART5,
    #[doc = "0x164 - Resource control register for clk_top_urt6"]
    pub resource_clk_top_uart6: RESOURCE_CLK_TOP_UART6,
    #[doc = "0x168 - Resource control register for clk_top_urt7"]
    pub resource_clk_top_uart7: RESOURCE_CLK_TOP_UART7,
    #[doc = "0x16c - Resource control register for clk_top_urt8"]
    pub resource_clk_top_uart8: RESOURCE_CLK_TOP_UART8,
    #[doc = "0x170 - Resource control register for clk_top_urt9"]
    pub resource_clk_top_uart9: RESOURCE_CLK_TOP_UART9,
    #[doc = "0x174 - Resource control register for clk_top_urta"]
    pub resource_clk_top_uart10: RESOURCE_CLK_TOP_UART10,
    #[doc = "0x178 - Resource control register for clk_top_urtb"]
    pub resource_clk_top_uart11: RESOURCE_CLK_TOP_UART11,
    #[doc = "0x17c - Resource control register for clk_top_urtc"]
    pub resource_clk_top_uart12: RESOURCE_CLK_TOP_UART12,
    #[doc = "0x180 - Resource control register for clk_top_urtd"]
    pub resource_clk_top_uart13: RESOURCE_CLK_TOP_UART13,
    #[doc = "0x184 - Resource control register for clk_top_urte"]
    pub resource_clk_top_uart14: RESOURCE_CLK_TOP_UART14,
    #[doc = "0x188 - Resource control register for clk_top_urtf"]
    pub resource_clk_top_uart15: RESOURCE_CLK_TOP_UART15,
    #[doc = "0x18c - Resource control register for clk_top_i2c0"]
    pub resource_clk_top_i2c0: RESOURCE_CLK_TOP_I2C0,
    #[doc = "0x190 - Resource control register for clk_top_i2c1"]
    pub resource_clk_top_i2c1: RESOURCE_CLK_TOP_I2C1,
    #[doc = "0x194 - Resource control register for clk_top_i2c2"]
    pub resource_clk_top_i2c2: RESOURCE_CLK_TOP_I2C2,
    #[doc = "0x198 - Resource control register for clk_top_i2c3"]
    pub resource_clk_top_i2c3: RESOURCE_CLK_TOP_I2C3,
    #[doc = "0x19c - Resource control register for clk_top_spi0"]
    pub resource_clk_top_spi0: RESOURCE_CLK_TOP_SPI0,
    #[doc = "0x1a0 - Resource control register for clk_top_spi1"]
    pub resource_clk_top_spi1: RESOURCE_CLK_TOP_SPI1,
    #[doc = "0x1a4 - Resource control register for clk_top_spi2"]
    pub resource_clk_top_spi2: RESOURCE_CLK_TOP_SPI2,
    #[doc = "0x1a8 - Resource control register for clk_top_spi3"]
    pub resource_clk_top_spi3: RESOURCE_CLK_TOP_SPI3,
    #[doc = "0x1ac - Resource control register for clk_top_can0"]
    pub resource_clk_top_can0: RESOURCE_CLK_TOP_CAN0,
    #[doc = "0x1b0 - Resource control register for clk_top_can1"]
    pub resource_clk_top_can1: RESOURCE_CLK_TOP_CAN1,
    #[doc = "0x1b4 - Resource control register for clk_top_can2"]
    pub resource_clk_top_can2: RESOURCE_CLK_TOP_CAN2,
    #[doc = "0x1b8 - Resource control register for clk_top_can3"]
    pub resource_clk_top_can3: RESOURCE_CLK_TOP_CAN3,
    #[doc = "0x1bc - Resource control register for clk_top_ptpc"]
    pub resource_clk_top_ptpc: RESOURCE_CLK_TOP_PTPC,
    #[doc = "0x1c0 - Resource control register for clk_top_ana0"]
    pub resource_clk_top_ana0: RESOURCE_CLK_TOP_ANA0,
    #[doc = "0x1c4 - Resource control register for clk_top_ana1"]
    pub resource_clk_top_ana1: RESOURCE_CLK_TOP_ANA1,
    #[doc = "0x1c8 - Resource control register for clk_top_ana2"]
    pub resource_clk_top_ana2: RESOURCE_CLK_TOP_ANA2,
    #[doc = "0x1cc - Resource control register for clk_top_aud0"]
    pub resource_clk_top_aud0: RESOURCE_CLK_TOP_AUD0,
    #[doc = "0x1d0 - Resource control register for clk_top_aud1"]
    pub resource_clk_top_aud1: RESOURCE_CLK_TOP_AUD1,
    #[doc = "0x1d4 - Resource control register for clk_top_aud2"]
    pub resource_clk_top_aud2: RESOURCE_CLK_TOP_AUD2,
    #[doc = "0x1d8 - Resource control register for clk_top_dis0"]
    pub resource_clk_top_lcdc: RESOURCE_CLK_TOP_LCDC,
    #[doc = "0x1dc - Resource control register for clk_top_cam0"]
    pub resource_clk_top_cam0: RESOURCE_CLK_TOP_CAM0,
    #[doc = "0x1e0 - Resource control register for clk_top_cam1"]
    pub resource_clk_top_cam1: RESOURCE_CLK_TOP_CAM1,
    #[doc = "0x1e4 - Resource control register for clk_top_eth0"]
    pub resource_clk_top_enet0: RESOURCE_CLK_TOP_ENET0,
    #[doc = "0x1e8 - Resource control register for clk_top_eth1"]
    pub resource_clk_top_enet1: RESOURCE_CLK_TOP_ENET1,
    #[doc = "0x1ec - Resource control register for clk_top_ptp0"]
    pub resource_clk_top_ptp0: RESOURCE_CLK_TOP_PTP0,
    #[doc = "0x1f0 - Resource control register for clk_top_ptp1"]
    pub resource_clk_top_ptp1: RESOURCE_CLK_TOP_PTP1,
    #[doc = "0x1f4 - Resource control register for clk_top_ref0"]
    pub resource_clk_top_ref0: RESOURCE_CLK_TOP_REF0,
    #[doc = "0x1f8 - Resource control register for clk_top_ref1"]
    pub resource_clk_top_ref1: RESOURCE_CLK_TOP_REF1,
    #[doc = "0x1fc - Resource control register for clk_top_ntm0"]
    pub resource_clk_top_ntmr0: RESOURCE_CLK_TOP_NTMR0,
    #[doc = "0x200 - Resource control register for clk_top_ntm1"]
    pub resource_clk_top_ntmr1: RESOURCE_CLK_TOP_NTMR1,
    #[doc = "0x204 - Resource control register for clk_top_sdc0"]
    pub resource_clk_top_sdxc0: RESOURCE_CLK_TOP_SDXC0,
    #[doc = "0x208 - Resource control register for clk_top_sdc1"]
    pub resource_clk_top_sdxc1: RESOURCE_CLK_TOP_SDXC1,
    _reserved93: [u8; 0xf4],
    #[doc = "0x300 - Resource control register for clk_top_adc0"]
    pub resource_clk_top_adc0: RESOURCE_CLK_TOP_ADC0,
    #[doc = "0x304 - Resource control register for clk_top_adc1"]
    pub resource_clk_top_adc1: RESOURCE_CLK_TOP_ADC1,
    #[doc = "0x308 - Resource control register for clk_top_adc2"]
    pub resource_clk_top_adc2: RESOURCE_CLK_TOP_ADC2,
    #[doc = "0x30c - Resource control register for clk_top_adc3"]
    pub resource_clk_top_adc3: RESOURCE_CLK_TOP_ADC3,
    #[doc = "0x310 - Resource control register for clk_top_i2s0"]
    pub resource_clk_top_i2s0: RESOURCE_CLK_TOP_I2S0,
    #[doc = "0x314 - Resource control register for clk_top_i2s1"]
    pub resource_clk_top_i2s1: RESOURCE_CLK_TOP_I2S1,
    #[doc = "0x318 - Resource control register for clk_top_i2s2"]
    pub resource_clk_top_i2s2: RESOURCE_CLK_TOP_I2S2,
    #[doc = "0x31c - Resource control register for clk_top_i2s3"]
    pub resource_clk_top_i2s3: RESOURCE_CLK_TOP_I2S3,
    _reserved101: [u8; 0xe0],
    #[doc = "0x400 - Resource control register for ahbp"]
    pub resource_ahbapb_bus: RESOURCE_AHBAPB_BUS,
    #[doc = "0x404 - Resource control register for axis"]
    pub resource_axi_bus: RESOURCE_AXI_BUS,
    #[doc = "0x408 - Resource control register for axic"]
    pub resource_conn_bus: RESOURCE_CONN_BUS,
    #[doc = "0x40c - Resource control register for axiv"]
    pub resource_vis_bus: RESOURCE_VIS_BUS,
    #[doc = "0x410 - Resource control register for dram"]
    pub resource_dram: RESOURCE_DRAM,
    #[doc = "0x414 - Resource control register for rom0"]
    pub resource_rom: RESOURCE_ROM,
    #[doc = "0x418 - Resource control register for lmm0"]
    pub resource_lmm0: RESOURCE_LMM0,
    #[doc = "0x41c - Resource control register for lmm1"]
    pub resource_lmm1: RESOURCE_LMM1,
    #[doc = "0x420 - Resource control register for mct0"]
    pub resource_mchtmr0: RESOURCE_MCHTMR0,
    #[doc = "0x424 - Resource control register for mct1"]
    pub resource_mchtmr1: RESOURCE_MCHTMR1,
    #[doc = "0x428 - Resource control register for ram0"]
    pub resource_axi_sram0: RESOURCE_AXI_SRAM0,
    #[doc = "0x42c - Resource control register for ram1"]
    pub resource_axi_sram1: RESOURCE_AXI_SRAM1,
    #[doc = "0x430 - Resource control register for xpi0"]
    pub resource_xpi0: RESOURCE_XPI0,
    #[doc = "0x434 - Resource control register for xpi1"]
    pub resource_xpi1: RESOURCE_XPI1,
    #[doc = "0x438 - Resource control register for sdp0"]
    pub resource_sdp: RESOURCE_SDP,
    #[doc = "0x43c - Resource control register for rng0"]
    pub resource_rng: RESOURCE_RNG,
    #[doc = "0x440 - Resource control register for kman"]
    pub resource_keym: RESOURCE_KEYM,
    #[doc = "0x444 - Resource control register for dma0"]
    pub resource_hdma: RESOURCE_HDMA,
    #[doc = "0x448 - Resource control register for dma1"]
    pub resource_xdma: RESOURCE_XDMA,
    #[doc = "0x44c - Resource control register for gpio"]
    pub resource_gpio: RESOURCE_GPIO,
    #[doc = "0x450 - Resource control register for mbx0"]
    pub resource_mbx0: RESOURCE_MBX0,
    #[doc = "0x454 - Resource control register for mbx1"]
    pub resource_mbx1: RESOURCE_MBX1,
    #[doc = "0x458 - Resource control register for wdg0"]
    pub resource_wdg0: RESOURCE_WDG0,
    #[doc = "0x45c - Resource control register for wdg1"]
    pub resource_wdg1: RESOURCE_WDG1,
    #[doc = "0x460 - Resource control register for wdg2"]
    pub resource_wdg2: RESOURCE_WDG2,
    #[doc = "0x464 - Resource control register for wdg3"]
    pub resource_wdg3: RESOURCE_WDG3,
    #[doc = "0x468 - Resource control register for tmr0"]
    pub resource_gptmr0: RESOURCE_GPTMR0,
    #[doc = "0x46c - Resource control register for tmr1"]
    pub resource_gptmr1: RESOURCE_GPTMR1,
    #[doc = "0x470 - Resource control register for tmr2"]
    pub resource_gptmr2: RESOURCE_GPTMR2,
    #[doc = "0x474 - Resource control register for tmr3"]
    pub resource_gptmr3: RESOURCE_GPTMR3,
    #[doc = "0x478 - Resource control register for tmr4"]
    pub resource_gptmr4: RESOURCE_GPTMR4,
    #[doc = "0x47c - Resource control register for tmr5"]
    pub resource_gptmr5: RESOURCE_GPTMR5,
    #[doc = "0x480 - Resource control register for tmr6"]
    pub resource_gptmr6: RESOURCE_GPTMR6,
    #[doc = "0x484 - Resource control register for tmr7"]
    pub resource_gptmr7: RESOURCE_GPTMR7,
    #[doc = "0x488 - Resource control register for urt0"]
    pub resource_uart0: RESOURCE_UART0,
    #[doc = "0x48c - Resource control register for urt1"]
    pub resource_uart1: RESOURCE_UART1,
    #[doc = "0x490 - Resource control register for urt2"]
    pub resource_uart2: RESOURCE_UART2,
    #[doc = "0x494 - Resource control register for urt3"]
    pub resource_uart3: RESOURCE_UART3,
    #[doc = "0x498 - Resource control register for urt4"]
    pub resource_uart4: RESOURCE_UART4,
    #[doc = "0x49c - Resource control register for urt5"]
    pub resource_uart5: RESOURCE_UART5,
    #[doc = "0x4a0 - Resource control register for urt6"]
    pub resource_uart6: RESOURCE_UART6,
    #[doc = "0x4a4 - Resource control register for urt7"]
    pub resource_uart7: RESOURCE_UART7,
    #[doc = "0x4a8 - Resource control register for urt8"]
    pub resource_uart8: RESOURCE_UART8,
    #[doc = "0x4ac - Resource control register for urt9"]
    pub resource_uart9: RESOURCE_UART9,
    #[doc = "0x4b0 - Resource control register for urta"]
    pub resource_uart10: RESOURCE_UART10,
    #[doc = "0x4b4 - Resource control register for urtb"]
    pub resource_uart11: RESOURCE_UART11,
    #[doc = "0x4b8 - Resource control register for urtc"]
    pub resource_uart12: RESOURCE_UART12,
    #[doc = "0x4bc - Resource control register for urtd"]
    pub resource_uart13: RESOURCE_UART13,
    #[doc = "0x4c0 - Resource control register for urte"]
    pub resource_uart14: RESOURCE_UART14,
    #[doc = "0x4c4 - Resource control register for urtf"]
    pub resource_uart15: RESOURCE_UART15,
    #[doc = "0x4c8 - Resource control register for i2c0"]
    pub resource_i2c0: RESOURCE_I2C0,
    #[doc = "0x4cc - Resource control register for i2c1"]
    pub resource_i2c1: RESOURCE_I2C1,
    #[doc = "0x4d0 - Resource control register for i2c2"]
    pub resource_i2c2: RESOURCE_I2C2,
    #[doc = "0x4d4 - Resource control register for i2c3"]
    pub resource_i2c3: RESOURCE_I2C3,
    #[doc = "0x4d8 - Resource control register for spi0"]
    pub resource_spi0: RESOURCE_SPI0,
    #[doc = "0x4dc - Resource control register for spi1"]
    pub resource_spi1: RESOURCE_SPI1,
    #[doc = "0x4e0 - Resource control register for spi2"]
    pub resource_spi2: RESOURCE_SPI2,
    #[doc = "0x4e4 - Resource control register for spi3"]
    pub resource_spi3: RESOURCE_SPI3,
    #[doc = "0x4e8 - Resource control register for can0"]
    pub resource_can0: RESOURCE_CAN0,
    #[doc = "0x4ec - Resource control register for can1"]
    pub resource_can1: RESOURCE_CAN1,
    #[doc = "0x4f0 - Resource control register for can2"]
    pub resource_can2: RESOURCE_CAN2,
    #[doc = "0x4f4 - Resource control register for can3"]
    pub resource_can3: RESOURCE_CAN3,
    #[doc = "0x4f8 - Resource control register for ptpc"]
    pub resource_ptpc: RESOURCE_PTPC,
    #[doc = "0x4fc - Resource control register for adc0"]
    pub resource_adc0: RESOURCE_ADC0,
    #[doc = "0x500 - Resource control register for adc1"]
    pub resource_adc1: RESOURCE_ADC1,
    #[doc = "0x504 - Resource control register for adc2"]
    pub resource_adc2: RESOURCE_ADC2,
    #[doc = "0x508 - Resource control register for adc3"]
    pub resource_adc3: RESOURCE_ADC3,
    #[doc = "0x50c - Resource control register for acmp"]
    pub resource_acmp: RESOURCE_ACMP,
    #[doc = "0x510 - Resource control register for i2s0"]
    pub resource_i2s0: RESOURCE_I2S0,
    #[doc = "0x514 - Resource control register for i2s1"]
    pub resource_i2s1: RESOURCE_I2S1,
    #[doc = "0x518 - Resource control register for i2s2"]
    pub resource_i2s2: RESOURCE_I2S2,
    #[doc = "0x51c - Resource control register for i2s3"]
    pub resource_i2s3: RESOURCE_I2S3,
    #[doc = "0x520 - Resource control register for pdm0"]
    pub resource_pdm: RESOURCE_PDM,
    #[doc = "0x524 - Resource control register for clsd"]
    pub resource_dao: RESOURCE_DAO,
    #[doc = "0x528 - Resource control register for msyn"]
    pub resource_synt: RESOURCE_SYNT,
    #[doc = "0x52c - Resource control register for mot0"]
    pub resource_mot0: RESOURCE_MOT0,
    #[doc = "0x530 - Resource control register for mot1"]
    pub resource_mot1: RESOURCE_MOT1,
    #[doc = "0x534 - Resource control register for mot2"]
    pub resource_mot2: RESOURCE_MOT2,
    #[doc = "0x538 - Resource control register for mot3"]
    pub resource_mot3: RESOURCE_MOT3,
    #[doc = "0x53c - Resource control register for dis0"]
    pub resource_lcdc: RESOURCE_LCDC,
    #[doc = "0x540 - Resource control register for cam0"]
    pub resource_cam0: RESOURCE_CAM0,
    #[doc = "0x544 - Resource control register for cam1"]
    pub resource_cam1: RESOURCE_CAM1,
    #[doc = "0x548 - Resource control register for jpeg"]
    pub resource_jpeg: RESOURCE_JPEG,
    #[doc = "0x54c - Resource control register for pdma"]
    pub resource_pdma: RESOURCE_PDMA,
    #[doc = "0x550 - Resource control register for eth0"]
    pub resource_enet0: RESOURCE_ENET0,
    #[doc = "0x554 - Resource control register for eth1"]
    pub resource_enet1: RESOURCE_ENET1,
    #[doc = "0x558 - Resource control register for ntm0"]
    pub resource_ntmr0: RESOURCE_NTMR0,
    #[doc = "0x55c - Resource control register for ntm1"]
    pub resource_ntmr1: RESOURCE_NTMR1,
    #[doc = "0x560 - Resource control register for sdc0"]
    pub resource_sdxc0: RESOURCE_SDXC0,
    #[doc = "0x564 - Resource control register for sdc1"]
    pub resource_sdxc1: RESOURCE_SDXC1,
    #[doc = "0x568 - Resource control register for usb0"]
    pub resource_usb0: RESOURCE_USB0,
    #[doc = "0x56c - Resource control register for usb1"]
    pub resource_usb1: RESOURCE_USB1,
    #[doc = "0x570 - Resource control register for ref0"]
    pub resource_ref0: RESOURCE_REF0,
    #[doc = "0x574 - Resource control register for ref1"]
    pub resource_ref1: RESOURCE_REF1,
    _reserved195: [u8; 0x0288],
    #[doc = "0x800 - Goup setting"]
    pub group0_0_value: GROUP0_0_VALUE,
    #[doc = "0x804 - Group set register"]
    pub group0_0_set: GROUP0_0_SET,
    #[doc = "0x808 - Goup setting"]
    pub group0_0_clear: GROUP0_0_CLEAR,
    #[doc = "0x80c - Goup setting"]
    pub group0_0_toggle: GROUP0_0_TOGGLE,
    #[doc = "0x810 - Goup setting"]
    pub group0_1_value: GROUP0_1_VALUE,
    #[doc = "0x814 - Goup setting"]
    pub group0_1_set: GROUP0_1_SET,
    #[doc = "0x818 - Goup setting"]
    pub group0_1_clear: GROUP0_1_CLEAR,
    #[doc = "0x81c - Goup setting"]
    pub group0_1_toggle: GROUP0_1_TOGGLE,
    #[doc = "0x820 - Goup setting"]
    pub group0_2_value: GROUP0_2_VALUE,
    #[doc = "0x824 - Goup setting"]
    pub group0_2_set: GROUP0_2_SET,
    #[doc = "0x828 - Goup setting"]
    pub group0_2_clear: GROUP0_2_CLEAR,
    #[doc = "0x82c - Goup setting"]
    pub group0_2_toggle: GROUP0_2_TOGGLE,
    _reserved207: [u8; 0x10],
    #[doc = "0x840 - Goup setting"]
    pub group1_0_value: GROUP1_0_VALUE,
    #[doc = "0x844 - Goup setting"]
    pub group1_0_set: GROUP1_0_SET,
    #[doc = "0x848 - Goup setting"]
    pub group1_0_clear: GROUP1_0_CLEAR,
    #[doc = "0x84c - Goup setting"]
    pub group1_0_toggle: GROUP1_0_TOGGLE,
    #[doc = "0x850 - Goup setting"]
    pub group1_1_value: GROUP1_1_VALUE,
    #[doc = "0x854 - Goup setting"]
    pub group1_1_set: GROUP1_1_SET,
    #[doc = "0x858 - Goup setting"]
    pub group1_1_clear: GROUP1_1_CLEAR,
    #[doc = "0x85c - Goup setting"]
    pub group1_1_toggle: GROUP1_1_TOGGLE,
    #[doc = "0x860 - Goup setting"]
    pub group1_2_value: GROUP1_2_VALUE,
    #[doc = "0x864 - Goup setting"]
    pub group1_2_set: GROUP1_2_SET,
    #[doc = "0x868 - Goup setting"]
    pub group1_2_clear: GROUP1_2_CLEAR,
    #[doc = "0x86c - Goup setting"]
    pub group1_2_toggle: GROUP1_2_TOGGLE,
    _reserved219: [u8; 0x90],
    #[doc = "0x900 - Affiliate of Group"]
    pub affiliate_cpu0_value: AFFILIATE_CPU0_VALUE,
    #[doc = "0x904 - Affiliate of Group"]
    pub affiliate_cpu0_set: AFFILIATE_CPU0_SET,
    #[doc = "0x908 - Affiliate of Group"]
    pub affiliate_cpu0_clear: AFFILIATE_CPU0_CLEAR,
    #[doc = "0x90c - Affiliate of Group"]
    pub affiliate_cpu0_toggle: AFFILIATE_CPU0_TOGGLE,
    #[doc = "0x910 - Affiliate of Group"]
    pub affiliate_cpu1_value: AFFILIATE_CPU1_VALUE,
    #[doc = "0x914 - Affiliate of Group"]
    pub affiliate_cpu1_set: AFFILIATE_CPU1_SET,
    #[doc = "0x918 - Affiliate of Group"]
    pub affiliate_cpu1_clear: AFFILIATE_CPU1_CLEAR,
    #[doc = "0x91c - Affiliate of Group"]
    pub affiliate_cpu1_toggle: AFFILIATE_CPU1_TOGGLE,
    #[doc = "0x920 - Retention Contol"]
    pub retention_cpu0_value: RETENTION_CPU0_VALUE,
    #[doc = "0x924 - Retention Contol"]
    pub retention_cpu0_set: RETENTION_CPU0_SET,
    #[doc = "0x928 - Retention Contol"]
    pub retention_cpu0_clear: RETENTION_CPU0_CLEAR,
    #[doc = "0x92c - Retention Contol"]
    pub retention_cpu0_toggle: RETENTION_CPU0_TOGGLE,
    #[doc = "0x930 - Retention Contol"]
    pub retention_cpu1_value: RETENTION_CPU1_VALUE,
    #[doc = "0x934 - Retention Contol"]
    pub retention_cpu1_set: RETENTION_CPU1_SET,
    #[doc = "0x938 - Retention Contol"]
    pub retention_cpu1_clear: RETENTION_CPU1_CLEAR,
    #[doc = "0x93c - Retention Contol"]
    pub retention_cpu1_toggle: RETENTION_CPU1_TOGGLE,
    _reserved235: [u8; 0x06c0],
    #[doc = "0x1000 - Power Setting"]
    pub power_cpu0_status: POWER_CPU0_STATUS,
    #[doc = "0x1004 - Power Setting"]
    pub power_cpu0_lf_wait: POWER_CPU0_LF_WAIT,
    _reserved237: [u8; 0x04],
    #[doc = "0x100c - Power Setting"]
    pub power_cpu0_off_wait: POWER_CPU0_OFF_WAIT,
    #[doc = "0x1010 - Power Setting"]
    pub power_cpu1_status: POWER_CPU1_STATUS,
    #[doc = "0x1014 - Power Setting"]
    pub power_cpu1_lf_wait: POWER_CPU1_LF_WAIT,
    _reserved240: [u8; 0x04],
    #[doc = "0x101c - Power Setting"]
    pub power_cpu1_off_wait: POWER_CPU1_OFF_WAIT,
    #[doc = "0x1020 - Power Setting"]
    pub power_con_status: POWER_CON_STATUS,
    #[doc = "0x1024 - Power Setting"]
    pub power_con_lf_wait: POWER_CON_LF_WAIT,
    _reserved243: [u8; 0x04],
    #[doc = "0x102c - Power Setting"]
    pub power_con_off_wait: POWER_CON_OFF_WAIT,
    #[doc = "0x1030 - Power Setting"]
    pub power_vis_status: POWER_VIS_STATUS,
    #[doc = "0x1034 - Power Setting"]
    pub power_vis_lf_wait: POWER_VIS_LF_WAIT,
    _reserved246: [u8; 0x04],
    #[doc = "0x103c - Power Setting"]
    pub power_vis_off_wait: POWER_VIS_OFF_WAIT,
    _reserved247: [u8; 0x03c0],
    #[doc = "0x1400 - Reset Setting"]
    pub reset_soc_control: RESET_SOC_CONTROL,
    #[doc = "0x1404 - Reset Setting"]
    pub reset_soc_config: RESET_SOC_CONFIG,
    _reserved249: [u8; 0x04],
    #[doc = "0x140c - Reset Setting"]
    pub reset_soc_counter: RESET_SOC_COUNTER,
    #[doc = "0x1410 - Reset Setting"]
    pub reset_con_control: RESET_CON_CONTROL,
    #[doc = "0x1414 - Reset Setting"]
    pub reset_con_config: RESET_CON_CONFIG,
    _reserved252: [u8; 0x04],
    #[doc = "0x141c - Reset Setting"]
    pub reset_con_counter: RESET_CON_COUNTER,
    #[doc = "0x1420 - Reset Setting"]
    pub reset_vis_control: RESET_VIS_CONTROL,
    #[doc = "0x1424 - Reset Setting"]
    pub reset_vis_config: RESET_VIS_CONFIG,
    _reserved255: [u8; 0x04],
    #[doc = "0x142c - Reset Setting"]
    pub reset_vis_counter: RESET_VIS_COUNTER,
    #[doc = "0x1430 - Reset Setting"]
    pub reset_cpu0_control: RESET_CPU0_CONTROL,
    #[doc = "0x1434 - Reset Setting"]
    pub reset_cpu0_config: RESET_CPU0_CONFIG,
    _reserved258: [u8; 0x04],
    #[doc = "0x143c - Reset Setting"]
    pub reset_cpu0_counter: RESET_CPU0_COUNTER,
    #[doc = "0x1440 - Reset Setting"]
    pub reset_cpu1_control: RESET_CPU1_CONTROL,
    #[doc = "0x1444 - Reset Setting"]
    pub reset_cpu1_config: RESET_CPU1_CONFIG,
    _reserved261: [u8; 0x04],
    #[doc = "0x144c - Reset Setting"]
    pub reset_cpu1_counter: RESET_CPU1_COUNTER,
    _reserved262: [u8; 0x03b0],
    #[doc = "0x1800 - Clock setting"]
    pub clock_clk_top_cpu0: CLOCK_CLK_TOP_CPU0,
    #[doc = "0x1804 - Clock setting"]
    pub clock_clk_top_mchtmr0: CLOCK_CLK_TOP_MCHTMR0,
    #[doc = "0x1808 - Clock setting"]
    pub clock_clk_top_cpu1: CLOCK_CLK_TOP_CPU1,
    #[doc = "0x180c - Clock setting"]
    pub clock_clk_top_mchtmr: CLOCK_CLK_TOP_MCHTMR,
    #[doc = "0x1810 - Clock setting"]
    pub clock_clk_top_axi: CLOCK_CLK_TOP_AXI,
    #[doc = "0x1814 - Clock setting"]
    pub clock_clk_top_conn: CLOCK_CLK_TOP_CONN,
    #[doc = "0x1818 - Clock setting"]
    pub clock_clk_top_vis: CLOCK_CLK_TOP_VIS,
    #[doc = "0x181c - Clock setting"]
    pub clock_clk_top_ahb: CLOCK_CLK_TOP_AHB,
    #[doc = "0x1820 - Clock setting"]
    pub clock_clk_top_dram: CLOCK_CLK_TOP_DRAM,
    #[doc = "0x1824 - Clock setting"]
    pub clock_clk_top_xpi0: CLOCK_CLK_TOP_XPI0,
    #[doc = "0x1828 - Clock setting"]
    pub clock_clk_top_xpi1: CLOCK_CLK_TOP_XPI1,
    #[doc = "0x182c - Clock setting"]
    pub clock_clk_top_gptmr0: CLOCK_CLK_TOP_GPTMR0,
    #[doc = "0x1830 - Clock setting"]
    pub clock_clk_top_gptmr1: CLOCK_CLK_TOP_GPTMR1,
    #[doc = "0x1834 - Clock setting"]
    pub clock_clk_top_gptmr2: CLOCK_CLK_TOP_GPTMR2,
    #[doc = "0x1838 - Clock setting"]
    pub clock_clk_top_gptmr3: CLOCK_CLK_TOP_GPTMR3,
    #[doc = "0x183c - Clock setting"]
    pub clock_clk_top_gptmr4: CLOCK_CLK_TOP_GPTMR4,
    #[doc = "0x1840 - Clock setting"]
    pub clock_clk_top_gptmr5: CLOCK_CLK_TOP_GPTMR5,
    #[doc = "0x1844 - Clock setting"]
    pub clock_clk_top_gptmr6: CLOCK_CLK_TOP_GPTMR6,
    #[doc = "0x1848 - Clock setting"]
    pub clock_clk_top_gptmr7: CLOCK_CLK_TOP_GPTMR7,
    #[doc = "0x184c - Clock setting"]
    pub clock_clk_top_uart0: CLOCK_CLK_TOP_UART0,
    #[doc = "0x1850 - Clock setting"]
    pub clock_clk_top_uart1: CLOCK_CLK_TOP_UART1,
    #[doc = "0x1854 - Clock setting"]
    pub clock_clk_top_uart2: CLOCK_CLK_TOP_UART2,
    #[doc = "0x1858 - Clock setting"]
    pub clock_clk_top_uart3: CLOCK_CLK_TOP_UART3,
    #[doc = "0x185c - Clock setting"]
    pub clock_clk_top_uart4: CLOCK_CLK_TOP_UART4,
    #[doc = "0x1860 - Clock setting"]
    pub clock_clk_top_uart5: CLOCK_CLK_TOP_UART5,
    #[doc = "0x1864 - Clock setting"]
    pub clock_clk_top_uart6: CLOCK_CLK_TOP_UART6,
    #[doc = "0x1868 - Clock setting"]
    pub clock_clk_top_uart7: CLOCK_CLK_TOP_UART7,
    #[doc = "0x186c - Clock setting"]
    pub clock_clk_top_uart8: CLOCK_CLK_TOP_UART8,
    #[doc = "0x1870 - Clock setting"]
    pub clock_clk_top_uart9: CLOCK_CLK_TOP_UART9,
    #[doc = "0x1874 - Clock setting"]
    pub clock_clk_top_uart10: CLOCK_CLK_TOP_UART10,
    #[doc = "0x1878 - Clock setting"]
    pub clock_clk_top_uart11: CLOCK_CLK_TOP_UART11,
    #[doc = "0x187c - Clock setting"]
    pub clock_clk_top_uart12: CLOCK_CLK_TOP_UART12,
    #[doc = "0x1880 - Clock setting"]
    pub clock_clk_top_uart13: CLOCK_CLK_TOP_UART13,
    #[doc = "0x1884 - Clock setting"]
    pub clock_clk_top_uart14: CLOCK_CLK_TOP_UART14,
    #[doc = "0x1888 - Clock setting"]
    pub clock_clk_top_uart15: CLOCK_CLK_TOP_UART15,
    #[doc = "0x188c - Clock setting"]
    pub clock_clk_top_i2c0: CLOCK_CLK_TOP_I2C0,
    #[doc = "0x1890 - Clock setting"]
    pub clock_clk_top_i2c1: CLOCK_CLK_TOP_I2C1,
    #[doc = "0x1894 - Clock setting"]
    pub clock_clk_top_i2c2: CLOCK_CLK_TOP_I2C2,
    #[doc = "0x1898 - Clock setting"]
    pub clock_clk_top_i2c3: CLOCK_CLK_TOP_I2C3,
    #[doc = "0x189c - Clock setting"]
    pub clock_clk_top_spi0: CLOCK_CLK_TOP_SPI0,
    #[doc = "0x18a0 - Clock setting"]
    pub clock_clk_top_spi1: CLOCK_CLK_TOP_SPI1,
    #[doc = "0x18a4 - Clock setting"]
    pub clock_clk_top_spi2: CLOCK_CLK_TOP_SPI2,
    #[doc = "0x18a8 - Clock setting"]
    pub clock_clk_top_spi3: CLOCK_CLK_TOP_SPI3,
    #[doc = "0x18ac - Clock setting"]
    pub clock_clk_top_can0: CLOCK_CLK_TOP_CAN0,
    #[doc = "0x18b0 - Clock setting"]
    pub clock_clk_top_can1: CLOCK_CLK_TOP_CAN1,
    #[doc = "0x18b4 - Clock setting"]
    pub clock_clk_top_can2: CLOCK_CLK_TOP_CAN2,
    #[doc = "0x18b8 - Clock setting"]
    pub clock_clk_top_can3: CLOCK_CLK_TOP_CAN3,
    #[doc = "0x18bc - Clock setting"]
    pub clock_clk_top_ptpc: CLOCK_CLK_TOP_PTPC,
    #[doc = "0x18c0 - Clock setting"]
    pub clock_clk_top_ana0: CLOCK_CLK_TOP_ANA0,
    #[doc = "0x18c4 - Clock setting"]
    pub clock_clk_top_ana1: CLOCK_CLK_TOP_ANA1,
    #[doc = "0x18c8 - Clock setting"]
    pub clock_clk_top_ana2: CLOCK_CLK_TOP_ANA2,
    #[doc = "0x18cc - Clock setting"]
    pub clock_clk_top_aud0: CLOCK_CLK_TOP_AUD0,
    #[doc = "0x18d0 - Clock setting"]
    pub clock_clk_top_aud1: CLOCK_CLK_TOP_AUD1,
    #[doc = "0x18d4 - Clock setting"]
    pub clock_clk_top_aud2: CLOCK_CLK_TOP_AUD2,
    #[doc = "0x18d8 - Clock setting"]
    pub clock_clk_top_lcdc: CLOCK_CLK_TOP_LCDC,
    #[doc = "0x18dc - Clock setting"]
    pub clock_clk_top_cam0: CLOCK_CLK_TOP_CAM0,
    #[doc = "0x18e0 - Clock setting"]
    pub clock_clk_top_cam1: CLOCK_CLK_TOP_CAM1,
    #[doc = "0x18e4 - Clock setting"]
    pub clock_clk_top_enet0: CLOCK_CLK_TOP_ENET0,
    #[doc = "0x18e8 - Clock setting"]
    pub clock_clk_top_enet1: CLOCK_CLK_TOP_ENET1,
    #[doc = "0x18ec - Clock setting"]
    pub clock_clk_top_ptp0: CLOCK_CLK_TOP_PTP0,
    #[doc = "0x18f0 - Clock setting"]
    pub clock_clk_top_ptp1: CLOCK_CLK_TOP_PTP1,
    #[doc = "0x18f4 - Clock setting"]
    pub clock_clk_top_ref0: CLOCK_CLK_TOP_REF0,
    #[doc = "0x18f8 - Clock setting"]
    pub clock_clk_top_ref1: CLOCK_CLK_TOP_REF1,
    #[doc = "0x18fc - Clock setting"]
    pub clock_clk_top_ntmr0: CLOCK_CLK_TOP_NTMR0,
    #[doc = "0x1900 - Clock setting"]
    pub clock_clk_top_ntmr1: CLOCK_CLK_TOP_NTMR1,
    #[doc = "0x1904 - Clock setting"]
    pub clock_clk_top_sdxc0: CLOCK_CLK_TOP_SDXC0,
    #[doc = "0x1908 - Clock setting"]
    pub clock_clk_top_sdxc1: CLOCK_CLK_TOP_SDXC1,
    _reserved329: [u8; 0x02f4],
    #[doc = "0x1c00 - Clock setting"]
    pub adcclk_clk_top_adc0: ADCCLK_CLK_TOP_ADC0,
    #[doc = "0x1c04 - Clock setting"]
    pub adcclk_clk_top_adc1: ADCCLK_CLK_TOP_ADC1,
    #[doc = "0x1c08 - Clock setting"]
    pub adcclk_clk_top_adc2: ADCCLK_CLK_TOP_ADC2,
    #[doc = "0x1c0c - Clock setting"]
    pub adcclk_clk_top_adc3: ADCCLK_CLK_TOP_ADC3,
    #[doc = "0x1c10 - Clock setting"]
    pub i2sclk_clk_top_i2s0: I2SCLK_CLK_TOP_I2S0,
    #[doc = "0x1c14 - Clock setting"]
    pub i2sclk_clk_top_i2s1: I2SCLK_CLK_TOP_I2S1,
    #[doc = "0x1c18 - Clock setting"]
    pub i2sclk_clk_top_i2s2: I2SCLK_CLK_TOP_I2S2,
    #[doc = "0x1c1c - Clock setting"]
    pub i2sclk_clk_top_i2s3: I2SCLK_CLK_TOP_I2S3,
    _reserved337: [u8; 0x03e0],
    #[doc = "0x2000 - Clock senario"]
    pub global00: GLOBAL00,
    _reserved338: [u8; 0x03fc],
    #[doc = "0x2400 - Clock measure and monitor control"]
    pub monitor_slice0_control: MONITOR_SLICE0_CONTROL,
    #[doc = "0x2404 - Clock measure result"]
    pub monitor_slice0_current: MONITOR_SLICE0_CURRENT,
    #[doc = "0x2408 - Clock lower limit"]
    pub monitor_slice0_low_limit: MONITOR_SLICE0_LOW_LIMIT,
    #[doc = "0x240c - Clock upper limit"]
    pub monitor_slice0_high_limit: MONITOR_SLICE0_HIGH_LIMIT,
    _reserved342: [u8; 0x10],
    #[doc = "0x2420 - Clock measure and monitor control"]
    pub monitor_slice1_control: MONITOR_SLICE1_CONTROL,
    #[doc = "0x2424 - Clock measure result"]
    pub monitor_slice1_current: MONITOR_SLICE1_CURRENT,
    #[doc = "0x2428 - Clock lower limit"]
    pub monitor_slice1_low_limit: MONITOR_SLICE1_LOW_LIMIT,
    #[doc = "0x242c - Clock upper limit"]
    pub monitor_slice1_high_limit: MONITOR_SLICE1_HIGH_LIMIT,
    _reserved346: [u8; 0x10],
    #[doc = "0x2440 - Clock measure and monitor control"]
    pub monitor_slice2_control: MONITOR_SLICE2_CONTROL,
    #[doc = "0x2444 - Clock measure result"]
    pub monitor_slice2_current: MONITOR_SLICE2_CURRENT,
    #[doc = "0x2448 - Clock lower limit"]
    pub monitor_slice2_low_limit: MONITOR_SLICE2_LOW_LIMIT,
    #[doc = "0x244c - Clock upper limit"]
    pub monitor_slice2_high_limit: MONITOR_SLICE2_HIGH_LIMIT,
    _reserved350: [u8; 0x10],
    #[doc = "0x2460 - Clock measure and monitor control"]
    pub monitor_slice3_control: MONITOR_SLICE3_CONTROL,
    #[doc = "0x2464 - Clock measure result"]
    pub monitor_slice3_current: MONITOR_SLICE3_CURRENT,
    #[doc = "0x2468 - Clock lower limit"]
    pub monitor_slice3_low_limit: MONITOR_SLICE3_LOW_LIMIT,
    #[doc = "0x246c - Clock upper limit"]
    pub monitor_slice3_high_limit: MONITOR_SLICE3_HIGH_LIMIT,
    _reserved354: [u8; 0x0390],
    #[doc = "0x2800 - No description avaiable"]
    pub cpu_cpu0_lp: CPU_CPU0_LP,
    #[doc = "0x2804 - No description avaiable"]
    pub cpu_cpu0_lock: CPU_CPU0_LOCK,
    #[doc = "0x2808 - No description avaiable"]
    pub cpu_cpu0_gpr0: CPU_CPU0_GPR0,
    #[doc = "0x280c - No description avaiable"]
    pub cpu_cpu0_gpr1: CPU_CPU0_GPR1,
    #[doc = "0x2810 - No description avaiable"]
    pub cpu_cpu0_gpr2: CPU_CPU0_GPR2,
    #[doc = "0x2814 - No description avaiable"]
    pub cpu_cpu0_gpr3: CPU_CPU0_GPR3,
    #[doc = "0x2818 - No description avaiable"]
    pub cpu_cpu0_gpr4: CPU_CPU0_GPR4,
    #[doc = "0x281c - No description avaiable"]
    pub cpu_cpu0_gpr5: CPU_CPU0_GPR5,
    #[doc = "0x2820 - No description avaiable"]
    pub cpu_cpu0_gpr6: CPU_CPU0_GPR6,
    #[doc = "0x2824 - No description avaiable"]
    pub cpu_cpu0_gpr7: CPU_CPU0_GPR7,
    #[doc = "0x2828 - No description avaiable"]
    pub cpu_cpu0_gpr8: CPU_CPU0_GPR8,
    #[doc = "0x282c - No description avaiable"]
    pub cpu_cpu0_gpr9: CPU_CPU0_GPR9,
    #[doc = "0x2830 - No description avaiable"]
    pub cpu_cpu0_gpr10: CPU_CPU0_GPR10,
    #[doc = "0x2834 - No description avaiable"]
    pub cpu_cpu0_gpr11: CPU_CPU0_GPR11,
    #[doc = "0x2838 - No description avaiable"]
    pub cpu_cpu0_gpr12: CPU_CPU0_GPR12,
    #[doc = "0x283c - No description avaiable"]
    pub cpu_cpu0_gpr13: CPU_CPU0_GPR13,
    #[doc = "0x2840 - No description avaiable"]
    pub cpu_cpu0_status0: CPU_CPU0_STATUS0,
    #[doc = "0x2844 - No description avaiable"]
    pub cpu_cpu0_status1: CPU_CPU0_STATUS1,
    #[doc = "0x2848 - No description avaiable"]
    pub cpu_cpu0_status2: CPU_CPU0_STATUS2,
    #[doc = "0x284c - No description avaiable"]
    pub cpu_cpu0_status3: CPU_CPU0_STATUS3,
    #[doc = "0x2850 - No description avaiable"]
    pub cpu_cpu0_status4: CPU_CPU0_STATUS4,
    #[doc = "0x2854 - No description avaiable"]
    pub cpu_cpu0_status5: CPU_CPU0_STATUS5,
    #[doc = "0x2858 - No description avaiable"]
    pub cpu_cpu0_status6: CPU_CPU0_STATUS6,
    #[doc = "0x285c - No description avaiable"]
    pub cpu_cpu0_status7: CPU_CPU0_STATUS7,
    _reserved378: [u8; 0x20],
    #[doc = "0x2880 - No description avaiable"]
    pub cpu_cpu0_enable0: CPU_CPU0_ENABLE0,
    #[doc = "0x2884 - No description avaiable"]
    pub cpu_cpu0_enable1: CPU_CPU0_ENABLE1,
    #[doc = "0x2888 - No description avaiable"]
    pub cpu_cpu0_enable2: CPU_CPU0_ENABLE2,
    #[doc = "0x288c - No description avaiable"]
    pub cpu_cpu0_enable3: CPU_CPU0_ENABLE3,
    #[doc = "0x2890 - No description avaiable"]
    pub cpu_cpu0_enable4: CPU_CPU0_ENABLE4,
    #[doc = "0x2894 - No description avaiable"]
    pub cpu_cpu0_enable5: CPU_CPU0_ENABLE5,
    #[doc = "0x2898 - No description avaiable"]
    pub cpu_cpu0_enable6: CPU_CPU0_ENABLE6,
    #[doc = "0x289c - No description avaiable"]
    pub cpu_cpu0_enable7: CPU_CPU0_ENABLE7,
    _reserved386: [u8; 0x0360],
    #[doc = "0x2c00 - No description avaiable"]
    pub cpu_cpu1_lp: CPU_CPU1_LP,
    #[doc = "0x2c04 - No description avaiable"]
    pub cpu_cpu1_lock: CPU_CPU1_LOCK,
    #[doc = "0x2c08 - No description avaiable"]
    pub cpu_cpu1_gpr0: CPU_CPU1_GPR0,
    #[doc = "0x2c0c - No description avaiable"]
    pub cpu_cpu1_gpr1: CPU_CPU1_GPR1,
    #[doc = "0x2c10 - No description avaiable"]
    pub cpu_cpu1_gpr2: CPU_CPU1_GPR2,
    #[doc = "0x2c14 - No description avaiable"]
    pub cpu_cpu1_gpr3: CPU_CPU1_GPR3,
    #[doc = "0x2c18 - No description avaiable"]
    pub cpu_cpu1_gpr4: CPU_CPU1_GPR4,
    #[doc = "0x2c1c - No description avaiable"]
    pub cpu_cpu1_gpr5: CPU_CPU1_GPR5,
    #[doc = "0x2c20 - No description avaiable"]
    pub cpu_cpu1_gpr6: CPU_CPU1_GPR6,
    #[doc = "0x2c24 - No description avaiable"]
    pub cpu_cpu1_gpr7: CPU_CPU1_GPR7,
    #[doc = "0x2c28 - No description avaiable"]
    pub cpu_cpu1_gpr8: CPU_CPU1_GPR8,
    #[doc = "0x2c2c - No description avaiable"]
    pub cpu_cpu1_gpr9: CPU_CPU1_GPR9,
    #[doc = "0x2c30 - No description avaiable"]
    pub cpu_cpu1_gpr10: CPU_CPU1_GPR10,
    #[doc = "0x2c34 - No description avaiable"]
    pub cpu_cpu1_gpr11: CPU_CPU1_GPR11,
    #[doc = "0x2c38 - No description avaiable"]
    pub cpu_cpu1_gpr12: CPU_CPU1_GPR12,
    #[doc = "0x2c3c - No description avaiable"]
    pub cpu_cpu1_gpr13: CPU_CPU1_GPR13,
    #[doc = "0x2c40 - No description avaiable"]
    pub cpu_cpu1_status0: CPU_CPU1_STATUS0,
    #[doc = "0x2c44 - No description avaiable"]
    pub cpu_cpu1_status1: CPU_CPU1_STATUS1,
    #[doc = "0x2c48 - No description avaiable"]
    pub cpu_cpu1_status2: CPU_CPU1_STATUS2,
    #[doc = "0x2c4c - No description avaiable"]
    pub cpu_cpu1_status3: CPU_CPU1_STATUS3,
    #[doc = "0x2c50 - No description avaiable"]
    pub cpu_cpu1_status4: CPU_CPU1_STATUS4,
    #[doc = "0x2c54 - No description avaiable"]
    pub cpu_cpu1_status5: CPU_CPU1_STATUS5,
    #[doc = "0x2c58 - No description avaiable"]
    pub cpu_cpu1_status6: CPU_CPU1_STATUS6,
    #[doc = "0x2c5c - No description avaiable"]
    pub cpu_cpu1_status7: CPU_CPU1_STATUS7,
    _reserved410: [u8; 0x20],
    #[doc = "0x2c80 - No description avaiable"]
    pub cpu_cpu1_enable0: CPU_CPU1_ENABLE0,
    #[doc = "0x2c84 - No description avaiable"]
    pub cpu_cpu1_enable1: CPU_CPU1_ENABLE1,
    #[doc = "0x2c88 - No description avaiable"]
    pub cpu_cpu1_enable2: CPU_CPU1_ENABLE2,
    #[doc = "0x2c8c - No description avaiable"]
    pub cpu_cpu1_enable3: CPU_CPU1_ENABLE3,
    #[doc = "0x2c90 - No description avaiable"]
    pub cpu_cpu1_enable4: CPU_CPU1_ENABLE4,
    #[doc = "0x2c94 - No description avaiable"]
    pub cpu_cpu1_enable5: CPU_CPU1_ENABLE5,
    #[doc = "0x2c98 - No description avaiable"]
    pub cpu_cpu1_enable6: CPU_CPU1_ENABLE6,
    #[doc = "0x2c9c - No description avaiable"]
    pub cpu_cpu1_enable7: CPU_CPU1_ENABLE7,
}
#[doc = "RESOURCE_CPU0_CORE (rw) register accessor: an alias for `Reg<RESOURCE_CPU0_CORE_SPEC>`"]
pub type RESOURCE_CPU0_CORE = crate::Reg<resource_cpu0_core::RESOURCE_CPU0_CORE_SPEC>;
#[doc = "Resource control register for cpu0"]
pub mod resource_cpu0_core;
#[doc = "RESOURCE_CPU0_SUBSYS (rw) register accessor: an alias for `Reg<RESOURCE_CPU0_SUBSYS_SPEC>`"]
pub type RESOURCE_CPU0_SUBSYS = crate::Reg<resource_cpu0_subsys::RESOURCE_CPU0_SUBSYS_SPEC>;
#[doc = "Resource control register for cpx0"]
pub mod resource_cpu0_subsys;
#[doc = "RESOURCE_CPU1_CORE (rw) register accessor: an alias for `Reg<RESOURCE_CPU1_CORE_SPEC>`"]
pub type RESOURCE_CPU1_CORE = crate::Reg<resource_cpu1_core::RESOURCE_CPU1_CORE_SPEC>;
#[doc = "Resource control register for cpu1"]
pub mod resource_cpu1_core;
#[doc = "RESOURCE_CPX1_SUBSYS (rw) register accessor: an alias for `Reg<RESOURCE_CPX1_SUBSYS_SPEC>`"]
pub type RESOURCE_CPX1_SUBSYS = crate::Reg<resource_cpx1_subsys::RESOURCE_CPX1_SUBSYS_SPEC>;
#[doc = "Resource control register for cpx1"]
pub mod resource_cpx1_subsys;
#[doc = "RESOURCE_POW_CON (rw) register accessor: an alias for `Reg<RESOURCE_POW_CON_SPEC>`"]
pub type RESOURCE_POW_CON = crate::Reg<resource_pow_con::RESOURCE_POW_CON_SPEC>;
#[doc = "Resource control register for pow_con"]
pub mod resource_pow_con;
#[doc = "RESOURCE_POW_VIS (rw) register accessor: an alias for `Reg<RESOURCE_POW_VIS_SPEC>`"]
pub type RESOURCE_POW_VIS = crate::Reg<resource_pow_vis::RESOURCE_POW_VIS_SPEC>;
#[doc = "Resource control register for pow_vis"]
pub mod resource_pow_vis;
#[doc = "RESOURCE_POW_CPU0 (rw) register accessor: an alias for `Reg<RESOURCE_POW_CPU0_SPEC>`"]
pub type RESOURCE_POW_CPU0 = crate::Reg<resource_pow_cpu0::RESOURCE_POW_CPU0_SPEC>;
#[doc = "Resource control register for pow_cpu0"]
pub mod resource_pow_cpu0;
#[doc = "RESOURCE_POW_CPU1 (rw) register accessor: an alias for `Reg<RESOURCE_POW_CPU1_SPEC>`"]
pub type RESOURCE_POW_CPU1 = crate::Reg<resource_pow_cpu1::RESOURCE_POW_CPU1_SPEC>;
#[doc = "Resource control register for pow_cpu1"]
pub mod resource_pow_cpu1;
#[doc = "RESOURCE_RST_SOC (rw) register accessor: an alias for `Reg<RESOURCE_RST_SOC_SPEC>`"]
pub type RESOURCE_RST_SOC = crate::Reg<resource_rst_soc::RESOURCE_RST_SOC_SPEC>;
#[doc = "Resource control register for rst_soc"]
pub mod resource_rst_soc;
#[doc = "RESOURCE_RST_CON (rw) register accessor: an alias for `Reg<RESOURCE_RST_CON_SPEC>`"]
pub type RESOURCE_RST_CON = crate::Reg<resource_rst_con::RESOURCE_RST_CON_SPEC>;
#[doc = "Resource control register for rst_con"]
pub mod resource_rst_con;
#[doc = "RESOURCE_RST_VIS (rw) register accessor: an alias for `Reg<RESOURCE_RST_VIS_SPEC>`"]
pub type RESOURCE_RST_VIS = crate::Reg<resource_rst_vis::RESOURCE_RST_VIS_SPEC>;
#[doc = "Resource control register for rst_vis"]
pub mod resource_rst_vis;
#[doc = "RESOURCE_RST_CPU0 (rw) register accessor: an alias for `Reg<RESOURCE_RST_CPU0_SPEC>`"]
pub type RESOURCE_RST_CPU0 = crate::Reg<resource_rst_cpu0::RESOURCE_RST_CPU0_SPEC>;
#[doc = "Resource control register for rst_cpu0"]
pub mod resource_rst_cpu0;
#[doc = "RESOURCE_RST_CPU1 (rw) register accessor: an alias for `Reg<RESOURCE_RST_CPU1_SPEC>`"]
pub type RESOURCE_RST_CPU1 = crate::Reg<resource_rst_cpu1::RESOURCE_RST_CPU1_SPEC>;
#[doc = "Resource control register for rst_cpu1"]
pub mod resource_rst_cpu1;
#[doc = "RESOURCE_CLK_SRC_XTAL (rw) register accessor: an alias for `Reg<RESOURCE_CLK_SRC_XTAL_SPEC>`"]
pub type RESOURCE_CLK_SRC_XTAL = crate::Reg<resource_clk_src_xtal::RESOURCE_CLK_SRC_XTAL_SPEC>;
#[doc = "Resource control register for xtal"]
pub mod resource_clk_src_xtal;
#[doc = "RESOURCE_CLK_SRC_PLL0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_SRC_PLL0_SPEC>`"]
pub type RESOURCE_CLK_SRC_PLL0 = crate::Reg<resource_clk_src_pll0::RESOURCE_CLK_SRC_PLL0_SPEC>;
#[doc = "Resource control register for pll0"]
pub mod resource_clk_src_pll0;
#[doc = "RESOURCE_CLK_SRC_PLL0CLK0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_SRC_PLL0CLK0_SPEC>`"]
pub type RESOURCE_CLK_SRC_PLL0CLK0 =
    crate::Reg<resource_clk_src_pll0clk0::RESOURCE_CLK_SRC_PLL0CLK0_SPEC>;
#[doc = "Resource control register for clk0_pll0"]
pub mod resource_clk_src_pll0clk0;
#[doc = "RESOURCE_CLK_SRC_PLL1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_SRC_PLL1_SPEC>`"]
pub type RESOURCE_CLK_SRC_PLL1 = crate::Reg<resource_clk_src_pll1::RESOURCE_CLK_SRC_PLL1_SPEC>;
#[doc = "Resource control register for pll1"]
pub mod resource_clk_src_pll1;
#[doc = "RESOURCE_CLK_SRC_PLL1CLK0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_SRC_PLL1CLK0_SPEC>`"]
pub type RESOURCE_CLK_SRC_PLL1CLK0 =
    crate::Reg<resource_clk_src_pll1clk0::RESOURCE_CLK_SRC_PLL1CLK0_SPEC>;
#[doc = "Resource control register for clk0_pll1"]
pub mod resource_clk_src_pll1clk0;
#[doc = "RESOURCE_CLK_SRC_PLL1CLK1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_SRC_PLL1CLK1_SPEC>`"]
pub type RESOURCE_CLK_SRC_PLL1CLK1 =
    crate::Reg<resource_clk_src_pll1clk1::RESOURCE_CLK_SRC_PLL1CLK1_SPEC>;
#[doc = "Resource control register for clk1_pll1"]
pub mod resource_clk_src_pll1clk1;
#[doc = "RESOURCE_CLK_SRC_PLL2 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_SRC_PLL2_SPEC>`"]
pub type RESOURCE_CLK_SRC_PLL2 = crate::Reg<resource_clk_src_pll2::RESOURCE_CLK_SRC_PLL2_SPEC>;
#[doc = "Resource control register for pll2"]
pub mod resource_clk_src_pll2;
#[doc = "RESOURCE_CLK_SRC_PLL2CLK0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_SRC_PLL2CLK0_SPEC>`"]
pub type RESOURCE_CLK_SRC_PLL2CLK0 =
    crate::Reg<resource_clk_src_pll2clk0::RESOURCE_CLK_SRC_PLL2CLK0_SPEC>;
#[doc = "Resource control register for clk0_pll2"]
pub mod resource_clk_src_pll2clk0;
#[doc = "RESOURCE_CLK_SRC_PLL2CLK1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_SRC_PLL2CLK1_SPEC>`"]
pub type RESOURCE_CLK_SRC_PLL2CLK1 =
    crate::Reg<resource_clk_src_pll2clk1::RESOURCE_CLK_SRC_PLL2CLK1_SPEC>;
#[doc = "Resource control register for clk1_pll2"]
pub mod resource_clk_src_pll2clk1;
#[doc = "RESOURCE_CLK_SRC_PLL3 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_SRC_PLL3_SPEC>`"]
pub type RESOURCE_CLK_SRC_PLL3 = crate::Reg<resource_clk_src_pll3::RESOURCE_CLK_SRC_PLL3_SPEC>;
#[doc = "Resource control register for pll3"]
pub mod resource_clk_src_pll3;
#[doc = "RESOURCE_CLK_SRC_PLL3CLK0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_SRC_PLL3CLK0_SPEC>`"]
pub type RESOURCE_CLK_SRC_PLL3CLK0 =
    crate::Reg<resource_clk_src_pll3clk0::RESOURCE_CLK_SRC_PLL3CLK0_SPEC>;
#[doc = "Resource control register for clk0_pll3"]
pub mod resource_clk_src_pll3clk0;
#[doc = "RESOURCE_CLK_SRC_PLL4 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_SRC_PLL4_SPEC>`"]
pub type RESOURCE_CLK_SRC_PLL4 = crate::Reg<resource_clk_src_pll4::RESOURCE_CLK_SRC_PLL4_SPEC>;
#[doc = "Resource control register for pll4"]
pub mod resource_clk_src_pll4;
#[doc = "RESOURCE_CLK_SRC_PLL4CLK0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_SRC_PLL4CLK0_SPEC>`"]
pub type RESOURCE_CLK_SRC_PLL4CLK0 =
    crate::Reg<resource_clk_src_pll4clk0::RESOURCE_CLK_SRC_PLL4CLK0_SPEC>;
#[doc = "Resource control register for clk0_pll4"]
pub mod resource_clk_src_pll4clk0;
#[doc = "RESOURCE_CLK_TOP_CPU0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_CPU0_SPEC>`"]
pub type RESOURCE_CLK_TOP_CPU0 = crate::Reg<resource_clk_top_cpu0::RESOURCE_CLK_TOP_CPU0_SPEC>;
#[doc = "Resource control register for clk_top_cpu0"]
pub mod resource_clk_top_cpu0;
#[doc = "RESOURCE_CLK_TOP_MCHTMR0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_MCHTMR0_SPEC>`"]
pub type RESOURCE_CLK_TOP_MCHTMR0 =
    crate::Reg<resource_clk_top_mchtmr0::RESOURCE_CLK_TOP_MCHTMR0_SPEC>;
#[doc = "Resource control register for clk_top_mct0"]
pub mod resource_clk_top_mchtmr0;
#[doc = "RESOURCE_CLK_TOP_CPU1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_CPU1_SPEC>`"]
pub type RESOURCE_CLK_TOP_CPU1 = crate::Reg<resource_clk_top_cpu1::RESOURCE_CLK_TOP_CPU1_SPEC>;
#[doc = "Resource control register for clk_top_cpu1"]
pub mod resource_clk_top_cpu1;
#[doc = "RESOURCE_CLK_TOP_MCHTMR1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_MCHTMR1_SPEC>`"]
pub type RESOURCE_CLK_TOP_MCHTMR1 =
    crate::Reg<resource_clk_top_mchtmr1::RESOURCE_CLK_TOP_MCHTMR1_SPEC>;
#[doc = "Resource control register for clk_top_mct1"]
pub mod resource_clk_top_mchtmr1;
#[doc = "RESOURCE_CLK_TOP_AXI (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_AXI_SPEC>`"]
pub type RESOURCE_CLK_TOP_AXI = crate::Reg<resource_clk_top_axi::RESOURCE_CLK_TOP_AXI_SPEC>;
#[doc = "Resource control register for clk_top_axi0"]
pub mod resource_clk_top_axi;
#[doc = "RESOURCE_CLK_TOP_CONN (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_CONN_SPEC>`"]
pub type RESOURCE_CLK_TOP_CONN = crate::Reg<resource_clk_top_conn::RESOURCE_CLK_TOP_CONN_SPEC>;
#[doc = "Resource control register for clk_top_axi1"]
pub mod resource_clk_top_conn;
#[doc = "RESOURCE_CLK_TOP_VIS (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_VIS_SPEC>`"]
pub type RESOURCE_CLK_TOP_VIS = crate::Reg<resource_clk_top_vis::RESOURCE_CLK_TOP_VIS_SPEC>;
#[doc = "Resource control register for clk_top_axi2"]
pub mod resource_clk_top_vis;
#[doc = "RESOURCE_CLK_TOP_AHB (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_AHB_SPEC>`"]
pub type RESOURCE_CLK_TOP_AHB = crate::Reg<resource_clk_top_ahb::RESOURCE_CLK_TOP_AHB_SPEC>;
#[doc = "Resource control register for clk_top_ahb0"]
pub mod resource_clk_top_ahb;
#[doc = "RESOURCE_CLK_TOP_DRAM (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_DRAM_SPEC>`"]
pub type RESOURCE_CLK_TOP_DRAM = crate::Reg<resource_clk_top_dram::RESOURCE_CLK_TOP_DRAM_SPEC>;
#[doc = "Resource control register for clk_top_dram"]
pub mod resource_clk_top_dram;
#[doc = "RESOURCE_CLK_TOP_XPI0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_XPI0_SPEC>`"]
pub type RESOURCE_CLK_TOP_XPI0 = crate::Reg<resource_clk_top_xpi0::RESOURCE_CLK_TOP_XPI0_SPEC>;
#[doc = "Resource control register for clk_top_xpi0"]
pub mod resource_clk_top_xpi0;
#[doc = "RESOURCE_CLK_TOP_XPI1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_XPI1_SPEC>`"]
pub type RESOURCE_CLK_TOP_XPI1 = crate::Reg<resource_clk_top_xpi1::RESOURCE_CLK_TOP_XPI1_SPEC>;
#[doc = "Resource control register for clk_top_xpi1"]
pub mod resource_clk_top_xpi1;
#[doc = "RESOURCE_CLK_TOP_GPTMR0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_GPTMR0_SPEC>`"]
pub type RESOURCE_CLK_TOP_GPTMR0 =
    crate::Reg<resource_clk_top_gptmr0::RESOURCE_CLK_TOP_GPTMR0_SPEC>;
#[doc = "Resource control register for clk_top_tmr0"]
pub mod resource_clk_top_gptmr0;
#[doc = "RESOURCE_CLK_TOP_GPTMR1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_GPTMR1_SPEC>`"]
pub type RESOURCE_CLK_TOP_GPTMR1 =
    crate::Reg<resource_clk_top_gptmr1::RESOURCE_CLK_TOP_GPTMR1_SPEC>;
#[doc = "Resource control register for clk_top_tmr1"]
pub mod resource_clk_top_gptmr1;
#[doc = "RESOURCE_CLK_TOP_GPTMR2 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_GPTMR2_SPEC>`"]
pub type RESOURCE_CLK_TOP_GPTMR2 =
    crate::Reg<resource_clk_top_gptmr2::RESOURCE_CLK_TOP_GPTMR2_SPEC>;
#[doc = "Resource control register for clk_top_tmr2"]
pub mod resource_clk_top_gptmr2;
#[doc = "RESOURCE_CLK_TOP_GPTMR3 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_GPTMR3_SPEC>`"]
pub type RESOURCE_CLK_TOP_GPTMR3 =
    crate::Reg<resource_clk_top_gptmr3::RESOURCE_CLK_TOP_GPTMR3_SPEC>;
#[doc = "Resource control register for clk_top_tmr3"]
pub mod resource_clk_top_gptmr3;
#[doc = "RESOURCE_CLK_TOP_GPTMR4 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_GPTMR4_SPEC>`"]
pub type RESOURCE_CLK_TOP_GPTMR4 =
    crate::Reg<resource_clk_top_gptmr4::RESOURCE_CLK_TOP_GPTMR4_SPEC>;
#[doc = "Resource control register for clk_top_tmr4"]
pub mod resource_clk_top_gptmr4;
#[doc = "RESOURCE_CLK_TOP_GPTMR5 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_GPTMR5_SPEC>`"]
pub type RESOURCE_CLK_TOP_GPTMR5 =
    crate::Reg<resource_clk_top_gptmr5::RESOURCE_CLK_TOP_GPTMR5_SPEC>;
#[doc = "Resource control register for clk_top_tmr5"]
pub mod resource_clk_top_gptmr5;
#[doc = "RESOURCE_CLK_TOP_GPTMR6 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_GPTMR6_SPEC>`"]
pub type RESOURCE_CLK_TOP_GPTMR6 =
    crate::Reg<resource_clk_top_gptmr6::RESOURCE_CLK_TOP_GPTMR6_SPEC>;
#[doc = "Resource control register for clk_top_tmr6"]
pub mod resource_clk_top_gptmr6;
#[doc = "RESOURCE_CLK_TOP_GPTMR7 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_GPTMR7_SPEC>`"]
pub type RESOURCE_CLK_TOP_GPTMR7 =
    crate::Reg<resource_clk_top_gptmr7::RESOURCE_CLK_TOP_GPTMR7_SPEC>;
#[doc = "Resource control register for clk_top_tmr7"]
pub mod resource_clk_top_gptmr7;
#[doc = "RESOURCE_CLK_TOP_UART0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART0_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART0 = crate::Reg<resource_clk_top_uart0::RESOURCE_CLK_TOP_UART0_SPEC>;
#[doc = "Resource control register for clk_top_urt0"]
pub mod resource_clk_top_uart0;
#[doc = "RESOURCE_CLK_TOP_UART1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART1_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART1 = crate::Reg<resource_clk_top_uart1::RESOURCE_CLK_TOP_UART1_SPEC>;
#[doc = "Resource control register for clk_top_urt1"]
pub mod resource_clk_top_uart1;
#[doc = "RESOURCE_CLK_TOP_UART2 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART2_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART2 = crate::Reg<resource_clk_top_uart2::RESOURCE_CLK_TOP_UART2_SPEC>;
#[doc = "Resource control register for clk_top_urt2"]
pub mod resource_clk_top_uart2;
#[doc = "RESOURCE_CLK_TOP_UART3 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART3_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART3 = crate::Reg<resource_clk_top_uart3::RESOURCE_CLK_TOP_UART3_SPEC>;
#[doc = "Resource control register for clk_top_urt3"]
pub mod resource_clk_top_uart3;
#[doc = "RESOURCE_CLK_TOP_UART4 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART4_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART4 = crate::Reg<resource_clk_top_uart4::RESOURCE_CLK_TOP_UART4_SPEC>;
#[doc = "Resource control register for clk_top_urt4"]
pub mod resource_clk_top_uart4;
#[doc = "RESOURCE_CLK_TOP_UART5 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART5_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART5 = crate::Reg<resource_clk_top_uart5::RESOURCE_CLK_TOP_UART5_SPEC>;
#[doc = "Resource control register for clk_top_urt5"]
pub mod resource_clk_top_uart5;
#[doc = "RESOURCE_CLK_TOP_UART6 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART6_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART6 = crate::Reg<resource_clk_top_uart6::RESOURCE_CLK_TOP_UART6_SPEC>;
#[doc = "Resource control register for clk_top_urt6"]
pub mod resource_clk_top_uart6;
#[doc = "RESOURCE_CLK_TOP_UART7 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART7_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART7 = crate::Reg<resource_clk_top_uart7::RESOURCE_CLK_TOP_UART7_SPEC>;
#[doc = "Resource control register for clk_top_urt7"]
pub mod resource_clk_top_uart7;
#[doc = "RESOURCE_CLK_TOP_UART8 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART8_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART8 = crate::Reg<resource_clk_top_uart8::RESOURCE_CLK_TOP_UART8_SPEC>;
#[doc = "Resource control register for clk_top_urt8"]
pub mod resource_clk_top_uart8;
#[doc = "RESOURCE_CLK_TOP_UART9 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART9_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART9 = crate::Reg<resource_clk_top_uart9::RESOURCE_CLK_TOP_UART9_SPEC>;
#[doc = "Resource control register for clk_top_urt9"]
pub mod resource_clk_top_uart9;
#[doc = "RESOURCE_CLK_TOP_UART10 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART10_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART10 =
    crate::Reg<resource_clk_top_uart10::RESOURCE_CLK_TOP_UART10_SPEC>;
#[doc = "Resource control register for clk_top_urta"]
pub mod resource_clk_top_uart10;
#[doc = "RESOURCE_CLK_TOP_UART11 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART11_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART11 =
    crate::Reg<resource_clk_top_uart11::RESOURCE_CLK_TOP_UART11_SPEC>;
#[doc = "Resource control register for clk_top_urtb"]
pub mod resource_clk_top_uart11;
#[doc = "RESOURCE_CLK_TOP_UART12 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART12_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART12 =
    crate::Reg<resource_clk_top_uart12::RESOURCE_CLK_TOP_UART12_SPEC>;
#[doc = "Resource control register for clk_top_urtc"]
pub mod resource_clk_top_uart12;
#[doc = "RESOURCE_CLK_TOP_UART13 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART13_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART13 =
    crate::Reg<resource_clk_top_uart13::RESOURCE_CLK_TOP_UART13_SPEC>;
#[doc = "Resource control register for clk_top_urtd"]
pub mod resource_clk_top_uart13;
#[doc = "RESOURCE_CLK_TOP_UART14 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART14_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART14 =
    crate::Reg<resource_clk_top_uart14::RESOURCE_CLK_TOP_UART14_SPEC>;
#[doc = "Resource control register for clk_top_urte"]
pub mod resource_clk_top_uart14;
#[doc = "RESOURCE_CLK_TOP_UART15 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_UART15_SPEC>`"]
pub type RESOURCE_CLK_TOP_UART15 =
    crate::Reg<resource_clk_top_uart15::RESOURCE_CLK_TOP_UART15_SPEC>;
#[doc = "Resource control register for clk_top_urtf"]
pub mod resource_clk_top_uart15;
#[doc = "RESOURCE_CLK_TOP_I2C0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_I2C0_SPEC>`"]
pub type RESOURCE_CLK_TOP_I2C0 = crate::Reg<resource_clk_top_i2c0::RESOURCE_CLK_TOP_I2C0_SPEC>;
#[doc = "Resource control register for clk_top_i2c0"]
pub mod resource_clk_top_i2c0;
#[doc = "RESOURCE_CLK_TOP_I2C1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_I2C1_SPEC>`"]
pub type RESOURCE_CLK_TOP_I2C1 = crate::Reg<resource_clk_top_i2c1::RESOURCE_CLK_TOP_I2C1_SPEC>;
#[doc = "Resource control register for clk_top_i2c1"]
pub mod resource_clk_top_i2c1;
#[doc = "RESOURCE_CLK_TOP_I2C2 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_I2C2_SPEC>`"]
pub type RESOURCE_CLK_TOP_I2C2 = crate::Reg<resource_clk_top_i2c2::RESOURCE_CLK_TOP_I2C2_SPEC>;
#[doc = "Resource control register for clk_top_i2c2"]
pub mod resource_clk_top_i2c2;
#[doc = "RESOURCE_CLK_TOP_I2C3 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_I2C3_SPEC>`"]
pub type RESOURCE_CLK_TOP_I2C3 = crate::Reg<resource_clk_top_i2c3::RESOURCE_CLK_TOP_I2C3_SPEC>;
#[doc = "Resource control register for clk_top_i2c3"]
pub mod resource_clk_top_i2c3;
#[doc = "RESOURCE_CLK_TOP_SPI0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_SPI0_SPEC>`"]
pub type RESOURCE_CLK_TOP_SPI0 = crate::Reg<resource_clk_top_spi0::RESOURCE_CLK_TOP_SPI0_SPEC>;
#[doc = "Resource control register for clk_top_spi0"]
pub mod resource_clk_top_spi0;
#[doc = "RESOURCE_CLK_TOP_SPI1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_SPI1_SPEC>`"]
pub type RESOURCE_CLK_TOP_SPI1 = crate::Reg<resource_clk_top_spi1::RESOURCE_CLK_TOP_SPI1_SPEC>;
#[doc = "Resource control register for clk_top_spi1"]
pub mod resource_clk_top_spi1;
#[doc = "RESOURCE_CLK_TOP_SPI2 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_SPI2_SPEC>`"]
pub type RESOURCE_CLK_TOP_SPI2 = crate::Reg<resource_clk_top_spi2::RESOURCE_CLK_TOP_SPI2_SPEC>;
#[doc = "Resource control register for clk_top_spi2"]
pub mod resource_clk_top_spi2;
#[doc = "RESOURCE_CLK_TOP_SPI3 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_SPI3_SPEC>`"]
pub type RESOURCE_CLK_TOP_SPI3 = crate::Reg<resource_clk_top_spi3::RESOURCE_CLK_TOP_SPI3_SPEC>;
#[doc = "Resource control register for clk_top_spi3"]
pub mod resource_clk_top_spi3;
#[doc = "RESOURCE_CLK_TOP_CAN0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_CAN0_SPEC>`"]
pub type RESOURCE_CLK_TOP_CAN0 = crate::Reg<resource_clk_top_can0::RESOURCE_CLK_TOP_CAN0_SPEC>;
#[doc = "Resource control register for clk_top_can0"]
pub mod resource_clk_top_can0;
#[doc = "RESOURCE_CLK_TOP_CAN1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_CAN1_SPEC>`"]
pub type RESOURCE_CLK_TOP_CAN1 = crate::Reg<resource_clk_top_can1::RESOURCE_CLK_TOP_CAN1_SPEC>;
#[doc = "Resource control register for clk_top_can1"]
pub mod resource_clk_top_can1;
#[doc = "RESOURCE_CLK_TOP_CAN2 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_CAN2_SPEC>`"]
pub type RESOURCE_CLK_TOP_CAN2 = crate::Reg<resource_clk_top_can2::RESOURCE_CLK_TOP_CAN2_SPEC>;
#[doc = "Resource control register for clk_top_can2"]
pub mod resource_clk_top_can2;
#[doc = "RESOURCE_CLK_TOP_CAN3 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_CAN3_SPEC>`"]
pub type RESOURCE_CLK_TOP_CAN3 = crate::Reg<resource_clk_top_can3::RESOURCE_CLK_TOP_CAN3_SPEC>;
#[doc = "Resource control register for clk_top_can3"]
pub mod resource_clk_top_can3;
#[doc = "RESOURCE_CLK_TOP_PTPC (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_PTPC_SPEC>`"]
pub type RESOURCE_CLK_TOP_PTPC = crate::Reg<resource_clk_top_ptpc::RESOURCE_CLK_TOP_PTPC_SPEC>;
#[doc = "Resource control register for clk_top_ptpc"]
pub mod resource_clk_top_ptpc;
#[doc = "RESOURCE_CLK_TOP_ANA0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_ANA0_SPEC>`"]
pub type RESOURCE_CLK_TOP_ANA0 = crate::Reg<resource_clk_top_ana0::RESOURCE_CLK_TOP_ANA0_SPEC>;
#[doc = "Resource control register for clk_top_ana0"]
pub mod resource_clk_top_ana0;
#[doc = "RESOURCE_CLK_TOP_ANA1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_ANA1_SPEC>`"]
pub type RESOURCE_CLK_TOP_ANA1 = crate::Reg<resource_clk_top_ana1::RESOURCE_CLK_TOP_ANA1_SPEC>;
#[doc = "Resource control register for clk_top_ana1"]
pub mod resource_clk_top_ana1;
#[doc = "RESOURCE_CLK_TOP_ANA2 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_ANA2_SPEC>`"]
pub type RESOURCE_CLK_TOP_ANA2 = crate::Reg<resource_clk_top_ana2::RESOURCE_CLK_TOP_ANA2_SPEC>;
#[doc = "Resource control register for clk_top_ana2"]
pub mod resource_clk_top_ana2;
#[doc = "RESOURCE_CLK_TOP_AUD0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_AUD0_SPEC>`"]
pub type RESOURCE_CLK_TOP_AUD0 = crate::Reg<resource_clk_top_aud0::RESOURCE_CLK_TOP_AUD0_SPEC>;
#[doc = "Resource control register for clk_top_aud0"]
pub mod resource_clk_top_aud0;
#[doc = "RESOURCE_CLK_TOP_AUD1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_AUD1_SPEC>`"]
pub type RESOURCE_CLK_TOP_AUD1 = crate::Reg<resource_clk_top_aud1::RESOURCE_CLK_TOP_AUD1_SPEC>;
#[doc = "Resource control register for clk_top_aud1"]
pub mod resource_clk_top_aud1;
#[doc = "RESOURCE_CLK_TOP_AUD2 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_AUD2_SPEC>`"]
pub type RESOURCE_CLK_TOP_AUD2 = crate::Reg<resource_clk_top_aud2::RESOURCE_CLK_TOP_AUD2_SPEC>;
#[doc = "Resource control register for clk_top_aud2"]
pub mod resource_clk_top_aud2;
#[doc = "RESOURCE_CLK_TOP_LCDC (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_LCDC_SPEC>`"]
pub type RESOURCE_CLK_TOP_LCDC = crate::Reg<resource_clk_top_lcdc::RESOURCE_CLK_TOP_LCDC_SPEC>;
#[doc = "Resource control register for clk_top_dis0"]
pub mod resource_clk_top_lcdc;
#[doc = "RESOURCE_CLK_TOP_CAM0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_CAM0_SPEC>`"]
pub type RESOURCE_CLK_TOP_CAM0 = crate::Reg<resource_clk_top_cam0::RESOURCE_CLK_TOP_CAM0_SPEC>;
#[doc = "Resource control register for clk_top_cam0"]
pub mod resource_clk_top_cam0;
#[doc = "RESOURCE_CLK_TOP_CAM1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_CAM1_SPEC>`"]
pub type RESOURCE_CLK_TOP_CAM1 = crate::Reg<resource_clk_top_cam1::RESOURCE_CLK_TOP_CAM1_SPEC>;
#[doc = "Resource control register for clk_top_cam1"]
pub mod resource_clk_top_cam1;
#[doc = "RESOURCE_CLK_TOP_ENET0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_ENET0_SPEC>`"]
pub type RESOURCE_CLK_TOP_ENET0 = crate::Reg<resource_clk_top_enet0::RESOURCE_CLK_TOP_ENET0_SPEC>;
#[doc = "Resource control register for clk_top_eth0"]
pub mod resource_clk_top_enet0;
#[doc = "RESOURCE_CLK_TOP_ENET1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_ENET1_SPEC>`"]
pub type RESOURCE_CLK_TOP_ENET1 = crate::Reg<resource_clk_top_enet1::RESOURCE_CLK_TOP_ENET1_SPEC>;
#[doc = "Resource control register for clk_top_eth1"]
pub mod resource_clk_top_enet1;
#[doc = "RESOURCE_CLK_TOP_PTP0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_PTP0_SPEC>`"]
pub type RESOURCE_CLK_TOP_PTP0 = crate::Reg<resource_clk_top_ptp0::RESOURCE_CLK_TOP_PTP0_SPEC>;
#[doc = "Resource control register for clk_top_ptp0"]
pub mod resource_clk_top_ptp0;
#[doc = "RESOURCE_CLK_TOP_PTP1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_PTP1_SPEC>`"]
pub type RESOURCE_CLK_TOP_PTP1 = crate::Reg<resource_clk_top_ptp1::RESOURCE_CLK_TOP_PTP1_SPEC>;
#[doc = "Resource control register for clk_top_ptp1"]
pub mod resource_clk_top_ptp1;
#[doc = "RESOURCE_CLK_TOP_REF0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_REF0_SPEC>`"]
pub type RESOURCE_CLK_TOP_REF0 = crate::Reg<resource_clk_top_ref0::RESOURCE_CLK_TOP_REF0_SPEC>;
#[doc = "Resource control register for clk_top_ref0"]
pub mod resource_clk_top_ref0;
#[doc = "RESOURCE_CLK_TOP_REF1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_REF1_SPEC>`"]
pub type RESOURCE_CLK_TOP_REF1 = crate::Reg<resource_clk_top_ref1::RESOURCE_CLK_TOP_REF1_SPEC>;
#[doc = "Resource control register for clk_top_ref1"]
pub mod resource_clk_top_ref1;
#[doc = "RESOURCE_CLK_TOP_NTMR0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_NTMR0_SPEC>`"]
pub type RESOURCE_CLK_TOP_NTMR0 = crate::Reg<resource_clk_top_ntmr0::RESOURCE_CLK_TOP_NTMR0_SPEC>;
#[doc = "Resource control register for clk_top_ntm0"]
pub mod resource_clk_top_ntmr0;
#[doc = "RESOURCE_CLK_TOP_NTMR1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_NTMR1_SPEC>`"]
pub type RESOURCE_CLK_TOP_NTMR1 = crate::Reg<resource_clk_top_ntmr1::RESOURCE_CLK_TOP_NTMR1_SPEC>;
#[doc = "Resource control register for clk_top_ntm1"]
pub mod resource_clk_top_ntmr1;
#[doc = "RESOURCE_CLK_TOP_SDXC0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_SDXC0_SPEC>`"]
pub type RESOURCE_CLK_TOP_SDXC0 = crate::Reg<resource_clk_top_sdxc0::RESOURCE_CLK_TOP_SDXC0_SPEC>;
#[doc = "Resource control register for clk_top_sdc0"]
pub mod resource_clk_top_sdxc0;
#[doc = "RESOURCE_CLK_TOP_SDXC1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_SDXC1_SPEC>`"]
pub type RESOURCE_CLK_TOP_SDXC1 = crate::Reg<resource_clk_top_sdxc1::RESOURCE_CLK_TOP_SDXC1_SPEC>;
#[doc = "Resource control register for clk_top_sdc1"]
pub mod resource_clk_top_sdxc1;
#[doc = "RESOURCE_CLK_TOP_ADC0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_ADC0_SPEC>`"]
pub type RESOURCE_CLK_TOP_ADC0 = crate::Reg<resource_clk_top_adc0::RESOURCE_CLK_TOP_ADC0_SPEC>;
#[doc = "Resource control register for clk_top_adc0"]
pub mod resource_clk_top_adc0;
#[doc = "RESOURCE_CLK_TOP_ADC1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_ADC1_SPEC>`"]
pub type RESOURCE_CLK_TOP_ADC1 = crate::Reg<resource_clk_top_adc1::RESOURCE_CLK_TOP_ADC1_SPEC>;
#[doc = "Resource control register for clk_top_adc1"]
pub mod resource_clk_top_adc1;
#[doc = "RESOURCE_CLK_TOP_ADC2 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_ADC2_SPEC>`"]
pub type RESOURCE_CLK_TOP_ADC2 = crate::Reg<resource_clk_top_adc2::RESOURCE_CLK_TOP_ADC2_SPEC>;
#[doc = "Resource control register for clk_top_adc2"]
pub mod resource_clk_top_adc2;
#[doc = "RESOURCE_CLK_TOP_ADC3 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_ADC3_SPEC>`"]
pub type RESOURCE_CLK_TOP_ADC3 = crate::Reg<resource_clk_top_adc3::RESOURCE_CLK_TOP_ADC3_SPEC>;
#[doc = "Resource control register for clk_top_adc3"]
pub mod resource_clk_top_adc3;
#[doc = "RESOURCE_CLK_TOP_I2S0 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_I2S0_SPEC>`"]
pub type RESOURCE_CLK_TOP_I2S0 = crate::Reg<resource_clk_top_i2s0::RESOURCE_CLK_TOP_I2S0_SPEC>;
#[doc = "Resource control register for clk_top_i2s0"]
pub mod resource_clk_top_i2s0;
#[doc = "RESOURCE_CLK_TOP_I2S1 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_I2S1_SPEC>`"]
pub type RESOURCE_CLK_TOP_I2S1 = crate::Reg<resource_clk_top_i2s1::RESOURCE_CLK_TOP_I2S1_SPEC>;
#[doc = "Resource control register for clk_top_i2s1"]
pub mod resource_clk_top_i2s1;
#[doc = "RESOURCE_CLK_TOP_I2S2 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_I2S2_SPEC>`"]
pub type RESOURCE_CLK_TOP_I2S2 = crate::Reg<resource_clk_top_i2s2::RESOURCE_CLK_TOP_I2S2_SPEC>;
#[doc = "Resource control register for clk_top_i2s2"]
pub mod resource_clk_top_i2s2;
#[doc = "RESOURCE_CLK_TOP_I2S3 (rw) register accessor: an alias for `Reg<RESOURCE_CLK_TOP_I2S3_SPEC>`"]
pub type RESOURCE_CLK_TOP_I2S3 = crate::Reg<resource_clk_top_i2s3::RESOURCE_CLK_TOP_I2S3_SPEC>;
#[doc = "Resource control register for clk_top_i2s3"]
pub mod resource_clk_top_i2s3;
#[doc = "RESOURCE_AHBAPB_BUS (rw) register accessor: an alias for `Reg<RESOURCE_AHBAPB_BUS_SPEC>`"]
pub type RESOURCE_AHBAPB_BUS = crate::Reg<resource_ahbapb_bus::RESOURCE_AHBAPB_BUS_SPEC>;
#[doc = "Resource control register for ahbp"]
pub mod resource_ahbapb_bus;
#[doc = "RESOURCE_AXI_BUS (rw) register accessor: an alias for `Reg<RESOURCE_AXI_BUS_SPEC>`"]
pub type RESOURCE_AXI_BUS = crate::Reg<resource_axi_bus::RESOURCE_AXI_BUS_SPEC>;
#[doc = "Resource control register for axis"]
pub mod resource_axi_bus;
#[doc = "RESOURCE_CONN_BUS (rw) register accessor: an alias for `Reg<RESOURCE_CONN_BUS_SPEC>`"]
pub type RESOURCE_CONN_BUS = crate::Reg<resource_conn_bus::RESOURCE_CONN_BUS_SPEC>;
#[doc = "Resource control register for axic"]
pub mod resource_conn_bus;
#[doc = "RESOURCE_VIS_BUS (rw) register accessor: an alias for `Reg<RESOURCE_VIS_BUS_SPEC>`"]
pub type RESOURCE_VIS_BUS = crate::Reg<resource_vis_bus::RESOURCE_VIS_BUS_SPEC>;
#[doc = "Resource control register for axiv"]
pub mod resource_vis_bus;
#[doc = "RESOURCE_DRAM (rw) register accessor: an alias for `Reg<RESOURCE_DRAM_SPEC>`"]
pub type RESOURCE_DRAM = crate::Reg<resource_dram::RESOURCE_DRAM_SPEC>;
#[doc = "Resource control register for dram"]
pub mod resource_dram;
#[doc = "RESOURCE_ROM (rw) register accessor: an alias for `Reg<RESOURCE_ROM_SPEC>`"]
pub type RESOURCE_ROM = crate::Reg<resource_rom::RESOURCE_ROM_SPEC>;
#[doc = "Resource control register for rom0"]
pub mod resource_rom;
#[doc = "RESOURCE_LMM0 (rw) register accessor: an alias for `Reg<RESOURCE_LMM0_SPEC>`"]
pub type RESOURCE_LMM0 = crate::Reg<resource_lmm0::RESOURCE_LMM0_SPEC>;
#[doc = "Resource control register for lmm0"]
pub mod resource_lmm0;
#[doc = "RESOURCE_LMM1 (rw) register accessor: an alias for `Reg<RESOURCE_LMM1_SPEC>`"]
pub type RESOURCE_LMM1 = crate::Reg<resource_lmm1::RESOURCE_LMM1_SPEC>;
#[doc = "Resource control register for lmm1"]
pub mod resource_lmm1;
#[doc = "RESOURCE_MCHTMR0 (rw) register accessor: an alias for `Reg<RESOURCE_MCHTMR0_SPEC>`"]
pub type RESOURCE_MCHTMR0 = crate::Reg<resource_mchtmr0::RESOURCE_MCHTMR0_SPEC>;
#[doc = "Resource control register for mct0"]
pub mod resource_mchtmr0;
#[doc = "RESOURCE_MCHTMR1 (rw) register accessor: an alias for `Reg<RESOURCE_MCHTMR1_SPEC>`"]
pub type RESOURCE_MCHTMR1 = crate::Reg<resource_mchtmr1::RESOURCE_MCHTMR1_SPEC>;
#[doc = "Resource control register for mct1"]
pub mod resource_mchtmr1;
#[doc = "RESOURCE_AXI_SRAM0 (rw) register accessor: an alias for `Reg<RESOURCE_AXI_SRAM0_SPEC>`"]
pub type RESOURCE_AXI_SRAM0 = crate::Reg<resource_axi_sram0::RESOURCE_AXI_SRAM0_SPEC>;
#[doc = "Resource control register for ram0"]
pub mod resource_axi_sram0;
#[doc = "RESOURCE_AXI_SRAM1 (rw) register accessor: an alias for `Reg<RESOURCE_AXI_SRAM1_SPEC>`"]
pub type RESOURCE_AXI_SRAM1 = crate::Reg<resource_axi_sram1::RESOURCE_AXI_SRAM1_SPEC>;
#[doc = "Resource control register for ram1"]
pub mod resource_axi_sram1;
#[doc = "RESOURCE_XPI0 (rw) register accessor: an alias for `Reg<RESOURCE_XPI0_SPEC>`"]
pub type RESOURCE_XPI0 = crate::Reg<resource_xpi0::RESOURCE_XPI0_SPEC>;
#[doc = "Resource control register for xpi0"]
pub mod resource_xpi0;
#[doc = "RESOURCE_XPI1 (rw) register accessor: an alias for `Reg<RESOURCE_XPI1_SPEC>`"]
pub type RESOURCE_XPI1 = crate::Reg<resource_xpi1::RESOURCE_XPI1_SPEC>;
#[doc = "Resource control register for xpi1"]
pub mod resource_xpi1;
#[doc = "RESOURCE_SDP (rw) register accessor: an alias for `Reg<RESOURCE_SDP_SPEC>`"]
pub type RESOURCE_SDP = crate::Reg<resource_sdp::RESOURCE_SDP_SPEC>;
#[doc = "Resource control register for sdp0"]
pub mod resource_sdp;
#[doc = "RESOURCE_RNG (rw) register accessor: an alias for `Reg<RESOURCE_RNG_SPEC>`"]
pub type RESOURCE_RNG = crate::Reg<resource_rng::RESOURCE_RNG_SPEC>;
#[doc = "Resource control register for rng0"]
pub mod resource_rng;
#[doc = "RESOURCE_KEYM (rw) register accessor: an alias for `Reg<RESOURCE_KEYM_SPEC>`"]
pub type RESOURCE_KEYM = crate::Reg<resource_keym::RESOURCE_KEYM_SPEC>;
#[doc = "Resource control register for kman"]
pub mod resource_keym;
#[doc = "RESOURCE_HDMA (rw) register accessor: an alias for `Reg<RESOURCE_HDMA_SPEC>`"]
pub type RESOURCE_HDMA = crate::Reg<resource_hdma::RESOURCE_HDMA_SPEC>;
#[doc = "Resource control register for dma0"]
pub mod resource_hdma;
#[doc = "RESOURCE_XDMA (rw) register accessor: an alias for `Reg<RESOURCE_XDMA_SPEC>`"]
pub type RESOURCE_XDMA = crate::Reg<resource_xdma::RESOURCE_XDMA_SPEC>;
#[doc = "Resource control register for dma1"]
pub mod resource_xdma;
#[doc = "RESOURCE_GPIO (rw) register accessor: an alias for `Reg<RESOURCE_GPIO_SPEC>`"]
pub type RESOURCE_GPIO = crate::Reg<resource_gpio::RESOURCE_GPIO_SPEC>;
#[doc = "Resource control register for gpio"]
pub mod resource_gpio;
#[doc = "RESOURCE_MBX0 (rw) register accessor: an alias for `Reg<RESOURCE_MBX0_SPEC>`"]
pub type RESOURCE_MBX0 = crate::Reg<resource_mbx0::RESOURCE_MBX0_SPEC>;
#[doc = "Resource control register for mbx0"]
pub mod resource_mbx0;
#[doc = "RESOURCE_MBX1 (rw) register accessor: an alias for `Reg<RESOURCE_MBX1_SPEC>`"]
pub type RESOURCE_MBX1 = crate::Reg<resource_mbx1::RESOURCE_MBX1_SPEC>;
#[doc = "Resource control register for mbx1"]
pub mod resource_mbx1;
#[doc = "RESOURCE_WDG0 (rw) register accessor: an alias for `Reg<RESOURCE_WDG0_SPEC>`"]
pub type RESOURCE_WDG0 = crate::Reg<resource_wdg0::RESOURCE_WDG0_SPEC>;
#[doc = "Resource control register for wdg0"]
pub mod resource_wdg0;
#[doc = "RESOURCE_WDG1 (rw) register accessor: an alias for `Reg<RESOURCE_WDG1_SPEC>`"]
pub type RESOURCE_WDG1 = crate::Reg<resource_wdg1::RESOURCE_WDG1_SPEC>;
#[doc = "Resource control register for wdg1"]
pub mod resource_wdg1;
#[doc = "RESOURCE_WDG2 (rw) register accessor: an alias for `Reg<RESOURCE_WDG2_SPEC>`"]
pub type RESOURCE_WDG2 = crate::Reg<resource_wdg2::RESOURCE_WDG2_SPEC>;
#[doc = "Resource control register for wdg2"]
pub mod resource_wdg2;
#[doc = "RESOURCE_WDG3 (rw) register accessor: an alias for `Reg<RESOURCE_WDG3_SPEC>`"]
pub type RESOURCE_WDG3 = crate::Reg<resource_wdg3::RESOURCE_WDG3_SPEC>;
#[doc = "Resource control register for wdg3"]
pub mod resource_wdg3;
#[doc = "RESOURCE_GPTMR0 (rw) register accessor: an alias for `Reg<RESOURCE_GPTMR0_SPEC>`"]
pub type RESOURCE_GPTMR0 = crate::Reg<resource_gptmr0::RESOURCE_GPTMR0_SPEC>;
#[doc = "Resource control register for tmr0"]
pub mod resource_gptmr0;
#[doc = "RESOURCE_GPTMR1 (rw) register accessor: an alias for `Reg<RESOURCE_GPTMR1_SPEC>`"]
pub type RESOURCE_GPTMR1 = crate::Reg<resource_gptmr1::RESOURCE_GPTMR1_SPEC>;
#[doc = "Resource control register for tmr1"]
pub mod resource_gptmr1;
#[doc = "RESOURCE_GPTMR2 (rw) register accessor: an alias for `Reg<RESOURCE_GPTMR2_SPEC>`"]
pub type RESOURCE_GPTMR2 = crate::Reg<resource_gptmr2::RESOURCE_GPTMR2_SPEC>;
#[doc = "Resource control register for tmr2"]
pub mod resource_gptmr2;
#[doc = "RESOURCE_GPTMR3 (rw) register accessor: an alias for `Reg<RESOURCE_GPTMR3_SPEC>`"]
pub type RESOURCE_GPTMR3 = crate::Reg<resource_gptmr3::RESOURCE_GPTMR3_SPEC>;
#[doc = "Resource control register for tmr3"]
pub mod resource_gptmr3;
#[doc = "RESOURCE_GPTMR4 (rw) register accessor: an alias for `Reg<RESOURCE_GPTMR4_SPEC>`"]
pub type RESOURCE_GPTMR4 = crate::Reg<resource_gptmr4::RESOURCE_GPTMR4_SPEC>;
#[doc = "Resource control register for tmr4"]
pub mod resource_gptmr4;
#[doc = "RESOURCE_GPTMR5 (rw) register accessor: an alias for `Reg<RESOURCE_GPTMR5_SPEC>`"]
pub type RESOURCE_GPTMR5 = crate::Reg<resource_gptmr5::RESOURCE_GPTMR5_SPEC>;
#[doc = "Resource control register for tmr5"]
pub mod resource_gptmr5;
#[doc = "RESOURCE_GPTMR6 (rw) register accessor: an alias for `Reg<RESOURCE_GPTMR6_SPEC>`"]
pub type RESOURCE_GPTMR6 = crate::Reg<resource_gptmr6::RESOURCE_GPTMR6_SPEC>;
#[doc = "Resource control register for tmr6"]
pub mod resource_gptmr6;
#[doc = "RESOURCE_GPTMR7 (rw) register accessor: an alias for `Reg<RESOURCE_GPTMR7_SPEC>`"]
pub type RESOURCE_GPTMR7 = crate::Reg<resource_gptmr7::RESOURCE_GPTMR7_SPEC>;
#[doc = "Resource control register for tmr7"]
pub mod resource_gptmr7;
#[doc = "RESOURCE_UART0 (rw) register accessor: an alias for `Reg<RESOURCE_UART0_SPEC>`"]
pub type RESOURCE_UART0 = crate::Reg<resource_uart0::RESOURCE_UART0_SPEC>;
#[doc = "Resource control register for urt0"]
pub mod resource_uart0;
#[doc = "RESOURCE_UART1 (rw) register accessor: an alias for `Reg<RESOURCE_UART1_SPEC>`"]
pub type RESOURCE_UART1 = crate::Reg<resource_uart1::RESOURCE_UART1_SPEC>;
#[doc = "Resource control register for urt1"]
pub mod resource_uart1;
#[doc = "RESOURCE_UART2 (rw) register accessor: an alias for `Reg<RESOURCE_UART2_SPEC>`"]
pub type RESOURCE_UART2 = crate::Reg<resource_uart2::RESOURCE_UART2_SPEC>;
#[doc = "Resource control register for urt2"]
pub mod resource_uart2;
#[doc = "RESOURCE_UART3 (rw) register accessor: an alias for `Reg<RESOURCE_UART3_SPEC>`"]
pub type RESOURCE_UART3 = crate::Reg<resource_uart3::RESOURCE_UART3_SPEC>;
#[doc = "Resource control register for urt3"]
pub mod resource_uart3;
#[doc = "RESOURCE_UART4 (rw) register accessor: an alias for `Reg<RESOURCE_UART4_SPEC>`"]
pub type RESOURCE_UART4 = crate::Reg<resource_uart4::RESOURCE_UART4_SPEC>;
#[doc = "Resource control register for urt4"]
pub mod resource_uart4;
#[doc = "RESOURCE_UART5 (rw) register accessor: an alias for `Reg<RESOURCE_UART5_SPEC>`"]
pub type RESOURCE_UART5 = crate::Reg<resource_uart5::RESOURCE_UART5_SPEC>;
#[doc = "Resource control register for urt5"]
pub mod resource_uart5;
#[doc = "RESOURCE_UART6 (rw) register accessor: an alias for `Reg<RESOURCE_UART6_SPEC>`"]
pub type RESOURCE_UART6 = crate::Reg<resource_uart6::RESOURCE_UART6_SPEC>;
#[doc = "Resource control register for urt6"]
pub mod resource_uart6;
#[doc = "RESOURCE_UART7 (rw) register accessor: an alias for `Reg<RESOURCE_UART7_SPEC>`"]
pub type RESOURCE_UART7 = crate::Reg<resource_uart7::RESOURCE_UART7_SPEC>;
#[doc = "Resource control register for urt7"]
pub mod resource_uart7;
#[doc = "RESOURCE_UART8 (rw) register accessor: an alias for `Reg<RESOURCE_UART8_SPEC>`"]
pub type RESOURCE_UART8 = crate::Reg<resource_uart8::RESOURCE_UART8_SPEC>;
#[doc = "Resource control register for urt8"]
pub mod resource_uart8;
#[doc = "RESOURCE_UART9 (rw) register accessor: an alias for `Reg<RESOURCE_UART9_SPEC>`"]
pub type RESOURCE_UART9 = crate::Reg<resource_uart9::RESOURCE_UART9_SPEC>;
#[doc = "Resource control register for urt9"]
pub mod resource_uart9;
#[doc = "RESOURCE_UART10 (rw) register accessor: an alias for `Reg<RESOURCE_UART10_SPEC>`"]
pub type RESOURCE_UART10 = crate::Reg<resource_uart10::RESOURCE_UART10_SPEC>;
#[doc = "Resource control register for urta"]
pub mod resource_uart10;
#[doc = "RESOURCE_UART11 (rw) register accessor: an alias for `Reg<RESOURCE_UART11_SPEC>`"]
pub type RESOURCE_UART11 = crate::Reg<resource_uart11::RESOURCE_UART11_SPEC>;
#[doc = "Resource control register for urtb"]
pub mod resource_uart11;
#[doc = "RESOURCE_UART12 (rw) register accessor: an alias for `Reg<RESOURCE_UART12_SPEC>`"]
pub type RESOURCE_UART12 = crate::Reg<resource_uart12::RESOURCE_UART12_SPEC>;
#[doc = "Resource control register for urtc"]
pub mod resource_uart12;
#[doc = "RESOURCE_UART13 (rw) register accessor: an alias for `Reg<RESOURCE_UART13_SPEC>`"]
pub type RESOURCE_UART13 = crate::Reg<resource_uart13::RESOURCE_UART13_SPEC>;
#[doc = "Resource control register for urtd"]
pub mod resource_uart13;
#[doc = "RESOURCE_UART14 (rw) register accessor: an alias for `Reg<RESOURCE_UART14_SPEC>`"]
pub type RESOURCE_UART14 = crate::Reg<resource_uart14::RESOURCE_UART14_SPEC>;
#[doc = "Resource control register for urte"]
pub mod resource_uart14;
#[doc = "RESOURCE_UART15 (rw) register accessor: an alias for `Reg<RESOURCE_UART15_SPEC>`"]
pub type RESOURCE_UART15 = crate::Reg<resource_uart15::RESOURCE_UART15_SPEC>;
#[doc = "Resource control register for urtf"]
pub mod resource_uart15;
#[doc = "RESOURCE_I2C0 (rw) register accessor: an alias for `Reg<RESOURCE_I2C0_SPEC>`"]
pub type RESOURCE_I2C0 = crate::Reg<resource_i2c0::RESOURCE_I2C0_SPEC>;
#[doc = "Resource control register for i2c0"]
pub mod resource_i2c0;
#[doc = "RESOURCE_I2C1 (rw) register accessor: an alias for `Reg<RESOURCE_I2C1_SPEC>`"]
pub type RESOURCE_I2C1 = crate::Reg<resource_i2c1::RESOURCE_I2C1_SPEC>;
#[doc = "Resource control register for i2c1"]
pub mod resource_i2c1;
#[doc = "RESOURCE_I2C2 (rw) register accessor: an alias for `Reg<RESOURCE_I2C2_SPEC>`"]
pub type RESOURCE_I2C2 = crate::Reg<resource_i2c2::RESOURCE_I2C2_SPEC>;
#[doc = "Resource control register for i2c2"]
pub mod resource_i2c2;
#[doc = "RESOURCE_I2C3 (rw) register accessor: an alias for `Reg<RESOURCE_I2C3_SPEC>`"]
pub type RESOURCE_I2C3 = crate::Reg<resource_i2c3::RESOURCE_I2C3_SPEC>;
#[doc = "Resource control register for i2c3"]
pub mod resource_i2c3;
#[doc = "RESOURCE_SPI0 (rw) register accessor: an alias for `Reg<RESOURCE_SPI0_SPEC>`"]
pub type RESOURCE_SPI0 = crate::Reg<resource_spi0::RESOURCE_SPI0_SPEC>;
#[doc = "Resource control register for spi0"]
pub mod resource_spi0;
#[doc = "RESOURCE_SPI1 (rw) register accessor: an alias for `Reg<RESOURCE_SPI1_SPEC>`"]
pub type RESOURCE_SPI1 = crate::Reg<resource_spi1::RESOURCE_SPI1_SPEC>;
#[doc = "Resource control register for spi1"]
pub mod resource_spi1;
#[doc = "RESOURCE_SPI2 (rw) register accessor: an alias for `Reg<RESOURCE_SPI2_SPEC>`"]
pub type RESOURCE_SPI2 = crate::Reg<resource_spi2::RESOURCE_SPI2_SPEC>;
#[doc = "Resource control register for spi2"]
pub mod resource_spi2;
#[doc = "RESOURCE_SPI3 (rw) register accessor: an alias for `Reg<RESOURCE_SPI3_SPEC>`"]
pub type RESOURCE_SPI3 = crate::Reg<resource_spi3::RESOURCE_SPI3_SPEC>;
#[doc = "Resource control register for spi3"]
pub mod resource_spi3;
#[doc = "RESOURCE_CAN0 (rw) register accessor: an alias for `Reg<RESOURCE_CAN0_SPEC>`"]
pub type RESOURCE_CAN0 = crate::Reg<resource_can0::RESOURCE_CAN0_SPEC>;
#[doc = "Resource control register for can0"]
pub mod resource_can0;
#[doc = "RESOURCE_CAN1 (rw) register accessor: an alias for `Reg<RESOURCE_CAN1_SPEC>`"]
pub type RESOURCE_CAN1 = crate::Reg<resource_can1::RESOURCE_CAN1_SPEC>;
#[doc = "Resource control register for can1"]
pub mod resource_can1;
#[doc = "RESOURCE_CAN2 (rw) register accessor: an alias for `Reg<RESOURCE_CAN2_SPEC>`"]
pub type RESOURCE_CAN2 = crate::Reg<resource_can2::RESOURCE_CAN2_SPEC>;
#[doc = "Resource control register for can2"]
pub mod resource_can2;
#[doc = "RESOURCE_CAN3 (rw) register accessor: an alias for `Reg<RESOURCE_CAN3_SPEC>`"]
pub type RESOURCE_CAN3 = crate::Reg<resource_can3::RESOURCE_CAN3_SPEC>;
#[doc = "Resource control register for can3"]
pub mod resource_can3;
#[doc = "RESOURCE_PTPC (rw) register accessor: an alias for `Reg<RESOURCE_PTPC_SPEC>`"]
pub type RESOURCE_PTPC = crate::Reg<resource_ptpc::RESOURCE_PTPC_SPEC>;
#[doc = "Resource control register for ptpc"]
pub mod resource_ptpc;
#[doc = "RESOURCE_ADC0 (rw) register accessor: an alias for `Reg<RESOURCE_ADC0_SPEC>`"]
pub type RESOURCE_ADC0 = crate::Reg<resource_adc0::RESOURCE_ADC0_SPEC>;
#[doc = "Resource control register for adc0"]
pub mod resource_adc0;
#[doc = "RESOURCE_ADC1 (rw) register accessor: an alias for `Reg<RESOURCE_ADC1_SPEC>`"]
pub type RESOURCE_ADC1 = crate::Reg<resource_adc1::RESOURCE_ADC1_SPEC>;
#[doc = "Resource control register for adc1"]
pub mod resource_adc1;
#[doc = "RESOURCE_ADC2 (rw) register accessor: an alias for `Reg<RESOURCE_ADC2_SPEC>`"]
pub type RESOURCE_ADC2 = crate::Reg<resource_adc2::RESOURCE_ADC2_SPEC>;
#[doc = "Resource control register for adc2"]
pub mod resource_adc2;
#[doc = "RESOURCE_ADC3 (rw) register accessor: an alias for `Reg<RESOURCE_ADC3_SPEC>`"]
pub type RESOURCE_ADC3 = crate::Reg<resource_adc3::RESOURCE_ADC3_SPEC>;
#[doc = "Resource control register for adc3"]
pub mod resource_adc3;
#[doc = "RESOURCE_ACMP (rw) register accessor: an alias for `Reg<RESOURCE_ACMP_SPEC>`"]
pub type RESOURCE_ACMP = crate::Reg<resource_acmp::RESOURCE_ACMP_SPEC>;
#[doc = "Resource control register for acmp"]
pub mod resource_acmp;
#[doc = "RESOURCE_I2S0 (rw) register accessor: an alias for `Reg<RESOURCE_I2S0_SPEC>`"]
pub type RESOURCE_I2S0 = crate::Reg<resource_i2s0::RESOURCE_I2S0_SPEC>;
#[doc = "Resource control register for i2s0"]
pub mod resource_i2s0;
#[doc = "RESOURCE_I2S1 (rw) register accessor: an alias for `Reg<RESOURCE_I2S1_SPEC>`"]
pub type RESOURCE_I2S1 = crate::Reg<resource_i2s1::RESOURCE_I2S1_SPEC>;
#[doc = "Resource control register for i2s1"]
pub mod resource_i2s1;
#[doc = "RESOURCE_I2S2 (rw) register accessor: an alias for `Reg<RESOURCE_I2S2_SPEC>`"]
pub type RESOURCE_I2S2 = crate::Reg<resource_i2s2::RESOURCE_I2S2_SPEC>;
#[doc = "Resource control register for i2s2"]
pub mod resource_i2s2;
#[doc = "RESOURCE_I2S3 (rw) register accessor: an alias for `Reg<RESOURCE_I2S3_SPEC>`"]
pub type RESOURCE_I2S3 = crate::Reg<resource_i2s3::RESOURCE_I2S3_SPEC>;
#[doc = "Resource control register for i2s3"]
pub mod resource_i2s3;
#[doc = "RESOURCE_PDM (rw) register accessor: an alias for `Reg<RESOURCE_PDM_SPEC>`"]
pub type RESOURCE_PDM = crate::Reg<resource_pdm::RESOURCE_PDM_SPEC>;
#[doc = "Resource control register for pdm0"]
pub mod resource_pdm;
#[doc = "RESOURCE_DAO (rw) register accessor: an alias for `Reg<RESOURCE_DAO_SPEC>`"]
pub type RESOURCE_DAO = crate::Reg<resource_dao::RESOURCE_DAO_SPEC>;
#[doc = "Resource control register for clsd"]
pub mod resource_dao;
#[doc = "RESOURCE_SYNT (rw) register accessor: an alias for `Reg<RESOURCE_SYNT_SPEC>`"]
pub type RESOURCE_SYNT = crate::Reg<resource_synt::RESOURCE_SYNT_SPEC>;
#[doc = "Resource control register for msyn"]
pub mod resource_synt;
#[doc = "RESOURCE_MOT0 (rw) register accessor: an alias for `Reg<RESOURCE_MOT0_SPEC>`"]
pub type RESOURCE_MOT0 = crate::Reg<resource_mot0::RESOURCE_MOT0_SPEC>;
#[doc = "Resource control register for mot0"]
pub mod resource_mot0;
#[doc = "RESOURCE_MOT1 (rw) register accessor: an alias for `Reg<RESOURCE_MOT1_SPEC>`"]
pub type RESOURCE_MOT1 = crate::Reg<resource_mot1::RESOURCE_MOT1_SPEC>;
#[doc = "Resource control register for mot1"]
pub mod resource_mot1;
#[doc = "RESOURCE_MOT2 (rw) register accessor: an alias for `Reg<RESOURCE_MOT2_SPEC>`"]
pub type RESOURCE_MOT2 = crate::Reg<resource_mot2::RESOURCE_MOT2_SPEC>;
#[doc = "Resource control register for mot2"]
pub mod resource_mot2;
#[doc = "RESOURCE_MOT3 (rw) register accessor: an alias for `Reg<RESOURCE_MOT3_SPEC>`"]
pub type RESOURCE_MOT3 = crate::Reg<resource_mot3::RESOURCE_MOT3_SPEC>;
#[doc = "Resource control register for mot3"]
pub mod resource_mot3;
#[doc = "RESOURCE_LCDC (rw) register accessor: an alias for `Reg<RESOURCE_LCDC_SPEC>`"]
pub type RESOURCE_LCDC = crate::Reg<resource_lcdc::RESOURCE_LCDC_SPEC>;
#[doc = "Resource control register for dis0"]
pub mod resource_lcdc;
#[doc = "RESOURCE_CAM0 (rw) register accessor: an alias for `Reg<RESOURCE_CAM0_SPEC>`"]
pub type RESOURCE_CAM0 = crate::Reg<resource_cam0::RESOURCE_CAM0_SPEC>;
#[doc = "Resource control register for cam0"]
pub mod resource_cam0;
#[doc = "RESOURCE_CAM1 (rw) register accessor: an alias for `Reg<RESOURCE_CAM1_SPEC>`"]
pub type RESOURCE_CAM1 = crate::Reg<resource_cam1::RESOURCE_CAM1_SPEC>;
#[doc = "Resource control register for cam1"]
pub mod resource_cam1;
#[doc = "RESOURCE_JPEG (rw) register accessor: an alias for `Reg<RESOURCE_JPEG_SPEC>`"]
pub type RESOURCE_JPEG = crate::Reg<resource_jpeg::RESOURCE_JPEG_SPEC>;
#[doc = "Resource control register for jpeg"]
pub mod resource_jpeg;
#[doc = "RESOURCE_PDMA (rw) register accessor: an alias for `Reg<RESOURCE_PDMA_SPEC>`"]
pub type RESOURCE_PDMA = crate::Reg<resource_pdma::RESOURCE_PDMA_SPEC>;
#[doc = "Resource control register for pdma"]
pub mod resource_pdma;
#[doc = "RESOURCE_ENET0 (rw) register accessor: an alias for `Reg<RESOURCE_ENET0_SPEC>`"]
pub type RESOURCE_ENET0 = crate::Reg<resource_enet0::RESOURCE_ENET0_SPEC>;
#[doc = "Resource control register for eth0"]
pub mod resource_enet0;
#[doc = "RESOURCE_ENET1 (rw) register accessor: an alias for `Reg<RESOURCE_ENET1_SPEC>`"]
pub type RESOURCE_ENET1 = crate::Reg<resource_enet1::RESOURCE_ENET1_SPEC>;
#[doc = "Resource control register for eth1"]
pub mod resource_enet1;
#[doc = "RESOURCE_NTMR0 (rw) register accessor: an alias for `Reg<RESOURCE_NTMR0_SPEC>`"]
pub type RESOURCE_NTMR0 = crate::Reg<resource_ntmr0::RESOURCE_NTMR0_SPEC>;
#[doc = "Resource control register for ntm0"]
pub mod resource_ntmr0;
#[doc = "RESOURCE_NTMR1 (rw) register accessor: an alias for `Reg<RESOURCE_NTMR1_SPEC>`"]
pub type RESOURCE_NTMR1 = crate::Reg<resource_ntmr1::RESOURCE_NTMR1_SPEC>;
#[doc = "Resource control register for ntm1"]
pub mod resource_ntmr1;
#[doc = "RESOURCE_SDXC0 (rw) register accessor: an alias for `Reg<RESOURCE_SDXC0_SPEC>`"]
pub type RESOURCE_SDXC0 = crate::Reg<resource_sdxc0::RESOURCE_SDXC0_SPEC>;
#[doc = "Resource control register for sdc0"]
pub mod resource_sdxc0;
#[doc = "RESOURCE_SDXC1 (rw) register accessor: an alias for `Reg<RESOURCE_SDXC1_SPEC>`"]
pub type RESOURCE_SDXC1 = crate::Reg<resource_sdxc1::RESOURCE_SDXC1_SPEC>;
#[doc = "Resource control register for sdc1"]
pub mod resource_sdxc1;
#[doc = "RESOURCE_USB0 (rw) register accessor: an alias for `Reg<RESOURCE_USB0_SPEC>`"]
pub type RESOURCE_USB0 = crate::Reg<resource_usb0::RESOURCE_USB0_SPEC>;
#[doc = "Resource control register for usb0"]
pub mod resource_usb0;
#[doc = "RESOURCE_USB1 (rw) register accessor: an alias for `Reg<RESOURCE_USB1_SPEC>`"]
pub type RESOURCE_USB1 = crate::Reg<resource_usb1::RESOURCE_USB1_SPEC>;
#[doc = "Resource control register for usb1"]
pub mod resource_usb1;
#[doc = "RESOURCE_REF0 (rw) register accessor: an alias for `Reg<RESOURCE_REF0_SPEC>`"]
pub type RESOURCE_REF0 = crate::Reg<resource_ref0::RESOURCE_REF0_SPEC>;
#[doc = "Resource control register for ref0"]
pub mod resource_ref0;
#[doc = "RESOURCE_REF1 (rw) register accessor: an alias for `Reg<RESOURCE_REF1_SPEC>`"]
pub type RESOURCE_REF1 = crate::Reg<resource_ref1::RESOURCE_REF1_SPEC>;
#[doc = "Resource control register for ref1"]
pub mod resource_ref1;
#[doc = "GROUP0_0_VALUE (rw) register accessor: an alias for `Reg<GROUP0_0_VALUE_SPEC>`"]
pub type GROUP0_0_VALUE = crate::Reg<group0_0_value::GROUP0_0_VALUE_SPEC>;
#[doc = "Goup setting"]
pub mod group0_0_value;
#[doc = "GROUP0_0_SET (w) register accessor: an alias for `Reg<GROUP0_0_SET_SPEC>`"]
pub type GROUP0_0_SET = crate::Reg<group0_0_set::GROUP0_0_SET_SPEC>;
#[doc = "Group set register"]
pub mod group0_0_set;
#[doc = "GROUP0_0_CLEAR (rw) register accessor: an alias for `Reg<GROUP0_0_CLEAR_SPEC>`"]
pub type GROUP0_0_CLEAR = crate::Reg<group0_0_clear::GROUP0_0_CLEAR_SPEC>;
#[doc = "Goup setting"]
pub mod group0_0_clear;
#[doc = "GROUP0_0_TOGGLE (rw) register accessor: an alias for `Reg<GROUP0_0_TOGGLE_SPEC>`"]
pub type GROUP0_0_TOGGLE = crate::Reg<group0_0_toggle::GROUP0_0_TOGGLE_SPEC>;
#[doc = "Goup setting"]
pub mod group0_0_toggle;
#[doc = "GROUP0_1_VALUE (rw) register accessor: an alias for `Reg<GROUP0_1_VALUE_SPEC>`"]
pub type GROUP0_1_VALUE = crate::Reg<group0_1_value::GROUP0_1_VALUE_SPEC>;
#[doc = "Goup setting"]
pub mod group0_1_value;
#[doc = "GROUP0_1_SET (rw) register accessor: an alias for `Reg<GROUP0_1_SET_SPEC>`"]
pub type GROUP0_1_SET = crate::Reg<group0_1_set::GROUP0_1_SET_SPEC>;
#[doc = "Goup setting"]
pub mod group0_1_set;
#[doc = "GROUP0_1_CLEAR (rw) register accessor: an alias for `Reg<GROUP0_1_CLEAR_SPEC>`"]
pub type GROUP0_1_CLEAR = crate::Reg<group0_1_clear::GROUP0_1_CLEAR_SPEC>;
#[doc = "Goup setting"]
pub mod group0_1_clear;
#[doc = "GROUP0_1_TOGGLE (rw) register accessor: an alias for `Reg<GROUP0_1_TOGGLE_SPEC>`"]
pub type GROUP0_1_TOGGLE = crate::Reg<group0_1_toggle::GROUP0_1_TOGGLE_SPEC>;
#[doc = "Goup setting"]
pub mod group0_1_toggle;
#[doc = "GROUP0_2_VALUE (rw) register accessor: an alias for `Reg<GROUP0_2_VALUE_SPEC>`"]
pub type GROUP0_2_VALUE = crate::Reg<group0_2_value::GROUP0_2_VALUE_SPEC>;
#[doc = "Goup setting"]
pub mod group0_2_value;
#[doc = "GROUP0_2_SET (rw) register accessor: an alias for `Reg<GROUP0_2_SET_SPEC>`"]
pub type GROUP0_2_SET = crate::Reg<group0_2_set::GROUP0_2_SET_SPEC>;
#[doc = "Goup setting"]
pub mod group0_2_set;
#[doc = "GROUP0_2_CLEAR (rw) register accessor: an alias for `Reg<GROUP0_2_CLEAR_SPEC>`"]
pub type GROUP0_2_CLEAR = crate::Reg<group0_2_clear::GROUP0_2_CLEAR_SPEC>;
#[doc = "Goup setting"]
pub mod group0_2_clear;
#[doc = "GROUP0_2_TOGGLE (rw) register accessor: an alias for `Reg<GROUP0_2_TOGGLE_SPEC>`"]
pub type GROUP0_2_TOGGLE = crate::Reg<group0_2_toggle::GROUP0_2_TOGGLE_SPEC>;
#[doc = "Goup setting"]
pub mod group0_2_toggle;
#[doc = "GROUP1_0_VALUE (rw) register accessor: an alias for `Reg<GROUP1_0_VALUE_SPEC>`"]
pub type GROUP1_0_VALUE = crate::Reg<group1_0_value::GROUP1_0_VALUE_SPEC>;
#[doc = "Goup setting"]
pub mod group1_0_value;
#[doc = "GROUP1_0_SET (rw) register accessor: an alias for `Reg<GROUP1_0_SET_SPEC>`"]
pub type GROUP1_0_SET = crate::Reg<group1_0_set::GROUP1_0_SET_SPEC>;
#[doc = "Goup setting"]
pub mod group1_0_set;
#[doc = "GROUP1_0_CLEAR (rw) register accessor: an alias for `Reg<GROUP1_0_CLEAR_SPEC>`"]
pub type GROUP1_0_CLEAR = crate::Reg<group1_0_clear::GROUP1_0_CLEAR_SPEC>;
#[doc = "Goup setting"]
pub mod group1_0_clear;
#[doc = "GROUP1_0_TOGGLE (rw) register accessor: an alias for `Reg<GROUP1_0_TOGGLE_SPEC>`"]
pub type GROUP1_0_TOGGLE = crate::Reg<group1_0_toggle::GROUP1_0_TOGGLE_SPEC>;
#[doc = "Goup setting"]
pub mod group1_0_toggle;
#[doc = "GROUP1_1_VALUE (rw) register accessor: an alias for `Reg<GROUP1_1_VALUE_SPEC>`"]
pub type GROUP1_1_VALUE = crate::Reg<group1_1_value::GROUP1_1_VALUE_SPEC>;
#[doc = "Goup setting"]
pub mod group1_1_value;
#[doc = "GROUP1_1_SET (rw) register accessor: an alias for `Reg<GROUP1_1_SET_SPEC>`"]
pub type GROUP1_1_SET = crate::Reg<group1_1_set::GROUP1_1_SET_SPEC>;
#[doc = "Goup setting"]
pub mod group1_1_set;
#[doc = "GROUP1_1_CLEAR (rw) register accessor: an alias for `Reg<GROUP1_1_CLEAR_SPEC>`"]
pub type GROUP1_1_CLEAR = crate::Reg<group1_1_clear::GROUP1_1_CLEAR_SPEC>;
#[doc = "Goup setting"]
pub mod group1_1_clear;
#[doc = "GROUP1_1_TOGGLE (rw) register accessor: an alias for `Reg<GROUP1_1_TOGGLE_SPEC>`"]
pub type GROUP1_1_TOGGLE = crate::Reg<group1_1_toggle::GROUP1_1_TOGGLE_SPEC>;
#[doc = "Goup setting"]
pub mod group1_1_toggle;
#[doc = "GROUP1_2_VALUE (rw) register accessor: an alias for `Reg<GROUP1_2_VALUE_SPEC>`"]
pub type GROUP1_2_VALUE = crate::Reg<group1_2_value::GROUP1_2_VALUE_SPEC>;
#[doc = "Goup setting"]
pub mod group1_2_value;
#[doc = "GROUP1_2_SET (rw) register accessor: an alias for `Reg<GROUP1_2_SET_SPEC>`"]
pub type GROUP1_2_SET = crate::Reg<group1_2_set::GROUP1_2_SET_SPEC>;
#[doc = "Goup setting"]
pub mod group1_2_set;
#[doc = "GROUP1_2_CLEAR (rw) register accessor: an alias for `Reg<GROUP1_2_CLEAR_SPEC>`"]
pub type GROUP1_2_CLEAR = crate::Reg<group1_2_clear::GROUP1_2_CLEAR_SPEC>;
#[doc = "Goup setting"]
pub mod group1_2_clear;
#[doc = "GROUP1_2_TOGGLE (rw) register accessor: an alias for `Reg<GROUP1_2_TOGGLE_SPEC>`"]
pub type GROUP1_2_TOGGLE = crate::Reg<group1_2_toggle::GROUP1_2_TOGGLE_SPEC>;
#[doc = "Goup setting"]
pub mod group1_2_toggle;
#[doc = "AFFILIATE_CPU0_VALUE (rw) register accessor: an alias for `Reg<AFFILIATE_CPU0_VALUE_SPEC>`"]
pub type AFFILIATE_CPU0_VALUE = crate::Reg<affiliate_cpu0_value::AFFILIATE_CPU0_VALUE_SPEC>;
#[doc = "Affiliate of Group"]
pub mod affiliate_cpu0_value;
#[doc = "AFFILIATE_CPU0_SET (rw) register accessor: an alias for `Reg<AFFILIATE_CPU0_SET_SPEC>`"]
pub type AFFILIATE_CPU0_SET = crate::Reg<affiliate_cpu0_set::AFFILIATE_CPU0_SET_SPEC>;
#[doc = "Affiliate of Group"]
pub mod affiliate_cpu0_set;
#[doc = "AFFILIATE_CPU0_CLEAR (rw) register accessor: an alias for `Reg<AFFILIATE_CPU0_CLEAR_SPEC>`"]
pub type AFFILIATE_CPU0_CLEAR = crate::Reg<affiliate_cpu0_clear::AFFILIATE_CPU0_CLEAR_SPEC>;
#[doc = "Affiliate of Group"]
pub mod affiliate_cpu0_clear;
#[doc = "AFFILIATE_CPU0_TOGGLE (rw) register accessor: an alias for `Reg<AFFILIATE_CPU0_TOGGLE_SPEC>`"]
pub type AFFILIATE_CPU0_TOGGLE = crate::Reg<affiliate_cpu0_toggle::AFFILIATE_CPU0_TOGGLE_SPEC>;
#[doc = "Affiliate of Group"]
pub mod affiliate_cpu0_toggle;
#[doc = "AFFILIATE_CPU1_VALUE (rw) register accessor: an alias for `Reg<AFFILIATE_CPU1_VALUE_SPEC>`"]
pub type AFFILIATE_CPU1_VALUE = crate::Reg<affiliate_cpu1_value::AFFILIATE_CPU1_VALUE_SPEC>;
#[doc = "Affiliate of Group"]
pub mod affiliate_cpu1_value;
#[doc = "AFFILIATE_CPU1_SET (rw) register accessor: an alias for `Reg<AFFILIATE_CPU1_SET_SPEC>`"]
pub type AFFILIATE_CPU1_SET = crate::Reg<affiliate_cpu1_set::AFFILIATE_CPU1_SET_SPEC>;
#[doc = "Affiliate of Group"]
pub mod affiliate_cpu1_set;
#[doc = "AFFILIATE_CPU1_CLEAR (rw) register accessor: an alias for `Reg<AFFILIATE_CPU1_CLEAR_SPEC>`"]
pub type AFFILIATE_CPU1_CLEAR = crate::Reg<affiliate_cpu1_clear::AFFILIATE_CPU1_CLEAR_SPEC>;
#[doc = "Affiliate of Group"]
pub mod affiliate_cpu1_clear;
#[doc = "AFFILIATE_CPU1_TOGGLE (rw) register accessor: an alias for `Reg<AFFILIATE_CPU1_TOGGLE_SPEC>`"]
pub type AFFILIATE_CPU1_TOGGLE = crate::Reg<affiliate_cpu1_toggle::AFFILIATE_CPU1_TOGGLE_SPEC>;
#[doc = "Affiliate of Group"]
pub mod affiliate_cpu1_toggle;
#[doc = "RETENTION_CPU0_VALUE (rw) register accessor: an alias for `Reg<RETENTION_CPU0_VALUE_SPEC>`"]
pub type RETENTION_CPU0_VALUE = crate::Reg<retention_cpu0_value::RETENTION_CPU0_VALUE_SPEC>;
#[doc = "Retention Contol"]
pub mod retention_cpu0_value;
#[doc = "RETENTION_CPU0_SET (rw) register accessor: an alias for `Reg<RETENTION_CPU0_SET_SPEC>`"]
pub type RETENTION_CPU0_SET = crate::Reg<retention_cpu0_set::RETENTION_CPU0_SET_SPEC>;
#[doc = "Retention Contol"]
pub mod retention_cpu0_set;
#[doc = "RETENTION_CPU0_CLEAR (rw) register accessor: an alias for `Reg<RETENTION_CPU0_CLEAR_SPEC>`"]
pub type RETENTION_CPU0_CLEAR = crate::Reg<retention_cpu0_clear::RETENTION_CPU0_CLEAR_SPEC>;
#[doc = "Retention Contol"]
pub mod retention_cpu0_clear;
#[doc = "RETENTION_CPU0_TOGGLE (rw) register accessor: an alias for `Reg<RETENTION_CPU0_TOGGLE_SPEC>`"]
pub type RETENTION_CPU0_TOGGLE = crate::Reg<retention_cpu0_toggle::RETENTION_CPU0_TOGGLE_SPEC>;
#[doc = "Retention Contol"]
pub mod retention_cpu0_toggle;
#[doc = "RETENTION_CPU1_VALUE (rw) register accessor: an alias for `Reg<RETENTION_CPU1_VALUE_SPEC>`"]
pub type RETENTION_CPU1_VALUE = crate::Reg<retention_cpu1_value::RETENTION_CPU1_VALUE_SPEC>;
#[doc = "Retention Contol"]
pub mod retention_cpu1_value;
#[doc = "RETENTION_CPU1_SET (rw) register accessor: an alias for `Reg<RETENTION_CPU1_SET_SPEC>`"]
pub type RETENTION_CPU1_SET = crate::Reg<retention_cpu1_set::RETENTION_CPU1_SET_SPEC>;
#[doc = "Retention Contol"]
pub mod retention_cpu1_set;
#[doc = "RETENTION_CPU1_CLEAR (rw) register accessor: an alias for `Reg<RETENTION_CPU1_CLEAR_SPEC>`"]
pub type RETENTION_CPU1_CLEAR = crate::Reg<retention_cpu1_clear::RETENTION_CPU1_CLEAR_SPEC>;
#[doc = "Retention Contol"]
pub mod retention_cpu1_clear;
#[doc = "RETENTION_CPU1_TOGGLE (rw) register accessor: an alias for `Reg<RETENTION_CPU1_TOGGLE_SPEC>`"]
pub type RETENTION_CPU1_TOGGLE = crate::Reg<retention_cpu1_toggle::RETENTION_CPU1_TOGGLE_SPEC>;
#[doc = "Retention Contol"]
pub mod retention_cpu1_toggle;
#[doc = "POWER_CPU0_STATUS (rw) register accessor: an alias for `Reg<POWER_CPU0_STATUS_SPEC>`"]
pub type POWER_CPU0_STATUS = crate::Reg<power_cpu0_status::POWER_CPU0_STATUS_SPEC>;
#[doc = "Power Setting"]
pub mod power_cpu0_status;
#[doc = "POWER_CPU0_LF_WAIT (rw) register accessor: an alias for `Reg<POWER_CPU0_LF_WAIT_SPEC>`"]
pub type POWER_CPU0_LF_WAIT = crate::Reg<power_cpu0_lf_wait::POWER_CPU0_LF_WAIT_SPEC>;
#[doc = "Power Setting"]
pub mod power_cpu0_lf_wait;
#[doc = "POWER_CPU0_OFF_WAIT (rw) register accessor: an alias for `Reg<POWER_CPU0_OFF_WAIT_SPEC>`"]
pub type POWER_CPU0_OFF_WAIT = crate::Reg<power_cpu0_off_wait::POWER_CPU0_OFF_WAIT_SPEC>;
#[doc = "Power Setting"]
pub mod power_cpu0_off_wait;
#[doc = "POWER_CPU1_STATUS (rw) register accessor: an alias for `Reg<POWER_CPU1_STATUS_SPEC>`"]
pub type POWER_CPU1_STATUS = crate::Reg<power_cpu1_status::POWER_CPU1_STATUS_SPEC>;
#[doc = "Power Setting"]
pub mod power_cpu1_status;
#[doc = "POWER_CPU1_LF_WAIT (rw) register accessor: an alias for `Reg<POWER_CPU1_LF_WAIT_SPEC>`"]
pub type POWER_CPU1_LF_WAIT = crate::Reg<power_cpu1_lf_wait::POWER_CPU1_LF_WAIT_SPEC>;
#[doc = "Power Setting"]
pub mod power_cpu1_lf_wait;
#[doc = "POWER_CPU1_OFF_WAIT (rw) register accessor: an alias for `Reg<POWER_CPU1_OFF_WAIT_SPEC>`"]
pub type POWER_CPU1_OFF_WAIT = crate::Reg<power_cpu1_off_wait::POWER_CPU1_OFF_WAIT_SPEC>;
#[doc = "Power Setting"]
pub mod power_cpu1_off_wait;
#[doc = "POWER_CON_STATUS (rw) register accessor: an alias for `Reg<POWER_CON_STATUS_SPEC>`"]
pub type POWER_CON_STATUS = crate::Reg<power_con_status::POWER_CON_STATUS_SPEC>;
#[doc = "Power Setting"]
pub mod power_con_status;
#[doc = "POWER_CON_LF_WAIT (rw) register accessor: an alias for `Reg<POWER_CON_LF_WAIT_SPEC>`"]
pub type POWER_CON_LF_WAIT = crate::Reg<power_con_lf_wait::POWER_CON_LF_WAIT_SPEC>;
#[doc = "Power Setting"]
pub mod power_con_lf_wait;
#[doc = "POWER_CON_OFF_WAIT (rw) register accessor: an alias for `Reg<POWER_CON_OFF_WAIT_SPEC>`"]
pub type POWER_CON_OFF_WAIT = crate::Reg<power_con_off_wait::POWER_CON_OFF_WAIT_SPEC>;
#[doc = "Power Setting"]
pub mod power_con_off_wait;
#[doc = "POWER_VIS_STATUS (rw) register accessor: an alias for `Reg<POWER_VIS_STATUS_SPEC>`"]
pub type POWER_VIS_STATUS = crate::Reg<power_vis_status::POWER_VIS_STATUS_SPEC>;
#[doc = "Power Setting"]
pub mod power_vis_status;
#[doc = "POWER_VIS_LF_WAIT (rw) register accessor: an alias for `Reg<POWER_VIS_LF_WAIT_SPEC>`"]
pub type POWER_VIS_LF_WAIT = crate::Reg<power_vis_lf_wait::POWER_VIS_LF_WAIT_SPEC>;
#[doc = "Power Setting"]
pub mod power_vis_lf_wait;
#[doc = "POWER_VIS_OFF_WAIT (rw) register accessor: an alias for `Reg<POWER_VIS_OFF_WAIT_SPEC>`"]
pub type POWER_VIS_OFF_WAIT = crate::Reg<power_vis_off_wait::POWER_VIS_OFF_WAIT_SPEC>;
#[doc = "Power Setting"]
pub mod power_vis_off_wait;
#[doc = "RESET_SOC_CONTROL (rw) register accessor: an alias for `Reg<RESET_SOC_CONTROL_SPEC>`"]
pub type RESET_SOC_CONTROL = crate::Reg<reset_soc_control::RESET_SOC_CONTROL_SPEC>;
#[doc = "Reset Setting"]
pub mod reset_soc_control;
#[doc = "RESET_SOC_CONFIG (rw) register accessor: an alias for `Reg<RESET_SOC_CONFIG_SPEC>`"]
pub type RESET_SOC_CONFIG = crate::Reg<reset_soc_config::RESET_SOC_CONFIG_SPEC>;
#[doc = "Reset Setting"]
pub mod reset_soc_config;
#[doc = "RESET_SOC_COUNTER (rw) register accessor: an alias for `Reg<RESET_SOC_COUNTER_SPEC>`"]
pub type RESET_SOC_COUNTER = crate::Reg<reset_soc_counter::RESET_SOC_COUNTER_SPEC>;
#[doc = "Reset Setting"]
pub mod reset_soc_counter;
#[doc = "RESET_CON_CONTROL (rw) register accessor: an alias for `Reg<RESET_CON_CONTROL_SPEC>`"]
pub type RESET_CON_CONTROL = crate::Reg<reset_con_control::RESET_CON_CONTROL_SPEC>;
#[doc = "Reset Setting"]
pub mod reset_con_control;
#[doc = "RESET_CON_CONFIG (rw) register accessor: an alias for `Reg<RESET_CON_CONFIG_SPEC>`"]
pub type RESET_CON_CONFIG = crate::Reg<reset_con_config::RESET_CON_CONFIG_SPEC>;
#[doc = "Reset Setting"]
pub mod reset_con_config;
#[doc = "RESET_CON_COUNTER (rw) register accessor: an alias for `Reg<RESET_CON_COUNTER_SPEC>`"]
pub type RESET_CON_COUNTER = crate::Reg<reset_con_counter::RESET_CON_COUNTER_SPEC>;
#[doc = "Reset Setting"]
pub mod reset_con_counter;
#[doc = "RESET_VIS_CONTROL (rw) register accessor: an alias for `Reg<RESET_VIS_CONTROL_SPEC>`"]
pub type RESET_VIS_CONTROL = crate::Reg<reset_vis_control::RESET_VIS_CONTROL_SPEC>;
#[doc = "Reset Setting"]
pub mod reset_vis_control;
#[doc = "RESET_VIS_CONFIG (rw) register accessor: an alias for `Reg<RESET_VIS_CONFIG_SPEC>`"]
pub type RESET_VIS_CONFIG = crate::Reg<reset_vis_config::RESET_VIS_CONFIG_SPEC>;
#[doc = "Reset Setting"]
pub mod reset_vis_config;
#[doc = "RESET_VIS_COUNTER (rw) register accessor: an alias for `Reg<RESET_VIS_COUNTER_SPEC>`"]
pub type RESET_VIS_COUNTER = crate::Reg<reset_vis_counter::RESET_VIS_COUNTER_SPEC>;
#[doc = "Reset Setting"]
pub mod reset_vis_counter;
#[doc = "RESET_CPU0_CONTROL (rw) register accessor: an alias for `Reg<RESET_CPU0_CONTROL_SPEC>`"]
pub type RESET_CPU0_CONTROL = crate::Reg<reset_cpu0_control::RESET_CPU0_CONTROL_SPEC>;
#[doc = "Reset Setting"]
pub mod reset_cpu0_control;
#[doc = "RESET_CPU0_CONFIG (rw) register accessor: an alias for `Reg<RESET_CPU0_CONFIG_SPEC>`"]
pub type RESET_CPU0_CONFIG = crate::Reg<reset_cpu0_config::RESET_CPU0_CONFIG_SPEC>;
#[doc = "Reset Setting"]
pub mod reset_cpu0_config;
#[doc = "RESET_CPU0_COUNTER (rw) register accessor: an alias for `Reg<RESET_CPU0_COUNTER_SPEC>`"]
pub type RESET_CPU0_COUNTER = crate::Reg<reset_cpu0_counter::RESET_CPU0_COUNTER_SPEC>;
#[doc = "Reset Setting"]
pub mod reset_cpu0_counter;
#[doc = "RESET_CPU1_CONTROL (rw) register accessor: an alias for `Reg<RESET_CPU1_CONTROL_SPEC>`"]
pub type RESET_CPU1_CONTROL = crate::Reg<reset_cpu1_control::RESET_CPU1_CONTROL_SPEC>;
#[doc = "Reset Setting"]
pub mod reset_cpu1_control;
#[doc = "RESET_CPU1_CONFIG (rw) register accessor: an alias for `Reg<RESET_CPU1_CONFIG_SPEC>`"]
pub type RESET_CPU1_CONFIG = crate::Reg<reset_cpu1_config::RESET_CPU1_CONFIG_SPEC>;
#[doc = "Reset Setting"]
pub mod reset_cpu1_config;
#[doc = "RESET_CPU1_COUNTER (rw) register accessor: an alias for `Reg<RESET_CPU1_COUNTER_SPEC>`"]
pub type RESET_CPU1_COUNTER = crate::Reg<reset_cpu1_counter::RESET_CPU1_COUNTER_SPEC>;
#[doc = "Reset Setting"]
pub mod reset_cpu1_counter;
#[doc = "CLOCK_CLK_TOP_CPU0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_CPU0_SPEC>`"]
pub type CLOCK_CLK_TOP_CPU0 = crate::Reg<clock_clk_top_cpu0::CLOCK_CLK_TOP_CPU0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_cpu0;
#[doc = "CLOCK_CLK_TOP_MCHTMR0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_MCHTMR0_SPEC>`"]
pub type CLOCK_CLK_TOP_MCHTMR0 = crate::Reg<clock_clk_top_mchtmr0::CLOCK_CLK_TOP_MCHTMR0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_mchtmr0;
#[doc = "CLOCK_CLK_TOP_CPU1 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_CPU1_SPEC>`"]
pub type CLOCK_CLK_TOP_CPU1 = crate::Reg<clock_clk_top_cpu1::CLOCK_CLK_TOP_CPU1_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_cpu1;
#[doc = "CLOCK_CLK_TOP_MCHTMR (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_MCHTMR_SPEC>`"]
pub type CLOCK_CLK_TOP_MCHTMR = crate::Reg<clock_clk_top_mchtmr::CLOCK_CLK_TOP_MCHTMR_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_mchtmr;
#[doc = "CLOCK_CLK_TOP_AXI (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_AXI_SPEC>`"]
pub type CLOCK_CLK_TOP_AXI = crate::Reg<clock_clk_top_axi::CLOCK_CLK_TOP_AXI_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_axi;
#[doc = "CLOCK_CLK_TOP_CONN (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_CONN_SPEC>`"]
pub type CLOCK_CLK_TOP_CONN = crate::Reg<clock_clk_top_conn::CLOCK_CLK_TOP_CONN_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_conn;
#[doc = "CLOCK_CLK_TOP_VIS (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_VIS_SPEC>`"]
pub type CLOCK_CLK_TOP_VIS = crate::Reg<clock_clk_top_vis::CLOCK_CLK_TOP_VIS_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_vis;
#[doc = "CLOCK_CLK_TOP_AHB (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_AHB_SPEC>`"]
pub type CLOCK_CLK_TOP_AHB = crate::Reg<clock_clk_top_ahb::CLOCK_CLK_TOP_AHB_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_ahb;
#[doc = "CLOCK_CLK_TOP_DRAM (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_DRAM_SPEC>`"]
pub type CLOCK_CLK_TOP_DRAM = crate::Reg<clock_clk_top_dram::CLOCK_CLK_TOP_DRAM_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_dram;
#[doc = "CLOCK_CLK_TOP_XPI0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_XPI0_SPEC>`"]
pub type CLOCK_CLK_TOP_XPI0 = crate::Reg<clock_clk_top_xpi0::CLOCK_CLK_TOP_XPI0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_xpi0;
#[doc = "CLOCK_CLK_TOP_XPI1 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_XPI1_SPEC>`"]
pub type CLOCK_CLK_TOP_XPI1 = crate::Reg<clock_clk_top_xpi1::CLOCK_CLK_TOP_XPI1_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_xpi1;
#[doc = "CLOCK_CLK_TOP_GPTMR0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_GPTMR0_SPEC>`"]
pub type CLOCK_CLK_TOP_GPTMR0 = crate::Reg<clock_clk_top_gptmr0::CLOCK_CLK_TOP_GPTMR0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_gptmr0;
#[doc = "CLOCK_CLK_TOP_GPTMR1 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_GPTMR1_SPEC>`"]
pub type CLOCK_CLK_TOP_GPTMR1 = crate::Reg<clock_clk_top_gptmr1::CLOCK_CLK_TOP_GPTMR1_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_gptmr1;
#[doc = "CLOCK_CLK_TOP_GPTMR2 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_GPTMR2_SPEC>`"]
pub type CLOCK_CLK_TOP_GPTMR2 = crate::Reg<clock_clk_top_gptmr2::CLOCK_CLK_TOP_GPTMR2_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_gptmr2;
#[doc = "CLOCK_CLK_TOP_GPTMR3 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_GPTMR3_SPEC>`"]
pub type CLOCK_CLK_TOP_GPTMR3 = crate::Reg<clock_clk_top_gptmr3::CLOCK_CLK_TOP_GPTMR3_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_gptmr3;
#[doc = "CLOCK_CLK_TOP_GPTMR4 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_GPTMR4_SPEC>`"]
pub type CLOCK_CLK_TOP_GPTMR4 = crate::Reg<clock_clk_top_gptmr4::CLOCK_CLK_TOP_GPTMR4_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_gptmr4;
#[doc = "CLOCK_CLK_TOP_GPTMR5 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_GPTMR5_SPEC>`"]
pub type CLOCK_CLK_TOP_GPTMR5 = crate::Reg<clock_clk_top_gptmr5::CLOCK_CLK_TOP_GPTMR5_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_gptmr5;
#[doc = "CLOCK_CLK_TOP_GPTMR6 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_GPTMR6_SPEC>`"]
pub type CLOCK_CLK_TOP_GPTMR6 = crate::Reg<clock_clk_top_gptmr6::CLOCK_CLK_TOP_GPTMR6_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_gptmr6;
#[doc = "CLOCK_CLK_TOP_GPTMR7 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_GPTMR7_SPEC>`"]
pub type CLOCK_CLK_TOP_GPTMR7 = crate::Reg<clock_clk_top_gptmr7::CLOCK_CLK_TOP_GPTMR7_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_gptmr7;
#[doc = "CLOCK_CLK_TOP_UART0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART0_SPEC>`"]
pub type CLOCK_CLK_TOP_UART0 = crate::Reg<clock_clk_top_uart0::CLOCK_CLK_TOP_UART0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart0;
#[doc = "CLOCK_CLK_TOP_UART1 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART1_SPEC>`"]
pub type CLOCK_CLK_TOP_UART1 = crate::Reg<clock_clk_top_uart1::CLOCK_CLK_TOP_UART1_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart1;
#[doc = "CLOCK_CLK_TOP_UART2 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART2_SPEC>`"]
pub type CLOCK_CLK_TOP_UART2 = crate::Reg<clock_clk_top_uart2::CLOCK_CLK_TOP_UART2_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart2;
#[doc = "CLOCK_CLK_TOP_UART3 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART3_SPEC>`"]
pub type CLOCK_CLK_TOP_UART3 = crate::Reg<clock_clk_top_uart3::CLOCK_CLK_TOP_UART3_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart3;
#[doc = "CLOCK_CLK_TOP_UART4 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART4_SPEC>`"]
pub type CLOCK_CLK_TOP_UART4 = crate::Reg<clock_clk_top_uart4::CLOCK_CLK_TOP_UART4_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart4;
#[doc = "CLOCK_CLK_TOP_UART5 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART5_SPEC>`"]
pub type CLOCK_CLK_TOP_UART5 = crate::Reg<clock_clk_top_uart5::CLOCK_CLK_TOP_UART5_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart5;
#[doc = "CLOCK_CLK_TOP_UART6 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART6_SPEC>`"]
pub type CLOCK_CLK_TOP_UART6 = crate::Reg<clock_clk_top_uart6::CLOCK_CLK_TOP_UART6_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart6;
#[doc = "CLOCK_CLK_TOP_UART7 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART7_SPEC>`"]
pub type CLOCK_CLK_TOP_UART7 = crate::Reg<clock_clk_top_uart7::CLOCK_CLK_TOP_UART7_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart7;
#[doc = "CLOCK_CLK_TOP_UART8 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART8_SPEC>`"]
pub type CLOCK_CLK_TOP_UART8 = crate::Reg<clock_clk_top_uart8::CLOCK_CLK_TOP_UART8_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart8;
#[doc = "CLOCK_CLK_TOP_UART9 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART9_SPEC>`"]
pub type CLOCK_CLK_TOP_UART9 = crate::Reg<clock_clk_top_uart9::CLOCK_CLK_TOP_UART9_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart9;
#[doc = "CLOCK_CLK_TOP_UART10 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART10_SPEC>`"]
pub type CLOCK_CLK_TOP_UART10 = crate::Reg<clock_clk_top_uart10::CLOCK_CLK_TOP_UART10_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart10;
#[doc = "CLOCK_CLK_TOP_UART11 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART11_SPEC>`"]
pub type CLOCK_CLK_TOP_UART11 = crate::Reg<clock_clk_top_uart11::CLOCK_CLK_TOP_UART11_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart11;
#[doc = "CLOCK_CLK_TOP_UART12 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART12_SPEC>`"]
pub type CLOCK_CLK_TOP_UART12 = crate::Reg<clock_clk_top_uart12::CLOCK_CLK_TOP_UART12_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart12;
#[doc = "CLOCK_CLK_TOP_UART13 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART13_SPEC>`"]
pub type CLOCK_CLK_TOP_UART13 = crate::Reg<clock_clk_top_uart13::CLOCK_CLK_TOP_UART13_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart13;
#[doc = "CLOCK_CLK_TOP_UART14 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART14_SPEC>`"]
pub type CLOCK_CLK_TOP_UART14 = crate::Reg<clock_clk_top_uart14::CLOCK_CLK_TOP_UART14_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart14;
#[doc = "CLOCK_CLK_TOP_UART15 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_UART15_SPEC>`"]
pub type CLOCK_CLK_TOP_UART15 = crate::Reg<clock_clk_top_uart15::CLOCK_CLK_TOP_UART15_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_uart15;
#[doc = "CLOCK_CLK_TOP_I2C0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_I2C0_SPEC>`"]
pub type CLOCK_CLK_TOP_I2C0 = crate::Reg<clock_clk_top_i2c0::CLOCK_CLK_TOP_I2C0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_i2c0;
#[doc = "CLOCK_CLK_TOP_I2C1 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_I2C1_SPEC>`"]
pub type CLOCK_CLK_TOP_I2C1 = crate::Reg<clock_clk_top_i2c1::CLOCK_CLK_TOP_I2C1_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_i2c1;
#[doc = "CLOCK_CLK_TOP_I2C2 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_I2C2_SPEC>`"]
pub type CLOCK_CLK_TOP_I2C2 = crate::Reg<clock_clk_top_i2c2::CLOCK_CLK_TOP_I2C2_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_i2c2;
#[doc = "CLOCK_CLK_TOP_I2C3 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_I2C3_SPEC>`"]
pub type CLOCK_CLK_TOP_I2C3 = crate::Reg<clock_clk_top_i2c3::CLOCK_CLK_TOP_I2C3_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_i2c3;
#[doc = "CLOCK_CLK_TOP_SPI0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_SPI0_SPEC>`"]
pub type CLOCK_CLK_TOP_SPI0 = crate::Reg<clock_clk_top_spi0::CLOCK_CLK_TOP_SPI0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_spi0;
#[doc = "CLOCK_CLK_TOP_SPI1 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_SPI1_SPEC>`"]
pub type CLOCK_CLK_TOP_SPI1 = crate::Reg<clock_clk_top_spi1::CLOCK_CLK_TOP_SPI1_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_spi1;
#[doc = "CLOCK_CLK_TOP_SPI2 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_SPI2_SPEC>`"]
pub type CLOCK_CLK_TOP_SPI2 = crate::Reg<clock_clk_top_spi2::CLOCK_CLK_TOP_SPI2_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_spi2;
#[doc = "CLOCK_CLK_TOP_SPI3 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_SPI3_SPEC>`"]
pub type CLOCK_CLK_TOP_SPI3 = crate::Reg<clock_clk_top_spi3::CLOCK_CLK_TOP_SPI3_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_spi3;
#[doc = "CLOCK_CLK_TOP_CAN0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_CAN0_SPEC>`"]
pub type CLOCK_CLK_TOP_CAN0 = crate::Reg<clock_clk_top_can0::CLOCK_CLK_TOP_CAN0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_can0;
#[doc = "CLOCK_CLK_TOP_CAN1 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_CAN1_SPEC>`"]
pub type CLOCK_CLK_TOP_CAN1 = crate::Reg<clock_clk_top_can1::CLOCK_CLK_TOP_CAN1_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_can1;
#[doc = "CLOCK_CLK_TOP_CAN2 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_CAN2_SPEC>`"]
pub type CLOCK_CLK_TOP_CAN2 = crate::Reg<clock_clk_top_can2::CLOCK_CLK_TOP_CAN2_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_can2;
#[doc = "CLOCK_CLK_TOP_CAN3 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_CAN3_SPEC>`"]
pub type CLOCK_CLK_TOP_CAN3 = crate::Reg<clock_clk_top_can3::CLOCK_CLK_TOP_CAN3_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_can3;
#[doc = "CLOCK_CLK_TOP_PTPC (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_PTPC_SPEC>`"]
pub type CLOCK_CLK_TOP_PTPC = crate::Reg<clock_clk_top_ptpc::CLOCK_CLK_TOP_PTPC_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_ptpc;
#[doc = "CLOCK_CLK_TOP_ANA0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_ANA0_SPEC>`"]
pub type CLOCK_CLK_TOP_ANA0 = crate::Reg<clock_clk_top_ana0::CLOCK_CLK_TOP_ANA0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_ana0;
#[doc = "CLOCK_CLK_TOP_ANA1 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_ANA1_SPEC>`"]
pub type CLOCK_CLK_TOP_ANA1 = crate::Reg<clock_clk_top_ana1::CLOCK_CLK_TOP_ANA1_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_ana1;
#[doc = "CLOCK_CLK_TOP_ANA2 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_ANA2_SPEC>`"]
pub type CLOCK_CLK_TOP_ANA2 = crate::Reg<clock_clk_top_ana2::CLOCK_CLK_TOP_ANA2_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_ana2;
#[doc = "CLOCK_CLK_TOP_AUD0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_AUD0_SPEC>`"]
pub type CLOCK_CLK_TOP_AUD0 = crate::Reg<clock_clk_top_aud0::CLOCK_CLK_TOP_AUD0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_aud0;
#[doc = "CLOCK_CLK_TOP_AUD1 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_AUD1_SPEC>`"]
pub type CLOCK_CLK_TOP_AUD1 = crate::Reg<clock_clk_top_aud1::CLOCK_CLK_TOP_AUD1_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_aud1;
#[doc = "CLOCK_CLK_TOP_AUD2 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_AUD2_SPEC>`"]
pub type CLOCK_CLK_TOP_AUD2 = crate::Reg<clock_clk_top_aud2::CLOCK_CLK_TOP_AUD2_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_aud2;
#[doc = "CLOCK_CLK_TOP_LCDC (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_LCDC_SPEC>`"]
pub type CLOCK_CLK_TOP_LCDC = crate::Reg<clock_clk_top_lcdc::CLOCK_CLK_TOP_LCDC_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_lcdc;
#[doc = "CLOCK_CLK_TOP_CAM0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_CAM0_SPEC>`"]
pub type CLOCK_CLK_TOP_CAM0 = crate::Reg<clock_clk_top_cam0::CLOCK_CLK_TOP_CAM0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_cam0;
#[doc = "CLOCK_CLK_TOP_CAM1 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_CAM1_SPEC>`"]
pub type CLOCK_CLK_TOP_CAM1 = crate::Reg<clock_clk_top_cam1::CLOCK_CLK_TOP_CAM1_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_cam1;
#[doc = "CLOCK_CLK_TOP_ENET0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_ENET0_SPEC>`"]
pub type CLOCK_CLK_TOP_ENET0 = crate::Reg<clock_clk_top_enet0::CLOCK_CLK_TOP_ENET0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_enet0;
#[doc = "CLOCK_CLK_TOP_ENET1 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_ENET1_SPEC>`"]
pub type CLOCK_CLK_TOP_ENET1 = crate::Reg<clock_clk_top_enet1::CLOCK_CLK_TOP_ENET1_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_enet1;
#[doc = "CLOCK_CLK_TOP_PTP0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_PTP0_SPEC>`"]
pub type CLOCK_CLK_TOP_PTP0 = crate::Reg<clock_clk_top_ptp0::CLOCK_CLK_TOP_PTP0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_ptp0;
#[doc = "CLOCK_CLK_TOP_PTP1 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_PTP1_SPEC>`"]
pub type CLOCK_CLK_TOP_PTP1 = crate::Reg<clock_clk_top_ptp1::CLOCK_CLK_TOP_PTP1_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_ptp1;
#[doc = "CLOCK_CLK_TOP_REF0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_REF0_SPEC>`"]
pub type CLOCK_CLK_TOP_REF0 = crate::Reg<clock_clk_top_ref0::CLOCK_CLK_TOP_REF0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_ref0;
#[doc = "CLOCK_CLK_TOP_REF1 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_REF1_SPEC>`"]
pub type CLOCK_CLK_TOP_REF1 = crate::Reg<clock_clk_top_ref1::CLOCK_CLK_TOP_REF1_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_ref1;
#[doc = "CLOCK_CLK_TOP_NTMR0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_NTMR0_SPEC>`"]
pub type CLOCK_CLK_TOP_NTMR0 = crate::Reg<clock_clk_top_ntmr0::CLOCK_CLK_TOP_NTMR0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_ntmr0;
#[doc = "CLOCK_CLK_TOP_NTMR1 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_NTMR1_SPEC>`"]
pub type CLOCK_CLK_TOP_NTMR1 = crate::Reg<clock_clk_top_ntmr1::CLOCK_CLK_TOP_NTMR1_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_ntmr1;
#[doc = "CLOCK_CLK_TOP_SDXC0 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_SDXC0_SPEC>`"]
pub type CLOCK_CLK_TOP_SDXC0 = crate::Reg<clock_clk_top_sdxc0::CLOCK_CLK_TOP_SDXC0_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_sdxc0;
#[doc = "CLOCK_CLK_TOP_SDXC1 (rw) register accessor: an alias for `Reg<CLOCK_CLK_TOP_SDXC1_SPEC>`"]
pub type CLOCK_CLK_TOP_SDXC1 = crate::Reg<clock_clk_top_sdxc1::CLOCK_CLK_TOP_SDXC1_SPEC>;
#[doc = "Clock setting"]
pub mod clock_clk_top_sdxc1;
#[doc = "ADCCLK_CLK_TOP_ADC0 (rw) register accessor: an alias for `Reg<ADCCLK_CLK_TOP_ADC0_SPEC>`"]
pub type ADCCLK_CLK_TOP_ADC0 = crate::Reg<adcclk_clk_top_adc0::ADCCLK_CLK_TOP_ADC0_SPEC>;
#[doc = "Clock setting"]
pub mod adcclk_clk_top_adc0;
#[doc = "ADCCLK_CLK_TOP_ADC1 (rw) register accessor: an alias for `Reg<ADCCLK_CLK_TOP_ADC1_SPEC>`"]
pub type ADCCLK_CLK_TOP_ADC1 = crate::Reg<adcclk_clk_top_adc1::ADCCLK_CLK_TOP_ADC1_SPEC>;
#[doc = "Clock setting"]
pub mod adcclk_clk_top_adc1;
#[doc = "ADCCLK_CLK_TOP_ADC2 (rw) register accessor: an alias for `Reg<ADCCLK_CLK_TOP_ADC2_SPEC>`"]
pub type ADCCLK_CLK_TOP_ADC2 = crate::Reg<adcclk_clk_top_adc2::ADCCLK_CLK_TOP_ADC2_SPEC>;
#[doc = "Clock setting"]
pub mod adcclk_clk_top_adc2;
#[doc = "ADCCLK_CLK_TOP_ADC3 (rw) register accessor: an alias for `Reg<ADCCLK_CLK_TOP_ADC3_SPEC>`"]
pub type ADCCLK_CLK_TOP_ADC3 = crate::Reg<adcclk_clk_top_adc3::ADCCLK_CLK_TOP_ADC3_SPEC>;
#[doc = "Clock setting"]
pub mod adcclk_clk_top_adc3;
#[doc = "I2SCLK_CLK_TOP_I2S0 (rw) register accessor: an alias for `Reg<I2SCLK_CLK_TOP_I2S0_SPEC>`"]
pub type I2SCLK_CLK_TOP_I2S0 = crate::Reg<i2sclk_clk_top_i2s0::I2SCLK_CLK_TOP_I2S0_SPEC>;
#[doc = "Clock setting"]
pub mod i2sclk_clk_top_i2s0;
#[doc = "I2SCLK_CLK_TOP_I2S1 (rw) register accessor: an alias for `Reg<I2SCLK_CLK_TOP_I2S1_SPEC>`"]
pub type I2SCLK_CLK_TOP_I2S1 = crate::Reg<i2sclk_clk_top_i2s1::I2SCLK_CLK_TOP_I2S1_SPEC>;
#[doc = "Clock setting"]
pub mod i2sclk_clk_top_i2s1;
#[doc = "I2SCLK_CLK_TOP_I2S2 (rw) register accessor: an alias for `Reg<I2SCLK_CLK_TOP_I2S2_SPEC>`"]
pub type I2SCLK_CLK_TOP_I2S2 = crate::Reg<i2sclk_clk_top_i2s2::I2SCLK_CLK_TOP_I2S2_SPEC>;
#[doc = "Clock setting"]
pub mod i2sclk_clk_top_i2s2;
#[doc = "I2SCLK_CLK_TOP_I2S3 (rw) register accessor: an alias for `Reg<I2SCLK_CLK_TOP_I2S3_SPEC>`"]
pub type I2SCLK_CLK_TOP_I2S3 = crate::Reg<i2sclk_clk_top_i2s3::I2SCLK_CLK_TOP_I2S3_SPEC>;
#[doc = "Clock setting"]
pub mod i2sclk_clk_top_i2s3;
#[doc = "GLOBAL00 (rw) register accessor: an alias for `Reg<GLOBAL00_SPEC>`"]
pub type GLOBAL00 = crate::Reg<global00::GLOBAL00_SPEC>;
#[doc = "Clock senario"]
pub mod global00;
#[doc = "MONITOR_SLICE0_CONTROL (rw) register accessor: an alias for `Reg<MONITOR_SLICE0_CONTROL_SPEC>`"]
pub type MONITOR_SLICE0_CONTROL = crate::Reg<monitor_slice0_control::MONITOR_SLICE0_CONTROL_SPEC>;
#[doc = "Clock measure and monitor control"]
pub mod monitor_slice0_control;
#[doc = "MONITOR_SLICE0_CURRENT (r) register accessor: an alias for `Reg<MONITOR_SLICE0_CURRENT_SPEC>`"]
pub type MONITOR_SLICE0_CURRENT = crate::Reg<monitor_slice0_current::MONITOR_SLICE0_CURRENT_SPEC>;
#[doc = "Clock measure result"]
pub mod monitor_slice0_current;
#[doc = "MONITOR_SLICE0_LOW_LIMIT (rw) register accessor: an alias for `Reg<MONITOR_SLICE0_LOW_LIMIT_SPEC>`"]
pub type MONITOR_SLICE0_LOW_LIMIT =
    crate::Reg<monitor_slice0_low_limit::MONITOR_SLICE0_LOW_LIMIT_SPEC>;
#[doc = "Clock lower limit"]
pub mod monitor_slice0_low_limit;
#[doc = "MONITOR_SLICE0_HIGH_LIMIT (rw) register accessor: an alias for `Reg<MONITOR_SLICE0_HIGH_LIMIT_SPEC>`"]
pub type MONITOR_SLICE0_HIGH_LIMIT =
    crate::Reg<monitor_slice0_high_limit::MONITOR_SLICE0_HIGH_LIMIT_SPEC>;
#[doc = "Clock upper limit"]
pub mod monitor_slice0_high_limit;
#[doc = "MONITOR_SLICE1_CONTROL (rw) register accessor: an alias for `Reg<MONITOR_SLICE1_CONTROL_SPEC>`"]
pub type MONITOR_SLICE1_CONTROL = crate::Reg<monitor_slice1_control::MONITOR_SLICE1_CONTROL_SPEC>;
#[doc = "Clock measure and monitor control"]
pub mod monitor_slice1_control;
#[doc = "MONITOR_SLICE1_CURRENT (r) register accessor: an alias for `Reg<MONITOR_SLICE1_CURRENT_SPEC>`"]
pub type MONITOR_SLICE1_CURRENT = crate::Reg<monitor_slice1_current::MONITOR_SLICE1_CURRENT_SPEC>;
#[doc = "Clock measure result"]
pub mod monitor_slice1_current;
#[doc = "MONITOR_SLICE1_LOW_LIMIT (rw) register accessor: an alias for `Reg<MONITOR_SLICE1_LOW_LIMIT_SPEC>`"]
pub type MONITOR_SLICE1_LOW_LIMIT =
    crate::Reg<monitor_slice1_low_limit::MONITOR_SLICE1_LOW_LIMIT_SPEC>;
#[doc = "Clock lower limit"]
pub mod monitor_slice1_low_limit;
#[doc = "MONITOR_SLICE1_HIGH_LIMIT (rw) register accessor: an alias for `Reg<MONITOR_SLICE1_HIGH_LIMIT_SPEC>`"]
pub type MONITOR_SLICE1_HIGH_LIMIT =
    crate::Reg<monitor_slice1_high_limit::MONITOR_SLICE1_HIGH_LIMIT_SPEC>;
#[doc = "Clock upper limit"]
pub mod monitor_slice1_high_limit;
#[doc = "MONITOR_SLICE2_CONTROL (rw) register accessor: an alias for `Reg<MONITOR_SLICE2_CONTROL_SPEC>`"]
pub type MONITOR_SLICE2_CONTROL = crate::Reg<monitor_slice2_control::MONITOR_SLICE2_CONTROL_SPEC>;
#[doc = "Clock measure and monitor control"]
pub mod monitor_slice2_control;
#[doc = "MONITOR_SLICE2_CURRENT (r) register accessor: an alias for `Reg<MONITOR_SLICE2_CURRENT_SPEC>`"]
pub type MONITOR_SLICE2_CURRENT = crate::Reg<monitor_slice2_current::MONITOR_SLICE2_CURRENT_SPEC>;
#[doc = "Clock measure result"]
pub mod monitor_slice2_current;
#[doc = "MONITOR_SLICE2_LOW_LIMIT (rw) register accessor: an alias for `Reg<MONITOR_SLICE2_LOW_LIMIT_SPEC>`"]
pub type MONITOR_SLICE2_LOW_LIMIT =
    crate::Reg<monitor_slice2_low_limit::MONITOR_SLICE2_LOW_LIMIT_SPEC>;
#[doc = "Clock lower limit"]
pub mod monitor_slice2_low_limit;
#[doc = "MONITOR_SLICE2_HIGH_LIMIT (rw) register accessor: an alias for `Reg<MONITOR_SLICE2_HIGH_LIMIT_SPEC>`"]
pub type MONITOR_SLICE2_HIGH_LIMIT =
    crate::Reg<monitor_slice2_high_limit::MONITOR_SLICE2_HIGH_LIMIT_SPEC>;
#[doc = "Clock upper limit"]
pub mod monitor_slice2_high_limit;
#[doc = "MONITOR_SLICE3_CONTROL (rw) register accessor: an alias for `Reg<MONITOR_SLICE3_CONTROL_SPEC>`"]
pub type MONITOR_SLICE3_CONTROL = crate::Reg<monitor_slice3_control::MONITOR_SLICE3_CONTROL_SPEC>;
#[doc = "Clock measure and monitor control"]
pub mod monitor_slice3_control;
#[doc = "MONITOR_SLICE3_CURRENT (r) register accessor: an alias for `Reg<MONITOR_SLICE3_CURRENT_SPEC>`"]
pub type MONITOR_SLICE3_CURRENT = crate::Reg<monitor_slice3_current::MONITOR_SLICE3_CURRENT_SPEC>;
#[doc = "Clock measure result"]
pub mod monitor_slice3_current;
#[doc = "MONITOR_SLICE3_LOW_LIMIT (rw) register accessor: an alias for `Reg<MONITOR_SLICE3_LOW_LIMIT_SPEC>`"]
pub type MONITOR_SLICE3_LOW_LIMIT =
    crate::Reg<monitor_slice3_low_limit::MONITOR_SLICE3_LOW_LIMIT_SPEC>;
#[doc = "Clock lower limit"]
pub mod monitor_slice3_low_limit;
#[doc = "MONITOR_SLICE3_HIGH_LIMIT (rw) register accessor: an alias for `Reg<MONITOR_SLICE3_HIGH_LIMIT_SPEC>`"]
pub type MONITOR_SLICE3_HIGH_LIMIT =
    crate::Reg<monitor_slice3_high_limit::MONITOR_SLICE3_HIGH_LIMIT_SPEC>;
#[doc = "Clock upper limit"]
pub mod monitor_slice3_high_limit;
#[doc = "CPU_CPU0_LP (rw) register accessor: an alias for `Reg<CPU_CPU0_LP_SPEC>`"]
pub type CPU_CPU0_LP = crate::Reg<cpu_cpu0_lp::CPU_CPU0_LP_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_lp;
#[doc = "CPU_CPU0_LOCK (rw) register accessor: an alias for `Reg<CPU_CPU0_LOCK_SPEC>`"]
pub type CPU_CPU0_LOCK = crate::Reg<cpu_cpu0_lock::CPU_CPU0_LOCK_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_lock;
#[doc = "CPU_CPU0_GPR0 (rw) register accessor: an alias for `Reg<CPU_CPU0_GPR0_SPEC>`"]
pub type CPU_CPU0_GPR0 = crate::Reg<cpu_cpu0_gpr0::CPU_CPU0_GPR0_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_gpr0;
#[doc = "CPU_CPU0_GPR1 (rw) register accessor: an alias for `Reg<CPU_CPU0_GPR1_SPEC>`"]
pub type CPU_CPU0_GPR1 = crate::Reg<cpu_cpu0_gpr1::CPU_CPU0_GPR1_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_gpr1;
#[doc = "CPU_CPU0_GPR2 (rw) register accessor: an alias for `Reg<CPU_CPU0_GPR2_SPEC>`"]
pub type CPU_CPU0_GPR2 = crate::Reg<cpu_cpu0_gpr2::CPU_CPU0_GPR2_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_gpr2;
#[doc = "CPU_CPU0_GPR3 (rw) register accessor: an alias for `Reg<CPU_CPU0_GPR3_SPEC>`"]
pub type CPU_CPU0_GPR3 = crate::Reg<cpu_cpu0_gpr3::CPU_CPU0_GPR3_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_gpr3;
#[doc = "CPU_CPU0_GPR4 (rw) register accessor: an alias for `Reg<CPU_CPU0_GPR4_SPEC>`"]
pub type CPU_CPU0_GPR4 = crate::Reg<cpu_cpu0_gpr4::CPU_CPU0_GPR4_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_gpr4;
#[doc = "CPU_CPU0_GPR5 (rw) register accessor: an alias for `Reg<CPU_CPU0_GPR5_SPEC>`"]
pub type CPU_CPU0_GPR5 = crate::Reg<cpu_cpu0_gpr5::CPU_CPU0_GPR5_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_gpr5;
#[doc = "CPU_CPU0_GPR6 (rw) register accessor: an alias for `Reg<CPU_CPU0_GPR6_SPEC>`"]
pub type CPU_CPU0_GPR6 = crate::Reg<cpu_cpu0_gpr6::CPU_CPU0_GPR6_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_gpr6;
#[doc = "CPU_CPU0_GPR7 (rw) register accessor: an alias for `Reg<CPU_CPU0_GPR7_SPEC>`"]
pub type CPU_CPU0_GPR7 = crate::Reg<cpu_cpu0_gpr7::CPU_CPU0_GPR7_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_gpr7;
#[doc = "CPU_CPU0_GPR8 (rw) register accessor: an alias for `Reg<CPU_CPU0_GPR8_SPEC>`"]
pub type CPU_CPU0_GPR8 = crate::Reg<cpu_cpu0_gpr8::CPU_CPU0_GPR8_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_gpr8;
#[doc = "CPU_CPU0_GPR9 (rw) register accessor: an alias for `Reg<CPU_CPU0_GPR9_SPEC>`"]
pub type CPU_CPU0_GPR9 = crate::Reg<cpu_cpu0_gpr9::CPU_CPU0_GPR9_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_gpr9;
#[doc = "CPU_CPU0_GPR10 (rw) register accessor: an alias for `Reg<CPU_CPU0_GPR10_SPEC>`"]
pub type CPU_CPU0_GPR10 = crate::Reg<cpu_cpu0_gpr10::CPU_CPU0_GPR10_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_gpr10;
#[doc = "CPU_CPU0_GPR11 (rw) register accessor: an alias for `Reg<CPU_CPU0_GPR11_SPEC>`"]
pub type CPU_CPU0_GPR11 = crate::Reg<cpu_cpu0_gpr11::CPU_CPU0_GPR11_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_gpr11;
#[doc = "CPU_CPU0_GPR12 (rw) register accessor: an alias for `Reg<CPU_CPU0_GPR12_SPEC>`"]
pub type CPU_CPU0_GPR12 = crate::Reg<cpu_cpu0_gpr12::CPU_CPU0_GPR12_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_gpr12;
#[doc = "CPU_CPU0_GPR13 (rw) register accessor: an alias for `Reg<CPU_CPU0_GPR13_SPEC>`"]
pub type CPU_CPU0_GPR13 = crate::Reg<cpu_cpu0_gpr13::CPU_CPU0_GPR13_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_gpr13;
#[doc = "CPU_CPU0_STATUS0 (r) register accessor: an alias for `Reg<CPU_CPU0_STATUS0_SPEC>`"]
pub type CPU_CPU0_STATUS0 = crate::Reg<cpu_cpu0_status0::CPU_CPU0_STATUS0_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_status0;
#[doc = "CPU_CPU0_STATUS1 (r) register accessor: an alias for `Reg<CPU_CPU0_STATUS1_SPEC>`"]
pub type CPU_CPU0_STATUS1 = crate::Reg<cpu_cpu0_status1::CPU_CPU0_STATUS1_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_status1;
#[doc = "CPU_CPU0_STATUS2 (r) register accessor: an alias for `Reg<CPU_CPU0_STATUS2_SPEC>`"]
pub type CPU_CPU0_STATUS2 = crate::Reg<cpu_cpu0_status2::CPU_CPU0_STATUS2_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_status2;
#[doc = "CPU_CPU0_STATUS3 (r) register accessor: an alias for `Reg<CPU_CPU0_STATUS3_SPEC>`"]
pub type CPU_CPU0_STATUS3 = crate::Reg<cpu_cpu0_status3::CPU_CPU0_STATUS3_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_status3;
#[doc = "CPU_CPU0_STATUS4 (r) register accessor: an alias for `Reg<CPU_CPU0_STATUS4_SPEC>`"]
pub type CPU_CPU0_STATUS4 = crate::Reg<cpu_cpu0_status4::CPU_CPU0_STATUS4_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_status4;
#[doc = "CPU_CPU0_STATUS5 (r) register accessor: an alias for `Reg<CPU_CPU0_STATUS5_SPEC>`"]
pub type CPU_CPU0_STATUS5 = crate::Reg<cpu_cpu0_status5::CPU_CPU0_STATUS5_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_status5;
#[doc = "CPU_CPU0_STATUS6 (r) register accessor: an alias for `Reg<CPU_CPU0_STATUS6_SPEC>`"]
pub type CPU_CPU0_STATUS6 = crate::Reg<cpu_cpu0_status6::CPU_CPU0_STATUS6_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_status6;
#[doc = "CPU_CPU0_STATUS7 (r) register accessor: an alias for `Reg<CPU_CPU0_STATUS7_SPEC>`"]
pub type CPU_CPU0_STATUS7 = crate::Reg<cpu_cpu0_status7::CPU_CPU0_STATUS7_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_status7;
#[doc = "CPU_CPU0_ENABLE0 (rw) register accessor: an alias for `Reg<CPU_CPU0_ENABLE0_SPEC>`"]
pub type CPU_CPU0_ENABLE0 = crate::Reg<cpu_cpu0_enable0::CPU_CPU0_ENABLE0_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_enable0;
#[doc = "CPU_CPU0_ENABLE1 (rw) register accessor: an alias for `Reg<CPU_CPU0_ENABLE1_SPEC>`"]
pub type CPU_CPU0_ENABLE1 = crate::Reg<cpu_cpu0_enable1::CPU_CPU0_ENABLE1_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_enable1;
#[doc = "CPU_CPU0_ENABLE2 (rw) register accessor: an alias for `Reg<CPU_CPU0_ENABLE2_SPEC>`"]
pub type CPU_CPU0_ENABLE2 = crate::Reg<cpu_cpu0_enable2::CPU_CPU0_ENABLE2_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_enable2;
#[doc = "CPU_CPU0_ENABLE3 (rw) register accessor: an alias for `Reg<CPU_CPU0_ENABLE3_SPEC>`"]
pub type CPU_CPU0_ENABLE3 = crate::Reg<cpu_cpu0_enable3::CPU_CPU0_ENABLE3_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_enable3;
#[doc = "CPU_CPU0_ENABLE4 (rw) register accessor: an alias for `Reg<CPU_CPU0_ENABLE4_SPEC>`"]
pub type CPU_CPU0_ENABLE4 = crate::Reg<cpu_cpu0_enable4::CPU_CPU0_ENABLE4_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_enable4;
#[doc = "CPU_CPU0_ENABLE5 (rw) register accessor: an alias for `Reg<CPU_CPU0_ENABLE5_SPEC>`"]
pub type CPU_CPU0_ENABLE5 = crate::Reg<cpu_cpu0_enable5::CPU_CPU0_ENABLE5_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_enable5;
#[doc = "CPU_CPU0_ENABLE6 (rw) register accessor: an alias for `Reg<CPU_CPU0_ENABLE6_SPEC>`"]
pub type CPU_CPU0_ENABLE6 = crate::Reg<cpu_cpu0_enable6::CPU_CPU0_ENABLE6_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_enable6;
#[doc = "CPU_CPU0_ENABLE7 (rw) register accessor: an alias for `Reg<CPU_CPU0_ENABLE7_SPEC>`"]
pub type CPU_CPU0_ENABLE7 = crate::Reg<cpu_cpu0_enable7::CPU_CPU0_ENABLE7_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu0_enable7;
#[doc = "CPU_CPU1_LP (rw) register accessor: an alias for `Reg<CPU_CPU1_LP_SPEC>`"]
pub type CPU_CPU1_LP = crate::Reg<cpu_cpu1_lp::CPU_CPU1_LP_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_lp;
#[doc = "CPU_CPU1_LOCK (rw) register accessor: an alias for `Reg<CPU_CPU1_LOCK_SPEC>`"]
pub type CPU_CPU1_LOCK = crate::Reg<cpu_cpu1_lock::CPU_CPU1_LOCK_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_lock;
#[doc = "CPU_CPU1_GPR0 (rw) register accessor: an alias for `Reg<CPU_CPU1_GPR0_SPEC>`"]
pub type CPU_CPU1_GPR0 = crate::Reg<cpu_cpu1_gpr0::CPU_CPU1_GPR0_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_gpr0;
#[doc = "CPU_CPU1_GPR1 (rw) register accessor: an alias for `Reg<CPU_CPU1_GPR1_SPEC>`"]
pub type CPU_CPU1_GPR1 = crate::Reg<cpu_cpu1_gpr1::CPU_CPU1_GPR1_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_gpr1;
#[doc = "CPU_CPU1_GPR2 (rw) register accessor: an alias for `Reg<CPU_CPU1_GPR2_SPEC>`"]
pub type CPU_CPU1_GPR2 = crate::Reg<cpu_cpu1_gpr2::CPU_CPU1_GPR2_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_gpr2;
#[doc = "CPU_CPU1_GPR3 (rw) register accessor: an alias for `Reg<CPU_CPU1_GPR3_SPEC>`"]
pub type CPU_CPU1_GPR3 = crate::Reg<cpu_cpu1_gpr3::CPU_CPU1_GPR3_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_gpr3;
#[doc = "CPU_CPU1_GPR4 (rw) register accessor: an alias for `Reg<CPU_CPU1_GPR4_SPEC>`"]
pub type CPU_CPU1_GPR4 = crate::Reg<cpu_cpu1_gpr4::CPU_CPU1_GPR4_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_gpr4;
#[doc = "CPU_CPU1_GPR5 (rw) register accessor: an alias for `Reg<CPU_CPU1_GPR5_SPEC>`"]
pub type CPU_CPU1_GPR5 = crate::Reg<cpu_cpu1_gpr5::CPU_CPU1_GPR5_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_gpr5;
#[doc = "CPU_CPU1_GPR6 (rw) register accessor: an alias for `Reg<CPU_CPU1_GPR6_SPEC>`"]
pub type CPU_CPU1_GPR6 = crate::Reg<cpu_cpu1_gpr6::CPU_CPU1_GPR6_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_gpr6;
#[doc = "CPU_CPU1_GPR7 (rw) register accessor: an alias for `Reg<CPU_CPU1_GPR7_SPEC>`"]
pub type CPU_CPU1_GPR7 = crate::Reg<cpu_cpu1_gpr7::CPU_CPU1_GPR7_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_gpr7;
#[doc = "CPU_CPU1_GPR8 (rw) register accessor: an alias for `Reg<CPU_CPU1_GPR8_SPEC>`"]
pub type CPU_CPU1_GPR8 = crate::Reg<cpu_cpu1_gpr8::CPU_CPU1_GPR8_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_gpr8;
#[doc = "CPU_CPU1_GPR9 (rw) register accessor: an alias for `Reg<CPU_CPU1_GPR9_SPEC>`"]
pub type CPU_CPU1_GPR9 = crate::Reg<cpu_cpu1_gpr9::CPU_CPU1_GPR9_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_gpr9;
#[doc = "CPU_CPU1_GPR10 (rw) register accessor: an alias for `Reg<CPU_CPU1_GPR10_SPEC>`"]
pub type CPU_CPU1_GPR10 = crate::Reg<cpu_cpu1_gpr10::CPU_CPU1_GPR10_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_gpr10;
#[doc = "CPU_CPU1_GPR11 (rw) register accessor: an alias for `Reg<CPU_CPU1_GPR11_SPEC>`"]
pub type CPU_CPU1_GPR11 = crate::Reg<cpu_cpu1_gpr11::CPU_CPU1_GPR11_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_gpr11;
#[doc = "CPU_CPU1_GPR12 (rw) register accessor: an alias for `Reg<CPU_CPU1_GPR12_SPEC>`"]
pub type CPU_CPU1_GPR12 = crate::Reg<cpu_cpu1_gpr12::CPU_CPU1_GPR12_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_gpr12;
#[doc = "CPU_CPU1_GPR13 (rw) register accessor: an alias for `Reg<CPU_CPU1_GPR13_SPEC>`"]
pub type CPU_CPU1_GPR13 = crate::Reg<cpu_cpu1_gpr13::CPU_CPU1_GPR13_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_gpr13;
#[doc = "CPU_CPU1_STATUS0 (r) register accessor: an alias for `Reg<CPU_CPU1_STATUS0_SPEC>`"]
pub type CPU_CPU1_STATUS0 = crate::Reg<cpu_cpu1_status0::CPU_CPU1_STATUS0_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_status0;
#[doc = "CPU_CPU1_STATUS1 (r) register accessor: an alias for `Reg<CPU_CPU1_STATUS1_SPEC>`"]
pub type CPU_CPU1_STATUS1 = crate::Reg<cpu_cpu1_status1::CPU_CPU1_STATUS1_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_status1;
#[doc = "CPU_CPU1_STATUS2 (r) register accessor: an alias for `Reg<CPU_CPU1_STATUS2_SPEC>`"]
pub type CPU_CPU1_STATUS2 = crate::Reg<cpu_cpu1_status2::CPU_CPU1_STATUS2_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_status2;
#[doc = "CPU_CPU1_STATUS3 (r) register accessor: an alias for `Reg<CPU_CPU1_STATUS3_SPEC>`"]
pub type CPU_CPU1_STATUS3 = crate::Reg<cpu_cpu1_status3::CPU_CPU1_STATUS3_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_status3;
#[doc = "CPU_CPU1_STATUS4 (r) register accessor: an alias for `Reg<CPU_CPU1_STATUS4_SPEC>`"]
pub type CPU_CPU1_STATUS4 = crate::Reg<cpu_cpu1_status4::CPU_CPU1_STATUS4_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_status4;
#[doc = "CPU_CPU1_STATUS5 (r) register accessor: an alias for `Reg<CPU_CPU1_STATUS5_SPEC>`"]
pub type CPU_CPU1_STATUS5 = crate::Reg<cpu_cpu1_status5::CPU_CPU1_STATUS5_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_status5;
#[doc = "CPU_CPU1_STATUS6 (r) register accessor: an alias for `Reg<CPU_CPU1_STATUS6_SPEC>`"]
pub type CPU_CPU1_STATUS6 = crate::Reg<cpu_cpu1_status6::CPU_CPU1_STATUS6_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_status6;
#[doc = "CPU_CPU1_STATUS7 (r) register accessor: an alias for `Reg<CPU_CPU1_STATUS7_SPEC>`"]
pub type CPU_CPU1_STATUS7 = crate::Reg<cpu_cpu1_status7::CPU_CPU1_STATUS7_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_status7;
#[doc = "CPU_CPU1_ENABLE0 (rw) register accessor: an alias for `Reg<CPU_CPU1_ENABLE0_SPEC>`"]
pub type CPU_CPU1_ENABLE0 = crate::Reg<cpu_cpu1_enable0::CPU_CPU1_ENABLE0_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_enable0;
#[doc = "CPU_CPU1_ENABLE1 (rw) register accessor: an alias for `Reg<CPU_CPU1_ENABLE1_SPEC>`"]
pub type CPU_CPU1_ENABLE1 = crate::Reg<cpu_cpu1_enable1::CPU_CPU1_ENABLE1_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_enable1;
#[doc = "CPU_CPU1_ENABLE2 (rw) register accessor: an alias for `Reg<CPU_CPU1_ENABLE2_SPEC>`"]
pub type CPU_CPU1_ENABLE2 = crate::Reg<cpu_cpu1_enable2::CPU_CPU1_ENABLE2_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_enable2;
#[doc = "CPU_CPU1_ENABLE3 (rw) register accessor: an alias for `Reg<CPU_CPU1_ENABLE3_SPEC>`"]
pub type CPU_CPU1_ENABLE3 = crate::Reg<cpu_cpu1_enable3::CPU_CPU1_ENABLE3_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_enable3;
#[doc = "CPU_CPU1_ENABLE4 (rw) register accessor: an alias for `Reg<CPU_CPU1_ENABLE4_SPEC>`"]
pub type CPU_CPU1_ENABLE4 = crate::Reg<cpu_cpu1_enable4::CPU_CPU1_ENABLE4_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_enable4;
#[doc = "CPU_CPU1_ENABLE5 (rw) register accessor: an alias for `Reg<CPU_CPU1_ENABLE5_SPEC>`"]
pub type CPU_CPU1_ENABLE5 = crate::Reg<cpu_cpu1_enable5::CPU_CPU1_ENABLE5_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_enable5;
#[doc = "CPU_CPU1_ENABLE6 (rw) register accessor: an alias for `Reg<CPU_CPU1_ENABLE6_SPEC>`"]
pub type CPU_CPU1_ENABLE6 = crate::Reg<cpu_cpu1_enable6::CPU_CPU1_ENABLE6_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_enable6;
#[doc = "CPU_CPU1_ENABLE7 (rw) register accessor: an alias for `Reg<CPU_CPU1_ENABLE7_SPEC>`"]
pub type CPU_CPU1_ENABLE7 = crate::Reg<cpu_cpu1_enable7::CPU_CPU1_ENABLE7_SPEC>;
#[doc = "No description avaiable"]
pub mod cpu_cpu1_enable7;
