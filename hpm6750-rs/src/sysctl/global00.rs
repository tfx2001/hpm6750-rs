#[doc = "Register `GLOBAL00` reader"]
pub struct R(crate::R<GLOBAL00_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLOBAL00_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLOBAL00_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLOBAL00_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLOBAL00` writer"]
pub struct W(crate::W<GLOBAL00_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLOBAL00_SPEC>;
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
impl From<crate::W<GLOBAL00_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLOBAL00_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESET` reader - global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3"]
pub type PRESET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESET` writer - global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3"]
pub type PRESET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLOBAL00_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3"]
    #[inline(always)]
    pub fn preset(&self) -> PRESET_R {
        PRESET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3"]
    #[inline(always)]
    pub fn preset(&mut self) -> PRESET_W<0> {
        PRESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock senario\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [global00](index.html) module"]
pub struct GLOBAL00_SPEC;
impl crate::RegisterSpec for GLOBAL00_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [global00::R](R) reader structure"]
impl crate::Readable for GLOBAL00_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [global00::W](W) writer structure"]
impl crate::Writable for GLOBAL00_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GLOBAL00 to value 0"]
impl crate::Resettable for GLOBAL00_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
