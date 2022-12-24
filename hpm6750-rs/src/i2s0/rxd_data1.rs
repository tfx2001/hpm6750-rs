#[doc = "Register `RXD_DATA1` reader"]
pub struct R(crate::R<RXD_DATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXD_DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXD_DATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXD_DATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D` reader - No description avaiable"]
pub type D_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new(self.bits)
    }
}
#[doc = "Rx Data1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxd_data1](index.html) module"]
pub struct RXD_DATA1_SPEC;
impl crate::RegisterSpec for RXD_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxd_data1::R](R) reader structure"]
impl crate::Readable for RXD_DATA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXD_DATA1 to value 0"]
impl crate::Resettable for RXD_DATA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
