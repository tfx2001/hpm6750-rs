#[doc = "Register `MONITOR_SLICE3_CONTROL` reader"]
pub struct R(crate::R<MONITOR_SLICE3_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MONITOR_SLICE3_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MONITOR_SLICE3_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MONITOR_SLICE3_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MONITOR_SLICE3_CONTROL` writer"]
pub struct W(crate::W<MONITOR_SLICE3_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MONITOR_SLICE3_CONTROL_SPEC>;
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
impl From<crate::W<MONITOR_SLICE3_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MONITOR_SLICE3_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELECTION` reader - clock measurement selection"]
pub type SELECTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELECTION` writer - clock measurement selection"]
pub type SELECTION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MONITOR_SLICE3_CONTROL_SPEC, u8, u8, 8, O>;
#[doc = "Field `REFERENCE` reader - refrence clock selection, 0: 32k 1: 24M"]
pub type REFERENCE_R = crate::BitReader<bool>;
#[doc = "Field `REFERENCE` writer - refrence clock selection, 0: 32k 1: 24M"]
pub type REFERENCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MONITOR_SLICE3_CONTROL_SPEC, bool, O>;
#[doc = "Field `ACCURACY` reader - measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz"]
pub type ACCURACY_R = crate::BitReader<bool>;
#[doc = "Field `ACCURACY` writer - measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz"]
pub type ACCURACY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MONITOR_SLICE3_CONTROL_SPEC, bool, O>;
#[doc = "Field `MODE` reader - work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MONITOR_SLICE3_CONTROL_SPEC, bool, O>;
#[doc = "Field `START` reader - start measurement"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - start measurement"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, MONITOR_SLICE3_CONTROL_SPEC, bool, O>;
#[doc = "Field `LOW` reader - clock frequency lower than lower limit"]
pub type LOW_R = crate::BitReader<bool>;
#[doc = "Field `LOW` writer - clock frequency lower than lower limit"]
pub type LOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, MONITOR_SLICE3_CONTROL_SPEC, bool, O>;
#[doc = "Field `HIGH` reader - clock frequency higher than upper limit"]
pub type HIGH_R = crate::BitReader<bool>;
#[doc = "Field `HIGH` writer - clock frequency higher than upper limit"]
pub type HIGH_W<'a, const O: u8> = crate::BitWriter<'a, u32, MONITOR_SLICE3_CONTROL_SPEC, bool, O>;
#[doc = "Field `DIV` reader - output divider"]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - output divider"]
pub type DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MONITOR_SLICE3_CONTROL_SPEC, u8, u8, 8, O>;
#[doc = "Field `OUTEN` reader - enable clock output"]
pub type OUTEN_R = crate::BitReader<bool>;
#[doc = "Field `OUTEN` writer - enable clock output"]
pub type OUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MONITOR_SLICE3_CONTROL_SPEC, bool, O>;
#[doc = "Field `DIV_BUSY` reader - divider is applying new setting"]
pub type DIV_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `VALID` reader - result is ready for read 0: not ready 1: result is ready"]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `VALID` writer - result is ready for read 0: not ready 1: result is ready"]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, MONITOR_SLICE3_CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - clock measurement selection"]
    #[inline(always)]
    pub fn selection(&self) -> SELECTION_R {
        SELECTION_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - refrence clock selection, 0: 32k 1: 24M"]
    #[inline(always)]
    pub fn reference(&self) -> REFERENCE_R {
        REFERENCE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz"]
    #[inline(always)]
    pub fn accuracy(&self) -> ACCURACY_R {
        ACCURACY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - start measurement"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - clock frequency lower than lower limit"]
    #[inline(always)]
    pub fn low(&self) -> LOW_R {
        LOW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - clock frequency higher than upper limit"]
    #[inline(always)]
    pub fn high(&self) -> HIGH_R {
        HIGH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - output divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - enable clock output"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - divider is applying new setting"]
    #[inline(always)]
    pub fn div_busy(&self) -> DIV_BUSY_R {
        DIV_BUSY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - result is ready for read 0: not ready 1: result is ready"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - clock measurement selection"]
    #[inline(always)]
    #[must_use]
    pub fn selection(&mut self) -> SELECTION_W<0> {
        SELECTION_W::new(self)
    }
    #[doc = "Bit 8 - refrence clock selection, 0: 32k 1: 24M"]
    #[inline(always)]
    #[must_use]
    pub fn reference(&mut self) -> REFERENCE_W<8> {
        REFERENCE_W::new(self)
    }
    #[doc = "Bit 9 - measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz"]
    #[inline(always)]
    #[must_use]
    pub fn accuracy(&mut self) -> ACCURACY_W<9> {
        ACCURACY_W::new(self)
    }
    #[doc = "Bit 10 - work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<10> {
        MODE_W::new(self)
    }
    #[doc = "Bit 12 - start measurement"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<12> {
        START_W::new(self)
    }
    #[doc = "Bit 14 - clock frequency lower than lower limit"]
    #[inline(always)]
    #[must_use]
    pub fn low(&mut self) -> LOW_W<14> {
        LOW_W::new(self)
    }
    #[doc = "Bit 15 - clock frequency higher than upper limit"]
    #[inline(always)]
    #[must_use]
    pub fn high(&mut self) -> HIGH_W<15> {
        HIGH_W::new(self)
    }
    #[doc = "Bits 16:23 - output divider"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<16> {
        DIV_W::new(self)
    }
    #[doc = "Bit 24 - enable clock output"]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OUTEN_W<24> {
        OUTEN_W::new(self)
    }
    #[doc = "Bit 31 - result is ready for read 0: not ready 1: result is ready"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<31> {
        VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock measure and monitor control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [monitor_slice3_control](index.html) module"]
pub struct MONITOR_SLICE3_CONTROL_SPEC;
impl crate::RegisterSpec for MONITOR_SLICE3_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [monitor_slice3_control::R](R) reader structure"]
impl crate::Readable for MONITOR_SLICE3_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [monitor_slice3_control::W](W) writer structure"]
impl crate::Writable for MONITOR_SLICE3_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MONITOR_SLICE3_CONTROL to value 0"]
impl crate::Resettable for MONITOR_SLICE3_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
