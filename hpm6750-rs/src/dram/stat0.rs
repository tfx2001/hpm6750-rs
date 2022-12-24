#[doc = "Register `STAT0` reader"]
pub struct R(crate::R<STAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDLE` reader - Indicating whether it is in IDLE state. When IDLE=1, it is in IDLE state. There is no pending AXI command in internal queue and no pending device access."]
pub type IDLE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicating whether it is in IDLE state. When IDLE=1, it is in IDLE state. There is no pending AXI command in internal queue and no pending device access."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat0](index.html) module"]
pub struct STAT0_SPEC;
impl crate::RegisterSpec for STAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat0::R](R) reader structure"]
impl crate::Readable for STAT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT0 to value 0"]
impl crate::Resettable for STAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
