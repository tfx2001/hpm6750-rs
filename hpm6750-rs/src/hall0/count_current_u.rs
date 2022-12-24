#[doc = "Register `COUNT_CURRENT_U` reader"]
pub struct R(crate::R<COUNT_CURRENT_U_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT_CURRENT_U_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT_CURRENT_U_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT_CURRENT_U_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UCNT` reader - ucnt counter"]
pub type UCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WSTAT` reader - this bit indicate W state"]
pub type WSTAT_R = crate::BitReader<bool>;
#[doc = "Field `VSTAT` reader - this bit indicate V state"]
pub type VSTAT_R = crate::BitReader<bool>;
#[doc = "Field `USTAT` reader - this bit indicate U state"]
pub type USTAT_R = crate::BitReader<bool>;
#[doc = "Field `DIR` reader - 1- reverse rotation 0- forward rotation"]
pub type DIR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:27 - ucnt counter"]
    #[inline(always)]
    pub fn ucnt(&self) -> UCNT_R {
        UCNT_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28 - this bit indicate W state"]
    #[inline(always)]
    pub fn wstat(&self) -> WSTAT_R {
        WSTAT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - this bit indicate V state"]
    #[inline(always)]
    pub fn vstat(&self) -> VSTAT_R {
        VSTAT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - this bit indicate U state"]
    #[inline(always)]
    pub fn ustat(&self) -> USTAT_R {
        USTAT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1- reverse rotation 0- forward rotation"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "U counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count_current_u](index.html) module"]
pub struct COUNT_CURRENT_U_SPEC;
impl crate::RegisterSpec for COUNT_CURRENT_U_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [count_current_u::R](R) reader structure"]
impl crate::Readable for COUNT_CURRENT_U_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COUNT_CURRENT_U to value 0"]
impl crate::Resettable for COUNT_CURRENT_U_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
