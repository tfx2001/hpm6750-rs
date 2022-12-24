#[doc = "Register `LED_INTENSE` reader"]
pub struct R(crate::R<LED_INTENSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LED_INTENSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LED_INTENSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LED_INTENSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LED_INTENSE` writer"]
pub struct W(crate::W<LED_INTENSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LED_INTENSE_SPEC>;
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
impl From<crate::W<LED_INTENSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LED_INTENSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLED` reader - Pbutton brightness 0"]
pub type PLED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLED` writer - Pbutton brightness 0"]
pub type PLED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LED_INTENSE_SPEC, u8, u8, 4, O>;
#[doc = "Field `RLED` reader - Rbutton brightness 0"]
pub type RLED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RLED` writer - Rbutton brightness 0"]
pub type RLED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LED_INTENSE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Pbutton brightness 0"]
    #[inline(always)]
    pub fn pled(&self) -> PLED_R {
        PLED_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Rbutton brightness 0"]
    #[inline(always)]
    pub fn rled(&self) -> RLED_R {
        RLED_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pbutton brightness 0"]
    #[inline(always)]
    #[must_use]
    pub fn pled(&mut self) -> PLED_W<0> {
        PLED_W::new(self)
    }
    #[doc = "Bits 16:19 - Rbutton brightness 0"]
    #[inline(always)]
    #[must_use]
    pub fn rled(&mut self) -> RLED_W<16> {
        RLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debounce setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [led_intense](index.html) module"]
pub struct LED_INTENSE_SPEC;
impl crate::RegisterSpec for LED_INTENSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [led_intense::R](R) reader structure"]
impl crate::Readable for LED_INTENSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [led_intense::W](W) writer structure"]
impl crate::Writable for LED_INTENSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LED_INTENSE to value 0"]
impl crate::Resettable for LED_INTENSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
