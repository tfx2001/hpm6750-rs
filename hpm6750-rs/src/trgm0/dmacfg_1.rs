#[doc = "Register `DMACFG_1` reader"]
pub struct R(crate::R<DMACFG_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACFG_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACFG_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACFG_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACFG_1` writer"]
pub struct W(crate::W<DMACFG_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACFG_1_SPEC>;
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
impl From<crate::W<DMACFG_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACFG_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMASRCSEL` reader - This field selects one of the DMA requests as the DMA request output."]
pub type DMASRCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMASRCSEL` writer - This field selects one of the DMA requests as the DMA request output."]
pub type DMASRCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACFG_1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - This field selects one of the DMA requests as the DMA request output."]
    #[inline(always)]
    pub fn dmasrcsel(&self) -> DMASRCSEL_R {
        DMASRCSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This field selects one of the DMA requests as the DMA request output."]
    #[inline(always)]
    pub fn dmasrcsel(&mut self) -> DMASRCSEL_W<0> {
        DMASRCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA request configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacfg_1](index.html) module"]
pub struct DMACFG_1_SPEC;
impl crate::RegisterSpec for DMACFG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacfg_1::R](R) reader structure"]
impl crate::Readable for DMACFG_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacfg_1::W](W) writer structure"]
impl crate::Writable for DMACFG_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACFG_1 to value 0"]
impl crate::Resettable for DMACFG_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
