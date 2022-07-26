#[doc = "Register `ADCCLK_CLK_TOP_ADC2` reader"]
pub struct R(crate::R<ADCCLK_CLK_TOP_ADC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCLK_CLK_TOP_ADC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCLK_CLK_TOP_ADC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCLK_CLK_TOP_ADC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCLK_CLK_TOP_ADC2` writer"]
pub struct W(crate::W<ADCCLK_CLK_TOP_ADC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCLK_CLK_TOP_ADC2_SPEC>;
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
impl From<crate::W<ADCCLK_CLK_TOP_ADC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCLK_CLK_TOP_ADC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GLB_BUSY` reader - global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
pub type GLB_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `LOC_BUSY` reader - local busy 0: a change is pending for current node 1: current node is changing status"]
pub type LOC_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `MUX` reader - clock source selection 0: ahb clock 1: adc clock 0 2: adc clock 1 3: adc clock 2"]
pub type MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUX` writer - clock source selection 0: ahb clock 1: adc clock 0 2: adc clock 1 3: adc clock 2"]
pub type MUX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCCLK_CLK_TOP_ADC2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 31 - global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    #[inline(always)]
    pub fn glb_busy(&self) -> GLB_BUSY_R {
        GLB_BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - local busy 0: a change is pending for current node 1: current node is changing status"]
    #[inline(always)]
    pub fn loc_busy(&self) -> LOC_BUSY_R {
        LOC_BUSY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 8:10 - clock source selection 0: ahb clock 1: adc clock 0 2: adc clock 1 3: adc clock 2"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - clock source selection 0: ahb clock 1: adc clock 0 2: adc clock 1 3: adc clock 2"]
    #[inline(always)]
    pub fn mux(&mut self) -> MUX_W<8> {
        MUX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcclk_clk_top_adc2](index.html) module"]
pub struct ADCCLK_CLK_TOP_ADC2_SPEC;
impl crate::RegisterSpec for ADCCLK_CLK_TOP_ADC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcclk_clk_top_adc2::R](R) reader structure"]
impl crate::Readable for ADCCLK_CLK_TOP_ADC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcclk_clk_top_adc2::W](W) writer structure"]
impl crate::Writable for ADCCLK_CLK_TOP_ADC2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCCLK_CLK_TOP_ADC2 to value 0"]
impl crate::Resettable for ADCCLK_CLK_TOP_ADC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
