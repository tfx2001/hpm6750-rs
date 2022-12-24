#[doc = "Register `PLL_PLL1_STATUS` reader"]
pub struct R(crate::R<PLL_PLL1_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_PLL1_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_PLL1_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_PLL1_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PLL_LOCK_SYNC` reader - No description avaiable"]
pub type PLL_LOCK_SYNC_R = crate::BitReader<bool>;
#[doc = "Field `PLL_LOCK_COMB` reader - No description avaiable"]
pub type PLL_LOCK_COMB_R = crate::BitReader<bool>;
#[doc = "Field `RESPONSE` reader - response to SYSCTL, PLL is power down when both enable and response are 0."]
pub type RESPONSE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` reader - enable from SYSCTL block"]
pub type ENABLE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn pll_lock_sync(&self) -> PLL_LOCK_SYNC_R {
        PLL_LOCK_SYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    pub fn pll_lock_comb(&self) -> PLL_LOCK_COMB_R {
        PLL_LOCK_COMB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - response to SYSCTL, PLL is power down when both enable and response are 0."]
    #[inline(always)]
    pub fn response(&self) -> RESPONSE_R {
        RESPONSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 27 - enable from SYSCTL block"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "PLLx status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_pll1_status](index.html) module"]
pub struct PLL_PLL1_STATUS_SPEC;
impl crate::RegisterSpec for PLL_PLL1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_pll1_status::R](R) reader structure"]
impl crate::Readable for PLL_PLL1_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PLL_PLL1_STATUS to value 0"]
impl crate::Resettable for PLL_PLL1_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
