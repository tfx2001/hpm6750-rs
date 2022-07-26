#[doc = "Register `ASSIGN_GPIOC_PIN_PIN05` reader"]
pub struct R(crate::R<ASSIGN_GPIOC_PIN_PIN05_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASSIGN_GPIOC_PIN_PIN05_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASSIGN_GPIOC_PIN_PIN05_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASSIGN_GPIOC_PIN_PIN05_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASSIGN_GPIOC_PIN_PIN05` writer"]
pub struct W(crate::W<ASSIGN_GPIOC_PIN_PIN05_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASSIGN_GPIOC_PIN_PIN05_SPEC>;
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
impl From<crate::W<ASSIGN_GPIOC_PIN_PIN05_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASSIGN_GPIOC_PIN_PIN05_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASSIGN_GPIOC_PIN_PIN05_SPEC, bool, O>;
#[doc = "Field `HIDE` reader - pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit1: 1, invisible to soc gpio1; 0: visible to soc gpio1 bit2: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio bit3: 1, invisible to cpu1 fast gpio; 0: visible to cpu1 fast gpio"]
pub type HIDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIDE` writer - pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit1: 1, invisible to soc gpio1; 0: visible to soc gpio1 bit2: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio bit3: 1, invisible to cpu1 fast gpio; 0: visible to cpu1 fast gpio"]
pub type HIDE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ASSIGN_GPIOC_PIN_PIN05_SPEC, u8, u8, 4, O>;
#[doc = "Field `SELECT` reader - select which gpio controls chip pin, 0: soc gpio0; 1: soc gpio1; 2: cpu0 fastgpio 3: cpu1 fast gpio"]
pub type SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELECT` writer - select which gpio controls chip pin, 0: soc gpio0; 1: soc gpio1; 2: cpu0 fastgpio 3: cpu1 fast gpio"]
pub type SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ASSIGN_GPIOC_PIN_PIN05_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 31 - lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 8:11 - pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit1: 1, invisible to soc gpio1; 0: visible to soc gpio1 bit2: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio bit3: 1, invisible to cpu1 fast gpio; 0: visible to cpu1 fast gpio"]
    #[inline(always)]
    pub fn hide(&self) -> HIDE_R {
        HIDE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1 - select which gpio controls chip pin, 0: soc gpio0; 1: soc gpio1; 2: cpu0 fastgpio 3: cpu1 fast gpio"]
    #[inline(always)]
    pub fn select(&self) -> SELECT_R {
        SELECT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Bits 8:11 - pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit1: 1, invisible to soc gpio1; 0: visible to soc gpio1 bit2: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio bit3: 1, invisible to cpu1 fast gpio; 0: visible to cpu1 fast gpio"]
    #[inline(always)]
    pub fn hide(&mut self) -> HIDE_W<8> {
        HIDE_W::new(self)
    }
    #[doc = "Bits 0:1 - select which gpio controls chip pin, 0: soc gpio0; 1: soc gpio1; 2: cpu0 fastgpio 3: cpu1 fast gpio"]
    #[inline(always)]
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
#[doc = "GPIO mananger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [assign_gpioc_pin_pin05](index.html) module"]
pub struct ASSIGN_GPIOC_PIN_PIN05_SPEC;
impl crate::RegisterSpec for ASSIGN_GPIOC_PIN_PIN05_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [assign_gpioc_pin_pin05::R](R) reader structure"]
impl crate::Readable for ASSIGN_GPIOC_PIN_PIN05_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [assign_gpioc_pin_pin05::W](W) writer structure"]
impl crate::Writable for ASSIGN_GPIOC_PIN_PIN05_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASSIGN_GPIOC_PIN_PIN05 to value 0"]
impl crate::Resettable for ASSIGN_GPIOC_PIN_PIN05_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
