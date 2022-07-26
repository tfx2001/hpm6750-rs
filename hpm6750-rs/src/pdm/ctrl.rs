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
#[doc = "Field `SFTRST` reader - software reset the module. Self-clear."]
pub type SFTRST_R = crate::BitReader<bool>;
#[doc = "Field `SFTRST` writer - software reset the module. Self-clear."]
pub type SFTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SOF_FEDGE` reader - asserted if the falling edge of the ref fclk from DAO is the start of a new frame. This is used to to align DAO feedback signal."]
pub type SOF_FEDGE_R = crate::BitReader<bool>;
#[doc = "Field `SOF_FEDGE` writer - asserted if the falling edge of the ref fclk from DAO is the start of a new frame. This is used to to align DAO feedback signal."]
pub type SOF_FEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `USE_COEF_RAM` reader - Asserted to use Coef RAM instead of Coef ROM"]
pub type USE_COEF_RAM_R = crate::BitReader<bool>;
#[doc = "Field `USE_COEF_RAM` writer - Asserted to use Coef RAM instead of Coef ROM"]
pub type USE_COEF_RAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FILT_CRX_ERR_IE` reader - data accessed out of boundary error interruput enable. The error happens when the module cannot calculate the enough number of data in time."]
pub type FILT_CRX_ERR_IE_R = crate::BitReader<bool>;
#[doc = "Field `FILT_CRX_ERR_IE` writer - data accessed out of boundary error interruput enable. The error happens when the module cannot calculate the enough number of data in time."]
pub type FILT_CRX_ERR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `OFIFO_OVFL_ERR_IE` reader - output fifo overflow error interrupt enable"]
pub type OFIFO_OVFL_ERR_IE_R = crate::BitReader<bool>;
#[doc = "Field `OFIFO_OVFL_ERR_IE` writer - output fifo overflow error interrupt enable"]
pub type OFIFO_OVFL_ERR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CIC_OVLD_ERR_IE` reader - CIC overload error interrupt enable"]
pub type CIC_OVLD_ERR_IE_R = crate::BitReader<bool>;
#[doc = "Field `CIC_OVLD_ERR_IE` writer - CIC overload error interrupt enable"]
pub type CIC_OVLD_ERR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CIC_SAT_ERR_IE` reader - Error interrupt enable This bit controls the generation of an interrupt when an error condition (CIC saturation) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled"]
pub type CIC_SAT_ERR_IE_R = crate::BitReader<bool>;
#[doc = "Field `CIC_SAT_ERR_IE` writer - Error interrupt enable This bit controls the generation of an interrupt when an error condition (CIC saturation) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled"]
pub type CIC_SAT_ERR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DEC_AFT_CIC` reader - decimation rate after CIC. Now it is forced to be 3."]
pub type DEC_AFT_CIC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEC_AFT_CIC` writer - decimation rate after CIC. Now it is forced to be 3."]
pub type DEC_AFT_CIC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CAPT_DLY` reader - Capture cycle delay>=0, should be less than PDM_CLK_HFDIV"]
pub type CAPT_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAPT_DLY` writer - Capture cycle delay>=0, should be less than PDM_CLK_HFDIV"]
pub type CAPT_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `PDM_CLK_HFDIV` reader - The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)"]
pub type PDM_CLK_HFDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDM_CLK_HFDIV` writer - The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)"]
pub type PDM_CLK_HFDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `PDM_CLK_DIV_BYPASS` reader - asserted to bypass the pdm clock divider"]
pub type PDM_CLK_DIV_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `PDM_CLK_DIV_BYPASS` writer - asserted to bypass the pdm clock divider"]
pub type PDM_CLK_DIV_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PDM_CLK_OE` reader - pdm_clk_output_en"]
pub type PDM_CLK_OE_R = crate::BitReader<bool>;
#[doc = "Field `PDM_CLK_OE` writer - pdm_clk_output_en"]
pub type PDM_CLK_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `HPF_EN` reader - pdm high pass filter enable. This order-1 HPF only applies to the PDM mic data."]
pub type HPF_EN_R = crate::BitReader<bool>;
#[doc = "Field `HPF_EN` writer - pdm high pass filter enable. This order-1 HPF only applies to the PDM mic data."]
pub type HPF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - software reset the module. Self-clear."]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 23 - asserted if the falling edge of the ref fclk from DAO is the start of a new frame. This is used to to align DAO feedback signal."]
    #[inline(always)]
    pub fn sof_fedge(&self) -> SOF_FEDGE_R {
        SOF_FEDGE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 20 - Asserted to use Coef RAM instead of Coef ROM"]
    #[inline(always)]
    pub fn use_coef_ram(&self) -> USE_COEF_RAM_R {
        USE_COEF_RAM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - data accessed out of boundary error interruput enable. The error happens when the module cannot calculate the enough number of data in time."]
    #[inline(always)]
    pub fn filt_crx_err_ie(&self) -> FILT_CRX_ERR_IE_R {
        FILT_CRX_ERR_IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - output fifo overflow error interrupt enable"]
    #[inline(always)]
    pub fn ofifo_ovfl_err_ie(&self) -> OFIFO_OVFL_ERR_IE_R {
        OFIFO_OVFL_ERR_IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - CIC overload error interrupt enable"]
    #[inline(always)]
    pub fn cic_ovld_err_ie(&self) -> CIC_OVLD_ERR_IE_R {
        CIC_OVLD_ERR_IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Error interrupt enable This bit controls the generation of an interrupt when an error condition (CIC saturation) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled"]
    #[inline(always)]
    pub fn cic_sat_err_ie(&self) -> CIC_SAT_ERR_IE_R {
        CIC_SAT_ERR_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 12:15 - decimation rate after CIC. Now it is forced to be 3."]
    #[inline(always)]
    pub fn dec_aft_cic(&self) -> DEC_AFT_CIC_R {
        DEC_AFT_CIC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 7:10 - Capture cycle delay>=0, should be less than PDM_CLK_HFDIV"]
    #[inline(always)]
    pub fn capt_dly(&self) -> CAPT_DLY_R {
        CAPT_DLY_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 3:6 - The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)"]
    #[inline(always)]
    pub fn pdm_clk_hfdiv(&self) -> PDM_CLK_HFDIV_R {
        PDM_CLK_HFDIV_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 2 - asserted to bypass the pdm clock divider"]
    #[inline(always)]
    pub fn pdm_clk_div_bypass(&self) -> PDM_CLK_DIV_BYPASS_R {
        PDM_CLK_DIV_BYPASS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - pdm_clk_output_en"]
    #[inline(always)]
    pub fn pdm_clk_oe(&self) -> PDM_CLK_OE_R {
        PDM_CLK_OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - pdm high pass filter enable. This order-1 HPF only applies to the PDM mic data."]
    #[inline(always)]
    pub fn hpf_en(&self) -> HPF_EN_R {
        HPF_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - software reset the module. Self-clear."]
    #[inline(always)]
    pub fn sftrst(&mut self) -> SFTRST_W<31> {
        SFTRST_W::new(self)
    }
    #[doc = "Bit 23 - asserted if the falling edge of the ref fclk from DAO is the start of a new frame. This is used to to align DAO feedback signal."]
    #[inline(always)]
    pub fn sof_fedge(&mut self) -> SOF_FEDGE_W<23> {
        SOF_FEDGE_W::new(self)
    }
    #[doc = "Bit 20 - Asserted to use Coef RAM instead of Coef ROM"]
    #[inline(always)]
    pub fn use_coef_ram(&mut self) -> USE_COEF_RAM_W<20> {
        USE_COEF_RAM_W::new(self)
    }
    #[doc = "Bit 19 - data accessed out of boundary error interruput enable. The error happens when the module cannot calculate the enough number of data in time."]
    #[inline(always)]
    pub fn filt_crx_err_ie(&mut self) -> FILT_CRX_ERR_IE_W<19> {
        FILT_CRX_ERR_IE_W::new(self)
    }
    #[doc = "Bit 18 - output fifo overflow error interrupt enable"]
    #[inline(always)]
    pub fn ofifo_ovfl_err_ie(&mut self) -> OFIFO_OVFL_ERR_IE_W<18> {
        OFIFO_OVFL_ERR_IE_W::new(self)
    }
    #[doc = "Bit 17 - CIC overload error interrupt enable"]
    #[inline(always)]
    pub fn cic_ovld_err_ie(&mut self) -> CIC_OVLD_ERR_IE_W<17> {
        CIC_OVLD_ERR_IE_W::new(self)
    }
    #[doc = "Bit 16 - Error interrupt enable This bit controls the generation of an interrupt when an error condition (CIC saturation) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled"]
    #[inline(always)]
    pub fn cic_sat_err_ie(&mut self) -> CIC_SAT_ERR_IE_W<16> {
        CIC_SAT_ERR_IE_W::new(self)
    }
    #[doc = "Bits 12:15 - decimation rate after CIC. Now it is forced to be 3."]
    #[inline(always)]
    pub fn dec_aft_cic(&mut self) -> DEC_AFT_CIC_W<12> {
        DEC_AFT_CIC_W::new(self)
    }
    #[doc = "Bits 7:10 - Capture cycle delay>=0, should be less than PDM_CLK_HFDIV"]
    #[inline(always)]
    pub fn capt_dly(&mut self) -> CAPT_DLY_W<7> {
        CAPT_DLY_W::new(self)
    }
    #[doc = "Bits 3:6 - The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)"]
    #[inline(always)]
    pub fn pdm_clk_hfdiv(&mut self) -> PDM_CLK_HFDIV_W<3> {
        PDM_CLK_HFDIV_W::new(self)
    }
    #[doc = "Bit 2 - asserted to bypass the pdm clock divider"]
    #[inline(always)]
    pub fn pdm_clk_div_bypass(&mut self) -> PDM_CLK_DIV_BYPASS_W<2> {
        PDM_CLK_DIV_BYPASS_W::new(self)
    }
    #[doc = "Bit 1 - pdm_clk_output_en"]
    #[inline(always)]
    pub fn pdm_clk_oe(&mut self) -> PDM_CLK_OE_W<1> {
        PDM_CLK_OE_W::new(self)
    }
    #[doc = "Bit 0 - pdm high pass filter enable. This order-1 HPF only applies to the PDM mic data."]
    #[inline(always)]
    pub fn hpf_en(&mut self) -> HPF_EN_W<0> {
        HPF_EN_W::new(self)
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
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
