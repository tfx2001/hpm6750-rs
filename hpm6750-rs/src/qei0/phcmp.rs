#[doc = "Register `PHCMP` reader"]
pub struct R(crate::R<PHCMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHCMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHCMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHCMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHCMP` writer"]
pub struct W(crate::W<PHCMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHCMP_SPEC>;
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
impl From<crate::W<PHCMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHCMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHCMP` reader - phcnt position compare value"]
pub type PHCMP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PHCMP` writer - phcnt position compare value"]
pub type PHCMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHCMP_SPEC, u32, u32, 21, O>;
#[doc = "Field `DIRCMP` reader - 0- position compare need positive rotation 1- position compare need negative rotation"]
pub type DIRCMP_R = crate::BitReader<bool>;
#[doc = "Field `DIRCMP` writer - 0- position compare need positive rotation 1- position compare need negative rotation"]
pub type DIRCMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHCMP_SPEC, bool, O>;
#[doc = "Field `DIRCMPDIS` reader - 1- postion compare not include rotation direction"]
pub type DIRCMPDIS_R = crate::BitReader<bool>;
#[doc = "Field `DIRCMPDIS` writer - 1- postion compare not include rotation direction"]
pub type DIRCMPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHCMP_SPEC, bool, O>;
#[doc = "Field `ZCMPDIS` reader - 1- postion compare not include zcnt"]
pub type ZCMPDIS_R = crate::BitReader<bool>;
#[doc = "Field `ZCMPDIS` writer - 1- postion compare not include zcnt"]
pub type ZCMPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHCMP_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:20 - phcnt position compare value"]
    #[inline(always)]
    pub fn phcmp(&self) -> PHCMP_R {
        PHCMP_R::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bit 29 - 0- position compare need positive rotation 1- position compare need negative rotation"]
    #[inline(always)]
    pub fn dircmp(&self) -> DIRCMP_R {
        DIRCMP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- postion compare not include rotation direction"]
    #[inline(always)]
    pub fn dircmpdis(&self) -> DIRCMPDIS_R {
        DIRCMPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1- postion compare not include zcnt"]
    #[inline(always)]
    pub fn zcmpdis(&self) -> ZCMPDIS_R {
        ZCMPDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:20 - phcnt position compare value"]
    #[inline(always)]
    #[must_use]
    pub fn phcmp(&mut self) -> PHCMP_W<0> {
        PHCMP_W::new(self)
    }
    #[doc = "Bit 29 - 0- position compare need positive rotation 1- position compare need negative rotation"]
    #[inline(always)]
    #[must_use]
    pub fn dircmp(&mut self) -> DIRCMP_W<29> {
        DIRCMP_W::new(self)
    }
    #[doc = "Bit 30 - 1- postion compare not include rotation direction"]
    #[inline(always)]
    #[must_use]
    pub fn dircmpdis(&mut self) -> DIRCMPDIS_W<30> {
        DIRCMPDIS_W::new(self)
    }
    #[doc = "Bit 31 - 1- postion compare not include zcnt"]
    #[inline(always)]
    #[must_use]
    pub fn zcmpdis(&mut self) -> ZCMPDIS_W<31> {
        ZCMPDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Phase comparator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phcmp](index.html) module"]
pub struct PHCMP_SPEC;
impl crate::RegisterSpec for PHCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phcmp::R](R) reader structure"]
impl crate::Readable for PHCMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phcmp::W](W) writer structure"]
impl crate::Writable for PHCMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHCMP to value 0"]
impl crate::Resettable for PHCMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
