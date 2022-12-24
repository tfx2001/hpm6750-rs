#[doc = "Register `TRGOEN` reader"]
pub struct R(crate::R<TRGOEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRGOEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRGOEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRGOEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRGOEN` writer"]
pub struct W(crate::W<TRGOEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRGOEN_SPEC>;
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
impl From<crate::W<TRGOEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRGOEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZPHFEN` reader - 1- enable trigger output when zphf flag set"]
pub type ZPHFEN_R = crate::BitReader<bool>;
#[doc = "Field `ZPHFEN` writer - 1- enable trigger output when zphf flag set"]
pub type ZPHFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRGOEN_SPEC, bool, O>;
#[doc = "Field `POSCMPFEN` reader - 1- enable trigger output when poscmpf flag set"]
pub type POSCMPFEN_R = crate::BitReader<bool>;
#[doc = "Field `POSCMPFEN` writer - 1- enable trigger output when poscmpf flag set"]
pub type POSCMPFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRGOEN_SPEC, bool, O>;
#[doc = "Field `HOMEFEN` reader - 1- enable trigger output when homef flag set"]
pub type HOMEFEN_R = crate::BitReader<bool>;
#[doc = "Field `HOMEFEN` writer - 1- enable trigger output when homef flag set"]
pub type HOMEFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRGOEN_SPEC, bool, O>;
#[doc = "Field `WDGFEN` reader - 1- enable trigger output when wdg flag set"]
pub type WDGFEN_R = crate::BitReader<bool>;
#[doc = "Field `WDGFEN` writer - 1- enable trigger output when wdg flag set"]
pub type WDGFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRGOEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 28 - 1- enable trigger output when zphf flag set"]
    #[inline(always)]
    pub fn zphfen(&self) -> ZPHFEN_R {
        ZPHFEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1- enable trigger output when poscmpf flag set"]
    #[inline(always)]
    pub fn poscmpfen(&self) -> POSCMPFEN_R {
        POSCMPFEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- enable trigger output when homef flag set"]
    #[inline(always)]
    pub fn homefen(&self) -> HOMEFEN_R {
        HOMEFEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1- enable trigger output when wdg flag set"]
    #[inline(always)]
    pub fn wdgfen(&self) -> WDGFEN_R {
        WDGFEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - 1- enable trigger output when zphf flag set"]
    #[inline(always)]
    #[must_use]
    pub fn zphfen(&mut self) -> ZPHFEN_W<28> {
        ZPHFEN_W::new(self)
    }
    #[doc = "Bit 29 - 1- enable trigger output when poscmpf flag set"]
    #[inline(always)]
    #[must_use]
    pub fn poscmpfen(&mut self) -> POSCMPFEN_W<29> {
        POSCMPFEN_W::new(self)
    }
    #[doc = "Bit 30 - 1- enable trigger output when homef flag set"]
    #[inline(always)]
    #[must_use]
    pub fn homefen(&mut self) -> HOMEFEN_W<30> {
        HOMEFEN_W::new(self)
    }
    #[doc = "Bit 31 - 1- enable trigger output when wdg flag set"]
    #[inline(always)]
    #[must_use]
    pub fn wdgfen(&mut self) -> WDGFEN_W<31> {
        WDGFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tigger output enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgoen](index.html) module"]
pub struct TRGOEN_SPEC;
impl crate::RegisterSpec for TRGOEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trgoen::R](R) reader structure"]
impl crate::Readable for TRGOEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trgoen::W](W) writer structure"]
impl crate::Writable for TRGOEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRGOEN to value 0"]
impl crate::Resettable for TRGOEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
