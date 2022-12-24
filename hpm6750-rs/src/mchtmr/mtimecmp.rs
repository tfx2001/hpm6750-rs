#[doc = "Register `MTIMECMP` reader"]
pub struct R(crate::R<MTIMECMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTIMECMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTIMECMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTIMECMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTIMECMP` writer"]
pub struct W(crate::W<MTIMECMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTIMECMP_SPEC>;
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
impl From<crate::W<MTIMECMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTIMECMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MTIMECMP` reader - Machine time compare"]
pub type MTIMECMP_R = crate::FieldReader<u64, u64>;
#[doc = "Field `MTIMECMP` writer - Machine time compare"]
pub type MTIMECMP_W<'a, const O: u8> = crate::FieldWriter<'a, u64, MTIMECMP_SPEC, u64, u64, 64, O>;
impl R {
    #[doc = "Bits 0:63 - Machine time compare"]
    #[inline(always)]
    pub fn mtimecmp(&self) -> MTIMECMP_R {
        MTIMECMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63 - Machine time compare"]
    #[inline(always)]
    #[must_use]
    pub fn mtimecmp(&mut self) -> MTIMECMP_W<0> {
        MTIMECMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Machine Time Compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtimecmp](index.html) module"]
pub struct MTIMECMP_SPEC;
impl crate::RegisterSpec for MTIMECMP_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [mtimecmp::R](R) reader structure"]
impl crate::Readable for MTIMECMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtimecmp::W](W) writer structure"]
impl crate::Writable for MTIMECMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTIMECMP to value 0x0002_0210"]
impl crate::Resettable for MTIMECMP_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0210;
}
