#[doc = "Register `TTCFG` reader"]
pub struct R(crate::R<TTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTCFG` writer"]
pub struct W(crate::W<TTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTCFG_SPEC>;
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
impl From<crate::W<TTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTEN` reader - Time Trigger Enable 1 - TTCAN enabled, timer is running0 - disabled"]
pub type TTEN_R = crate::BitReader<bool>;
#[doc = "Field `TTEN` writer - Time Trigger Enable 1 - TTCAN enabled, timer is running0 - disabled"]
pub type TTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, TTCFG_SPEC, bool, O>;
#[doc = "Field `T_PRESC` reader - TTCAN Timer PRESCaler 00b - 1 01b - 2 10b - 4 11b - 8 The TTCAN time base is a CAN bittime defined by S_PRES, S_SEG_1 and S_SEG_2.With T_PRESC an additional prescaling factor of 1, 2, 4 or 8 is defined. T_PRESC can only be modified if TTEN=0, but it is possible to modify T_PRESC and setTTEN simultaneously with one write access."]
pub type T_PRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_PRESC` writer - TTCAN Timer PRESCaler 00b - 1 01b - 2 10b - 4 11b - 8 The TTCAN time base is a CAN bittime defined by S_PRES, S_SEG_1 and S_SEG_2.With T_PRESC an additional prescaling factor of 1, 2, 4 or 8 is defined. T_PRESC can only be modified if TTEN=0, but it is possible to modify T_PRESC and setTTEN simultaneously with one write access."]
pub type T_PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TTCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `TTIF` reader - Time Trigger Interrupt Flag TTIF will be set if TTIE is set and the cycle time is equal to the trigger time TT_TRIG. Writing an one to TTIF resets it. Writing a zero has no impact.TTIF will be set only once. If TT_TRIG gets not updated, then TTIF will be not setagain in the next basic cycle."]
pub type TTIF_R = crate::BitReader<bool>;
#[doc = "Field `TTIF` writer - Time Trigger Interrupt Flag TTIF will be set if TTIE is set and the cycle time is equal to the trigger time TT_TRIG. Writing an one to TTIF resets it. Writing a zero has no impact.TTIF will be set only once. If TT_TRIG gets not updated, then TTIF will be not setagain in the next basic cycle."]
pub type TTIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, TTCFG_SPEC, bool, O>;
#[doc = "Field `TTIE` reader - Time Trigger Interrupt Enable If TTIE is set, then TTIF will be set if the cycle time is equal to the trigger timeTT_TRIG."]
pub type TTIE_R = crate::BitReader<bool>;
#[doc = "Field `TTIE` writer - Time Trigger Interrupt Enable If TTIE is set, then TTIF will be set if the cycle time is equal to the trigger timeTT_TRIG."]
pub type TTIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TTCFG_SPEC, bool, O>;
#[doc = "Field `TEIF` reader - Trigger Error Interrupt Flag The conditions when TEIF will be set, are defined in Chapter 6.4. There is no bit toenable or disable the handling of TEIF"]
pub type TEIF_R = crate::BitReader<bool>;
#[doc = "Field `TEIF` writer - Trigger Error Interrupt Flag The conditions when TEIF will be set, are defined in Chapter 6.4. There is no bit toenable or disable the handling of TEIF"]
pub type TEIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, TTCFG_SPEC, bool, O>;
#[doc = "Field `WTIF` reader - Watch Trigger Interrupt Flag WTIF will be set if the cycle count reaches the limited defined by TT_WTRIG and WTIE is set."]
pub type WTIF_R = crate::BitReader<bool>;
#[doc = "Field `WTIF` writer - Watch Trigger Interrupt Flag WTIF will be set if the cycle count reaches the limited defined by TT_WTRIG and WTIE is set."]
pub type WTIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, TTCFG_SPEC, bool, O>;
#[doc = "Field `WTIE` reader - Watch Trigger Interrupt Enable"]
pub type WTIE_R = crate::BitReader<bool>;
#[doc = "Field `WTIE` writer - Watch Trigger Interrupt Enable"]
pub type WTIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, TTCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Time Trigger Enable 1 - TTCAN enabled, timer is running0 - disabled"]
    #[inline(always)]
    pub fn tten(&self) -> TTEN_R {
        TTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - TTCAN Timer PRESCaler 00b - 1 01b - 2 10b - 4 11b - 8 The TTCAN time base is a CAN bittime defined by S_PRES, S_SEG_1 and S_SEG_2.With T_PRESC an additional prescaling factor of 1, 2, 4 or 8 is defined. T_PRESC can only be modified if TTEN=0, but it is possible to modify T_PRESC and setTTEN simultaneously with one write access."]
    #[inline(always)]
    pub fn t_presc(&self) -> T_PRESC_R {
        T_PRESC_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Time Trigger Interrupt Flag TTIF will be set if TTIE is set and the cycle time is equal to the trigger time TT_TRIG. Writing an one to TTIF resets it. Writing a zero has no impact.TTIF will be set only once. If TT_TRIG gets not updated, then TTIF will be not setagain in the next basic cycle."]
    #[inline(always)]
    pub fn ttif(&self) -> TTIF_R {
        TTIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Time Trigger Interrupt Enable If TTIE is set, then TTIF will be set if the cycle time is equal to the trigger timeTT_TRIG."]
    #[inline(always)]
    pub fn ttie(&self) -> TTIE_R {
        TTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger Error Interrupt Flag The conditions when TEIF will be set, are defined in Chapter 6.4. There is no bit toenable or disable the handling of TEIF"]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watch Trigger Interrupt Flag WTIF will be set if the cycle count reaches the limited defined by TT_WTRIG and WTIE is set."]
    #[inline(always)]
    pub fn wtif(&self) -> WTIF_R {
        WTIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn wtie(&self) -> WTIE_R {
        WTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time Trigger Enable 1 - TTCAN enabled, timer is running0 - disabled"]
    #[inline(always)]
    #[must_use]
    pub fn tten(&mut self) -> TTEN_W<0> {
        TTEN_W::new(self)
    }
    #[doc = "Bits 1:2 - TTCAN Timer PRESCaler 00b - 1 01b - 2 10b - 4 11b - 8 The TTCAN time base is a CAN bittime defined by S_PRES, S_SEG_1 and S_SEG_2.With T_PRESC an additional prescaling factor of 1, 2, 4 or 8 is defined. T_PRESC can only be modified if TTEN=0, but it is possible to modify T_PRESC and setTTEN simultaneously with one write access."]
    #[inline(always)]
    #[must_use]
    pub fn t_presc(&mut self) -> T_PRESC_W<1> {
        T_PRESC_W::new(self)
    }
    #[doc = "Bit 3 - Time Trigger Interrupt Flag TTIF will be set if TTIE is set and the cycle time is equal to the trigger time TT_TRIG. Writing an one to TTIF resets it. Writing a zero has no impact.TTIF will be set only once. If TT_TRIG gets not updated, then TTIF will be not setagain in the next basic cycle."]
    #[inline(always)]
    #[must_use]
    pub fn ttif(&mut self) -> TTIF_W<3> {
        TTIF_W::new(self)
    }
    #[doc = "Bit 4 - Time Trigger Interrupt Enable If TTIE is set, then TTIF will be set if the cycle time is equal to the trigger timeTT_TRIG."]
    #[inline(always)]
    #[must_use]
    pub fn ttie(&mut self) -> TTIE_W<4> {
        TTIE_W::new(self)
    }
    #[doc = "Bit 5 - Trigger Error Interrupt Flag The conditions when TEIF will be set, are defined in Chapter 6.4. There is no bit toenable or disable the handling of TEIF"]
    #[inline(always)]
    #[must_use]
    pub fn teif(&mut self) -> TEIF_W<5> {
        TEIF_W::new(self)
    }
    #[doc = "Bit 6 - Watch Trigger Interrupt Flag WTIF will be set if the cycle count reaches the limited defined by TT_WTRIG and WTIE is set."]
    #[inline(always)]
    #[must_use]
    pub fn wtif(&mut self) -> WTIF_W<6> {
        WTIF_W::new(self)
    }
    #[doc = "Bit 7 - Watch Trigger Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wtie(&mut self) -> WTIE_W<7> {
        WTIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TTCAN: Time Trigger Configuration TTCFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttcfg](index.html) module"]
pub struct TTCFG_SPEC;
impl crate::RegisterSpec for TTCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ttcfg::R](R) reader structure"]
impl crate::Readable for TTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttcfg::W](W) writer structure"]
impl crate::Writable for TTCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TTCFG to value 0"]
impl crate::Resettable for TTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
