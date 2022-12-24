#[doc = "Register `POR_SELECT` reader"]
pub struct R(crate::R<POR_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POR_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POR_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POR_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POR_SELECT` writer"]
pub struct W(crate::W<POR_SELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POR_SELECT_SPEC>;
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
impl From<crate::W<POR_SELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POR_SELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELECT` reader - Power on cause select, each bit represnts one cause, value 1 enables corresponding cause bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO"]
pub type SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELECT` writer - Power on cause select, each bit represnts one cause, value 1 enables corresponding cause bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO"]
pub type SELECT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POR_SELECT_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Power on cause select, each bit represnts one cause, value 1 enables corresponding cause bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO"]
    #[inline(always)]
    pub fn select(&self) -> SELECT_R {
        SELECT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Power on cause select, each bit represnts one cause, value 1 enables corresponding cause bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SELECT_W<0> {
        SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power on select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [por_select](index.html) module"]
pub struct POR_SELECT_SPEC;
impl crate::RegisterSpec for POR_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [por_select::R](R) reader structure"]
impl crate::Readable for POR_SELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [por_select::W](W) writer structure"]
impl crate::Writable for POR_SELECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POR_SELECT to value 0"]
impl crate::Resettable for POR_SELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
