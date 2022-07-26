#[doc = "Register `LDO2P5` reader"]
pub struct R(crate::R<LDO2P5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO2P5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO2P5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO2P5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDO2P5` writer"]
pub struct W(crate::W<LDO2P5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO2P5_SPEC>;
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
impl From<crate::W<LDO2P5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO2P5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READY` reader - Ready flag, will set 1ms after enabled or voltage change 0: LDO is not ready for use 1: LDO is ready"]
pub type READY_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` reader - LDO enable 0: turn off LDO 1: turn on LDO"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - LDO enable 0: turn off LDO 1: turn on LDO"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO2P5_SPEC, bool, O>;
#[doc = "Field `VOLT` reader - LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV"]
pub type VOLT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VOLT` writer - LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV"]
pub type VOLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO2P5_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bit 28 - Ready flag, will set 1ms after enabled or voltage change 0: LDO is not ready for use 1: LDO is ready"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 16 - LDO enable 0: turn off LDO 1: turn on LDO"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 0:11 - LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV"]
    #[inline(always)]
    pub fn volt(&self) -> VOLT_R {
        VOLT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - LDO enable 0: turn off LDO 1: turn on LDO"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<16> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 0:11 - LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV"]
    #[inline(always)]
    pub fn volt(&mut self) -> VOLT_W<0> {
        VOLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "2.5V LDO config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo2p5](index.html) module"]
pub struct LDO2P5_SPEC;
impl crate::RegisterSpec for LDO2P5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo2p5::R](R) reader structure"]
impl crate::Readable for LDO2P5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo2p5::W](W) writer structure"]
impl crate::Writable for LDO2P5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LDO2P5 to value 0x09c4"]
impl crate::Resettable for LDO2P5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x09c4
    }
}
