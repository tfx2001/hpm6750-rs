#[doc = "Register `LAYER_7_CSC_COEF1` reader"]
pub struct R(crate::R<LAYER_7_CSC_COEF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYER_7_CSC_COEF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYER_7_CSC_COEF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYER_7_CSC_COEF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYER_7_CSC_COEF1` writer"]
pub struct W(crate::W<LAYER_7_CSC_COEF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYER_7_CSC_COEF1_SPEC>;
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
impl From<crate::W<LAYER_7_CSC_COEF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYER_7_CSC_COEF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C4` reader - Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
pub type C4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C4` writer - Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
pub type C4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYER_7_CSC_COEF1_SPEC, u16, u16, 11, O>;
#[doc = "Field `C1` reader - Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
pub type C1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `C1` writer - Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
pub type C1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYER_7_CSC_COEF1_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
    #[inline(always)]
    pub fn c4(&self) -> C4_R {
        C4_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
    #[inline(always)]
    pub fn c1(&self) -> C1_R {
        C1_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
    #[inline(always)]
    #[must_use]
    pub fn c4(&mut self) -> C4_W<0> {
        C4_W::new(self)
    }
    #[doc = "Bits 16:26 - Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
    #[inline(always)]
    #[must_use]
    pub fn c1(&mut self) -> C1_W<16> {
        C1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer Color Space Conversion Config Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layer_7_csc_coef1](index.html) module"]
pub struct LAYER_7_CSC_COEF1_SPEC;
impl crate::RegisterSpec for LAYER_7_CSC_COEF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layer_7_csc_coef1::R](R) reader structure"]
impl crate::Readable for LAYER_7_CSC_COEF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layer_7_csc_coef1::W](W) writer structure"]
impl crate::Writable for LAYER_7_CSC_COEF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYER_7_CSC_COEF1 to value 0"]
impl crate::Resettable for LAYER_7_CSC_COEF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
