#[doc = "Register `POWER_CON_STATUS` reader"]
pub struct R(crate::R<POWER_CON_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_CON_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_CON_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_CON_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER_CON_STATUS` writer"]
pub struct W(crate::W<POWER_CON_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_CON_STATUS_SPEC>;
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
impl From<crate::W<POWER_CON_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_CON_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLAG` reader - flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit"]
pub type FLAG_R = crate::BitReader<bool>;
#[doc = "Field `FLAG` writer - flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit"]
pub type FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_CON_STATUS_SPEC, bool, O>;
#[doc = "Field `FLAG_WAKE` reader - flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit"]
pub type FLAG_WAKE_R = crate::BitReader<bool>;
#[doc = "Field `FLAG_WAKE` writer - flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit"]
pub type FLAG_WAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_CON_STATUS_SPEC, bool, O>;
#[doc = "Field `LF_DISABLE` reader - low fanout power switch disable 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
pub type LF_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `LF_ACK` reader - low fanout power switch feedback 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
pub type LF_ACK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 31 - flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit"]
    #[inline(always)]
    pub fn flag(&self) -> FLAG_R {
        FLAG_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit"]
    #[inline(always)]
    pub fn flag_wake(&self) -> FLAG_WAKE_R {
        FLAG_WAKE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 12 - low fanout power switch disable 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
    #[inline(always)]
    pub fn lf_disable(&self) -> LF_DISABLE_R {
        LF_DISABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 8 - low fanout power switch feedback 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
    #[inline(always)]
    pub fn lf_ack(&self) -> LF_ACK_R {
        LF_ACK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit"]
    #[inline(always)]
    pub fn flag(&mut self) -> FLAG_W<31> {
        FLAG_W::new(self)
    }
    #[doc = "Bit 30 - flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit"]
    #[inline(always)]
    pub fn flag_wake(&mut self) -> FLAG_WAKE_W<30> {
        FLAG_WAKE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_con_status](index.html) module"]
pub struct POWER_CON_STATUS_SPEC;
impl crate::RegisterSpec for POWER_CON_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power_con_status::R](R) reader structure"]
impl crate::Readable for POWER_CON_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_con_status::W](W) writer structure"]
impl crate::Writable for POWER_CON_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POWER_CON_STATUS to value 0x8000_0000"]
impl crate::Resettable for POWER_CON_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
