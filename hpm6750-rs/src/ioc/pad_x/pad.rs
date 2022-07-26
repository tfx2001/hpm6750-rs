#[doc = "Register `pad` reader"]
pub struct R(crate::R<PAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pad` writer"]
pub struct W(crate::W<PAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_SPEC>;
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
impl From<crate::W<PAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select pin voltage; only effective to high speed pads\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOLTAGE_A {
    #[doc = "0: 3.3V"]
    V3P3 = 0,
    #[doc = "1: 1.8V"]
    V1P8 = 1,
}
impl From<VOLTAGE_A> for bool {
    #[inline(always)]
    fn from(variant: VOLTAGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `voltage` reader - Select pin voltage; only effective to high speed pads"]
pub type VOLTAGE_R = crate::BitReader<VOLTAGE_A>;
impl VOLTAGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOLTAGE_A {
        match self.bits {
            false => VOLTAGE_A::V3P3,
            true => VOLTAGE_A::V1P8,
        }
    }
    #[doc = "Checks if the value of the field is `V3P3`"]
    #[inline(always)]
    pub fn is_v3p3(&self) -> bool {
        *self == VOLTAGE_A::V3P3
    }
    #[doc = "Checks if the value of the field is `V1P8`"]
    #[inline(always)]
    pub fn is_v1p8(&self) -> bool {
        *self == VOLTAGE_A::V1P8
    }
}
#[doc = "Field `voltage` writer - Select pin voltage; only effective to high speed pads"]
pub type VOLTAGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_SPEC, VOLTAGE_A, O>;
impl<'a, const O: u8> VOLTAGE_W<'a, O> {
    #[doc = "3.3V"]
    #[inline(always)]
    pub fn v3p3(self) -> &'a mut W {
        self.variant(VOLTAGE_A::V3P3)
    }
    #[doc = "1.8V"]
    #[inline(always)]
    pub fn v1p8(self) -> &'a mut W {
        self.variant(VOLTAGE_A::V1P8)
    }
}
#[doc = "Enable or disable open-drain output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPEN_DRAIN_A {
    #[doc = "1: Function enabled"]
    ENABLE = 1,
    #[doc = "0: Function disabled"]
    DISABLE = 0,
}
impl From<OPEN_DRAIN_A> for bool {
    #[inline(always)]
    fn from(variant: OPEN_DRAIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `open_drain` reader - Enable or disable open-drain output"]
pub type OPEN_DRAIN_R = crate::BitReader<OPEN_DRAIN_A>;
impl OPEN_DRAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPEN_DRAIN_A {
        match self.bits {
            true => OPEN_DRAIN_A::ENABLE,
            false => OPEN_DRAIN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OPEN_DRAIN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OPEN_DRAIN_A::DISABLE
    }
}
#[doc = "Field `open_drain` writer - Enable or disable open-drain output"]
pub type OPEN_DRAIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_SPEC, OPEN_DRAIN_A, O>;
impl<'a, const O: u8> OPEN_DRAIN_W<'a, O> {
    #[doc = "Function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(OPEN_DRAIN_A::ENABLE)
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OPEN_DRAIN_A::DISABLE)
    }
}
#[doc = "schmitt trigger enable, only avaiable in high-speed IO"]
pub use OPEN_DRAIN_A as SCHMITT_A;
#[doc = "Field `schmitt` reader - schmitt trigger enable, only avaiable in high-speed IO"]
pub use OPEN_DRAIN_R as SCHMITT_R;
#[doc = "Field `schmitt` writer - schmitt trigger enable, only avaiable in high-speed IO"]
pub use OPEN_DRAIN_W as SCHMITT_W;
#[doc = "Pull direction on this pad if pull enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PULL_DIRECTION_A {
    #[doc = "0: This pad is internally pulled up"]
    UP = 0,
    #[doc = "1: This pad is internally pulled down"]
    DOWN = 1,
}
impl From<PULL_DIRECTION_A> for bool {
    #[inline(always)]
    fn from(variant: PULL_DIRECTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pull_direction` reader - Pull direction on this pad if pull enabled"]
pub type PULL_DIRECTION_R = crate::BitReader<PULL_DIRECTION_A>;
impl PULL_DIRECTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PULL_DIRECTION_A {
        match self.bits {
            false => PULL_DIRECTION_A::UP,
            true => PULL_DIRECTION_A::DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == PULL_DIRECTION_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == PULL_DIRECTION_A::DOWN
    }
}
#[doc = "Field `pull_direction` writer - Pull direction on this pad if pull enabled"]
pub type PULL_DIRECTION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PAD_SPEC, PULL_DIRECTION_A, O>;
impl<'a, const O: u8> PULL_DIRECTION_W<'a, O> {
    #[doc = "This pad is internally pulled up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(PULL_DIRECTION_A::UP)
    }
    #[doc = "This pad is internally pulled down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(PULL_DIRECTION_A::DOWN)
    }
}
#[doc = "Enable internal pull on this pin"]
pub use super::function::LOOP_BACK_A as PULL_ENABLE_A;
#[doc = "Field `pull_enable` reader - Enable internal pull on this pin"]
pub use super::function::LOOP_BACK_R as PULL_ENABLE_R;
#[doc = "Field `pull_enable` writer - Enable internal pull on this pin"]
pub type PULL_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_SPEC, PULL_ENABLE_A, O>;
impl<'a, const O: u8> PULL_ENABLE_W<'a, O> {
    #[doc = "Function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PULL_ENABLE_A::ENABLE)
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PULL_ENABLE_A::DISABLE)
    }
}
#[doc = "Pad drive strength, differs from pin type and voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRIVE_STRENGTH_A {
    #[doc = "0: 85.61Ω on 3.3V high speed, 84.07Ω on 1.8V high speed, 4mA on normal pad"]
    DRIVE_0 = 0,
    #[doc = "1: 61.2Ω on 3.3V high speed, 60.14Ω on 1.8V high speed, 8mA on normal pad"]
    DRIVE_1 = 1,
    #[doc = "2: 42.88Ω on 3.3V high speed, 42.15Ω on 1.8V high speed, N/A on normal pad"]
    DRIVE_2 = 2,
    #[doc = "3: 35.76Ω on 3.3V high speed, 35.19Ω on 1.8V high speed, 12mA on normal pad"]
    DRIVE_3 = 3,
    #[doc = "7: 30.67Ω on 3.3V high speed, 30.2Ω on 1.8V high speed, N/A on normal pad"]
    DRIVE_7 = 7,
}
impl From<DRIVE_STRENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVE_STRENGTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `drive_strength` reader - Pad drive strength, differs from pin type and voltage"]
pub type DRIVE_STRENGTH_R = crate::FieldReader<u8, DRIVE_STRENGTH_A>;
impl DRIVE_STRENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DRIVE_STRENGTH_A> {
        match self.bits {
            0 => Some(DRIVE_STRENGTH_A::DRIVE_0),
            1 => Some(DRIVE_STRENGTH_A::DRIVE_1),
            2 => Some(DRIVE_STRENGTH_A::DRIVE_2),
            3 => Some(DRIVE_STRENGTH_A::DRIVE_3),
            7 => Some(DRIVE_STRENGTH_A::DRIVE_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DRIVE_0`"]
    #[inline(always)]
    pub fn is_drive_0(&self) -> bool {
        *self == DRIVE_STRENGTH_A::DRIVE_0
    }
    #[doc = "Checks if the value of the field is `DRIVE_1`"]
    #[inline(always)]
    pub fn is_drive_1(&self) -> bool {
        *self == DRIVE_STRENGTH_A::DRIVE_1
    }
    #[doc = "Checks if the value of the field is `DRIVE_2`"]
    #[inline(always)]
    pub fn is_drive_2(&self) -> bool {
        *self == DRIVE_STRENGTH_A::DRIVE_2
    }
    #[doc = "Checks if the value of the field is `DRIVE_3`"]
    #[inline(always)]
    pub fn is_drive_3(&self) -> bool {
        *self == DRIVE_STRENGTH_A::DRIVE_3
    }
    #[doc = "Checks if the value of the field is `DRIVE_7`"]
    #[inline(always)]
    pub fn is_drive_7(&self) -> bool {
        *self == DRIVE_STRENGTH_A::DRIVE_7
    }
}
#[doc = "Field `drive_strength` writer - Pad drive strength, differs from pin type and voltage"]
pub type DRIVE_STRENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_SPEC, u8, DRIVE_STRENGTH_A, 3, O>;
impl<'a, const O: u8> DRIVE_STRENGTH_W<'a, O> {
    #[doc = "85.61Ω on 3.3V high speed, 84.07Ω on 1.8V high speed, 4mA on normal pad"]
    #[inline(always)]
    pub fn drive_0(self) -> &'a mut W {
        self.variant(DRIVE_STRENGTH_A::DRIVE_0)
    }
    #[doc = "61.2Ω on 3.3V high speed, 60.14Ω on 1.8V high speed, 8mA on normal pad"]
    #[inline(always)]
    pub fn drive_1(self) -> &'a mut W {
        self.variant(DRIVE_STRENGTH_A::DRIVE_1)
    }
    #[doc = "42.88Ω on 3.3V high speed, 42.15Ω on 1.8V high speed, N/A on normal pad"]
    #[inline(always)]
    pub fn drive_2(self) -> &'a mut W {
        self.variant(DRIVE_STRENGTH_A::DRIVE_2)
    }
    #[doc = "35.76Ω on 3.3V high speed, 35.19Ω on 1.8V high speed, 12mA on normal pad"]
    #[inline(always)]
    pub fn drive_3(self) -> &'a mut W {
        self.variant(DRIVE_STRENGTH_A::DRIVE_3)
    }
    #[doc = "30.67Ω on 3.3V high speed, 30.2Ω on 1.8V high speed, N/A on normal pad"]
    #[inline(always)]
    pub fn drive_7(self) -> &'a mut W {
        self.variant(DRIVE_STRENGTH_A::DRIVE_7)
    }
}
impl R {
    #[doc = "Bit 14 - Select pin voltage; only effective to high speed pads"]
    #[inline(always)]
    pub fn voltage(&self) -> VOLTAGE_R {
        VOLTAGE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable or disable open-drain output"]
    #[inline(always)]
    pub fn open_drain(&self) -> OPEN_DRAIN_R {
        OPEN_DRAIN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - schmitt trigger enable, only avaiable in high-speed IO"]
    #[inline(always)]
    pub fn schmitt(&self) -> SCHMITT_R {
        SCHMITT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Pull direction on this pad if pull enabled"]
    #[inline(always)]
    pub fn pull_direction(&self) -> PULL_DIRECTION_R {
        PULL_DIRECTION_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable internal pull on this pin"]
    #[inline(always)]
    pub fn pull_enable(&self) -> PULL_ENABLE_R {
        PULL_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:2 - Pad drive strength, differs from pin type and voltage"]
    #[inline(always)]
    pub fn drive_strength(&self) -> DRIVE_STRENGTH_R {
        DRIVE_STRENGTH_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 14 - Select pin voltage; only effective to high speed pads"]
    #[inline(always)]
    pub fn voltage(&mut self) -> VOLTAGE_W<14> {
        VOLTAGE_W::new(self)
    }
    #[doc = "Bit 13 - Enable or disable open-drain output"]
    #[inline(always)]
    pub fn open_drain(&mut self) -> OPEN_DRAIN_W<13> {
        OPEN_DRAIN_W::new(self)
    }
    #[doc = "Bit 12 - schmitt trigger enable, only avaiable in high-speed IO"]
    #[inline(always)]
    pub fn schmitt(&mut self) -> SCHMITT_W<12> {
        SCHMITT_W::new(self)
    }
    #[doc = "Bit 11 - Pull direction on this pad if pull enabled"]
    #[inline(always)]
    pub fn pull_direction(&mut self) -> PULL_DIRECTION_W<11> {
        PULL_DIRECTION_W::new(self)
    }
    #[doc = "Bit 4 - Enable internal pull on this pin"]
    #[inline(always)]
    pub fn pull_enable(&mut self) -> PULL_ENABLE_W<4> {
        PULL_ENABLE_W::new(self)
    }
    #[doc = "Bits 0:2 - Pad drive strength, differs from pin type and voltage"]
    #[inline(always)]
    pub fn drive_strength(&mut self) -> DRIVE_STRENGTH_W<0> {
        DRIVE_STRENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configurate pad settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad](index.html) module"]
pub struct PAD_SPEC;
impl crate::RegisterSpec for PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad::R](R) reader structure"]
impl crate::Readable for PAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad::W](W) writer structure"]
impl crate::Writable for PAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pad to value 0x1010"]
impl crate::Resettable for PAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1010
    }
}
