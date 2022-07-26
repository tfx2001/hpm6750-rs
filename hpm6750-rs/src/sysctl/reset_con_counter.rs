#[doc = "Register `RESET_CON_COUNTER` reader"]
pub struct R(crate::R<RESET_CON_COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_CON_COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_CON_COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_CON_COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_CON_COUNTER` writer"]
pub struct W(crate::W<RESET_CON_COUNTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_CON_COUNTER_SPEC>;
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
impl From<crate::W<RESET_CON_COUNTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_CON_COUNTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER` reader - self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
pub type COUNTER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNTER` writer - self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
pub type COUNTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RESET_CON_COUNTER_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    pub fn counter(&mut self) -> COUNTER_W<0> {
        COUNTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_con_counter](index.html) module"]
pub struct RESET_CON_COUNTER_SPEC;
impl crate::RegisterSpec for RESET_CON_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_con_counter::R](R) reader structure"]
impl crate::Readable for RESET_CON_COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_con_counter::W](W) writer structure"]
impl crate::Writable for RESET_CON_COUNTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESET_CON_COUNTER to value 0x03"]
impl crate::Resettable for RESET_CON_COUNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
