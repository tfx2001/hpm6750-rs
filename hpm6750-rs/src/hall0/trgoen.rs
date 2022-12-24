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
#[doc = "Field `WFEN` reader - 1- enable trigger output when w flag set"]
pub type WFEN_R = crate::BitReader<bool>;
#[doc = "Field `WFEN` writer - 1- enable trigger output when w flag set"]
pub type WFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRGOEN_SPEC, bool, O>;
#[doc = "Field `VFEN` reader - 1- enable trigger output when v flag set"]
pub type VFEN_R = crate::BitReader<bool>;
#[doc = "Field `VFEN` writer - 1- enable trigger output when v flag set"]
pub type VFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRGOEN_SPEC, bool, O>;
#[doc = "Field `UFEN` reader - 1- enable trigger output when u flag set"]
pub type UFEN_R = crate::BitReader<bool>;
#[doc = "Field `UFEN` writer - 1- enable trigger output when u flag set"]
pub type UFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRGOEN_SPEC, bool, O>;
#[doc = "Field `PHDLYEN` reader - 1- enable trigger output when phdly flag set"]
pub type PHDLYEN_R = crate::BitReader<bool>;
#[doc = "Field `PHDLYEN` writer - 1- enable trigger output when phdly flag set"]
pub type PHDLYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRGOEN_SPEC, bool, O>;
#[doc = "Field `PHPREEN` reader - 1- enable trigger output when phpre flag set"]
pub type PHPREEN_R = crate::BitReader<bool>;
#[doc = "Field `PHPREEN` writer - 1- enable trigger output when phpre flag set"]
pub type PHPREEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRGOEN_SPEC, bool, O>;
#[doc = "Field `PHUPTEN` reader - 1- enable trigger output when phupt flag set"]
pub type PHUPTEN_R = crate::BitReader<bool>;
#[doc = "Field `PHUPTEN` writer - 1- enable trigger output when phupt flag set"]
pub type PHUPTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRGOEN_SPEC, bool, O>;
#[doc = "Field `WDGEN` reader - 1- enable trigger output when wdg flag set"]
pub type WDGEN_R = crate::BitReader<bool>;
#[doc = "Field `WDGEN` writer - 1- enable trigger output when wdg flag set"]
pub type WDGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRGOEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 21 - 1- enable trigger output when w flag set"]
    #[inline(always)]
    pub fn wfen(&self) -> WFEN_R {
        WFEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1- enable trigger output when v flag set"]
    #[inline(always)]
    pub fn vfen(&self) -> VFEN_R {
        VFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1- enable trigger output when u flag set"]
    #[inline(always)]
    pub fn ufen(&self) -> UFEN_R {
        UFEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - 1- enable trigger output when phdly flag set"]
    #[inline(always)]
    pub fn phdlyen(&self) -> PHDLYEN_R {
        PHDLYEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1- enable trigger output when phpre flag set"]
    #[inline(always)]
    pub fn phpreen(&self) -> PHPREEN_R {
        PHPREEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- enable trigger output when phupt flag set"]
    #[inline(always)]
    pub fn phupten(&self) -> PHUPTEN_R {
        PHUPTEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1- enable trigger output when wdg flag set"]
    #[inline(always)]
    pub fn wdgen(&self) -> WDGEN_R {
        WDGEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - 1- enable trigger output when w flag set"]
    #[inline(always)]
    #[must_use]
    pub fn wfen(&mut self) -> WFEN_W<21> {
        WFEN_W::new(self)
    }
    #[doc = "Bit 22 - 1- enable trigger output when v flag set"]
    #[inline(always)]
    #[must_use]
    pub fn vfen(&mut self) -> VFEN_W<22> {
        VFEN_W::new(self)
    }
    #[doc = "Bit 23 - 1- enable trigger output when u flag set"]
    #[inline(always)]
    #[must_use]
    pub fn ufen(&mut self) -> UFEN_W<23> {
        UFEN_W::new(self)
    }
    #[doc = "Bit 28 - 1- enable trigger output when phdly flag set"]
    #[inline(always)]
    #[must_use]
    pub fn phdlyen(&mut self) -> PHDLYEN_W<28> {
        PHDLYEN_W::new(self)
    }
    #[doc = "Bit 29 - 1- enable trigger output when phpre flag set"]
    #[inline(always)]
    #[must_use]
    pub fn phpreen(&mut self) -> PHPREEN_W<29> {
        PHPREEN_W::new(self)
    }
    #[doc = "Bit 30 - 1- enable trigger output when phupt flag set"]
    #[inline(always)]
    #[must_use]
    pub fn phupten(&mut self) -> PHUPTEN_W<30> {
        PHUPTEN_W::new(self)
    }
    #[doc = "Bit 31 - 1- enable trigger output when wdg flag set"]
    #[inline(always)]
    #[must_use]
    pub fn wdgen(&mut self) -> WDGEN_W<31> {
        WDGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger output enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgoen](index.html) module"]
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
