#[doc = "Register `PTPC_1_CAPT_SNAPH` reader"]
pub struct R(crate::R<PTPC_1_CAPT_SNAPH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPC_1_CAPT_SNAPH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPC_1_CAPT_SNAPH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPC_1_CAPT_SNAPH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPT_SNAP_HIGH` reader - take snapshot for input capture signal, at pos or neg or both; the result can be kept or updated at each event according to cfg0.bit8"]
pub type CAPT_SNAP_HIGH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - take snapshot for input capture signal, at pos or neg or both; the result can be kept or updated at each event according to cfg0.bit8"]
    #[inline(always)]
    pub fn capt_snap_high(&self) -> CAPT_SNAP_HIGH_R {
        CAPT_SNAP_HIGH_R::new(self.bits)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptpc_1_capt_snaph](index.html) module"]
pub struct PTPC_1_CAPT_SNAPH_SPEC;
impl crate::RegisterSpec for PTPC_1_CAPT_SNAPH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptpc_1_capt_snaph::R](R) reader structure"]
impl crate::Readable for PTPC_1_CAPT_SNAPH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTPC_1_CAPT_SNAPH to value 0"]
impl crate::Resettable for PTPC_1_CAPT_SNAPH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
