#[doc = "Register `DMACFG_2` reader"]
pub struct R(crate::R<DMACFG_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACFG_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACFG_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACFG_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACFG_2` writer"]
pub struct W(crate::W<DMACFG_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACFG_2_SPEC>;
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
impl From<crate::W<DMACFG_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACFG_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMASRCSEL` reader - This field selects one of the DMA requests as the DMA request output."]
pub type DMASRCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMASRCSEL` writer - This field selects one of the DMA requests as the DMA request output."]
pub type DMASRCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACFG_2_SPEC, u8, u8, 5, O>;
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
    #[must_use]
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
#[doc = "DMA request configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacfg_2](index.html) module"]
pub struct DMACFG_2_SPEC;
impl crate::RegisterSpec for DMACFG_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacfg_2::R](R) reader structure"]
impl crate::Readable for DMACFG_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacfg_2::W](W) writer structure"]
impl crate::Writable for DMACFG_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACFG_2 to value 0"]
impl crate::Resettable for DMACFG_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
