#[doc = "Register `PERIODICLISTBASE` reader"]
pub struct R(crate::R<PERIODICLISTBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIODICLISTBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIODICLISTBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIODICLISTBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIODICLISTBASE` writer"]
pub struct W(crate::W<PERIODICLISTBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIODICLISTBASE_SPEC>;
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
impl From<crate::W<PERIODICLISTBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIODICLISTBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASEADR` reader - BASEADR Base Address (Low). These bits correspond to memory address signals \\[31:12\\], respectively. Only used by the host controller."]
pub type BASEADR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BASEADR` writer - BASEADR Base Address (Low). These bits correspond to memory address signals \\[31:12\\], respectively. Only used by the host controller."]
pub type BASEADR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERIODICLISTBASE_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 12:31 - BASEADR Base Address (Low). These bits correspond to memory address signals \\[31:12\\], respectively. Only used by the host controller."]
    #[inline(always)]
    pub fn baseadr(&self) -> BASEADR_R {
        BASEADR_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - BASEADR Base Address (Low). These bits correspond to memory address signals \\[31:12\\], respectively. Only used by the host controller."]
    #[inline(always)]
    #[must_use]
    pub fn baseadr(&mut self) -> BASEADR_W<12> {
        BASEADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame List Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periodiclistbase](index.html) module"]
pub struct PERIODICLISTBASE_SPEC;
impl crate::RegisterSpec for PERIODICLISTBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periodiclistbase::R](R) reader structure"]
impl crate::Readable for PERIODICLISTBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [periodiclistbase::W](W) writer structure"]
impl crate::Writable for PERIODICLISTBASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERIODICLISTBASE to value 0"]
impl crate::Resettable for PERIODICLISTBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
