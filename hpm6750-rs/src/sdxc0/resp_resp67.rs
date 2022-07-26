#[doc = "Register `RESP_RESP67` reader"]
pub struct R(crate::R<RESP_RESP67_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP_RESP67_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP_RESP67_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP_RESP67_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESP01` reader - Command Response These bits reflect 39-8 bits of SD/eMMC Response Field. Note: For Auto CMD, the 32-bit response (bits 39-8 of the Response Field) is updated in the RESP67_R register."]
pub type RESP01_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response These bits reflect 39-8 bits of SD/eMMC Response Field. Note: For Auto CMD, the 32-bit response (bits 39-8 of the Response Field) is updated in the RESP67_R register."]
    #[inline(always)]
    pub fn resp01(&self) -> RESP01_R {
        RESP01_R::new(self.bits)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp_resp67](index.html) module"]
pub struct RESP_RESP67_SPEC;
impl crate::RegisterSpec for RESP_RESP67_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp_resp67::R](R) reader structure"]
impl crate::Readable for RESP_RESP67_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP_RESP67 to value 0"]
impl crate::Resettable for RESP_RESP67_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
