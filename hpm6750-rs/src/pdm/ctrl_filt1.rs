#[doc = "Register `CTRL_FILT1` reader"]
pub struct R(crate::R<CTRL_FILT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_FILT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_FILT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_FILT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_FILT1` writer"]
pub struct W(crate::W<CTRL_FILT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_FILT1_SPEC>;
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
impl From<crate::W<CTRL_FILT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_FILT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COEF_LEN_M1` reader - Coef length of filter type 2'b01 in coef memory"]
pub type COEF_LEN_M1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEF_LEN_M1` writer - Coef length of filter type 2'b01 in coef memory"]
pub type COEF_LEN_M1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_FILT1_SPEC, u8, u8, 8, O>;
#[doc = "Field `COEF_START_ADDR` reader - Starting address of Coef of filter type 2'b01 in coef memory"]
pub type COEF_START_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COEF_START_ADDR` writer - Starting address of Coef of filter type 2'b01 in coef memory"]
pub type COEF_START_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_FILT1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 8:15 - Coef length of filter type 2'b01 in coef memory"]
    #[inline(always)]
    pub fn coef_len_m1(&self) -> COEF_LEN_M1_R {
        COEF_LEN_M1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Starting address of Coef of filter type 2'b01 in coef memory"]
    #[inline(always)]
    pub fn coef_start_addr(&self) -> COEF_START_ADDR_R {
        COEF_START_ADDR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Coef length of filter type 2'b01 in coef memory"]
    #[inline(always)]
    pub fn coef_len_m1(&mut self) -> COEF_LEN_M1_W<8> {
        COEF_LEN_M1_W::new(self)
    }
    #[doc = "Bits 0:7 - Starting address of Coef of filter type 2'b01 in coef memory"]
    #[inline(always)]
    pub fn coef_start_addr(&mut self) -> COEF_START_ADDR_W<0> {
        COEF_START_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_filt1](index.html) module"]
pub struct CTRL_FILT1_SPEC;
impl crate::RegisterSpec for CTRL_FILT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_filt1::R](R) reader structure"]
impl crate::Readable for CTRL_FILT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_filt1::W](W) writer structure"]
impl crate::Writable for CTRL_FILT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_FILT1 to value 0"]
impl crate::Resettable for CTRL_FILT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
