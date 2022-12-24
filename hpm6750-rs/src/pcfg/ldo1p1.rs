#[doc = "Register `LDO1P1` reader"]
pub struct R(crate::R<LDO1P1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO1P1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO1P1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO1P1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDO1P1` writer"]
pub struct W(crate::W<LDO1P1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO1P1_SPEC>;
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
impl From<crate::W<LDO1P1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO1P1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VOLT` reader - LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV"]
pub type VOLT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VOLT` writer - LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV"]
pub type VOLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO1P1_SPEC, u16, u16, 12, O>;
#[doc = "Field `ENABLE` reader - LDO enable 0: turn off LDO 1: turn on LDO"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - LDO enable 0: turn off LDO 1: turn on LDO"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO1P1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV"]
    #[inline(always)]
    pub fn volt(&self) -> VOLT_R {
        VOLT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - LDO enable 0: turn off LDO 1: turn on LDO"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV"]
    #[inline(always)]
    #[must_use]
    pub fn volt(&mut self) -> VOLT_W<0> {
        VOLT_W::new(self)
    }
    #[doc = "Bit 16 - LDO enable 0: turn off LDO 1: turn on LDO"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<16> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1V LDO config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo1p1](index.html) module"]
pub struct LDO1P1_SPEC;
impl crate::RegisterSpec for LDO1P1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo1p1::R](R) reader structure"]
impl crate::Readable for LDO1P1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo1p1::W](W) writer structure"]
impl crate::Writable for LDO1P1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LDO1P1 to value 0x0001_044c"]
impl crate::Resettable for LDO1P1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_044c;
}
