#[doc = "Register `SYS_CTRL` reader"]
pub struct R(crate::R<SYS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_CTRL` writer"]
pub struct W(crate::W<SYS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SYS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERNAL_CLK_EN` reader - Internal Clock Enable This bit is set to 0 when the Host Driver is not using the Host Controller or the Host Controller awaits a wakeup interrupt. The Host Controller must stop its internal clock to enter a very low power state. However, registers can still be read and written to. The value is reflected on the intclk_en signal. Note: If this bit is not used to control the internal clock (base clock and master clock), it is recommended to set this bit to '1' . Values: 0x0 (FALSE): Stop 0x1 (TRUE): Oscillate"]
pub type INTERNAL_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `INTERNAL_CLK_EN` writer - Internal Clock Enable This bit is set to 0 when the Host Driver is not using the Host Controller or the Host Controller awaits a wakeup interrupt. The Host Controller must stop its internal clock to enter a very low power state. However, registers can still be read and written to. The value is reflected on the intclk_en signal. Note: If this bit is not used to control the internal clock (base clock and master clock), it is recommended to set this bit to '1' . Values: 0x0 (FALSE): Stop 0x1 (TRUE): Oscillate"]
pub type INTERNAL_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CTRL_SPEC, bool, O>;
#[doc = "Field `INTERNAL_CLK_STABLE` reader - Internal Clock Stable This bit enables the Host Driver to check the clock stability twice after the Internal Clock Enable bit is set and after the PLL Enable bit is set. This bit reflects the synchronized value of the intclk_stable signal after the Internal Clock Enable bit is set to 1, and also reflects the synchronized value of the card_clk_stable signal after the PLL Enable bit is set to 1. Values: 0x0 (FALSE): Not Ready 0x1 (TRUE): Ready"]
pub type INTERNAL_CLK_STABLE_R = crate::BitReader<bool>;
#[doc = "Field `INTERNAL_CLK_STABLE` writer - Internal Clock Stable This bit enables the Host Driver to check the clock stability twice after the Internal Clock Enable bit is set and after the PLL Enable bit is set. This bit reflects the synchronized value of the intclk_stable signal after the Internal Clock Enable bit is set to 1, and also reflects the synchronized value of the card_clk_stable signal after the PLL Enable bit is set to 1. Values: 0x0 (FALSE): Not Ready 0x1 (TRUE): Ready"]
pub type INTERNAL_CLK_STABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CTRL_SPEC, bool, O>;
#[doc = "Field `SD_CLK_EN` reader - SD/eMMC Clock Enable This bit stops the SDCLK or RCLK when set to 0. The SDCLK/RCLK Frequency Select bit can be changed when this bit is set to 0. The value is reflected on the clk2card_on pin. Values: 0x0 (FALSE): Disable providing SDCLK/RCLK 0x1 (TRUE): Enable providing SDCLK/RCLK"]
pub type SD_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SD_CLK_EN` writer - SD/eMMC Clock Enable This bit stops the SDCLK or RCLK when set to 0. The SDCLK/RCLK Frequency Select bit can be changed when this bit is set to 0. The value is reflected on the clk2card_on pin. Values: 0x0 (FALSE): Disable providing SDCLK/RCLK 0x1 (TRUE): Enable providing SDCLK/RCLK"]
pub type SD_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CTRL_SPEC, bool, O>;
#[doc = "Field `PLL_ENABLE` reader - PLL Enable This bit is used to activate the PLL (applicable when Host Version 4 Enable = 1). When Host Version 4 Enable = 0, INTERNAL_CLK_EN bit may be used to activate PLL. The value is reflected on the card_clk_en signal. Note: If this bit is not used to to active the PLL when Host Version 4 Enable = 1, it is recommended to set this bit to '1' . Values: 0x0 (FALSE): PLL is in low power mode 0x1 (TRUE): PLL is enabled"]
pub type PLL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PLL_ENABLE` writer - PLL Enable This bit is used to activate the PLL (applicable when Host Version 4 Enable = 1). When Host Version 4 Enable = 0, INTERNAL_CLK_EN bit may be used to activate PLL. The value is reflected on the card_clk_en signal. Note: If this bit is not used to to active the PLL when Host Version 4 Enable = 1, it is recommended to set this bit to '1' . Values: 0x0 (FALSE): PLL is in low power mode 0x1 (TRUE): PLL is enabled"]
pub type PLL_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CTRL_SPEC, bool, O>;
#[doc = "Field `CLK_GEN_SELECT` reader - Clock Generator Select This bit is used to select the clock generator mode in SDCLK/RCLK Frequency Select. If Preset Value Enable = 0, this bit is set by the Host Driver. If Preset Value Enable = 1, this bit is automatically set to a value specified in one of the Preset Value registers. The value is reflected on the card_clk_gen_sel signal. Values: 0x0 (FALSE): Divided Clock Mode 0x1 (TRUE): Programmable Clock Mode"]
pub type CLK_GEN_SELECT_R = crate::BitReader<bool>;
#[doc = "Field `CLK_GEN_SELECT` writer - Clock Generator Select This bit is used to select the clock generator mode in SDCLK/RCLK Frequency Select. If Preset Value Enable = 0, this bit is set by the Host Driver. If Preset Value Enable = 1, this bit is automatically set to a value specified in one of the Preset Value registers. The value is reflected on the card_clk_gen_sel signal. Values: 0x0 (FALSE): Divided Clock Mode 0x1 (TRUE): Programmable Clock Mode"]
pub type CLK_GEN_SELECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CTRL_SPEC, bool, O>;
#[doc = "Field `UPPER_FREQ_SEL` reader - These bits specify the upper 2 bits of 10-bit SDCLK/RCLK Frequency Select control. The value is reflected on the upper 2 bits of the card_clk_freq_sel signal."]
pub type UPPER_FREQ_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UPPER_FREQ_SEL` writer - These bits specify the upper 2 bits of 10-bit SDCLK/RCLK Frequency Select control. The value is reflected on the upper 2 bits of the card_clk_freq_sel signal."]
pub type UPPER_FREQ_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYS_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `FREQ_SEL` reader - SDCLK/RCLK Frequency Select These bits are used to select the frequency of the SDCLK signal. These bits depend on setting of Preset Value Enable in the Host Control 2 register. If Preset Value Enable = 0, these bits are set by the Host Driver. If Preset Value Enable = 1, these bits are automatically set to a value specified in one of the Preset Value register. The value is reflected on the lower 8-bit of the card_clk_freq_selsignal. 10-bit Divided Clock Mode: 0x3FF : 1/2046 Divided clock .......... N : 1/2N Divided Clock .......... 0x002 : 1/4 Divided Clock 0x001 : 1/2 Divided Clock 0x000 : Base clock (10MHz - 255 MHz) Programmable Clock Mode : Enables the Host System to select a fine grain SD clock frequency: 0x3FF : Base clock * M /1024 .......... N-1 : Base clock * M /N .......... 0x002 : Base clock * M /3 0x001 : Base clock * M /2 0x000 : Base clock * M"]
pub type FREQ_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FREQ_SEL` writer - SDCLK/RCLK Frequency Select These bits are used to select the frequency of the SDCLK signal. These bits depend on setting of Preset Value Enable in the Host Control 2 register. If Preset Value Enable = 0, these bits are set by the Host Driver. If Preset Value Enable = 1, these bits are automatically set to a value specified in one of the Preset Value register. The value is reflected on the lower 8-bit of the card_clk_freq_selsignal. 10-bit Divided Clock Mode: 0x3FF : 1/2046 Divided clock .......... N : 1/2N Divided Clock .......... 0x002 : 1/4 Divided Clock 0x001 : 1/2 Divided Clock 0x000 : Base clock (10MHz - 255 MHz) Programmable Clock Mode : Enables the Host System to select a fine grain SD clock frequency: 0x3FF : Base clock * M /1024 .......... N-1 : Base clock * M /N .......... 0x002 : Base clock * M /3 0x001 : Base clock * M /2 0x000 : Base clock * M"]
pub type FREQ_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYS_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `TOUT_CNT` reader - Data Timeout Counter Value. This value determines the interval by which DAT line timeouts are detected. The Timeout clock frequency is generated by dividing the base clock TMCLK value by this value. When setting this register, prevent inadvertent timeout events by clearing the Data Timeout Error Status Enable (in the Error Interrupt Status Enable register). The values for these bits are: 0xF : Reserved 0xE : TMCLK x 2^27 ......... 0x1 : TMCLK x 2^14 0x0 : TMCLK x 2^13 Note: During a boot operating in an eMMC mode, an application must configure the boot data timeout value (approximately 1 sec) in this bit."]
pub type TOUT_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOUT_CNT` writer - Data Timeout Counter Value. This value determines the interval by which DAT line timeouts are detected. The Timeout clock frequency is generated by dividing the base clock TMCLK value by this value. When setting this register, prevent inadvertent timeout events by clearing the Data Timeout Error Status Enable (in the Error Interrupt Status Enable register). The values for these bits are: 0xF : Reserved 0xE : TMCLK x 2^27 ......... 0x1 : TMCLK x 2^14 0x0 : TMCLK x 2^13 Note: During a boot operating in an eMMC mode, an application must configure the boot data timeout value (approximately 1 sec) in this bit."]
pub type TOUT_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYS_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SW_RST_ALL` reader - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: 0x0 (FALSE): Work 0x1 (TRUE): Reset"]
pub type SW_RST_ALL_R = crate::BitReader<bool>;
#[doc = "Field `SW_RST_ALL` writer - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: 0x0 (FALSE): Work 0x1 (TRUE): Reset"]
pub type SW_RST_ALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CTRL_SPEC, bool, O>;
#[doc = "Field `SW_RST_CMD` reader - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. It bit is also used to initialize a UHS-II command circuit. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: Present State register : Command Inhibit (CMD) bit Normal Interrupt Status register : Command Complete bit Error Interrupt Status : Response error statuses related to Command Inhibit (CMD) bit Values: 0x0 (FALSE): Work 0x1 (TRUE): Reset"]
pub type SW_RST_CMD_R = crate::BitReader<bool>;
#[doc = "Field `SW_RST_CMD` writer - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. It bit is also used to initialize a UHS-II command circuit. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: Present State register : Command Inhibit (CMD) bit Normal Interrupt Status register : Command Complete bit Error Interrupt Status : Response error statuses related to Command Inhibit (CMD) bit Values: 0x0 (FALSE): Work 0x1 (TRUE): Reset"]
pub type SW_RST_CMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CTRL_SPEC, bool, O>;
#[doc = "Field `SW_RST_DAT` reader - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: Buffer Data Port register -Buffer is cleared and initialized. Present state register -Buffer Read Enable -Buffer Write Enable -Read Transfer Active -Write Transfer Active -DAT Line Active -Command Inhibit (DAT) Block Gap Control register -Continue Request -Stop At Block Gap Request Normal Interrupt status register -Buffer Read Ready -Buffer Write Ready -DMA Interrupt -Block Gap Event -Transfer Complete In UHS-II mode, this bit shall be set to 0 Values: 0x0 (FALSE): Work 0x1 (TRUE): Reset"]
pub type SW_RST_DAT_R = crate::BitReader<bool>;
#[doc = "Field `SW_RST_DAT` writer - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: Buffer Data Port register -Buffer is cleared and initialized. Present state register -Buffer Read Enable -Buffer Write Enable -Read Transfer Active -Write Transfer Active -DAT Line Active -Command Inhibit (DAT) Block Gap Control register -Continue Request -Stop At Block Gap Request Normal Interrupt status register -Buffer Read Ready -Buffer Write Ready -DMA Interrupt -Block Gap Event -Transfer Complete In UHS-II mode, this bit shall be set to 0 Values: 0x0 (FALSE): Work 0x1 (TRUE): Reset"]
pub type SW_RST_DAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Internal Clock Enable This bit is set to 0 when the Host Driver is not using the Host Controller or the Host Controller awaits a wakeup interrupt. The Host Controller must stop its internal clock to enter a very low power state. However, registers can still be read and written to. The value is reflected on the intclk_en signal. Note: If this bit is not used to control the internal clock (base clock and master clock), it is recommended to set this bit to '1' . Values: 0x0 (FALSE): Stop 0x1 (TRUE): Oscillate"]
    #[inline(always)]
    pub fn internal_clk_en(&self) -> INTERNAL_CLK_EN_R {
        INTERNAL_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable This bit enables the Host Driver to check the clock stability twice after the Internal Clock Enable bit is set and after the PLL Enable bit is set. This bit reflects the synchronized value of the intclk_stable signal after the Internal Clock Enable bit is set to 1, and also reflects the synchronized value of the card_clk_stable signal after the PLL Enable bit is set to 1. Values: 0x0 (FALSE): Not Ready 0x1 (TRUE): Ready"]
    #[inline(always)]
    pub fn internal_clk_stable(&self) -> INTERNAL_CLK_STABLE_R {
        INTERNAL_CLK_STABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD/eMMC Clock Enable This bit stops the SDCLK or RCLK when set to 0. The SDCLK/RCLK Frequency Select bit can be changed when this bit is set to 0. The value is reflected on the clk2card_on pin. Values: 0x0 (FALSE): Disable providing SDCLK/RCLK 0x1 (TRUE): Enable providing SDCLK/RCLK"]
    #[inline(always)]
    pub fn sd_clk_en(&self) -> SD_CLK_EN_R {
        SD_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PLL Enable This bit is used to activate the PLL (applicable when Host Version 4 Enable = 1). When Host Version 4 Enable = 0, INTERNAL_CLK_EN bit may be used to activate PLL. The value is reflected on the card_clk_en signal. Note: If this bit is not used to to active the PLL when Host Version 4 Enable = 1, it is recommended to set this bit to '1' . Values: 0x0 (FALSE): PLL is in low power mode 0x1 (TRUE): PLL is enabled"]
    #[inline(always)]
    pub fn pll_enable(&self) -> PLL_ENABLE_R {
        PLL_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select This bit is used to select the clock generator mode in SDCLK/RCLK Frequency Select. If Preset Value Enable = 0, this bit is set by the Host Driver. If Preset Value Enable = 1, this bit is automatically set to a value specified in one of the Preset Value registers. The value is reflected on the card_clk_gen_sel signal. Values: 0x0 (FALSE): Divided Clock Mode 0x1 (TRUE): Programmable Clock Mode"]
    #[inline(always)]
    pub fn clk_gen_select(&self) -> CLK_GEN_SELECT_R {
        CLK_GEN_SELECT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - These bits specify the upper 2 bits of 10-bit SDCLK/RCLK Frequency Select control. The value is reflected on the upper 2 bits of the card_clk_freq_sel signal."]
    #[inline(always)]
    pub fn upper_freq_sel(&self) -> UPPER_FREQ_SEL_R {
        UPPER_FREQ_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK/RCLK Frequency Select These bits are used to select the frequency of the SDCLK signal. These bits depend on setting of Preset Value Enable in the Host Control 2 register. If Preset Value Enable = 0, these bits are set by the Host Driver. If Preset Value Enable = 1, these bits are automatically set to a value specified in one of the Preset Value register. The value is reflected on the lower 8-bit of the card_clk_freq_selsignal. 10-bit Divided Clock Mode: 0x3FF : 1/2046 Divided clock .......... N : 1/2N Divided Clock .......... 0x002 : 1/4 Divided Clock 0x001 : 1/2 Divided Clock 0x000 : Base clock (10MHz - 255 MHz) Programmable Clock Mode : Enables the Host System to select a fine grain SD clock frequency: 0x3FF : Base clock * M /1024 .......... N-1 : Base clock * M /N .......... 0x002 : Base clock * M /3 0x001 : Base clock * M /2 0x000 : Base clock * M"]
    #[inline(always)]
    pub fn freq_sel(&self) -> FREQ_SEL_R {
        FREQ_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value. This value determines the interval by which DAT line timeouts are detected. The Timeout clock frequency is generated by dividing the base clock TMCLK value by this value. When setting this register, prevent inadvertent timeout events by clearing the Data Timeout Error Status Enable (in the Error Interrupt Status Enable register). The values for these bits are: 0xF : Reserved 0xE : TMCLK x 2^27 ......... 0x1 : TMCLK x 2^14 0x0 : TMCLK x 2^13 Note: During a boot operating in an eMMC mode, an application must configure the boot data timeout value (approximately 1 sec) in this bit."]
    #[inline(always)]
    pub fn tout_cnt(&self) -> TOUT_CNT_R {
        TOUT_CNT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: 0x0 (FALSE): Work 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_all(&self) -> SW_RST_ALL_R {
        SW_RST_ALL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. It bit is also used to initialize a UHS-II command circuit. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: Present State register : Command Inhibit (CMD) bit Normal Interrupt Status register : Command Complete bit Error Interrupt Status : Response error statuses related to Command Inhibit (CMD) bit Values: 0x0 (FALSE): Work 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_cmd(&self) -> SW_RST_CMD_R {
        SW_RST_CMD_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: Buffer Data Port register -Buffer is cleared and initialized. Present state register -Buffer Read Enable -Buffer Write Enable -Read Transfer Active -Write Transfer Active -DAT Line Active -Command Inhibit (DAT) Block Gap Control register -Continue Request -Stop At Block Gap Request Normal Interrupt status register -Buffer Read Ready -Buffer Write Ready -DMA Interrupt -Block Gap Event -Transfer Complete In UHS-II mode, this bit shall be set to 0 Values: 0x0 (FALSE): Work 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_dat(&self) -> SW_RST_DAT_R {
        SW_RST_DAT_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Clock Enable This bit is set to 0 when the Host Driver is not using the Host Controller or the Host Controller awaits a wakeup interrupt. The Host Controller must stop its internal clock to enter a very low power state. However, registers can still be read and written to. The value is reflected on the intclk_en signal. Note: If this bit is not used to control the internal clock (base clock and master clock), it is recommended to set this bit to '1' . Values: 0x0 (FALSE): Stop 0x1 (TRUE): Oscillate"]
    #[inline(always)]
    #[must_use]
    pub fn internal_clk_en(&mut self) -> INTERNAL_CLK_EN_W<0> {
        INTERNAL_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Internal Clock Stable This bit enables the Host Driver to check the clock stability twice after the Internal Clock Enable bit is set and after the PLL Enable bit is set. This bit reflects the synchronized value of the intclk_stable signal after the Internal Clock Enable bit is set to 1, and also reflects the synchronized value of the card_clk_stable signal after the PLL Enable bit is set to 1. Values: 0x0 (FALSE): Not Ready 0x1 (TRUE): Ready"]
    #[inline(always)]
    #[must_use]
    pub fn internal_clk_stable(&mut self) -> INTERNAL_CLK_STABLE_W<1> {
        INTERNAL_CLK_STABLE_W::new(self)
    }
    #[doc = "Bit 2 - SD/eMMC Clock Enable This bit stops the SDCLK or RCLK when set to 0. The SDCLK/RCLK Frequency Select bit can be changed when this bit is set to 0. The value is reflected on the clk2card_on pin. Values: 0x0 (FALSE): Disable providing SDCLK/RCLK 0x1 (TRUE): Enable providing SDCLK/RCLK"]
    #[inline(always)]
    #[must_use]
    pub fn sd_clk_en(&mut self) -> SD_CLK_EN_W<2> {
        SD_CLK_EN_W::new(self)
    }
    #[doc = "Bit 3 - PLL Enable This bit is used to activate the PLL (applicable when Host Version 4 Enable = 1). When Host Version 4 Enable = 0, INTERNAL_CLK_EN bit may be used to activate PLL. The value is reflected on the card_clk_en signal. Note: If this bit is not used to to active the PLL when Host Version 4 Enable = 1, it is recommended to set this bit to '1' . Values: 0x0 (FALSE): PLL is in low power mode 0x1 (TRUE): PLL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn pll_enable(&mut self) -> PLL_ENABLE_W<3> {
        PLL_ENABLE_W::new(self)
    }
    #[doc = "Bit 5 - Clock Generator Select This bit is used to select the clock generator mode in SDCLK/RCLK Frequency Select. If Preset Value Enable = 0, this bit is set by the Host Driver. If Preset Value Enable = 1, this bit is automatically set to a value specified in one of the Preset Value registers. The value is reflected on the card_clk_gen_sel signal. Values: 0x0 (FALSE): Divided Clock Mode 0x1 (TRUE): Programmable Clock Mode"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gen_select(&mut self) -> CLK_GEN_SELECT_W<5> {
        CLK_GEN_SELECT_W::new(self)
    }
    #[doc = "Bits 6:7 - These bits specify the upper 2 bits of 10-bit SDCLK/RCLK Frequency Select control. The value is reflected on the upper 2 bits of the card_clk_freq_sel signal."]
    #[inline(always)]
    #[must_use]
    pub fn upper_freq_sel(&mut self) -> UPPER_FREQ_SEL_W<6> {
        UPPER_FREQ_SEL_W::new(self)
    }
    #[doc = "Bits 8:15 - SDCLK/RCLK Frequency Select These bits are used to select the frequency of the SDCLK signal. These bits depend on setting of Preset Value Enable in the Host Control 2 register. If Preset Value Enable = 0, these bits are set by the Host Driver. If Preset Value Enable = 1, these bits are automatically set to a value specified in one of the Preset Value register. The value is reflected on the lower 8-bit of the card_clk_freq_selsignal. 10-bit Divided Clock Mode: 0x3FF : 1/2046 Divided clock .......... N : 1/2N Divided Clock .......... 0x002 : 1/4 Divided Clock 0x001 : 1/2 Divided Clock 0x000 : Base clock (10MHz - 255 MHz) Programmable Clock Mode : Enables the Host System to select a fine grain SD clock frequency: 0x3FF : Base clock * M /1024 .......... N-1 : Base clock * M /N .......... 0x002 : Base clock * M /3 0x001 : Base clock * M /2 0x000 : Base clock * M"]
    #[inline(always)]
    #[must_use]
    pub fn freq_sel(&mut self) -> FREQ_SEL_W<8> {
        FREQ_SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value. This value determines the interval by which DAT line timeouts are detected. The Timeout clock frequency is generated by dividing the base clock TMCLK value by this value. When setting this register, prevent inadvertent timeout events by clearing the Data Timeout Error Status Enable (in the Error Interrupt Status Enable register). The values for these bits are: 0xF : Reserved 0xE : TMCLK x 2^27 ......... 0x1 : TMCLK x 2^14 0x0 : TMCLK x 2^13 Note: During a boot operating in an eMMC mode, an application must configure the boot data timeout value (approximately 1 sec) in this bit."]
    #[inline(always)]
    #[must_use]
    pub fn tout_cnt(&mut self) -> TOUT_CNT_W<16> {
        TOUT_CNT_W::new(self)
    }
    #[doc = "Bit 24 - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: 0x0 (FALSE): Work 0x1 (TRUE): Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_all(&mut self) -> SW_RST_ALL_W<24> {
        SW_RST_ALL_W::new(self)
    }
    #[doc = "Bit 25 - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. It bit is also used to initialize a UHS-II command circuit. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: Present State register : Command Inhibit (CMD) bit Normal Interrupt Status register : Command Complete bit Error Interrupt Status : Response error statuses related to Command Inhibit (CMD) bit Values: 0x0 (FALSE): Work 0x1 (TRUE): Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_cmd(&mut self) -> SW_RST_CMD_W<25> {
        SW_RST_CMD_W::new(self)
    }
    #[doc = "Bit 26 - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: Buffer Data Port register -Buffer is cleared and initialized. Present state register -Buffer Read Enable -Buffer Write Enable -Read Transfer Active -Write Transfer Active -DAT Line Active -Command Inhibit (DAT) Block Gap Control register -Continue Request -Stop At Block Gap Request Normal Interrupt status register -Buffer Read Ready -Buffer Write Ready -DMA Interrupt -Block Gap Event -Transfer Complete In UHS-II mode, this bit shall be set to 0 Values: 0x0 (FALSE): Work 0x1 (TRUE): Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_dat(&mut self) -> SW_RST_DAT_W<26> {
        SW_RST_DAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_ctrl](index.html) module"]
pub struct SYS_CTRL_SPEC;
impl crate::RegisterSpec for SYS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_ctrl::R](R) reader structure"]
impl crate::Readable for SYS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_ctrl::W](W) writer structure"]
impl crate::Writable for SYS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYS_CTRL to value 0"]
impl crate::Resettable for SYS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
