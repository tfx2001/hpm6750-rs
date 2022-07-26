#[doc = "Register `WIDTH` reader"]
pub struct R(crate::R<WIDTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIDTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIDTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIDTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIDTH` writer"]
pub struct W(crate::W<WIDTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIDTH_SPEC>;
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
impl From<crate::W<WIDTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIDTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMG` reader - Image Width (it is the max index of pixel counting from 0, assuming the top left pixel is indexed as \\[0,0\\])"]
pub type IMG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IMG` writer - Image Width (it is the max index of pixel counting from 0, assuming the top left pixel is indexed as \\[0,0\\])"]
pub type IMG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WIDTH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Image Width (it is the max index of pixel counting from 0, assuming the top left pixel is indexed as \\[0,0\\])"]
    #[inline(always)]
    pub fn img(&self) -> IMG_R {
        IMG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Image Width (it is the max index of pixel counting from 0, assuming the top left pixel is indexed as \\[0,0\\])"]
    #[inline(always)]
    pub fn img(&mut self) -> IMG_W<0> {
        IMG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Image width register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [width](index.html) module"]
pub struct WIDTH_SPEC;
impl crate::RegisterSpec for WIDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [width::R](R) reader structure"]
impl crate::Readable for WIDTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [width::W](W) writer structure"]
impl crate::Writable for WIDTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WIDTH to value 0"]
impl crate::Resettable for WIDTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
