#[doc = "Register `ADC_CFG0` reader"]
pub struct R(crate::R<ADC_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CFG0` writer"]
pub struct W(crate::W<ADC_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CFG0_SPEC>;
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
impl From<crate::W<ADC_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL_SYNC_AHB` reader - set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode"]
pub type SEL_SYNC_AHB_R = crate::BitReader<bool>;
#[doc = "Field `SEL_SYNC_AHB` writer - set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode"]
pub type SEL_SYNC_AHB_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFG0_SPEC, bool, O>;
#[doc = "Field `ADC_AHB_EN` reader - set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;"]
pub type ADC_AHB_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC_AHB_EN` writer - set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;"]
pub type ADC_AHB_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFG0_SPEC, bool, O>;
#[doc = "Field `CONVERT_DURATION` reader - for trigger queue, from trg_sample_req to trg_convert_req"]
pub type CONVERT_DURATION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CONVERT_DURATION` writer - for trigger queue, from trg_sample_req to trg_convert_req"]
pub type CONVERT_DURATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC_CFG0_SPEC, u16, u16, 16, O>;
#[doc = "Field `PORT3_REALTIME` reader - set to enable trg queue stop other queues"]
pub type PORT3_REALTIME_R = crate::BitReader<bool>;
#[doc = "Field `PORT3_REALTIME` writer - set to enable trg queue stop other queues"]
pub type PORT3_REALTIME_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC_CFG0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode"]
    #[inline(always)]
    pub fn sel_sync_ahb(&self) -> SEL_SYNC_AHB_R {
        SEL_SYNC_AHB_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 29 - set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;"]
    #[inline(always)]
    pub fn adc_ahb_en(&self) -> ADC_AHB_EN_R {
        ADC_AHB_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 12:27 - for trigger queue, from trg_sample_req to trg_convert_req"]
    #[inline(always)]
    pub fn convert_duration(&self) -> CONVERT_DURATION_R {
        CONVERT_DURATION_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bit 0 - set to enable trg queue stop other queues"]
    #[inline(always)]
    pub fn port3_realtime(&self) -> PORT3_REALTIME_R {
        PORT3_REALTIME_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode"]
    #[inline(always)]
    pub fn sel_sync_ahb(&mut self) -> SEL_SYNC_AHB_W<31> {
        SEL_SYNC_AHB_W::new(self)
    }
    #[doc = "Bit 29 - set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;"]
    #[inline(always)]
    pub fn adc_ahb_en(&mut self) -> ADC_AHB_EN_W<29> {
        ADC_AHB_EN_W::new(self)
    }
    #[doc = "Bits 12:27 - for trigger queue, from trg_sample_req to trg_convert_req"]
    #[inline(always)]
    pub fn convert_duration(&mut self) -> CONVERT_DURATION_W<12> {
        CONVERT_DURATION_W::new(self)
    }
    #[doc = "Bit 0 - set to enable trg queue stop other queues"]
    #[inline(always)]
    pub fn port3_realtime(&mut self) -> PORT3_REALTIME_W<0> {
        PORT3_REALTIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cfg0](index.html) module"]
pub struct ADC_CFG0_SPEC;
impl crate::RegisterSpec for ADC_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_cfg0::R](R) reader structure"]
impl crate::Readable for ADC_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_cfg0::W](W) writer structure"]
impl crate::Writable for ADC_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CFG0 to value 0"]
impl crate::Resettable for ADC_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
