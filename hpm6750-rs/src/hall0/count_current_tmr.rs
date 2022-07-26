#[doc = "Register `COUNT_CURRENT_TMR` reader"]
pub struct R(crate::R<COUNT_CURRENT_TMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT_CURRENT_TMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT_CURRENT_TMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT_CURRENT_TMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER` reader - 32 bit free run timer"]
pub type TIMER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bit free run timer"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits)
    }
}
#[doc = "Timer counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_current_tmr](index.html) module"]
pub struct COUNT_CURRENT_TMR_SPEC;
impl crate::RegisterSpec for COUNT_CURRENT_TMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [count_current_tmr::R](R) reader structure"]
impl crate::Readable for COUNT_CURRENT_TMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COUNT_CURRENT_TMR to value 0"]
impl crate::Resettable for COUNT_CURRENT_TMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
