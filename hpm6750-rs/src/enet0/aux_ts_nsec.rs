#[doc = "Register `AUX_TS_NSEC` reader"]
pub struct R(crate::R<AUX_TS_NSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUX_TS_NSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUX_TS_NSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUX_TS_NSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUX_TS_NSEC` writer"]
pub struct W(crate::W<AUX_TS_NSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUX_TS_NSEC_SPEC>;
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
impl From<crate::W<AUX_TS_NSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUX_TS_NSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUXTSLO` reader - Contains the lower 31 bits (nano-seconds field) of the auxiliary timestamp."]
pub type AUXTSLO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AUXTSLO` writer - Contains the lower 31 bits (nano-seconds field) of the auxiliary timestamp."]
pub type AUXTSLO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUX_TS_NSEC_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bits 0:30 - Contains the lower 31 bits (nano-seconds field) of the auxiliary timestamp."]
    #[inline(always)]
    pub fn auxtslo(&self) -> AUXTSLO_R {
        AUXTSLO_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Contains the lower 31 bits (nano-seconds field) of the auxiliary timestamp."]
    #[inline(always)]
    #[must_use]
    pub fn auxtslo(&mut self) -> AUXTSLO_W<0> {
        AUXTSLO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Timestamp - Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux_ts_nsec](index.html) module"]
pub struct AUX_TS_NSEC_SPEC;
impl crate::RegisterSpec for AUX_TS_NSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aux_ts_nsec::R](R) reader structure"]
impl crate::Readable for AUX_TS_NSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aux_ts_nsec::W](W) writer structure"]
impl crate::Writable for AUX_TS_NSEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUX_TS_NSEC to value 0"]
impl crate::Resettable for AUX_TS_NSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
