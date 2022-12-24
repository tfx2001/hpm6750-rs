#[doc = "Register `RESET_VIS_CONTROL` reader"]
pub struct R(crate::R<RESET_VIS_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_VIS_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_VIS_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_VIS_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_VIS_CONTROL` writer"]
pub struct W(crate::W<RESET_VIS_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_VIS_CONTROL_SPEC>;
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
impl From<crate::W<RESET_VIS_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_VIS_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET` reader - perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_VIS_CONTROL_SPEC, bool, O>;
#[doc = "Field `HOLD` reader - perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold"]
pub type HOLD_R = crate::BitReader<bool>;
#[doc = "Field `HOLD` writer - perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold"]
pub type HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_VIS_CONTROL_SPEC, bool, O>;
#[doc = "Field `FLAG_WAKE` reader - flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit"]
pub type FLAG_WAKE_R = crate::BitReader<bool>;
#[doc = "Field `FLAG_WAKE` writer - flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit"]
pub type FLAG_WAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_VIS_CONTROL_SPEC, bool, O>;
#[doc = "Field `FLAG` reader - flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit"]
pub type FLAG_R = crate::BitReader<bool>;
#[doc = "Field `FLAG` writer - flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit"]
pub type FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESET_VIS_CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold"]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 30 - flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit"]
    #[inline(always)]
    pub fn flag_wake(&self) -> FLAG_WAKE_R {
        FLAG_WAKE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit"]
    #[inline(always)]
    pub fn flag(&self) -> FLAG_R {
        FLAG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<0> {
        RESET_W::new(self)
    }
    #[doc = "Bit 4 - perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold"]
    #[inline(always)]
    #[must_use]
    pub fn hold(&mut self) -> HOLD_W<4> {
        HOLD_W::new(self)
    }
    #[doc = "Bit 30 - flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit"]
    #[inline(always)]
    #[must_use]
    pub fn flag_wake(&mut self) -> FLAG_WAKE_W<30> {
        FLAG_WAKE_W::new(self)
    }
    #[doc = "Bit 31 - flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit"]
    #[inline(always)]
    #[must_use]
    pub fn flag(&mut self) -> FLAG_W<31> {
        FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_vis_control](index.html) module"]
pub struct RESET_VIS_CONTROL_SPEC;
impl crate::RegisterSpec for RESET_VIS_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_vis_control::R](R) reader structure"]
impl crate::Readable for RESET_VIS_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_vis_control::W](W) writer structure"]
impl crate::Writable for RESET_VIS_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESET_VIS_CONTROL to value 0x8000_0000"]
impl crate::Resettable for RESET_VIS_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
