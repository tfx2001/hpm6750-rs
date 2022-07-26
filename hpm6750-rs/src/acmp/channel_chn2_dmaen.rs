#[doc = "Register `CHANNEL_CHN2_DMAEN` reader"]
pub struct R(crate::R<CHANNEL_CHN2_DMAEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_CHN2_DMAEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_CHN2_DMAEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_CHN2_DMAEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNEL_CHN2_DMAEN` writer"]
pub struct W(crate::W<CHANNEL_CHN2_DMAEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNEL_CHN2_DMAEN_SPEC>;
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
impl From<crate::W<CHANNEL_CHN2_DMAEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNEL_CHN2_DMAEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEDGEN` reader - Output falling edge flag DMA request enable bit."]
pub type FEDGEN_R = crate::BitReader<bool>;
#[doc = "Field `FEDGEN` writer - Output falling edge flag DMA request enable bit."]
pub type FEDGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CHN2_DMAEN_SPEC, bool, O>;
#[doc = "Field `REDGEN` reader - Output rising edge flag DMA request enable bit."]
pub type REDGEN_R = crate::BitReader<bool>;
#[doc = "Field `REDGEN` writer - Output rising edge flag DMA request enable bit."]
pub type REDGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNEL_CHN2_DMAEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Output falling edge flag DMA request enable bit."]
    #[inline(always)]
    pub fn fedgen(&self) -> FEDGEN_R {
        FEDGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Output rising edge flag DMA request enable bit."]
    #[inline(always)]
    pub fn redgen(&self) -> REDGEN_R {
        REDGEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Output falling edge flag DMA request enable bit."]
    #[inline(always)]
    pub fn fedgen(&mut self) -> FEDGEN_W<1> {
        FEDGEN_W::new(self)
    }
    #[doc = "Bit 0 - Output rising edge flag DMA request enable bit."]
    #[inline(always)]
    pub fn redgen(&mut self) -> REDGEN_W<0> {
        REDGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA request enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_chn2_dmaen](index.html) module"]
pub struct CHANNEL_CHN2_DMAEN_SPEC;
impl crate::RegisterSpec for CHANNEL_CHN2_DMAEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_chn2_dmaen::R](R) reader structure"]
impl crate::Readable for CHANNEL_CHN2_DMAEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channel_chn2_dmaen::W](W) writer structure"]
impl crate::Writable for CHANNEL_CHN2_DMAEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHANNEL_CHN2_DMAEN to value 0"]
impl crate::Resettable for CHANNEL_CHN2_DMAEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
