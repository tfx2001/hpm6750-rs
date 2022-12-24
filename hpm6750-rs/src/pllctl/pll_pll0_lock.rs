#[doc = "Register `PLL_PLL0_LOCK` reader"]
pub struct R(crate::R<PLL_PLL0_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_PLL0_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_PLL0_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_PLL0_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_PLL0_LOCK` writer"]
pub struct W(crate::W<PLL_PLL0_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_PLL0_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PLL_PLL0_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_PLL0_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK_SS_DIVVAL` reader - lock bit of field ss_divval 0: field is open foe software to change 1: field is locked, not changeable"]
pub type LOCK_SS_DIVVAL_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_SS_DIVVAL` writer - lock bit of field ss_divval 0: field is open foe software to change 1: field is locked, not changeable"]
pub type LOCK_SS_DIVVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_PLL0_LOCK_SPEC, bool, O>;
#[doc = "Field `LOCK_SS_SPEAD` reader - lock bit of field ss_spead 0: field is open foe software to change 1: field is locked, not changeable"]
pub type LOCK_SS_SPEAD_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_SS_SPEAD` writer - lock bit of field ss_spead 0: field is open foe software to change 1: field is locked, not changeable"]
pub type LOCK_SS_SPEAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_PLL0_LOCK_SPEC, bool, O>;
#[doc = "Field `LOCK_POSTDIV1` reader - lock bit of field postdiv1 0: field is open foe software to change 1: field is locked, not changeable"]
pub type LOCK_POSTDIV1_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_POSTDIV1` writer - lock bit of field postdiv1 0: field is open foe software to change 1: field is locked, not changeable"]
pub type LOCK_POSTDIV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_PLL0_LOCK_SPEC, bool, O>;
#[doc = "Field `LOCK_REFDIV` reader - lock bit of field refdiv 0: field is open foe software to change 1: field is locked, not changeable"]
pub type LOCK_REFDIV_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REFDIV` writer - lock bit of field refdiv 0: field is open foe software to change 1: field is locked, not changeable"]
pub type LOCK_REFDIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_PLL0_LOCK_SPEC, bool, O>;
#[doc = "Field `LOCK_SS_RSTPTR` reader - lock bit of field ss_rstptr 0: field is open foe software to change 1: field is locked, not changeable"]
pub type LOCK_SS_RSTPTR_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_SS_RSTPTR` writer - lock bit of field ss_rstptr 0: field is open foe software to change 1: field is locked, not changeable"]
pub type LOCK_SS_RSTPTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_PLL0_LOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - lock bit of field ss_divval 0: field is open foe software to change 1: field is locked, not changeable"]
    #[inline(always)]
    pub fn lock_ss_divval(&self) -> LOCK_SS_DIVVAL_R {
        LOCK_SS_DIVVAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - lock bit of field ss_spead 0: field is open foe software to change 1: field is locked, not changeable"]
    #[inline(always)]
    pub fn lock_ss_spead(&self) -> LOCK_SS_SPEAD_R {
        LOCK_SS_SPEAD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 20 - lock bit of field postdiv1 0: field is open foe software to change 1: field is locked, not changeable"]
    #[inline(always)]
    pub fn lock_postdiv1(&self) -> LOCK_POSTDIV1_R {
        LOCK_POSTDIV1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - lock bit of field refdiv 0: field is open foe software to change 1: field is locked, not changeable"]
    #[inline(always)]
    pub fn lock_refdiv(&self) -> LOCK_REFDIV_R {
        LOCK_REFDIV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - lock bit of field ss_rstptr 0: field is open foe software to change 1: field is locked, not changeable"]
    #[inline(always)]
    pub fn lock_ss_rstptr(&self) -> LOCK_SS_RSTPTR_R {
        LOCK_SS_RSTPTR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - lock bit of field ss_divval 0: field is open foe software to change 1: field is locked, not changeable"]
    #[inline(always)]
    #[must_use]
    pub fn lock_ss_divval(&mut self) -> LOCK_SS_DIVVAL_W<8> {
        LOCK_SS_DIVVAL_W::new(self)
    }
    #[doc = "Bit 14 - lock bit of field ss_spead 0: field is open foe software to change 1: field is locked, not changeable"]
    #[inline(always)]
    #[must_use]
    pub fn lock_ss_spead(&mut self) -> LOCK_SS_SPEAD_W<14> {
        LOCK_SS_SPEAD_W::new(self)
    }
    #[doc = "Bit 20 - lock bit of field postdiv1 0: field is open foe software to change 1: field is locked, not changeable"]
    #[inline(always)]
    #[must_use]
    pub fn lock_postdiv1(&mut self) -> LOCK_POSTDIV1_W<20> {
        LOCK_POSTDIV1_W::new(self)
    }
    #[doc = "Bit 24 - lock bit of field refdiv 0: field is open foe software to change 1: field is locked, not changeable"]
    #[inline(always)]
    #[must_use]
    pub fn lock_refdiv(&mut self) -> LOCK_REFDIV_W<24> {
        LOCK_REFDIV_W::new(self)
    }
    #[doc = "Bit 31 - lock bit of field ss_rstptr 0: field is open foe software to change 1: field is locked, not changeable"]
    #[inline(always)]
    #[must_use]
    pub fn lock_ss_rstptr(&mut self) -> LOCK_SS_RSTPTR_W<31> {
        LOCK_SS_RSTPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLx lock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_pll0_lock](index.html) module"]
pub struct PLL_PLL0_LOCK_SPEC;
impl crate::RegisterSpec for PLL_PLL0_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_pll0_lock::R](R) reader structure"]
impl crate::Readable for PLL_PLL0_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_pll0_lock::W](W) writer structure"]
impl crate::Writable for PLL_PLL0_LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_PLL0_LOCK to value 0"]
impl crate::Resettable for PLL_PLL0_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
