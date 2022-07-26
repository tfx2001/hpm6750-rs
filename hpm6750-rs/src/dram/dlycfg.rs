#[doc = "Register `DLYCFG` reader"]
pub struct R(crate::R<DLYCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLYCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLYCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLYCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLYCFG` writer"]
pub struct W(crate::W<DLYCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLYCFG_SPEC>;
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
impl From<crate::W<DLYCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLYCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OE` reader - delay clock output enable, should be set after setting DLYEN and DLYSEL"]
pub type OE_R = crate::BitReader<bool>;
#[doc = "Field `OE` writer - delay clock output enable, should be set after setting DLYEN and DLYSEL"]
pub type OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLYCFG_SPEC, bool, O>;
#[doc = "Field `DLYSEL` reader - delay line select, 0 for 1 cell, 31 for all 32 cells"]
pub type DLYSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYSEL` writer - delay line select, 0 for 1 cell, 31 for all 32 cells"]
pub type DLYSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLYCFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `DLYEN` reader - delay line enable"]
pub type DLYEN_R = crate::BitReader<bool>;
#[doc = "Field `DLYEN` writer - delay line enable"]
pub type DLYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLYCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 13 - delay clock output enable, should be set after setting DLYEN and DLYSEL"]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 1:5 - delay line select, 0 for 1 cell, 31 for all 32 cells"]
    #[inline(always)]
    pub fn dlysel(&self) -> DLYSEL_R {
        DLYSEL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - delay line enable"]
    #[inline(always)]
    pub fn dlyen(&self) -> DLYEN_R {
        DLYEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - delay clock output enable, should be set after setting DLYEN and DLYSEL"]
    #[inline(always)]
    pub fn oe(&mut self) -> OE_W<13> {
        OE_W::new(self)
    }
    #[doc = "Bits 1:5 - delay line select, 0 for 1 cell, 31 for all 32 cells"]
    #[inline(always)]
    pub fn dlysel(&mut self) -> DLYSEL_W<1> {
        DLYSEL_W::new(self)
    }
    #[doc = "Bit 0 - delay line enable"]
    #[inline(always)]
    pub fn dlyen(&mut self) -> DLYEN_W<0> {
        DLYEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delay Line Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlycfg](index.html) module"]
pub struct DLYCFG_SPEC;
impl crate::RegisterSpec for DLYCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlycfg::R](R) reader structure"]
impl crate::Readable for DLYCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlycfg::W](W) writer structure"]
impl crate::Writable for DLYCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLYCFG to value 0"]
impl crate::Resettable for DLYCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
