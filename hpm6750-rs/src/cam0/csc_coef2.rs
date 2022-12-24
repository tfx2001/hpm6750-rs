#[doc = "Register `CSC_COEF2` reader"]
pub struct R(crate::R<CSC_COEF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSC_COEF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSC_COEF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSC_COEF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSC_COEF2` writer"]
pub struct W(crate::W<CSC_COEF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSC_COEF2_SPEC>;
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
impl From<crate::W<CSC_COEF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSC_COEF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C3` reader - Two's compliment Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
pub type C3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C3` writer - Two's compliment Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
pub type C3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSC_COEF2_SPEC, u16, u16, 11, O>;
#[doc = "Field `C2` reader - Two's compliment Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
pub type C2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C2` writer - Two's compliment Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
pub type C2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSC_COEF2_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Two's compliment Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
    #[inline(always)]
    pub fn c3(&self) -> C3_R {
        C3_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Two's compliment Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Two's compliment Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
    #[inline(always)]
    #[must_use]
    pub fn c3(&mut self) -> C3_W<0> {
        C3_W::new(self)
    }
    #[doc = "Bits 16:26 - Two's compliment Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
    #[inline(always)]
    #[must_use]
    pub fn c2(&mut self) -> C2_W<16> {
        C2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Color Space Conversion Config Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csc_coef2](index.html) module"]
pub struct CSC_COEF2_SPEC;
impl crate::RegisterSpec for CSC_COEF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csc_coef2::R](R) reader structure"]
impl crate::Readable for CSC_COEF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csc_coef2::W](W) writer structure"]
impl crate::Writable for CSC_COEF2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSC_COEF2 to value 0"]
impl crate::Resettable for CSC_COEF2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
