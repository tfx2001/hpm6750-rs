#[doc = "Register `MSHC_VER_TYPE` reader"]
pub struct R(crate::R<MSHC_VER_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSHC_VER_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSHC_VER_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSHC_VER_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VER_TYPE` reader - No description avaiable"]
pub type VER_TYPE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    pub fn ver_type(&self) -> VER_TYPE_R {
        VER_TYPE_R::new(self.bits)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mshc_ver_type](index.html) module"]
pub struct MSHC_VER_TYPE_SPEC;
impl crate::RegisterSpec for MSHC_VER_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mshc_ver_type::R](R) reader structure"]
impl crate::Readable for MSHC_VER_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSHC_VER_TYPE to value 0"]
impl crate::Resettable for MSHC_VER_TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
