#[doc = "Register `ST` reader"]
pub struct R(crate::R<ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ST` writer"]
pub struct W(crate::W<ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ST_SPEC>;
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
impl From<crate::W<ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `URGENT_UNDERRUN` reader - Asserted when the output buffer urgent underrun condition encountered"]
pub type URGENT_UNDERRUN_R = crate::BitReader<bool>;
#[doc = "Field `URGENT_UNDERRUN` writer - Asserted when the output buffer urgent underrun condition encountered"]
pub type URGENT_UNDERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
#[doc = "Field `VS_BLANK` reader - Asserted when in vertical blanking period. At the start of VSYNC"]
pub type VS_BLANK_R = crate::BitReader<bool>;
#[doc = "Field `VS_BLANK` writer - Asserted when in vertical blanking period. At the start of VSYNC"]
pub type VS_BLANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
#[doc = "Field `UNDERRUN` reader - Asserted when the output buffer underrun condition encountered"]
pub type UNDERRUN_R = crate::BitReader<bool>;
#[doc = "Field `UNDERRUN` writer - Asserted when the output buffer underrun condition encountered"]
pub type UNDERRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
#[doc = "Field `VSYNC` reader - Asserted when in vertical blanking period. At the end of VSYNC"]
pub type VSYNC_R = crate::BitReader<bool>;
#[doc = "Field `VSYNC` writer - Asserted when in vertical blanking period. At the end of VSYNC"]
pub type VSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - Asserted when the output buffer urgent underrun condition encountered"]
    #[inline(always)]
    pub fn urgent_underrun(&self) -> URGENT_UNDERRUN_R {
        URGENT_UNDERRUN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Asserted when in vertical blanking period. At the start of VSYNC"]
    #[inline(always)]
    pub fn vs_blank(&self) -> VS_BLANK_R {
        VS_BLANK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Asserted when the output buffer underrun condition encountered"]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Asserted when in vertical blanking period. At the end of VSYNC"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Asserted when the output buffer urgent underrun condition encountered"]
    #[inline(always)]
    pub fn urgent_underrun(&mut self) -> URGENT_UNDERRUN_W<3> {
        URGENT_UNDERRUN_W::new(self)
    }
    #[doc = "Bit 2 - Asserted when in vertical blanking period. At the start of VSYNC"]
    #[inline(always)]
    pub fn vs_blank(&mut self) -> VS_BLANK_W<2> {
        VS_BLANK_W::new(self)
    }
    #[doc = "Bit 1 - Asserted when the output buffer underrun condition encountered"]
    #[inline(always)]
    pub fn underrun(&mut self) -> UNDERRUN_W<1> {
        UNDERRUN_W::new(self)
    }
    #[doc = "Bit 0 - Asserted when in vertical blanking period. At the end of VSYNC"]
    #[inline(always)]
    pub fn vsync(&mut self) -> VSYNC_W<0> {
        VSYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st](index.html) module"]
pub struct ST_SPEC;
impl crate::RegisterSpec for ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st::R](R) reader structure"]
impl crate::Readable for ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [st::W](W) writer structure"]
impl crate::Writable for ST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ST to value 0"]
impl crate::Resettable for ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
