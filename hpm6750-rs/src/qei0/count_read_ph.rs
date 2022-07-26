#[doc = "Register `COUNT_READ_PH` reader"]
pub struct R(crate::R<COUNT_READ_PH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT_READ_PH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT_READ_PH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT_READ_PH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESERVE` reader - reversed"]
pub type RESERVE_R = crate::BitReader<bool>;
#[doc = "Field `DIR` reader - 1- reverse rotation 0- forward rotation"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `ASTAT` reader - 1- a input is high 0- a input is low"]
pub type ASTAT_R = crate::BitReader<bool>;
#[doc = "Field `BSTAT` reader - 1- b input is high 0- b input is low"]
pub type BSTAT_R = crate::BitReader<bool>;
#[doc = "Field `PHCNT` reader - phcnt value"]
pub type PHCNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 31 - reversed"]
    #[inline(always)]
    pub fn reserve(&self) -> RESERVE_R {
        RESERVE_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- reverse rotation 0- forward rotation"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 26 - 1- a input is high 0- a input is low"]
    #[inline(always)]
    pub fn astat(&self) -> ASTAT_R {
        ASTAT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - 1- b input is high 0- b input is low"]
    #[inline(always)]
    pub fn bstat(&self) -> BSTAT_R {
        BSTAT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 0:20 - phcnt value"]
    #[inline(always)]
    pub fn phcnt(&self) -> PHCNT_R {
        PHCNT_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
#[doc = "Phase counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_read_ph](index.html) module"]
pub struct COUNT_READ_PH_SPEC;
impl crate::RegisterSpec for COUNT_READ_PH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [count_read_ph::R](R) reader structure"]
impl crate::Readable for COUNT_READ_PH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COUNT_READ_PH to value 0"]
impl crate::Resettable for COUNT_READ_PH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
