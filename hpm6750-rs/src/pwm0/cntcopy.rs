#[doc = "Register `CNTCOPY` reader"]
pub struct R(crate::R<CNTCOPY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTCOPY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTCOPY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTCOPY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - current clock counter value"]
pub type CNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `XCNT` reader - current extended counter value"]
pub type XCNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 4:27 - current clock counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits >> 4) & 0x00ff_ffff)
    }
    #[doc = "Bits 28:31 - current extended counter value"]
    #[inline(always)]
    pub fn xcnt(&self) -> XCNT_R {
        XCNT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Counter copy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntcopy](index.html) module"]
pub struct CNTCOPY_SPEC;
impl crate::RegisterSpec for CNTCOPY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntcopy::R](R) reader structure"]
impl crate::Readable for CNTCOPY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CNTCOPY to value 0"]
impl crate::Resettable for CNTCOPY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
