#[doc = "Register `PWMCFG_4` reader"]
pub struct R(crate::R<PWMCFG_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMCFG_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMCFG_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMCFG_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWMCFG_4` writer"]
pub struct W(crate::W<PWMCFG_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMCFG_4_SPEC>;
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
impl From<crate::W<PWMCFG_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMCFG_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEADAREA` reader - This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
pub type DEADAREA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DEADAREA` writer - This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
pub type DEADAREA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWMCFG_4_SPEC, u32, u32, 20, O>;
#[doc = "Field `PAIR` reader - 1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
pub type PAIR_R = crate::BitReader<bool>;
#[doc = "Field `PAIR` writer - 1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
pub type PAIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCFG_4_SPEC, bool, O>;
#[doc = "Field `FRCSRCSEL` reader - Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
pub type FRCSRCSEL_R = crate::BitReader<bool>;
#[doc = "Field `FRCSRCSEL` writer - Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
pub type FRCSRCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCFG_4_SPEC, bool, O>;
#[doc = "Field `FAULTRECTIME` reader - This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
pub type FAULTRECTIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAULTRECTIME` writer - This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
pub type FAULTRECTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWMCFG_4_SPEC, u8, u8, 2, O>;
#[doc = "Field `FAULTMODE` reader - This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
pub type FAULTMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAULTMODE` writer - This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
pub type FAULTMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWMCFG_4_SPEC, u8, u8, 2, O>;
#[doc = "Field `FRCSHDWUPT` reader - This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
pub type FRCSHDWUPT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRCSHDWUPT` writer - This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
pub type FRCSHDWUPT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWMCFG_4_SPEC, u8, u8, 2, O>;
#[doc = "Field `OEN` reader - PWM output enable 1- output is enabled 0- output is disabled"]
pub type OEN_R = crate::BitReader<bool>;
#[doc = "Field `OEN` writer - PWM output enable 1- output is enabled 0- output is disabled"]
pub type OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWMCFG_4_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:19 - This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
    #[inline(always)]
    pub fn deadarea(&self) -> DEADAREA_R {
        DEADAREA_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - 1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
    #[inline(always)]
    pub fn pair(&self) -> PAIR_R {
        PAIR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
    #[inline(always)]
    pub fn frcsrcsel(&self) -> FRCSRCSEL_R {
        FRCSRCSEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
    #[inline(always)]
    pub fn faultrectime(&self) -> FAULTRECTIME_R {
        FAULTRECTIME_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
    #[inline(always)]
    pub fn faultmode(&self) -> FAULTMODE_R {
        FAULTMODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    #[inline(always)]
    pub fn frcshdwupt(&self) -> FRCSHDWUPT_R {
        FRCSHDWUPT_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - PWM output enable 1- output is enabled 0- output is disabled"]
    #[inline(always)]
    pub fn oen(&self) -> OEN_R {
        OEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn deadarea(&mut self) -> DEADAREA_W<0> {
        DEADAREA_W::new(self)
    }
    #[doc = "Bit 20 - 1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
    #[inline(always)]
    #[must_use]
    pub fn pair(&mut self) -> PAIR_W<20> {
        PAIR_W::new(self)
    }
    #[doc = "Bit 21 - Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
    #[inline(always)]
    #[must_use]
    pub fn frcsrcsel(&mut self) -> FRCSRCSEL_W<21> {
        FRCSRCSEL_W::new(self)
    }
    #[doc = "Bits 22:23 - This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
    #[inline(always)]
    #[must_use]
    pub fn faultrectime(&mut self) -> FAULTRECTIME_W<22> {
        FAULTRECTIME_W::new(self)
    }
    #[doc = "Bits 24:25 - This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
    #[inline(always)]
    #[must_use]
    pub fn faultmode(&mut self) -> FAULTMODE_W<24> {
        FAULTMODE_W::new(self)
    }
    #[doc = "Bits 26:27 - This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    #[inline(always)]
    #[must_use]
    pub fn frcshdwupt(&mut self) -> FRCSHDWUPT_W<26> {
        FRCSHDWUPT_W::new(self)
    }
    #[doc = "Bit 28 - PWM output enable 1- output is enabled 0- output is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn oen(&mut self) -> OEN_W<28> {
        OEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM channel configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmcfg_4](index.html) module"]
pub struct PWMCFG_4_SPEC;
impl crate::RegisterSpec for PWMCFG_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmcfg_4::R](R) reader structure"]
impl crate::Readable for PWMCFG_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmcfg_4::W](W) writer structure"]
impl crate::Writable for PWMCFG_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWMCFG_4 to value 0"]
impl crate::Resettable for PWMCFG_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
