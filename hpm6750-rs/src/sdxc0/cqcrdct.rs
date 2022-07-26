#[doc = "Register `CQCRDCT` reader"]
pub struct R(crate::R<CQCRDCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQCRDCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQCRDCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQCRDCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DCMD_RESP` reader - This register contains the response of the command generated by the last direct command (DCMD) task that was sent. Contents of this register are valid only after bit 31 of CQTDBR register is cleared by the controller."]
pub type DCMD_RESP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register contains the response of the command generated by the last direct command (DCMD) task that was sent. Contents of this register are valid only after bit 31 of CQTDBR register is cleared by the controller."]
    #[inline(always)]
    pub fn dcmd_resp(&self) -> DCMD_RESP_R {
        DCMD_RESP_R::new(self.bits)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcrdct](index.html) module"]
pub struct CQCRDCT_SPEC;
impl crate::RegisterSpec for CQCRDCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqcrdct::R](R) reader structure"]
impl crate::Readable for CQCRDCT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CQCRDCT to value 0"]
impl crate::Resettable for CQCRDCT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}