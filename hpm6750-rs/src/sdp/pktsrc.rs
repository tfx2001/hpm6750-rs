#[doc = "Register `PKTSRC` reader"]
pub struct R(crate::R<PKTSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKTSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKTSRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKTSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKTSRC` writer"]
pub struct W(crate::W<PKTSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKTSRC_SPEC>;
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
impl From<crate::W<PKTSRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKTSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKTSRC` reader - Packet Memory Source Address"]
pub type PKTSRC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PKTSRC` writer - Packet Memory Source Address"]
pub type PKTSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PKTSRC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Packet Memory Source Address"]
    #[inline(always)]
    pub fn pktsrc(&self) -> PKTSRC_R {
        PKTSRC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Packet Memory Source Address"]
    #[inline(always)]
    #[must_use]
    pub fn pktsrc(&mut self) -> PKTSRC_W<0> {
        PKTSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet Memory Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pktsrc](index.html) module"]
pub struct PKTSRC_SPEC;
impl crate::RegisterSpec for PKTSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pktsrc::R](R) reader structure"]
impl crate::Readable for PKTSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pktsrc::W](W) writer structure"]
impl crate::Writable for PKTSRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PKTSRC to value 0"]
impl crate::Resettable for PKTSRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
