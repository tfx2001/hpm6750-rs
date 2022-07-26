#[doc = "Register `CQVER` reader"]
pub struct R(crate::R<CQVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EMMC_VER_MAHOR` reader - This bit indicates the eMMC major version (1st digit left of decimal point) in BCD format."]
pub type EMMC_VER_MAHOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMMC_VER_MINOR` reader - This bit indicates the eMMC minor version (1st digit right of decimal point) in BCD format."]
pub type EMMC_VER_MINOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMMC_VER_SUFFIX` reader - This bit indicates the eMMC version suffix (2nd digit right of decimal point) in BCD format."]
pub type EMMC_VER_SUFFIX_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 8:11 - This bit indicates the eMMC major version (1st digit left of decimal point) in BCD format."]
    #[inline(always)]
    pub fn emmc_ver_mahor(&self) -> EMMC_VER_MAHOR_R {
        EMMC_VER_MAHOR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This bit indicates the eMMC minor version (1st digit right of decimal point) in BCD format."]
    #[inline(always)]
    pub fn emmc_ver_minor(&self) -> EMMC_VER_MINOR_R {
        EMMC_VER_MINOR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - This bit indicates the eMMC version suffix (2nd digit right of decimal point) in BCD format."]
    #[inline(always)]
    pub fn emmc_ver_suffix(&self) -> EMMC_VER_SUFFIX_R {
        EMMC_VER_SUFFIX_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqver](index.html) module"]
pub struct CQVER_SPEC;
impl crate::RegisterSpec for CQVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqver::R](R) reader structure"]
impl crate::Readable for CQVER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CQVER to value 0"]
impl crate::Resettable for CQVER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
