#[doc = "Register `POWER_TRAP` reader"]
pub struct R(crate::R<POWER_TRAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_TRAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_TRAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_TRAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER_TRAP` writer"]
pub struct W(crate::W<POWER_TRAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_TRAP_SPEC>;
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
impl From<crate::W<POWER_TRAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_TRAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRAP` reader - Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned."]
pub type TRAP_R = crate::BitReader<bool>;
#[doc = "Field `TRAP` writer - Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned."]
pub type TRAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_TRAP_SPEC, bool, O>;
#[doc = "Field `RETENTION` reader - DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage"]
pub type RETENTION_R = crate::BitReader<bool>;
#[doc = "Field `RETENTION` writer - DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage"]
pub type RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_TRAP_SPEC, bool, O>;
#[doc = "Field `TRIGGERED` reader - Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered"]
pub type TRIGGERED_R = crate::BitReader<bool>;
#[doc = "Field `TRIGGERED` writer - Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered"]
pub type TRIGGERED_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_TRAP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned."]
    #[inline(always)]
    pub fn trap(&self) -> TRAP_R {
        TRAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage"]
    #[inline(always)]
    pub fn retention(&self) -> RETENTION_R {
        RETENTION_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered"]
    #[inline(always)]
    pub fn triggered(&self) -> TRIGGERED_R {
        TRIGGERED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned."]
    #[inline(always)]
    #[must_use]
    pub fn trap(&mut self) -> TRAP_W<0> {
        TRAP_W::new(self)
    }
    #[doc = "Bit 16 - DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage"]
    #[inline(always)]
    #[must_use]
    pub fn retention(&mut self) -> RETENTION_W<16> {
        RETENTION_W::new(self)
    }
    #[doc = "Bit 31 - Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered"]
    #[inline(always)]
    #[must_use]
    pub fn triggered(&mut self) -> TRIGGERED_W<31> {
        TRIGGERED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SOC power trap\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power_trap](index.html) module"]
pub struct POWER_TRAP_SPEC;
impl crate::RegisterSpec for POWER_TRAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power_trap::R](R) reader structure"]
impl crate::Readable for POWER_TRAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power_trap::W](W) writer structure"]
impl crate::Writable for POWER_TRAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWER_TRAP to value 0"]
impl crate::Resettable for POWER_TRAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
