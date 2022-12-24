#[doc = "Register `BTN_STATUS` reader"]
pub struct R(crate::R<BTN_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTN_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTN_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTN_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTN_STATUS` writer"]
pub struct W(crate::W<BTN_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTN_STATUS_SPEC>;
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
impl From<crate::W<BTN_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTN_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBTN` reader - Power button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
pub type PBTN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PBTN` writer - Power button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
pub type PBTN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTN_STATUS_SPEC, u8, u8, 4, O>;
#[doc = "Field `WBTN` reader - Wake button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
pub type WBTN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WBTN` writer - Wake button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
pub type WBTN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTN_STATUS_SPEC, u8, u8, 4, O>;
#[doc = "Field `DBTN` reader - Dual button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
pub type DBTN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBTN` writer - Dual button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
pub type DBTN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTN_STATUS_SPEC, u8, u8, 4, O>;
#[doc = "Field `PCLICK` reader - power button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type PCLICK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCLICK` writer - power button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type PCLICK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTN_STATUS_SPEC, u8, u8, 3, O>;
#[doc = "Field `XPCLICK` reader - power button click status when wake button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type XPCLICK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XPCLICK` writer - power button click status when wake button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type XPCLICK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTN_STATUS_SPEC, u8, u8, 3, O>;
#[doc = "Field `WCLICK` reader - wake button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type WCLICK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WCLICK` writer - wake button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type WCLICK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTN_STATUS_SPEC, u8, u8, 3, O>;
#[doc = "Field `XWCLICK` reader - wake button click status when power button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type XWCLICK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XWCLICK` writer - wake button click status when power button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type XWCLICK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTN_STATUS_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:3 - Power button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    #[inline(always)]
    pub fn pbtn(&self) -> PBTN_R {
        PBTN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Wake button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    #[inline(always)]
    pub fn wbtn(&self) -> WBTN_R {
        WBTN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Dual button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    #[inline(always)]
    pub fn dbtn(&self) -> DBTN_R {
        DBTN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - power button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    pub fn pclick(&self) -> PCLICK_R {
        PCLICK_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - power button click status when wake button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    pub fn xpclick(&self) -> XPCLICK_R {
        XPCLICK_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - wake button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    pub fn wclick(&self) -> WCLICK_R {
        WCLICK_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - wake button click status when power button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    pub fn xwclick(&self) -> XWCLICK_R {
        XWCLICK_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Power button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    #[inline(always)]
    #[must_use]
    pub fn pbtn(&mut self) -> PBTN_W<0> {
        PBTN_W::new(self)
    }
    #[doc = "Bits 4:7 - Wake button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    #[inline(always)]
    #[must_use]
    pub fn wbtn(&mut self) -> WBTN_W<4> {
        WBTN_W::new(self)
    }
    #[doc = "Bits 8:11 - Dual button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    #[inline(always)]
    #[must_use]
    pub fn dbtn(&mut self) -> DBTN_W<8> {
        DBTN_W::new(self)
    }
    #[doc = "Bits 16:18 - power button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    #[must_use]
    pub fn pclick(&mut self) -> PCLICK_W<16> {
        PCLICK_W::new(self)
    }
    #[doc = "Bits 20:22 - power button click status when wake button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    #[must_use]
    pub fn xpclick(&mut self) -> XPCLICK_W<20> {
        XPCLICK_W::new(self)
    }
    #[doc = "Bits 24:26 - wake button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    #[must_use]
    pub fn wclick(&mut self) -> WCLICK_W<24> {
        WCLICK_W::new(self)
    }
    #[doc = "Bits 28:30 - wake button click status when power button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    #[must_use]
    pub fn xwclick(&mut self) -> XWCLICK_W<28> {
        XWCLICK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Button status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btn_status](index.html) module"]
pub struct BTN_STATUS_SPEC;
impl crate::RegisterSpec for BTN_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btn_status::R](R) reader structure"]
impl crate::Readable for BTN_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btn_status::W](W) writer structure"]
impl crate::Writable for BTN_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BTN_STATUS to value 0"]
impl crate::Resettable for BTN_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
