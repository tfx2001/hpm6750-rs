#[doc = "Register `function` reader"]
pub struct R(crate::R<FUNCTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNCTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNCTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNCTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `function` writer"]
pub struct W(crate::W<FUNCTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNCTION_SPEC>;
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
impl From<crate::W<FUNCTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNCTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable loop-back output into input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOP_BACK_A {
    #[doc = "1: Function enabled"]
    ENABLE = 1,
    #[doc = "0: Function disabled"]
    DISABLE = 0,
}
impl From<LOOP_BACK_A> for bool {
    #[inline(always)]
    fn from(variant: LOOP_BACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `loop_back` reader - Enable loop-back output into input"]
pub type LOOP_BACK_R = crate::BitReader<LOOP_BACK_A>;
impl LOOP_BACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOP_BACK_A {
        match self.bits {
            true => LOOP_BACK_A::ENABLE,
            false => LOOP_BACK_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LOOP_BACK_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LOOP_BACK_A::DISABLE
    }
}
#[doc = "Field `loop_back` writer - Enable loop-back output into input"]
pub type LOOP_BACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION_SPEC, LOOP_BACK_A, O>;
impl<'a, const O: u8> LOOP_BACK_W<'a, O> {
    #[doc = "Function enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOOP_BACK_A::ENABLE)
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOOP_BACK_A::DISABLE)
    }
}
#[doc = "Enable analog feature in pad"]
pub use LOOP_BACK_A as ANALOG_A;
#[doc = "Field `analog` reader - Enable analog feature in pad"]
pub use LOOP_BACK_R as ANALOG_R;
#[doc = "Field `analog` writer - Enable analog feature in pad"]
pub use LOOP_BACK_W as ANALOG_W;
#[doc = "Field `alternate` reader - Select alternate function for this pad"]
pub type ALTERNATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `alternate` writer - Select alternate function for this pad"]
pub type ALTERNATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FUNCTION_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 16 - Enable loop-back output into input"]
    #[inline(always)]
    pub fn loop_back(&self) -> LOOP_BACK_R {
        LOOP_BACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable analog feature in pad"]
    #[inline(always)]
    pub fn analog(&self) -> ANALOG_R {
        ANALOG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:4 - Select alternate function for this pad"]
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Enable loop-back output into input"]
    #[inline(always)]
    pub fn loop_back(&mut self) -> LOOP_BACK_W<16> {
        LOOP_BACK_W::new(self)
    }
    #[doc = "Bit 8 - Enable analog feature in pad"]
    #[inline(always)]
    pub fn analog(&mut self) -> ANALOG_W<8> {
        ANALOG_W::new(self)
    }
    #[doc = "Bits 0:4 - Select alternate function for this pad"]
    #[inline(always)]
    pub fn alternate(&mut self) -> ALTERNATE_W<0> {
        ALTERNATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select function for this pad\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [function](index.html) module"]
pub struct FUNCTION_SPEC;
impl crate::RegisterSpec for FUNCTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [function::R](R) reader structure"]
impl crate::Readable for FUNCTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [function::W](W) writer structure"]
impl crate::Writable for FUNCTION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets function to value 0"]
impl crate::Resettable for FUNCTION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
