#[doc = "Register `PHCFG` reader"]
pub struct R(crate::R<PHCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHCFG` writer"]
pub struct W(crate::W<PHCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHCFG_SPEC>;
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
impl From<crate::W<PHCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLYSEL` reader - This bit select delay start time: 1- start counting delay after pre-trigger 0- start counting delay after u,v,w toggle"]
pub type DLYSEL_R = crate::BitReader<bool>;
#[doc = "Field `DLYSEL` writer - This bit select delay start time: 1- start counting delay after pre-trigger 0- start counting delay after u,v,w toggle"]
pub type DLYSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHCFG_SPEC, bool, O>;
#[doc = "Field `DLYCNT` reader - delay clock cycles number"]
pub type DLYCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DLYCNT` writer - delay clock cycles number"]
pub type DLYCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHCFG_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 31 - This bit select delay start time: 1- start counting delay after pre-trigger 0- start counting delay after u,v,w toggle"]
    #[inline(always)]
    pub fn dlysel(&self) -> DLYSEL_R {
        DLYSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 0:23 - delay clock cycles number"]
    #[inline(always)]
    pub fn dlycnt(&self) -> DLYCNT_R {
        DLYCNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - This bit select delay start time: 1- start counting delay after pre-trigger 0- start counting delay after u,v,w toggle"]
    #[inline(always)]
    pub fn dlysel(&mut self) -> DLYSEL_W<31> {
        DLYSEL_W::new(self)
    }
    #[doc = "Bits 0:23 - delay clock cycles number"]
    #[inline(always)]
    pub fn dlycnt(&mut self) -> DLYCNT_W<0> {
        DLYCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Phase configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phcfg](index.html) module"]
pub struct PHCFG_SPEC;
impl crate::RegisterSpec for PHCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phcfg::R](R) reader structure"]
impl crate::Readable for PHCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phcfg::W](W) writer structure"]
impl crate::Writable for PHCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PHCFG to value 0"]
impl crate::Resettable for PHCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
