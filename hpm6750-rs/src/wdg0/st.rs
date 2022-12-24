#[doc = "Register `ST` reader"]
pub struct R(crate::R<ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ST` writer"]
pub struct W(crate::W<ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ST_SPEC>;
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
impl From<crate::W<ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEXPIRED` reader - The status of the watchdog interrupt timer 0: timer is not expired yet 1: timer is expired"]
pub type INTEXPIRED_R = crate::BitReader<bool>;
#[doc = "Field `INTEXPIRED` writer - The status of the watchdog interrupt timer 0: timer is not expired yet 1: timer is expired"]
pub type INTEXPIRED_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The status of the watchdog interrupt timer 0: timer is not expired yet 1: timer is expired"]
    #[inline(always)]
    pub fn intexpired(&self) -> INTEXPIRED_R {
        INTEXPIRED_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The status of the watchdog interrupt timer 0: timer is not expired yet 1: timer is expired"]
    #[inline(always)]
    #[must_use]
    pub fn intexpired(&mut self) -> INTEXPIRED_W<0> {
        INTEXPIRED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st](index.html) module"]
pub struct ST_SPEC;
impl crate::RegisterSpec for ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st::R](R) reader structure"]
impl crate::Readable for ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [st::W](W) writer structure"]
impl crate::Writable for ST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST to value 0"]
impl crate::Resettable for ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
