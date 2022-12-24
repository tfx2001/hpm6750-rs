#[doc = "Register `COUNT_READ_V` reader"]
pub struct R(crate::R<COUNT_READ_V_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT_READ_V_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT_READ_V_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT_READ_V_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VCNT` reader - vcnt counter"]
pub type VCNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:27 - vcnt counter"]
    #[inline(always)]
    pub fn vcnt(&self) -> VCNT_R {
        VCNT_R::new(self.bits & 0x0fff_ffff)
    }
}
#[doc = "V read register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_read_v](index.html) module"]
pub struct COUNT_READ_V_SPEC;
impl crate::RegisterSpec for COUNT_READ_V_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [count_read_v::R](R) reader structure"]
impl crate::Readable for COUNT_READ_V_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COUNT_READ_V to value 0"]
impl crate::Resettable for COUNT_READ_V_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
