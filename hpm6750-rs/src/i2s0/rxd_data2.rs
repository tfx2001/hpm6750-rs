#[doc = "Register `RXD_DATA2` reader"]
pub struct R(crate::R<RXD_DATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXD_DATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXD_DATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXD_DATA2_SPEC>) -> Self {
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
#[doc = "Rx Data2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxd_data2](index.html) module"]
pub struct RXD_DATA2_SPEC;
impl crate::RegisterSpec for RXD_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxd_data2::R](R) reader structure"]
impl crate::Readable for RXD_DATA2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXD_DATA2 to value 0"]
impl crate::Resettable for RXD_DATA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
