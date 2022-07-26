#[doc = "Register `TRGOCFG_TRGM_OUT0` reader"]
pub struct R(crate::R<TRGOCFG_TRGM_OUT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRGOCFG_TRGM_OUT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRGOCFG_TRGM_OUT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRGOCFG_TRGM_OUT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRGOCFG_TRGM_OUT0` writer"]
pub struct W(crate::W<TRGOCFG_TRGM_OUT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRGOCFG_TRGM_OUT0_SPEC>;
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
impl From<crate::W<TRGOCFG_TRGM_OUT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRGOCFG_TRGM_OUT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTINV` reader - 1- Invert the output"]
pub type OUTINV_R = crate::BitReader<bool>;
#[doc = "Field `OUTINV` writer - 1- Invert the output"]
pub type OUTINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRGOCFG_TRGM_OUT0_SPEC, bool, O>;
#[doc = "Field `FEDG2PEN` reader - 1- The selected input signal falling edge will be convert to an pulse on output."]
pub type FEDG2PEN_R = crate::BitReader<bool>;
#[doc = "Field `FEDG2PEN` writer - 1- The selected input signal falling edge will be convert to an pulse on output."]
pub type FEDG2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRGOCFG_TRGM_OUT0_SPEC, bool, O>;
#[doc = "Field `REDG2PEN` reader - 1- The selected input signal rising edge will be convert to an pulse on output."]
pub type REDG2PEN_R = crate::BitReader<bool>;
#[doc = "Field `REDG2PEN` writer - 1- The selected input signal rising edge will be convert to an pulse on output."]
pub type REDG2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRGOCFG_TRGM_OUT0_SPEC, bool, O>;
#[doc = "Field `TRIGOSEL` reader - This bitfield selects one of the TRGM inputs as output."]
pub type TRIGOSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGOSEL` writer - This bitfield selects one of the TRGM inputs as output."]
pub type TRIGOSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRGOCFG_TRGM_OUT0_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 8 - 1- Invert the output"]
    #[inline(always)]
    pub fn outinv(&self) -> OUTINV_R {
        OUTINV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - 1- The selected input signal falling edge will be convert to an pulse on output."]
    #[inline(always)]
    pub fn fedg2pen(&self) -> FEDG2PEN_R {
        FEDG2PEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - 1- The selected input signal rising edge will be convert to an pulse on output."]
    #[inline(always)]
    pub fn redg2pen(&self) -> REDG2PEN_R {
        REDG2PEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 0:5 - This bitfield selects one of the TRGM inputs as output."]
    #[inline(always)]
    pub fn trigosel(&self) -> TRIGOSEL_R {
        TRIGOSEL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - 1- Invert the output"]
    #[inline(always)]
    pub fn outinv(&mut self) -> OUTINV_W<8> {
        OUTINV_W::new(self)
    }
    #[doc = "Bit 7 - 1- The selected input signal falling edge will be convert to an pulse on output."]
    #[inline(always)]
    pub fn fedg2pen(&mut self) -> FEDG2PEN_W<7> {
        FEDG2PEN_W::new(self)
    }
    #[doc = "Bit 6 - 1- The selected input signal rising edge will be convert to an pulse on output."]
    #[inline(always)]
    pub fn redg2pen(&mut self) -> REDG2PEN_W<6> {
        REDG2PEN_W::new(self)
    }
    #[doc = "Bits 0:5 - This bitfield selects one of the TRGM inputs as output."]
    #[inline(always)]
    pub fn trigosel(&mut self) -> TRIGOSEL_W<0> {
        TRIGOSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger manager output configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgocfg_trgm_out0](index.html) module"]
pub struct TRGOCFG_TRGM_OUT0_SPEC;
impl crate::RegisterSpec for TRGOCFG_TRGM_OUT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trgocfg_trgm_out0::R](R) reader structure"]
impl crate::Readable for TRGOCFG_TRGM_OUT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trgocfg_trgm_out0::W](W) writer structure"]
impl crate::Writable for TRGOCFG_TRGM_OUT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRGOCFG_TRGM_OUT0 to value 0"]
impl crate::Resettable for TRGOCFG_TRGM_OUT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
