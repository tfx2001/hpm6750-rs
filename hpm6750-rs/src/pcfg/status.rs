#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEL32K` reader - track is using XTAL32K 0: track is not using XTAL32K 1: track is using XTAL32K"]
pub type SEL32K_R = crate::BitReader<bool>;
#[doc = "Field `SEL24M` reader - track is using XTAL24M 0: track is not using XTAL24M 1: track is using XTAL24M"]
pub type SEL24M_R = crate::BitReader<bool>;
#[doc = "Field `EN_TRIM` reader - default value takes effect 0: default value is invalid 1: default value is valid"]
pub type EN_TRIM_R = crate::BitReader<bool>;
#[doc = "Field `TRIM_C` reader - default coarse trim value"]
pub type TRIM_C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_F` reader - default fine trim value"]
pub type TRIM_F_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 20 - track is using XTAL32K 0: track is not using XTAL32K 1: track is using XTAL32K"]
    #[inline(always)]
    pub fn sel32k(&self) -> SEL32K_R {
        SEL32K_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 16 - track is using XTAL24M 0: track is not using XTAL24M 1: track is using XTAL24M"]
    #[inline(always)]
    pub fn sel24m(&self) -> SEL24M_R {
        SEL24M_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - default value takes effect 0: default value is invalid 1: default value is valid"]
    #[inline(always)]
    pub fn en_trim(&self) -> EN_TRIM_R {
        EN_TRIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 8:10 - default coarse trim value"]
    #[inline(always)]
    pub fn trim_c(&self) -> TRIM_C_R {
        TRIM_C_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 0:4 - default fine trim value"]
    #[inline(always)]
    pub fn trim_f(&self) -> TRIM_F_R {
        TRIM_F_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "RC 24M track status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
