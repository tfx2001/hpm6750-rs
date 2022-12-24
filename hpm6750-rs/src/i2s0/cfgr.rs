#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSIZ` reader - Channel length (number of bits per audio channel) 0: 16-bit wide 1: 32-bit wide The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled."]
pub type CHSIZ_R = crate::BitReader<bool>;
#[doc = "Field `CHSIZ` writer - Channel length (number of bits per audio channel) 0: 16-bit wide 1: 32-bit wide The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled."]
pub type CHSIZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `DATSIZ` reader - Data length to be transferred 00: 16-bit data length 01: 24-bit data length 10: 32-bit data length 11: Not allowed Note: For correct operation, these bits should be configured when the I2S is disabled."]
pub type DATSIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATSIZ` writer - Data length to be transferred 00: 16-bit data length 01: 24-bit data length 10: 32-bit data length 11: Not allowed Note: For correct operation, these bits should be configured when the I2S is disabled."]
pub type DATSIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `STD` reader - I2S standard selection 00: I2S Philips standard. 01: MSB justified standard (left justified) 10: LSB justified standard (right justified) 11: PCM standard Note: For correct operation, these bits should be configured when the I2S is disabled."]
pub type STD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STD` writer - I2S standard selection 00: I2S Philips standard. 01: MSB justified standard (left justified) 10: LSB justified standard (right justified) 11: PCM standard Note: For correct operation, these bits should be configured when the I2S is disabled."]
pub type STD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TDM_EN` reader - TDM mode 0: not TDM mode 1: TDM mode"]
pub type TDM_EN_R = crate::BitReader<bool>;
#[doc = "Field `TDM_EN` writer - TDM mode 0: not TDM mode 1: TDM mode"]
pub type TDM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `CH_MAX` reader - CH_MAX\\[3:0\\]
s the number of channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 4'h2: 2 channels 4'h4: 4 channels ..."]
pub type CH_MAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH_MAX` writer - CH_MAX\\[3:0\\]
s the number of channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 4'h2: 2 channels 4'h4: 4 channels ..."]
pub type CH_MAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 5, O>;
#[doc = "Field `FRAME_EDGE` reader - The start edge of a frame 0: Falling edge indicates a new frame (Just like standard I2S Philips standard) 1: Rising edge indicates a new frame"]
pub type FRAME_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_EDGE` writer - The start edge of a frame 0: Falling edge indicates a new frame (Just like standard I2S Philips standard) 1: Rising edge indicates a new frame"]
pub type FRAME_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `MCK_SEL_OP` reader - asserted to use external clk source"]
pub type MCK_SEL_OP_R = crate::BitReader<bool>;
#[doc = "Field `MCK_SEL_OP` writer - asserted to use external clk source"]
pub type MCK_SEL_OP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `FCLK_SEL_OP` reader - asserted to use external clk source"]
pub type FCLK_SEL_OP_R = crate::BitReader<bool>;
#[doc = "Field `FCLK_SEL_OP` writer - asserted to use external clk source"]
pub type FCLK_SEL_OP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `BCLK_SEL_OP` reader - asserted to use external clk source"]
pub type BCLK_SEL_OP_R = crate::BitReader<bool>;
#[doc = "Field `BCLK_SEL_OP` writer - asserted to use external clk source"]
pub type BCLK_SEL_OP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `INV_MCLK_IN` reader - Invert the MCLK pad input before using it internally. Only valid in MCLK slave mode"]
pub type INV_MCLK_IN_R = crate::BitReader<bool>;
#[doc = "Field `INV_MCLK_IN` writer - Invert the MCLK pad input before using it internally. Only valid in MCLK slave mode"]
pub type INV_MCLK_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `INV_MCLK_OUT` reader - Invert the MCLK before sending it out to pad. Only valid in MCLK master mode"]
pub type INV_MCLK_OUT_R = crate::BitReader<bool>;
#[doc = "Field `INV_MCLK_OUT` writer - Invert the MCLK before sending it out to pad. Only valid in MCLK master mode"]
pub type INV_MCLK_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `INV_FCLK_IN` reader - Invert the FCLK pad input before using it internally. Only valid in FCLK slave mode"]
pub type INV_FCLK_IN_R = crate::BitReader<bool>;
#[doc = "Field `INV_FCLK_IN` writer - Invert the FCLK pad input before using it internally. Only valid in FCLK slave mode"]
pub type INV_FCLK_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `INV_FCLK_OUT` reader - Invert the FCLK before sending it out to pad. Only valid in FCLK master mode"]
pub type INV_FCLK_OUT_R = crate::BitReader<bool>;
#[doc = "Field `INV_FCLK_OUT` writer - Invert the FCLK before sending it out to pad. Only valid in FCLK master mode"]
pub type INV_FCLK_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `INV_BCLK_IN` reader - Invert the BCLK pad input before using it internally. Only valid in BCLK slave mode"]
pub type INV_BCLK_IN_R = crate::BitReader<bool>;
#[doc = "Field `INV_BCLK_IN` writer - Invert the BCLK pad input before using it internally. Only valid in BCLK slave mode"]
pub type INV_BCLK_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `INV_BCLK_OUT` reader - Invert the BCLK before sending it out to pad. Only valid in BCLK master mode"]
pub type INV_BCLK_OUT_R = crate::BitReader<bool>;
#[doc = "Field `INV_BCLK_OUT` writer - Invert the BCLK before sending it out to pad. Only valid in BCLK master mode"]
pub type INV_BCLK_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `BCLK_DIV` reader - Linear prescaler to generate BCLK from MCLK. BCLK_DIV \\[8:0\\]
= 0: BCLK=No CLK. BCLK_DIV \\[8:0\\]
= 1: BCLK=MCLK/1 BCLK_DIV \\[8:0\\]
= n: BCLK=MCLK/(n). Note: These bits should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
pub type BCLK_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BCLK_DIV` writer - Linear prescaler to generate BCLK from MCLK. BCLK_DIV \\[8:0\\]
= 0: BCLK=No CLK. BCLK_DIV \\[8:0\\]
= 1: BCLK=MCLK/1 BCLK_DIV \\[8:0\\]
= n: BCLK=MCLK/(n). Note: These bits should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
pub type BCLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u16, u16, 9, O>;
#[doc = "Field `BCLK_GATEOFF` reader - Gate off the bclk. Asserted to gate-off the BCLK."]
pub type BCLK_GATEOFF_R = crate::BitReader<bool>;
#[doc = "Field `BCLK_GATEOFF` writer - Gate off the bclk. Asserted to gate-off the BCLK."]
pub type BCLK_GATEOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel) 0: 16-bit wide 1: 32-bit wide The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled."]
    #[inline(always)]
    pub fn chsiz(&self) -> CHSIZ_R {
        CHSIZ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data length to be transferred 00: 16-bit data length 01: 24-bit data length 10: 32-bit data length 11: Not allowed Note: For correct operation, these bits should be configured when the I2S is disabled."]
    #[inline(always)]
    pub fn datsiz(&self) -> DATSIZ_R {
        DATSIZ_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - I2S standard selection 00: I2S Philips standard. 01: MSB justified standard (left justified) 10: LSB justified standard (right justified) 11: PCM standard Note: For correct operation, these bits should be configured when the I2S is disabled."]
    #[inline(always)]
    pub fn std(&self) -> STD_R {
        STD_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - TDM mode 0: not TDM mode 1: TDM mode"]
    #[inline(always)]
    pub fn tdm_en(&self) -> TDM_EN_R {
        TDM_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - CH_MAX\\[3:0\\]
s the number of channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 4'h2: 2 channels 4'h4: 4 channels ..."]
    #[inline(always)]
    pub fn ch_max(&self) -> CH_MAX_R {
        CH_MAX_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - The start edge of a frame 0: Falling edge indicates a new frame (Just like standard I2S Philips standard) 1: Rising edge indicates a new frame"]
    #[inline(always)]
    pub fn frame_edge(&self) -> FRAME_EDGE_R {
        FRAME_EDGE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - asserted to use external clk source"]
    #[inline(always)]
    pub fn mck_sel_op(&self) -> MCK_SEL_OP_R {
        MCK_SEL_OP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - asserted to use external clk source"]
    #[inline(always)]
    pub fn fclk_sel_op(&self) -> FCLK_SEL_OP_R {
        FCLK_SEL_OP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - asserted to use external clk source"]
    #[inline(always)]
    pub fn bclk_sel_op(&self) -> BCLK_SEL_OP_R {
        BCLK_SEL_OP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Invert the MCLK pad input before using it internally. Only valid in MCLK slave mode"]
    #[inline(always)]
    pub fn inv_mclk_in(&self) -> INV_MCLK_IN_R {
        INV_MCLK_IN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Invert the MCLK before sending it out to pad. Only valid in MCLK master mode"]
    #[inline(always)]
    pub fn inv_mclk_out(&self) -> INV_MCLK_OUT_R {
        INV_MCLK_OUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Invert the FCLK pad input before using it internally. Only valid in FCLK slave mode"]
    #[inline(always)]
    pub fn inv_fclk_in(&self) -> INV_FCLK_IN_R {
        INV_FCLK_IN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Invert the FCLK before sending it out to pad. Only valid in FCLK master mode"]
    #[inline(always)]
    pub fn inv_fclk_out(&self) -> INV_FCLK_OUT_R {
        INV_FCLK_OUT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Invert the BCLK pad input before using it internally. Only valid in BCLK slave mode"]
    #[inline(always)]
    pub fn inv_bclk_in(&self) -> INV_BCLK_IN_R {
        INV_BCLK_IN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Invert the BCLK before sending it out to pad. Only valid in BCLK master mode"]
    #[inline(always)]
    pub fn inv_bclk_out(&self) -> INV_BCLK_OUT_R {
        INV_BCLK_OUT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:29 - Linear prescaler to generate BCLK from MCLK. BCLK_DIV \\[8:0\\]
= 0: BCLK=No CLK. BCLK_DIV \\[8:0\\]
= 1: BCLK=MCLK/1 BCLK_DIV \\[8:0\\]
= n: BCLK=MCLK/(n). Note: These bits should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
    #[inline(always)]
    pub fn bclk_div(&self) -> BCLK_DIV_R {
        BCLK_DIV_R::new(((self.bits >> 21) & 0x01ff) as u16)
    }
    #[doc = "Bit 30 - Gate off the bclk. Asserted to gate-off the BCLK."]
    #[inline(always)]
    pub fn bclk_gateoff(&self) -> BCLK_GATEOFF_R {
        BCLK_GATEOFF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel) 0: 16-bit wide 1: 32-bit wide The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn chsiz(&mut self) -> CHSIZ_W<0> {
        CHSIZ_W::new(self)
    }
    #[doc = "Bits 1:2 - Data length to be transferred 00: 16-bit data length 01: 24-bit data length 10: 32-bit data length 11: Not allowed Note: For correct operation, these bits should be configured when the I2S is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn datsiz(&mut self) -> DATSIZ_W<1> {
        DATSIZ_W::new(self)
    }
    #[doc = "Bits 3:4 - I2S standard selection 00: I2S Philips standard. 01: MSB justified standard (left justified) 10: LSB justified standard (right justified) 11: PCM standard Note: For correct operation, these bits should be configured when the I2S is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn std(&mut self) -> STD_W<3> {
        STD_W::new(self)
    }
    #[doc = "Bit 5 - TDM mode 0: not TDM mode 1: TDM mode"]
    #[inline(always)]
    #[must_use]
    pub fn tdm_en(&mut self) -> TDM_EN_W<5> {
        TDM_EN_W::new(self)
    }
    #[doc = "Bits 6:10 - CH_MAX\\[3:0\\]
s the number of channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\]
is always 0. 4'h2: 2 channels 4'h4: 4 channels ..."]
    #[inline(always)]
    #[must_use]
    pub fn ch_max(&mut self) -> CH_MAX_W<6> {
        CH_MAX_W::new(self)
    }
    #[doc = "Bit 11 - The start edge of a frame 0: Falling edge indicates a new frame (Just like standard I2S Philips standard) 1: Rising edge indicates a new frame"]
    #[inline(always)]
    #[must_use]
    pub fn frame_edge(&mut self) -> FRAME_EDGE_W<11> {
        FRAME_EDGE_W::new(self)
    }
    #[doc = "Bit 12 - asserted to use external clk source"]
    #[inline(always)]
    #[must_use]
    pub fn mck_sel_op(&mut self) -> MCK_SEL_OP_W<12> {
        MCK_SEL_OP_W::new(self)
    }
    #[doc = "Bit 13 - asserted to use external clk source"]
    #[inline(always)]
    #[must_use]
    pub fn fclk_sel_op(&mut self) -> FCLK_SEL_OP_W<13> {
        FCLK_SEL_OP_W::new(self)
    }
    #[doc = "Bit 14 - asserted to use external clk source"]
    #[inline(always)]
    #[must_use]
    pub fn bclk_sel_op(&mut self) -> BCLK_SEL_OP_W<14> {
        BCLK_SEL_OP_W::new(self)
    }
    #[doc = "Bit 15 - Invert the MCLK pad input before using it internally. Only valid in MCLK slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn inv_mclk_in(&mut self) -> INV_MCLK_IN_W<15> {
        INV_MCLK_IN_W::new(self)
    }
    #[doc = "Bit 16 - Invert the MCLK before sending it out to pad. Only valid in MCLK master mode"]
    #[inline(always)]
    #[must_use]
    pub fn inv_mclk_out(&mut self) -> INV_MCLK_OUT_W<16> {
        INV_MCLK_OUT_W::new(self)
    }
    #[doc = "Bit 17 - Invert the FCLK pad input before using it internally. Only valid in FCLK slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn inv_fclk_in(&mut self) -> INV_FCLK_IN_W<17> {
        INV_FCLK_IN_W::new(self)
    }
    #[doc = "Bit 18 - Invert the FCLK before sending it out to pad. Only valid in FCLK master mode"]
    #[inline(always)]
    #[must_use]
    pub fn inv_fclk_out(&mut self) -> INV_FCLK_OUT_W<18> {
        INV_FCLK_OUT_W::new(self)
    }
    #[doc = "Bit 19 - Invert the BCLK pad input before using it internally. Only valid in BCLK slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn inv_bclk_in(&mut self) -> INV_BCLK_IN_W<19> {
        INV_BCLK_IN_W::new(self)
    }
    #[doc = "Bit 20 - Invert the BCLK before sending it out to pad. Only valid in BCLK master mode"]
    #[inline(always)]
    #[must_use]
    pub fn inv_bclk_out(&mut self) -> INV_BCLK_OUT_W<20> {
        INV_BCLK_OUT_W::new(self)
    }
    #[doc = "Bits 21:29 - Linear prescaler to generate BCLK from MCLK. BCLK_DIV \\[8:0\\]
= 0: BCLK=No CLK. BCLK_DIV \\[8:0\\]
= 1: BCLK=MCLK/1 BCLK_DIV \\[8:0\\]
= n: BCLK=MCLK/(n). Note: These bits should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
    #[inline(always)]
    #[must_use]
    pub fn bclk_div(&mut self) -> BCLK_DIV_W<21> {
        BCLK_DIV_W::new(self)
    }
    #[doc = "Bit 30 - Gate off the bclk. Asserted to gate-off the BCLK."]
    #[inline(always)]
    #[must_use]
    pub fn bclk_gateoff(&mut self) -> BCLK_GATEOFF_W<30> {
        BCLK_GATEOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configruation Regsiters\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR to value 0x4000_0000"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
