#[doc = "Register `PS_1_BKGD` reader"]
pub struct R(crate::R<PS_1_BKGD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PS_1_BKGD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PS_1_BKGD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PS_1_BKGD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PS_1_BKGD` writer"]
pub struct W(crate::W<PS_1_BKGD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PS_1_BKGD_SPEC>;
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
impl From<crate::W<PS_1_BKGD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PS_1_BKGD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COLOR` reader - Background color (in 32bpp format) for any pixels not within the scaled range of the picture, but within the buffer range specified by the PS ULC/LRC. The top 8-bit is the alpha channel."]
pub type COLOR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COLOR` writer - Background color (in 32bpp format) for any pixels not within the scaled range of the picture, but within the buffer range specified by the PS ULC/LRC. The top 8-bit is the alpha channel."]
pub type COLOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PS_1_BKGD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Background color (in 32bpp format) for any pixels not within the scaled range of the picture, but within the buffer range specified by the PS ULC/LRC. The top 8-bit is the alpha channel."]
    #[inline(always)]
    pub fn color(&self) -> COLOR_R {
        COLOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Background color (in 32bpp format) for any pixels not within the scaled range of the picture, but within the buffer range specified by the PS ULC/LRC. The top 8-bit is the alpha channel."]
    #[inline(always)]
    #[must_use]
    pub fn color(&mut self) -> COLOR_W<0> {
        COLOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer background color register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_1_bkgd](index.html) module"]
pub struct PS_1_BKGD_SPEC;
impl crate::RegisterSpec for PS_1_BKGD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ps_1_bkgd::R](R) reader structure"]
impl crate::Readable for PS_1_BKGD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ps_1_bkgd::W](W) writer structure"]
impl crate::Writable for PS_1_BKGD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PS_1_BKGD to value 0"]
impl crate::Resettable for PS_1_BKGD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
