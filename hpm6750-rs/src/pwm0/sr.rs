#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAULTF` reader - fault condition flag"]
pub type FAULTF_R = crate::BitReader<bool>;
#[doc = "Field `FAULTF` writer - fault condition flag"]
pub type FAULTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `XRLDF` reader - extended reload flag, this flag set when xcnt count to xrld value or when SYNCI assert"]
pub type XRLDF_R = crate::BitReader<bool>;
#[doc = "Field `XRLDF` writer - extended reload flag, this flag set when xcnt count to xrld value or when SYNCI assert"]
pub type XRLDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `HALFRLDF` reader - half reload flag, this flag set when cnt count to rld/2"]
pub type HALFRLDF_R = crate::BitReader<bool>;
#[doc = "Field `HALFRLDF` writer - half reload flag, this flag set when cnt count to rld/2"]
pub type HALFRLDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `RLDF` reader - reload flag, this flag set when cnt count to rld value or when SYNCI assert"]
pub type RLDF_R = crate::BitReader<bool>;
#[doc = "Field `RLDF` writer - reload flag, this flag set when cnt count to rld value or when SYNCI assert"]
pub type RLDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `CMPFX` reader - comparator output compare or input capture flag"]
pub type CMPFX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMPFX` writer - comparator output compare or input capture flag"]
pub type CMPFX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 27 - fault condition flag"]
    #[inline(always)]
    pub fn faultf(&self) -> FAULTF_R {
        FAULTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - extended reload flag, this flag set when xcnt count to xrld value or when SYNCI assert"]
    #[inline(always)]
    pub fn xrldf(&self) -> XRLDF_R {
        XRLDF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - half reload flag, this flag set when cnt count to rld/2"]
    #[inline(always)]
    pub fn halfrldf(&self) -> HALFRLDF_R {
        HALFRLDF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - reload flag, this flag set when cnt count to rld value or when SYNCI assert"]
    #[inline(always)]
    pub fn rldf(&self) -> RLDF_R {
        RLDF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 0:23 - comparator output compare or input capture flag"]
    #[inline(always)]
    pub fn cmpfx(&self) -> CMPFX_R {
        CMPFX_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 27 - fault condition flag"]
    #[inline(always)]
    pub fn faultf(&mut self) -> FAULTF_W<27> {
        FAULTF_W::new(self)
    }
    #[doc = "Bit 26 - extended reload flag, this flag set when xcnt count to xrld value or when SYNCI assert"]
    #[inline(always)]
    pub fn xrldf(&mut self) -> XRLDF_W<26> {
        XRLDF_W::new(self)
    }
    #[doc = "Bit 25 - half reload flag, this flag set when cnt count to rld/2"]
    #[inline(always)]
    pub fn halfrldf(&mut self) -> HALFRLDF_W<25> {
        HALFRLDF_W::new(self)
    }
    #[doc = "Bit 24 - reload flag, this flag set when cnt count to rld value or when SYNCI assert"]
    #[inline(always)]
    pub fn rldf(&mut self) -> RLDF_W<24> {
        RLDF_W::new(self)
    }
    #[doc = "Bits 0:23 - comparator output compare or input capture flag"]
    #[inline(always)]
    pub fn cmpfx(&mut self) -> CMPFX_W<0> {
        CMPFX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
