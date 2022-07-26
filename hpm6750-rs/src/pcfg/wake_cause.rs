#[doc = "Register `WAKE_CAUSE` reader"]
pub struct R(crate::R<WAKE_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKE_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKE_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKE_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKE_CAUSE` writer"]
pub struct W(crate::W<WAKE_CAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKE_CAUSE_SPEC>;
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
impl From<crate::W<WAKE_CAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKE_CAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAUSE` reader - wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 1: debug wakeup bit 4: fuse interrupt bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit11: Security monitor interrupt bit12: Security in PMIC event bit16: Security violation in BATT bit17: GPIO in BATT interrupt bit18: BATT Button interrupt bit19: RTC alarm interrupt"]
pub type CAUSE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CAUSE` writer - wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 1: debug wakeup bit 4: fuse interrupt bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit11: Security monitor interrupt bit12: Security in PMIC event bit16: Security violation in BATT bit17: GPIO in BATT interrupt bit18: BATT Button interrupt bit19: RTC alarm interrupt"]
pub type CAUSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WAKE_CAUSE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 1: debug wakeup bit 4: fuse interrupt bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit11: Security monitor interrupt bit12: Security in PMIC event bit16: Security violation in BATT bit17: GPIO in BATT interrupt bit18: BATT Button interrupt bit19: RTC alarm interrupt"]
    #[inline(always)]
    pub fn cause(&self) -> CAUSE_R {
        CAUSE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 1: debug wakeup bit 4: fuse interrupt bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit11: Security monitor interrupt bit12: Security in PMIC event bit16: Security violation in BATT bit17: GPIO in BATT interrupt bit18: BATT Button interrupt bit19: RTC alarm interrupt"]
    #[inline(always)]
    pub fn cause(&mut self) -> CAUSE_W<0> {
        CAUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake up source\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wake_cause](index.html) module"]
pub struct WAKE_CAUSE_SPEC;
impl crate::RegisterSpec for WAKE_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wake_cause::R](R) reader structure"]
impl crate::Readable for WAKE_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wake_cause::W](W) writer structure"]
impl crate::Writable for WAKE_CAUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKE_CAUSE to value 0"]
impl crate::Resettable for WAKE_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
