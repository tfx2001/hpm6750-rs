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
#[doc = "Field `ZCNTCFG` reader - 1- zcnt will increment when phcnt upcount to phmax, decrement when phcnt downcount to 0 0- zcnt will increment or decrement when Z input assert"]
pub type ZCNTCFG_R = crate::BitReader<bool>;
#[doc = "Field `ZCNTCFG` writer - 1- zcnt will increment when phcnt upcount to phmax, decrement when phcnt downcount to 0 0- zcnt will increment or decrement when Z input assert"]
pub type ZCNTCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHCFG_SPEC, bool, O>;
#[doc = "Field `PHCALIZ` reader - 1- phcnt will set to phidx when Z input assert"]
pub type PHCALIZ_R = crate::BitReader<bool>;
#[doc = "Field `PHCALIZ` writer - 1- phcnt will set to phidx when Z input assert"]
pub type PHCALIZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHCFG_SPEC, bool, O>;
#[doc = "Field `PHMAX` reader - maximum phcnt number, phcnt will rollover to 0 when it upcount to phmax"]
pub type PHMAX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PHMAX` writer - maximum phcnt number, phcnt will rollover to 0 when it upcount to phmax"]
pub type PHMAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PHCFG_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bit 22 - 1- zcnt will increment when phcnt upcount to phmax, decrement when phcnt downcount to 0 0- zcnt will increment or decrement when Z input assert"]
    #[inline(always)]
    pub fn zcntcfg(&self) -> ZCNTCFG_R {
        ZCNTCFG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - 1- phcnt will set to phidx when Z input assert"]
    #[inline(always)]
    pub fn phcaliz(&self) -> PHCALIZ_R {
        PHCALIZ_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 0:20 - maximum phcnt number, phcnt will rollover to 0 when it upcount to phmax"]
    #[inline(always)]
    pub fn phmax(&self) -> PHMAX_R {
        PHMAX_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 22 - 1- zcnt will increment when phcnt upcount to phmax, decrement when phcnt downcount to 0 0- zcnt will increment or decrement when Z input assert"]
    #[inline(always)]
    pub fn zcntcfg(&mut self) -> ZCNTCFG_W<22> {
        ZCNTCFG_W::new(self)
    }
    #[doc = "Bit 21 - 1- phcnt will set to phidx when Z input assert"]
    #[inline(always)]
    pub fn phcaliz(&mut self) -> PHCALIZ_W<21> {
        PHCALIZ_W::new(self)
    }
    #[doc = "Bits 0:20 - maximum phcnt number, phcnt will rollover to 0 when it upcount to phmax"]
    #[inline(always)]
    pub fn phmax(&mut self) -> PHMAX_W<0> {
        PHMAX_W::new(self)
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
