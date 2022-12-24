#[doc = "Register `COUNT_SNAP0_TMR` reader"]
pub struct R(crate::R<COUNT_SNAP0_TMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT_SNAP0_TMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT_SNAP0_TMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT_SNAP0_TMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TMRCNT` reader - 32 bit free run timer"]
pub type TMRCNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bit free run timer"]
    #[inline(always)]
    pub fn tmrcnt(&self) -> TMRCNT_R {
        TMRCNT_R::new(self.bits)
    }
}
#[doc = "Timer snap register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_snap0_tmr](index.html) module"]
pub struct COUNT_SNAP0_TMR_SPEC;
impl crate::RegisterSpec for COUNT_SNAP0_TMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [count_snap0_tmr::R](R) reader structure"]
impl crate::Readable for COUNT_SNAP0_TMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COUNT_SNAP0_TMR to value 0"]
impl crate::Resettable for COUNT_SNAP0_TMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
