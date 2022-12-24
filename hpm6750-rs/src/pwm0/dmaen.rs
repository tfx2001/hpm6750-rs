#[doc = "Register `DMAEN` reader"]
pub struct R(crate::R<DMAEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAEN` writer"]
pub struct W(crate::W<DMAEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAEN_SPEC>;
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
impl From<crate::W<DMAEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPENX` reader - comparator output compare or input capture flag DMA request enable"]
pub type CMPENX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMPENX` writer - comparator output compare or input capture flag DMA request enable"]
pub type CMPENX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAEN_SPEC, u32, u32, 24, O>;
#[doc = "Field `RLDEN` reader - reload flag DMA request enable"]
pub type RLDEN_R = crate::BitReader<bool>;
#[doc = "Field `RLDEN` writer - reload flag DMA request enable"]
pub type RLDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEN_SPEC, bool, O>;
#[doc = "Field `HALFRLDEN` reader - half reload flag DMA request enable"]
pub type HALFRLDEN_R = crate::BitReader<bool>;
#[doc = "Field `HALFRLDEN` writer - half reload flag DMA request enable"]
pub type HALFRLDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEN_SPEC, bool, O>;
#[doc = "Field `XRLDEN` reader - extended reload flag DMA request enable"]
pub type XRLDEN_R = crate::BitReader<bool>;
#[doc = "Field `XRLDEN` writer - extended reload flag DMA request enable"]
pub type XRLDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEN_SPEC, bool, O>;
#[doc = "Field `FAULTEN` reader - fault condition DMA request enable"]
pub type FAULTEN_R = crate::BitReader<bool>;
#[doc = "Field `FAULTEN` writer - fault condition DMA request enable"]
pub type FAULTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAEN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:23 - comparator output compare or input capture flag DMA request enable"]
    #[inline(always)]
    pub fn cmpenx(&self) -> CMPENX_R {
        CMPENX_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - reload flag DMA request enable"]
    #[inline(always)]
    pub fn rlden(&self) -> RLDEN_R {
        RLDEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - half reload flag DMA request enable"]
    #[inline(always)]
    pub fn halfrlden(&self) -> HALFRLDEN_R {
        HALFRLDEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - extended reload flag DMA request enable"]
    #[inline(always)]
    pub fn xrlden(&self) -> XRLDEN_R {
        XRLDEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - fault condition DMA request enable"]
    #[inline(always)]
    pub fn faulten(&self) -> FAULTEN_R {
        FAULTEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - comparator output compare or input capture flag DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpenx(&mut self) -> CMPENX_W<0> {
        CMPENX_W::new(self)
    }
    #[doc = "Bit 24 - reload flag DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn rlden(&mut self) -> RLDEN_W<24> {
        RLDEN_W::new(self)
    }
    #[doc = "Bit 25 - half reload flag DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn halfrlden(&mut self) -> HALFRLDEN_W<25> {
        HALFRLDEN_W::new(self)
    }
    #[doc = "Bit 26 - extended reload flag DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn xrlden(&mut self) -> XRLDEN_W<26> {
        XRLDEN_W::new(self)
    }
    #[doc = "Bit 27 - fault condition DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn faulten(&mut self) -> FAULTEN_W<27> {
        FAULTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA request enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaen](index.html) module"]
pub struct DMAEN_SPEC;
impl crate::RegisterSpec for DMAEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaen::R](R) reader structure"]
impl crate::Readable for DMAEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaen::W](W) writer structure"]
impl crate::Writable for DMAEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAEN to value 0"]
impl crate::Resettable for DMAEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
