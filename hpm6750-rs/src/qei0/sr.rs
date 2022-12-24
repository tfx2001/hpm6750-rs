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
#[doc = "Field `ZPHF` reader - z input flag"]
pub type ZPHF_R = crate::BitReader<bool>;
#[doc = "Field `ZPHF` writer - z input flag"]
pub type ZPHF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `POSCMPF` reader - postion compare match flag"]
pub type POSCMPF_R = crate::BitReader<bool>;
#[doc = "Field `POSCMPF` writer - postion compare match flag"]
pub type POSCMPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `HOMEF` reader - home flag"]
pub type HOMEF_R = crate::BitReader<bool>;
#[doc = "Field `HOMEF` writer - home flag"]
pub type HOMEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `WDGF` reader - watchdog flag"]
pub type WDGF_R = crate::BitReader<bool>;
#[doc = "Field `WDGF` writer - watchdog flag"]
pub type WDGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 28 - z input flag"]
    #[inline(always)]
    pub fn zphf(&self) -> ZPHF_R {
        ZPHF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - postion compare match flag"]
    #[inline(always)]
    pub fn poscmpf(&self) -> POSCMPF_R {
        POSCMPF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - home flag"]
    #[inline(always)]
    pub fn homef(&self) -> HOMEF_R {
        HOMEF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - watchdog flag"]
    #[inline(always)]
    pub fn wdgf(&self) -> WDGF_R {
        WDGF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - z input flag"]
    #[inline(always)]
    #[must_use]
    pub fn zphf(&mut self) -> ZPHF_W<28> {
        ZPHF_W::new(self)
    }
    #[doc = "Bit 29 - postion compare match flag"]
    #[inline(always)]
    #[must_use]
    pub fn poscmpf(&mut self) -> POSCMPF_W<29> {
        POSCMPF_W::new(self)
    }
    #[doc = "Bit 30 - home flag"]
    #[inline(always)]
    #[must_use]
    pub fn homef(&mut self) -> HOMEF_W<30> {
        HOMEF_W::new(self)
    }
    #[doc = "Bit 31 - watchdog flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdgf(&mut self) -> WDGF_W<31> {
        WDGF_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
