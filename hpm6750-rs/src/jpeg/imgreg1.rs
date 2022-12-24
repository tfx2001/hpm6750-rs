#[doc = "Register `IMGREG1` reader"]
pub struct R(crate::R<IMGREG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMGREG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMGREG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMGREG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMGREG1` writer"]
pub struct W(crate::W<IMGREG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMGREG1_SPEC>;
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
impl From<crate::W<IMGREG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMGREG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCOL` reader - Ncol is the number of color components in the image data to process minus 1. For example, for a grayscale image Ncol=0, for an RGB image, Ncol=2"]
pub type NCOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NCOL` writer - Ncol is the number of color components in the image data to process minus 1. For example, for a grayscale image Ncol=0, for an RGB image, Ncol=2"]
pub type NCOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMGREG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `RE` reader - Encoder Use only. Asseted to enable the Restart Marker processing. A Restart Marker is inserted in the outputted ECS (Entropy Coded Segment) every NRST+1 MCUs"]
pub type RE_R = crate::BitReader<bool>;
#[doc = "Field `RE` writer - Encoder Use only. Asseted to enable the Restart Marker processing. A Restart Marker is inserted in the outputted ECS (Entropy Coded Segment) every NRST+1 MCUs"]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMGREG1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Ncol is the number of color components in the image data to process minus 1. For example, for a grayscale image Ncol=0, for an RGB image, Ncol=2"]
    #[inline(always)]
    pub fn ncol(&self) -> NCOL_R {
        NCOL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Encoder Use only. Asseted to enable the Restart Marker processing. A Restart Marker is inserted in the outputted ECS (Entropy Coded Segment) every NRST+1 MCUs"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Ncol is the number of color components in the image data to process minus 1. For example, for a grayscale image Ncol=0, for an RGB image, Ncol=2"]
    #[inline(always)]
    #[must_use]
    pub fn ncol(&mut self) -> NCOL_W<0> {
        NCOL_W::new(self)
    }
    #[doc = "Bit 2 - Encoder Use only. Asseted to enable the Restart Marker processing. A Restart Marker is inserted in the outputted ECS (Entropy Coded Segment) every NRST+1 MCUs"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Image Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imgreg1](index.html) module"]
pub struct IMGREG1_SPEC;
impl crate::RegisterSpec for IMGREG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imgreg1::R](R) reader structure"]
impl crate::Readable for IMGREG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imgreg1::W](W) writer structure"]
impl crate::Writable for IMGREG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMGREG1 to value 0"]
impl crate::Resettable for IMGREG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
