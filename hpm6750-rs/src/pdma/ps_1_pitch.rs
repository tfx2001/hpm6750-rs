#[doc = "Register `PS_1_PITCH` reader"]
pub struct R(crate::R<PS_1_PITCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PS_1_PITCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PS_1_PITCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PS_1_PITCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PS_1_PITCH` writer"]
pub struct W(crate::W<PS_1_PITCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PS_1_PITCH_SPEC>;
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
impl From<crate::W<PS_1_PITCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PS_1_PITCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTELEN` reader - Indicates the number of bytes in memory between two vertically adjacent pixels."]
pub type BYTELEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BYTELEN` writer - Indicates the number of bytes in memory between two vertically adjacent pixels."]
pub type BYTELEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PS_1_PITCH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Indicates the number of bytes in memory between two vertically adjacent pixels."]
    #[inline(always)]
    pub fn bytelen(&self) -> BYTELEN_R {
        BYTELEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates the number of bytes in memory between two vertically adjacent pixels."]
    #[inline(always)]
    pub fn bytelen(&mut self) -> BYTELEN_W<0> {
        BYTELEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer data pitch register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_1_pitch](index.html) module"]
pub struct PS_1_PITCH_SPEC;
impl crate::RegisterSpec for PS_1_PITCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ps_1_pitch::R](R) reader structure"]
impl crate::Readable for PS_1_PITCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ps_1_pitch::W](W) writer structure"]
impl crate::Writable for PS_1_PITCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PS_1_PITCH to value 0"]
impl crate::Resettable for PS_1_PITCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
