#[doc = "Register `WDGCFG` reader"]
pub struct R(crate::R<WDGCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDGCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDGCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDGCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDGCFG` writer"]
pub struct W(crate::W<WDGCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDGCFG_SPEC>;
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
impl From<crate::W<WDGCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDGCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDGTO` reader - watch dog timeout value"]
pub type WDGTO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WDGTO` writer - watch dog timeout value"]
pub type WDGTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDGCFG_SPEC, u32, u32, 31, O>;
#[doc = "Field `WDGEN` reader - 1- enable wdog counter"]
pub type WDGEN_R = crate::BitReader<bool>;
#[doc = "Field `WDGEN` writer - 1- enable wdog counter"]
pub type WDGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WDGCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:30 - watch dog timeout value"]
    #[inline(always)]
    pub fn wdgto(&self) -> WDGTO_R {
        WDGTO_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - 1- enable wdog counter"]
    #[inline(always)]
    pub fn wdgen(&self) -> WDGEN_R {
        WDGEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - watch dog timeout value"]
    #[inline(always)]
    #[must_use]
    pub fn wdgto(&mut self) -> WDGTO_W<0> {
        WDGTO_W::new(self)
    }
    #[doc = "Bit 31 - 1- enable wdog counter"]
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
#[doc = "Watchdog configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdgcfg](index.html) module"]
pub struct WDGCFG_SPEC;
impl crate::RegisterSpec for WDGCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdgcfg::R](R) reader structure"]
impl crate::Readable for WDGCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdgcfg::W](W) writer structure"]
impl crate::Writable for WDGCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDGCFG to value 0"]
impl crate::Resettable for WDGCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
