#[doc = "Register `RESET_FLAG` reader"]
pub struct R(crate::R<RESET_FLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_FLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_FLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_FLAG` writer"]
pub struct W(crate::W<RESET_FLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_FLAG_SPEC>;
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
impl From<crate::W<RESET_FLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_FLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLAG` reader - reset reason of last hard reset, write 1 to clear each bit 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
pub type FLAG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLAG` writer - reset reason of last hard reset, write 1 to clear each bit 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
pub type FLAG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESET_FLAG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - reset reason of last hard reset, write 1 to clear each bit 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
    #[inline(always)]
    pub fn flag(&self) -> FLAG_R {
        FLAG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reset reason of last hard reset, write 1 to clear each bit 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
    #[inline(always)]
    pub fn flag(&mut self) -> FLAG_W<0> {
        FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "flag indicate reset source\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_flag](index.html) module"]
pub struct RESET_FLAG_SPEC;
impl crate::RegisterSpec for RESET_FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_flag::R](R) reader structure"]
impl crate::Readable for RESET_FLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_flag::W](W) writer structure"]
impl crate::Writable for RESET_FLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESET_FLAG to value 0"]
impl crate::Resettable for RESET_FLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
