#[doc = "Register `READEN` reader"]
pub struct R(crate::R<READEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READEN` writer"]
pub struct W(crate::W<READEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READEN_SPEC>;
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
impl From<crate::W<READEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZPHFEN` reader - 1- load counters to their read registers when zphf flag set"]
pub type ZPHFEN_R = crate::BitReader<bool>;
#[doc = "Field `ZPHFEN` writer - 1- load counters to their read registers when zphf flag set"]
pub type ZPHFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, READEN_SPEC, bool, O>;
#[doc = "Field `POSCMPFEN` reader - 1- load counters to their read registers when poscmpf flag set"]
pub type POSCMPFEN_R = crate::BitReader<bool>;
#[doc = "Field `POSCMPFEN` writer - 1- load counters to their read registers when poscmpf flag set"]
pub type POSCMPFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, READEN_SPEC, bool, O>;
#[doc = "Field `HOMEFEN` reader - 1- load counters to their read registers when homef flag set"]
pub type HOMEFEN_R = crate::BitReader<bool>;
#[doc = "Field `HOMEFEN` writer - 1- load counters to their read registers when homef flag set"]
pub type HOMEFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, READEN_SPEC, bool, O>;
#[doc = "Field `WDGFEN` reader - 1- load counters to their read registers when wdg flag set"]
pub type WDGFEN_R = crate::BitReader<bool>;
#[doc = "Field `WDGFEN` writer - 1- load counters to their read registers when wdg flag set"]
pub type WDGFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, READEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 28 - 1- load counters to their read registers when zphf flag set"]
    #[inline(always)]
    pub fn zphfen(&self) -> ZPHFEN_R {
        ZPHFEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1- load counters to their read registers when poscmpf flag set"]
    #[inline(always)]
    pub fn poscmpfen(&self) -> POSCMPFEN_R {
        POSCMPFEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- load counters to their read registers when homef flag set"]
    #[inline(always)]
    pub fn homefen(&self) -> HOMEFEN_R {
        HOMEFEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1- load counters to their read registers when wdg flag set"]
    #[inline(always)]
    pub fn wdgfen(&self) -> WDGFEN_R {
        WDGFEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - 1- load counters to their read registers when zphf flag set"]
    #[inline(always)]
    #[must_use]
    pub fn zphfen(&mut self) -> ZPHFEN_W<28> {
        ZPHFEN_W::new(self)
    }
    #[doc = "Bit 29 - 1- load counters to their read registers when poscmpf flag set"]
    #[inline(always)]
    #[must_use]
    pub fn poscmpfen(&mut self) -> POSCMPFEN_W<29> {
        POSCMPFEN_W::new(self)
    }
    #[doc = "Bit 30 - 1- load counters to their read registers when homef flag set"]
    #[inline(always)]
    #[must_use]
    pub fn homefen(&mut self) -> HOMEFEN_W<30> {
        HOMEFEN_W::new(self)
    }
    #[doc = "Bit 31 - 1- load counters to their read registers when wdg flag set"]
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
#[doc = "Read event enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readen](index.html) module"]
pub struct READEN_SPEC;
impl crate::RegisterSpec for READEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readen::R](R) reader structure"]
impl crate::Readable for READEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [readen::W](W) writer structure"]
impl crate::Writable for READEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets READEN to value 0"]
impl crate::Resettable for READEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
