#[doc = "Register `OUT_PITCH` reader"]
pub struct R(crate::R<OUT_PITCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_PITCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_PITCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_PITCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_PITCH` writer"]
pub struct W(crate::W<OUT_PITCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_PITCH_SPEC>;
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
impl From<crate::W<OUT_PITCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_PITCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYTELEN` reader - Indicates the number of bytes in memory between two vertically adjacent pixels."]
pub type BYTELEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BYTELEN` writer - Indicates the number of bytes in memory between two vertically adjacent pixels."]
pub type BYTELEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUT_PITCH_SPEC, u16, u16, 16, O>;
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
#[doc = "Outlayer Pitch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_pitch](index.html) module"]
pub struct OUT_PITCH_SPEC;
impl crate::RegisterSpec for OUT_PITCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_pitch::R](R) reader structure"]
impl crate::Readable for OUT_PITCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_pitch::W](W) writer structure"]
impl crate::Writable for OUT_PITCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_PITCH to value 0"]
impl crate::Resettable for OUT_PITCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
