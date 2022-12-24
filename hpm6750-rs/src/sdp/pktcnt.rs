#[doc = "Register `PKTCNT` reader"]
pub struct R(crate::R<PKTCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKTCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKTCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKTCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKTCNT` writer"]
pub struct W(crate::W<PKTCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKTCNT_SPEC>;
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
impl From<crate::W<PKTCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKTCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTINCR` reader - The value written to this field is added to the spacket count."]
pub type CNTINCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNTINCR` writer - The value written to this field is added to the spacket count."]
pub type CNTINCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PKTCNT_SPEC, u8, u8, 8, O>;
#[doc = "Field `CNTVAL` reader - This read-only field shows the current (instantaneous) value of the packet counter"]
pub type CNTVAL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - The value written to this field is added to the spacket count."]
    #[inline(always)]
    pub fn cntincr(&self) -> CNTINCR_R {
        CNTINCR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This read-only field shows the current (instantaneous) value of the packet counter"]
    #[inline(always)]
    pub fn cntval(&self) -> CNTVAL_R {
        CNTVAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The value written to this field is added to the spacket count."]
    #[inline(always)]
    #[must_use]
    pub fn cntincr(&mut self) -> CNTINCR_W<0> {
        CNTINCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "packet counter registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pktcnt](index.html) module"]
pub struct PKTCNT_SPEC;
impl crate::RegisterSpec for PKTCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pktcnt::R](R) reader structure"]
impl crate::Readable for PKTCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pktcnt::W](W) writer structure"]
impl crate::Writable for PKTCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PKTCNT to value 0"]
impl crate::Resettable for PKTCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
