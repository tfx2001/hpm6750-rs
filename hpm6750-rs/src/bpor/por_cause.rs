#[doc = "Register `POR_CAUSE` reader"]
pub struct R(crate::R<POR_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POR_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POR_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POR_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POR_CAUSE` writer"]
pub struct W(crate::W<POR_CAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POR_CAUSE_SPEC>;
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
impl From<crate::W<POR_CAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POR_CAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAUSE` reader - Power on cause, each bit represnts one cause, write 1 to clear each bit bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO"]
pub type CAUSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAUSE` writer - Power on cause, each bit represnts one cause, write 1 to clear each bit bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO"]
pub type CAUSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POR_CAUSE_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Power on cause, each bit represnts one cause, write 1 to clear each bit bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO"]
    #[inline(always)]
    pub fn cause(&self) -> CAUSE_R {
        CAUSE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Power on cause, each bit represnts one cause, write 1 to clear each bit bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO"]
    #[inline(always)]
    pub fn cause(&mut self) -> CAUSE_W<0> {
        CAUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power on cause\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [por_cause](index.html) module"]
pub struct POR_CAUSE_SPEC;
impl crate::RegisterSpec for POR_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [por_cause::R](R) reader structure"]
impl crate::Readable for POR_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [por_cause::W](W) writer structure"]
impl crate::Writable for POR_CAUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POR_CAUSE to value 0"]
impl crate::Resettable for POR_CAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
