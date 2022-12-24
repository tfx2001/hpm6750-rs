#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHNUM` reader - the number of channels to be stored in buffer. Asserted to enable 2 channels."]
pub type CHNUM_R = crate::BitReader<bool>;
#[doc = "Field `CHNUM` writer - the number of channels to be stored in buffer. Asserted to enable 2 channels."]
pub type CHNUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CH_POL` reader - Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
pub type CH_POL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH_POL` writer - Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
pub type CH_POL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PDM_CLK_OE` reader - pdm_clk_output_en"]
pub type PDM_CLK_OE_R = crate::BitReader<bool>;
#[doc = "Field `PDM_CLK_OE` writer - pdm_clk_output_en"]
pub type PDM_CLK_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PDM_CLK_DIV_BYPASS` reader - asserted to bypass the pdm clock divider"]
pub type PDM_CLK_DIV_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `PDM_CLK_DIV_BYPASS` writer - asserted to bypass the pdm clock divider"]
pub type PDM_CLK_DIV_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FIFO_THRSH` reader - OFIFO threshold to generate ofifo_av (when fillings >= threshold) (fifo size: max 16 items, 16*32bits)"]
pub type FIFO_THRSH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFO_THRSH` writer - OFIFO threshold to generate ofifo_av (when fillings >= threshold) (fifo size: max 16 items, 16*32bits)"]
pub type FIFO_THRSH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `MEMBUF_DISABLE` reader - asserted to disable membuf"]
pub type MEMBUF_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `MEMBUF_DISABLE` writer - asserted to disable membuf"]
pub type MEMBUF_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CIC_SAT_ERR_IE` reader - CIC saturation Interrupt Enable"]
pub type CIC_SAT_ERR_IE_R = crate::BitReader<bool>;
#[doc = "Field `CIC_SAT_ERR_IE` writer - CIC saturation Interrupt Enable"]
pub type CIC_SAT_ERR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CIC_OVLD_ERR_IE` reader - CIC overload Interrupt Enable"]
pub type CIC_OVLD_ERR_IE_R = crate::BitReader<bool>;
#[doc = "Field `CIC_OVLD_ERR_IE` writer - CIC overload Interrupt Enable"]
pub type CIC_OVLD_ERR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `IIR_OVFL_ERR_IE` reader - IIR overflow error interrupt enable"]
pub type IIR_OVFL_ERR_IE_R = crate::BitReader<bool>;
#[doc = "Field `IIR_OVFL_ERR_IE` writer - IIR overflow error interrupt enable"]
pub type IIR_OVFL_ERR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `IIR_OVLD_ERR_IE` reader - IIR overload error interrupt enable"]
pub type IIR_OVLD_ERR_IE_R = crate::BitReader<bool>;
#[doc = "Field `IIR_OVLD_ERR_IE` writer - IIR overload error interrupt enable"]
pub type IIR_OVLD_ERR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `OFIFO_OVFL_ERR_IE` reader - OFIFO overflow error interrupt enable"]
pub type OFIFO_OVFL_ERR_IE_R = crate::BitReader<bool>;
#[doc = "Field `OFIFO_OVFL_ERR_IE` writer - OFIFO overflow error interrupt enable"]
pub type OFIFO_OVFL_ERR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `MEMBUF_EMPTY_IE` reader - Buf empty interrupt enable"]
pub type MEMBUF_EMPTY_IE_R = crate::BitReader<bool>;
#[doc = "Field `MEMBUF_EMPTY_IE` writer - Buf empty interrupt enable"]
pub type MEMBUF_EMPTY_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `OFIFO_AV_IE` reader - OFIFO data available interrupt enable"]
pub type OFIFO_AV_IE_R = crate::BitReader<bool>;
#[doc = "Field `OFIFO_AV_IE` writer - OFIFO data available interrupt enable"]
pub type OFIFO_AV_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `VAD_IE` reader - VAD event interrupt enable"]
pub type VAD_IE_R = crate::BitReader<bool>;
#[doc = "Field `VAD_IE` writer - VAD event interrupt enable"]
pub type VAD_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PDM_CLK_HFDIV` reader - The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)"]
pub type PDM_CLK_HFDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDM_CLK_HFDIV` writer - The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)"]
pub type PDM_CLK_HFDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CAPT_DLY` reader - Capture cycle delay>=0, should be less than PDM_CLK_HFDIV"]
pub type CAPT_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAPT_DLY` writer - Capture cycle delay>=0, should be less than PDM_CLK_HFDIV"]
pub type CAPT_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - the number of channels to be stored in buffer. Asserted to enable 2 channels."]
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
    #[inline(always)]
    pub fn ch_pol(&self) -> CH_POL_R {
        CH_POL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - pdm_clk_output_en"]
    #[inline(always)]
    pub fn pdm_clk_oe(&self) -> PDM_CLK_OE_R {
        PDM_CLK_OE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - asserted to bypass the pdm clock divider"]
    #[inline(always)]
    pub fn pdm_clk_div_bypass(&self) -> PDM_CLK_DIV_BYPASS_R {
        PDM_CLK_DIV_BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - OFIFO threshold to generate ofifo_av (when fillings >= threshold) (fifo size: max 16 items, 16*32bits)"]
    #[inline(always)]
    pub fn fifo_thrsh(&self) -> FIFO_THRSH_R {
        FIFO_THRSH_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - asserted to disable membuf"]
    #[inline(always)]
    pub fn membuf_disable(&self) -> MEMBUF_DISABLE_R {
        MEMBUF_DISABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - CIC saturation Interrupt Enable"]
    #[inline(always)]
    pub fn cic_sat_err_ie(&self) -> CIC_SAT_ERR_IE_R {
        CIC_SAT_ERR_IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CIC overload Interrupt Enable"]
    #[inline(always)]
    pub fn cic_ovld_err_ie(&self) -> CIC_OVLD_ERR_IE_R {
        CIC_OVLD_ERR_IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IIR overflow error interrupt enable"]
    #[inline(always)]
    pub fn iir_ovfl_err_ie(&self) -> IIR_OVFL_ERR_IE_R {
        IIR_OVFL_ERR_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - IIR overload error interrupt enable"]
    #[inline(always)]
    pub fn iir_ovld_err_ie(&self) -> IIR_OVLD_ERR_IE_R {
        IIR_OVLD_ERR_IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OFIFO overflow error interrupt enable"]
    #[inline(always)]
    pub fn ofifo_ovfl_err_ie(&self) -> OFIFO_OVFL_ERR_IE_R {
        OFIFO_OVFL_ERR_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Buf empty interrupt enable"]
    #[inline(always)]
    pub fn membuf_empty_ie(&self) -> MEMBUF_EMPTY_IE_R {
        MEMBUF_EMPTY_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OFIFO data available interrupt enable"]
    #[inline(always)]
    pub fn ofifo_av_ie(&self) -> OFIFO_AV_IE_R {
        OFIFO_AV_IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - VAD event interrupt enable"]
    #[inline(always)]
    pub fn vad_ie(&self) -> VAD_IE_R {
        VAD_IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)"]
    #[inline(always)]
    pub fn pdm_clk_hfdiv(&self) -> PDM_CLK_HFDIV_R {
        PDM_CLK_HFDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Capture cycle delay>=0, should be less than PDM_CLK_HFDIV"]
    #[inline(always)]
    pub fn capt_dly(&self) -> CAPT_DLY_R {
        CAPT_DLY_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - the number of channels to be stored in buffer. Asserted to enable 2 channels."]
    #[inline(always)]
    #[must_use]
    pub fn chnum(&mut self) -> CHNUM_W<0> {
        CHNUM_W::new(self)
    }
    #[doc = "Bits 1:2 - Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
    #[inline(always)]
    #[must_use]
    pub fn ch_pol(&mut self) -> CH_POL_W<1> {
        CH_POL_W::new(self)
    }
    #[doc = "Bit 3 - pdm_clk_output_en"]
    #[inline(always)]
    #[must_use]
    pub fn pdm_clk_oe(&mut self) -> PDM_CLK_OE_W<3> {
        PDM_CLK_OE_W::new(self)
    }
    #[doc = "Bit 4 - asserted to bypass the pdm clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn pdm_clk_div_bypass(&mut self) -> PDM_CLK_DIV_BYPASS_W<4> {
        PDM_CLK_DIV_BYPASS_W::new(self)
    }
    #[doc = "Bits 5:8 - OFIFO threshold to generate ofifo_av (when fillings >= threshold) (fifo size: max 16 items, 16*32bits)"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_thrsh(&mut self) -> FIFO_THRSH_W<5> {
        FIFO_THRSH_W::new(self)
    }
    #[doc = "Bit 9 - asserted to disable membuf"]
    #[inline(always)]
    #[must_use]
    pub fn membuf_disable(&mut self) -> MEMBUF_DISABLE_W<9> {
        MEMBUF_DISABLE_W::new(self)
    }
    #[doc = "Bit 11 - CIC saturation Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cic_sat_err_ie(&mut self) -> CIC_SAT_ERR_IE_W<11> {
        CIC_SAT_ERR_IE_W::new(self)
    }
    #[doc = "Bit 12 - CIC overload Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cic_ovld_err_ie(&mut self) -> CIC_OVLD_ERR_IE_W<12> {
        CIC_OVLD_ERR_IE_W::new(self)
    }
    #[doc = "Bit 13 - IIR overflow error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn iir_ovfl_err_ie(&mut self) -> IIR_OVFL_ERR_IE_W<13> {
        IIR_OVFL_ERR_IE_W::new(self)
    }
    #[doc = "Bit 14 - IIR overload error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn iir_ovld_err_ie(&mut self) -> IIR_OVLD_ERR_IE_W<14> {
        IIR_OVLD_ERR_IE_W::new(self)
    }
    #[doc = "Bit 15 - OFIFO overflow error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ofifo_ovfl_err_ie(&mut self) -> OFIFO_OVFL_ERR_IE_W<15> {
        OFIFO_OVFL_ERR_IE_W::new(self)
    }
    #[doc = "Bit 16 - Buf empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn membuf_empty_ie(&mut self) -> MEMBUF_EMPTY_IE_W<16> {
        MEMBUF_EMPTY_IE_W::new(self)
    }
    #[doc = "Bit 17 - OFIFO data available interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ofifo_av_ie(&mut self) -> OFIFO_AV_IE_W<17> {
        OFIFO_AV_IE_W::new(self)
    }
    #[doc = "Bit 18 - VAD event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn vad_ie(&mut self) -> VAD_IE_W<18> {
        VAD_IE_W::new(self)
    }
    #[doc = "Bits 20:23 - The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)"]
    #[inline(always)]
    #[must_use]
    pub fn pdm_clk_hfdiv(&mut self) -> PDM_CLK_HFDIV_W<20> {
        PDM_CLK_HFDIV_W::new(self)
    }
    #[doc = "Bits 24:27 - Capture cycle delay>=0, should be less than PDM_CLK_HFDIV"]
    #[inline(always)]
    #[must_use]
    pub fn capt_dly(&mut self) -> CAPT_DLY_W<24> {
        CAPT_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
