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
#[doc = "Field `WDGF` reader - watchdog count timeout flag"]
pub type WDGF_R = crate::BitReader<bool>;
#[doc = "Field `WDGF` writer - watchdog count timeout flag"]
pub type WDGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `PHUPTF` reader - phase update flag, will set when any of u, v, w signal toggle"]
pub type PHUPTF_R = crate::BitReader<bool>;
#[doc = "Field `PHUPTF` writer - phase update flag, will set when any of u, v, w signal toggle"]
pub type PHUPTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `PHPREF` reader - phase update pre flag, will set PRECNT cycles before any of u, v, w signal toggle"]
pub type PHPREF_R = crate::BitReader<bool>;
#[doc = "Field `PHPREF` writer - phase update pre flag, will set PRECNT cycles before any of u, v, w signal toggle"]
pub type PHPREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `PHDLYF` reader - phase update delay flag, will set DLYCNT cycles after any of u, v, w signal toggle or after the phpre flag depands on DLYSEL setting"]
pub type PHDLYF_R = crate::BitReader<bool>;
#[doc = "Field `PHDLYF` writer - phase update delay flag, will set DLYCNT cycles after any of u, v, w signal toggle or after the phpre flag depands on DLYSEL setting"]
pub type PHDLYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `UF` reader - u flag, will set when u signal toggle"]
pub type UF_R = crate::BitReader<bool>;
#[doc = "Field `UF` writer - u flag, will set when u signal toggle"]
pub type UF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `VF` reader - v flag, will set when v signal toggle"]
pub type VF_R = crate::BitReader<bool>;
#[doc = "Field `VF` writer - v flag, will set when v signal toggle"]
pub type VF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `WF` reader - w flag, will set when w signal toggle"]
pub type WF_R = crate::BitReader<bool>;
#[doc = "Field `WF` writer - w flag, will set when w signal toggle"]
pub type WF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - watchdog count timeout flag"]
    #[inline(always)]
    pub fn wdgf(&self) -> WDGF_R {
        WDGF_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - phase update flag, will set when any of u, v, w signal toggle"]
    #[inline(always)]
    pub fn phuptf(&self) -> PHUPTF_R {
        PHUPTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - phase update pre flag, will set PRECNT cycles before any of u, v, w signal toggle"]
    #[inline(always)]
    pub fn phpref(&self) -> PHPREF_R {
        PHPREF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - phase update delay flag, will set DLYCNT cycles after any of u, v, w signal toggle or after the phpre flag depands on DLYSEL setting"]
    #[inline(always)]
    pub fn phdlyf(&self) -> PHDLYF_R {
        PHDLYF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 23 - u flag, will set when u signal toggle"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - v flag, will set when v signal toggle"]
    #[inline(always)]
    pub fn vf(&self) -> VF_R {
        VF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - w flag, will set when w signal toggle"]
    #[inline(always)]
    pub fn wf(&self) -> WF_R {
        WF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - watchdog count timeout flag"]
    #[inline(always)]
    pub fn wdgf(&mut self) -> WDGF_W<31> {
        WDGF_W::new(self)
    }
    #[doc = "Bit 30 - phase update flag, will set when any of u, v, w signal toggle"]
    #[inline(always)]
    pub fn phuptf(&mut self) -> PHUPTF_W<30> {
        PHUPTF_W::new(self)
    }
    #[doc = "Bit 29 - phase update pre flag, will set PRECNT cycles before any of u, v, w signal toggle"]
    #[inline(always)]
    pub fn phpref(&mut self) -> PHPREF_W<29> {
        PHPREF_W::new(self)
    }
    #[doc = "Bit 28 - phase update delay flag, will set DLYCNT cycles after any of u, v, w signal toggle or after the phpre flag depands on DLYSEL setting"]
    #[inline(always)]
    pub fn phdlyf(&mut self) -> PHDLYF_W<28> {
        PHDLYF_W::new(self)
    }
    #[doc = "Bit 23 - u flag, will set when u signal toggle"]
    #[inline(always)]
    pub fn uf(&mut self) -> UF_W<23> {
        UF_W::new(self)
    }
    #[doc = "Bit 22 - v flag, will set when v signal toggle"]
    #[inline(always)]
    pub fn vf(&mut self) -> VF_W<22> {
        VF_W::new(self)
    }
    #[doc = "Bit 21 - w flag, will set when w signal toggle"]
    #[inline(always)]
    pub fn wf(&mut self) -> WF_W<21> {
        WF_W::new(self)
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
