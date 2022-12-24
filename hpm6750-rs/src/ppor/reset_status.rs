#[doc = "Register `RESET_STATUS` reader"]
pub struct R(crate::R<RESET_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_STATUS` writer"]
pub struct W(crate::W<RESET_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_STATUS_SPEC>;
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
impl From<crate::W<RESET_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATUS` reader - current status of reset sources 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
pub type STATUS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STATUS` writer - current status of reset sources 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
pub type STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RESET_STATUS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - current status of reset sources 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - current status of reset sources 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> STATUS_W<0> {
        STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reset source status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_status](index.html) module"]
pub struct RESET_STATUS_SPEC;
impl crate::RegisterSpec for RESET_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_status::R](R) reader structure"]
impl crate::Readable for RESET_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_status::W](W) writer structure"]
impl crate::Writable for RESET_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESET_STATUS to value 0"]
impl crate::Resettable for RESET_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
