#[doc = "Register `TRIG_CFG` reader"]
pub struct R(crate::R<TRIG_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIG_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIG_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIG_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIG_CFG` writer"]
pub struct W(crate::W<TRIG_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIG_CFG_SPEC>;
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
impl From<crate::W<TRIG_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIG_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEW` reader - Transmit Enable Window For a single shot transmit trigger there is a time of up to 16 ticks of the cycle time where the frame is allowed to start. TWE+1 defines the number of ticks. TEW=0 is a valid setting and shortens the transmit enable window to 1 tick"]
pub type TEW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEW` writer - Transmit Enable Window For a single shot transmit trigger there is a time of up to 16 ticks of the cycle time where the frame is allowed to start. TWE+1 defines the number of ticks. TEW=0 is a valid setting and shortens the transmit enable window to 1 tick"]
pub type TEW_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TRIG_CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `TTYPE` reader - Trigger Type 000b - Immediate Trigger for immediate transmission 001b - Time Trigger for receive triggers 010b - Single Shot Transmit Trigger for exclusive time windows 011b - Transmit Start Trigger for merged arbitrating time windows 100b - Transmit Stop Trigger for merged arbitrating time windows others - no action The time of the trigger is defined by TT_TRIG. TTPTR selects the TB slot for the transmit triggers. See Chapter 6.4 for more details."]
pub type TTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTYPE` writer - Trigger Type 000b - Immediate Trigger for immediate transmission 001b - Time Trigger for receive triggers 010b - Single Shot Transmit Trigger for exclusive time windows 011b - Transmit Start Trigger for merged arbitrating time windows 100b - Transmit Stop Trigger for merged arbitrating time windows others - no action The time of the trigger is defined by TT_TRIG. TTPTR selects the TB slot for the transmit triggers. See Chapter 6.4 for more details."]
pub type TTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TRIG_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `TTPTR` reader - Transmit Trigger TB slot Pointer If TTPTR is too big and points to a slot that is not available, then TEIF is set and no new trigger can be activated after a write access to TT_TRIG_1. If TTPTR points to an empty slot, then TEIF will be set at the moment, when the trigger time is reached."]
pub type TTPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTPTR` writer - Transmit Trigger TB slot Pointer If TTPTR is too big and points to a slot that is not available, then TEIF is set and no new trigger can be activated after a write access to TT_TRIG_1. If TTPTR points to an empty slot, then TEIF will be set at the moment, when the trigger time is reached."]
pub type TTPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TRIG_CFG_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 12:15 - Transmit Enable Window For a single shot transmit trigger there is a time of up to 16 ticks of the cycle time where the frame is allowed to start. TWE+1 defines the number of ticks. TEW=0 is a valid setting and shortens the transmit enable window to 1 tick"]
    #[inline(always)]
    pub fn tew(&self) -> TEW_R {
        TEW_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Trigger Type 000b - Immediate Trigger for immediate transmission 001b - Time Trigger for receive triggers 010b - Single Shot Transmit Trigger for exclusive time windows 011b - Transmit Start Trigger for merged arbitrating time windows 100b - Transmit Stop Trigger for merged arbitrating time windows others - no action The time of the trigger is defined by TT_TRIG. TTPTR selects the TB slot for the transmit triggers. See Chapter 6.4 for more details."]
    #[inline(always)]
    pub fn ttype(&self) -> TTYPE_R {
        TTYPE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 0:5 - Transmit Trigger TB slot Pointer If TTPTR is too big and points to a slot that is not available, then TEIF is set and no new trigger can be activated after a write access to TT_TRIG_1. If TTPTR points to an empty slot, then TEIF will be set at the moment, when the trigger time is reached."]
    #[inline(always)]
    pub fn ttptr(&self) -> TTPTR_R {
        TTPTR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Transmit Enable Window For a single shot transmit trigger there is a time of up to 16 ticks of the cycle time where the frame is allowed to start. TWE+1 defines the number of ticks. TEW=0 is a valid setting and shortens the transmit enable window to 1 tick"]
    #[inline(always)]
    pub fn tew(&mut self) -> TEW_W<12> {
        TEW_W::new(self)
    }
    #[doc = "Bits 8:10 - Trigger Type 000b - Immediate Trigger for immediate transmission 001b - Time Trigger for receive triggers 010b - Single Shot Transmit Trigger for exclusive time windows 011b - Transmit Start Trigger for merged arbitrating time windows 100b - Transmit Stop Trigger for merged arbitrating time windows others - no action The time of the trigger is defined by TT_TRIG. TTPTR selects the TB slot for the transmit triggers. See Chapter 6.4 for more details."]
    #[inline(always)]
    pub fn ttype(&mut self) -> TTYPE_W<8> {
        TTYPE_W::new(self)
    }
    #[doc = "Bits 0:5 - Transmit Trigger TB slot Pointer If TTPTR is too big and points to a slot that is not available, then TEIF is set and no new trigger can be activated after a write access to TT_TRIG_1. If TTPTR points to an empty slot, then TEIF will be set at the moment, when the trigger time is reached."]
    #[inline(always)]
    pub fn ttptr(&mut self) -> TTPTR_W<0> {
        TTPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TTCAN: Trigger Configuration TRIG_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig_cfg](index.html) module"]
pub struct TRIG_CFG_SPEC;
impl crate::RegisterSpec for TRIG_CFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [trig_cfg::R](R) reader structure"]
impl crate::Readable for TRIG_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trig_cfg::W](W) writer structure"]
impl crate::Writable for TRIG_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIG_CFG to value 0"]
impl crate::Resettable for TRIG_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
