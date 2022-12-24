#[doc = "Register `RESET_VIS_CONFIG` reader"]
pub struct R(crate::R<RESET_VIS_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_VIS_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_VIS_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_VIS_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_VIS_CONFIG` writer"]
pub struct W(crate::W<RESET_VIS_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_VIS_CONFIG_SPEC>;
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
impl From<crate::W<RESET_VIS_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_VIS_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POST_WAIT` reader - time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
pub type POST_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POST_WAIT` writer - time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
pub type POST_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RESET_VIS_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `RSTCLK_NUM` reader - reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M"]
pub type RSTCLK_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSTCLK_NUM` writer - reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M"]
pub type RSTCLK_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RESET_VIS_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `PRE_WAIT` reader - wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
pub type PRE_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRE_WAIT` writer - wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
pub type PRE_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RESET_VIS_CONFIG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    pub fn post_wait(&self) -> POST_WAIT_R {
        POST_WAIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    pub fn rstclk_num(&self) -> RSTCLK_NUM_R {
        RSTCLK_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    pub fn pre_wait(&self) -> PRE_WAIT_R {
        PRE_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    #[must_use]
    pub fn post_wait(&mut self) -> POST_WAIT_W<0> {
        POST_WAIT_W::new(self)
    }
    #[doc = "Bits 8:15 - reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    #[must_use]
    pub fn rstclk_num(&mut self) -> RSTCLK_NUM_W<8> {
        RSTCLK_NUM_W::new(self)
    }
    #[doc = "Bits 16:23 - wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    #[inline(always)]
    #[must_use]
    pub fn pre_wait(&mut self) -> PRE_WAIT_W<16> {
        PRE_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_vis_config](index.html) module"]
pub struct RESET_VIS_CONFIG_SPEC;
impl crate::RegisterSpec for RESET_VIS_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_vis_config::R](R) reader structure"]
impl crate::Readable for RESET_VIS_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_vis_config::W](W) writer structure"]
impl crate::Writable for RESET_VIS_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESET_VIS_CONFIG to value 0x64"]
impl crate::Resettable for RESET_VIS_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x64;
}
