#[doc = "Register `STA` reader"]
pub struct R(crate::R<STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STA` writer"]
pub struct W(crate::W<STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STA_SPEC>;
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
impl From<crate::W<STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XSTA` reader - pwm timer counter extended start point, should back to this value after reach xrld"]
pub type XSTA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XSTA` writer - pwm timer counter extended start point, should back to this value after reach xrld"]
pub type XSTA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STA_SPEC, u8, u8, 4, O>;
#[doc = "Field `STA` reader - pwm timer counter start value sta/rld will be loaded from shadow register to work register at main counter reload time, or software write unlk.shunlk"]
pub type STA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STA` writer - pwm timer counter start value sta/rld will be loaded from shadow register to work register at main counter reload time, or software write unlk.shunlk"]
pub type STA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STA_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 28:31 - pwm timer counter extended start point, should back to this value after reach xrld"]
    #[inline(always)]
    pub fn xsta(&self) -> XSTA_R {
        XSTA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 4:27 - pwm timer counter start value sta/rld will be loaded from shadow register to work register at main counter reload time, or software write unlk.shunlk"]
    #[inline(always)]
    pub fn sta(&self) -> STA_R {
        STA_R::new(((self.bits >> 4) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 28:31 - pwm timer counter extended start point, should back to this value after reach xrld"]
    #[inline(always)]
    pub fn xsta(&mut self) -> XSTA_W<28> {
        XSTA_W::new(self)
    }
    #[doc = "Bits 4:27 - pwm timer counter start value sta/rld will be loaded from shadow register to work register at main counter reload time, or software write unlk.shunlk"]
    #[inline(always)]
    pub fn sta(&mut self) -> STA_W<4> {
        STA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter start register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](index.html) module"]
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sta::R](R) reader structure"]
impl crate::Readable for STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sta::W](W) writer structure"]
impl crate::Writable for STA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for STA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
