#[doc = "Register `CMP_4` reader"]
pub struct R(crate::R<CMP_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMP_4` writer"]
pub struct W(crate::W<CMP_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP_4_SPEC>;
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
impl From<crate::W<CMP_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XCMP` reader - extended counter compare value"]
pub type XCMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XCMP` writer - extended counter compare value"]
pub type XCMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMP_4_SPEC, u8, u8, 4, O>;
#[doc = "Field `CMP` reader - clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
pub type CMP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMP` writer - clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
pub type CMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMP_4_SPEC, u32, u32, 24, O>;
#[doc = "Field `CMPHLF` reader - half clock counter compare value"]
pub type CMPHLF_R = crate::BitReader<bool>;
#[doc = "Field `CMPHLF` writer - half clock counter compare value"]
pub type CMPHLF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMP_4_SPEC, bool, O>;
#[doc = "Field `CMPJIT` reader - jitter counter compare value"]
pub type CMPJIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPJIT` writer - jitter counter compare value"]
pub type CMPJIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMP_4_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 28:31 - extended counter compare value"]
    #[inline(always)]
    pub fn xcmp(&self) -> XCMP_R {
        XCMP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 4:27 - clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 4) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 3 - half clock counter compare value"]
    #[inline(always)]
    pub fn cmphlf(&self) -> CMPHLF_R {
        CMPHLF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 0:2 - jitter counter compare value"]
    #[inline(always)]
    pub fn cmpjit(&self) -> CMPJIT_R {
        CMPJIT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - extended counter compare value"]
    #[inline(always)]
    pub fn xcmp(&mut self) -> XCMP_W<28> {
        XCMP_W::new(self)
    }
    #[doc = "Bits 4:27 - clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    #[inline(always)]
    pub fn cmp(&mut self) -> CMP_W<4> {
        CMP_W::new(self)
    }
    #[doc = "Bit 3 - half clock counter compare value"]
    #[inline(always)]
    pub fn cmphlf(&mut self) -> CMPHLF_W<3> {
        CMPHLF_W::new(self)
    }
    #[doc = "Bits 0:2 - jitter counter compare value"]
    #[inline(always)]
    pub fn cmpjit(&mut self) -> CMPJIT_W<0> {
        CMPJIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp_4](index.html) module"]
pub struct CMP_4_SPEC;
impl crate::RegisterSpec for CMP_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmp_4::R](R) reader structure"]
impl crate::Readable for CMP_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmp_4::W](W) writer structure"]
impl crate::Writable for CMP_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMP_4 to value 0"]
impl crate::Resettable for CMP_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
