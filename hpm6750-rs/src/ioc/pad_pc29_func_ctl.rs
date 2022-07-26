#[doc = "Register `PAD_PC29_FUNC_CTL` reader"]
pub struct R(crate::R<PAD_PC29_FUNC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD_PC29_FUNC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD_PC29_FUNC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD_PC29_FUNC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_PC29_FUNC_CTL` writer"]
pub struct W(crate::W<PAD_PC29_FUNC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD_PC29_FUNC_CTL_SPEC>;
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
impl From<crate::W<PAD_PC29_FUNC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD_PC29_FUNC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOOP_BACK` reader - force input on 0: disable 1: enable"]
pub type LOOP_BACK_R = crate::BitReader<bool>;
#[doc = "Field `LOOP_BACK` writer - force input on 0: disable 1: enable"]
pub type LOOP_BACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_PC29_FUNC_CTL_SPEC, bool, O>;
#[doc = "Field `ANALOG` reader - select analog pin in pad 0: disable 1: enable"]
pub type ANALOG_R = crate::BitReader<bool>;
#[doc = "Field `ANALOG` writer - select analog pin in pad 0: disable 1: enable"]
pub type ANALOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PAD_PC29_FUNC_CTL_SPEC, bool, O>;
#[doc = "Field `ALT_SELECT` reader - alt select 0: ALT0 1: ALT1 … 31:ALT31"]
pub type ALT_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALT_SELECT` writer - alt select 0: ALT0 1: ALT1 … 31:ALT31"]
pub type ALT_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PAD_PC29_FUNC_CTL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 16 - force input on 0: disable 1: enable"]
    #[inline(always)]
    pub fn loop_back(&self) -> LOOP_BACK_R {
        LOOP_BACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 8 - select analog pin in pad 0: disable 1: enable"]
    #[inline(always)]
    pub fn analog(&self) -> ANALOG_R {
        ANALOG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:4 - alt select 0: ALT0 1: ALT1 … 31:ALT31"]
    #[inline(always)]
    pub fn alt_select(&self) -> ALT_SELECT_R {
        ALT_SELECT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - force input on 0: disable 1: enable"]
    #[inline(always)]
    pub fn loop_back(&mut self) -> LOOP_BACK_W<16> {
        LOOP_BACK_W::new(self)
    }
    #[doc = "Bit 8 - select analog pin in pad 0: disable 1: enable"]
    #[inline(always)]
    pub fn analog(&mut self) -> ANALOG_W<8> {
        ANALOG_W::new(self)
    }
    #[doc = "Bits 0:4 - alt select 0: ALT0 1: ALT1 … 31:ALT31"]
    #[inline(always)]
    pub fn alt_select(&mut self) -> ALT_SELECT_W<0> {
        ALT_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ALT SELECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad_pc29_func_ctl](index.html) module"]
pub struct PAD_PC29_FUNC_CTL_SPEC;
impl crate::RegisterSpec for PAD_PC29_FUNC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad_pc29_func_ctl::R](R) reader structure"]
impl crate::Readable for PAD_PC29_FUNC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad_pc29_func_ctl::W](W) writer structure"]
impl crate::Writable for PAD_PC29_FUNC_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_PC29_FUNC_CTL to value 0"]
impl crate::Resettable for PAD_PC29_FUNC_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
