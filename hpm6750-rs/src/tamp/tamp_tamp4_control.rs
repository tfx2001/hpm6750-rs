#[doc = "Register `TAMP_TAMP4_CONTROL` reader"]
pub struct R(crate::R<TAMP_TAMP4_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_TAMP4_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_TAMP4_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_TAMP4_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP_TAMP4_CONTROL` writer"]
pub struct W(crate::W<TAMP_TAMP4_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_TAMP4_CONTROL_SPEC>;
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
impl From<crate::W<TAMP_TAMP4_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_TAMP4_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - enable tamper 0: tamper disableed 1: tamper enabled"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - enable tamper 0: tamper disableed 1: tamper enabled"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_TAMP4_CONTROL_SPEC, bool, O>;
#[doc = "Field `ACTIVE` reader - select active or passive tamper 0: passive tamper 1: active tamper"]
pub type ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE` writer - select active or passive tamper 0: passive tamper 1: active tamper"]
pub type ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_TAMP4_CONTROL_SPEC, bool, O>;
#[doc = "Field `RECOVER` reader - tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover"]
pub type RECOVER_R = crate::BitReader<bool>;
#[doc = "Field `RECOVER` writer - tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover"]
pub type RECOVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_TAMP4_CONTROL_SPEC, bool, O>;
#[doc = "Field `SPEED` reader - tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second"]
pub type SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPEED` writer - tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second"]
pub type SPEED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TAMP_TAMP4_CONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `VALUE` reader - pin value for passive tamper"]
pub type VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VALUE` writer - pin value for passive tamper"]
pub type VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TAMP_TAMP4_CONTROL_SPEC, u8, u8, 2, O>;
#[doc = "Field `FILTER` reader - filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle"]
pub type FILTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTER` writer - filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle"]
pub type FILTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TAMP_TAMP4_CONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `BYPASS` reader - bypass tamper violation filter 0: filter applied 1: filter not used"]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - bypass tamper violation filter 0: filter applied 1: filter not used"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_TAMP4_CONTROL_SPEC, bool, O>;
#[doc = "Field `LOCK` reader - lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_TAMP4_CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - enable tamper 0: tamper disableed 1: tamper enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - select active or passive tamper 0: passive tamper 1: active tamper"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover"]
    #[inline(always)]
    pub fn recover(&self) -> RECOVER_R {
        RECOVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - pin value for passive tamper"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:19 - filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - bypass tamper violation filter 0: filter applied 1: filter not used"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable tamper 0: tamper disableed 1: tamper enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - select active or passive tamper 0: passive tamper 1: active tamper"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<1> {
        ACTIVE_W::new(self)
    }
    #[doc = "Bit 2 - tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover"]
    #[inline(always)]
    #[must_use]
    pub fn recover(&mut self) -> RECOVER_W<2> {
        RECOVER_W::new(self)
    }
    #[doc = "Bits 4:7 - tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<4> {
        SPEED_W::new(self)
    }
    #[doc = "Bits 8:9 - pin value for passive tamper"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<8> {
        VALUE_W::new(self)
    }
    #[doc = "Bits 16:19 - filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FILTER_W<16> {
        FILTER_W::new(self)
    }
    #[doc = "Bit 20 - bypass tamper violation filter 0: filter applied 1: filter not used"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<20> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 31 - lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tamper4 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_tamp4_control](index.html) module"]
pub struct TAMP_TAMP4_CONTROL_SPEC;
impl crate::RegisterSpec for TAMP_TAMP4_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_tamp4_control::R](R) reader structure"]
impl crate::Readable for TAMP_TAMP4_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp_tamp4_control::W](W) writer structure"]
impl crate::Writable for TAMP_TAMP4_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMP_TAMP4_CONTROL to value 0"]
impl crate::Resettable for TAMP_TAMP4_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
