#[doc = "Register `SCG_CTRL` reader"]
pub struct R(crate::R<SCG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCG_CTRL` writer"]
pub struct W(crate::W<SCG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCG_CTRL_SPEC>;
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
impl From<crate::W<SCG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCG` reader - control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: clock gated according to low power flow 10: clock is always off 11: clock is always on bit0-1: fuse bit2-3: sram bit4-5: vad bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart bit16-17:debug"]
pub type SCG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SCG` writer - control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: clock gated according to low power flow 10: clock is always off 11: clock is always on bit0-1: fuse bit2-3: sram bit4-5: vad bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart bit16-17:debug"]
pub type SCG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCG_CTRL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: clock gated according to low power flow 10: clock is always off 11: clock is always on bit0-1: fuse bit2-3: sram bit4-5: vad bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart bit16-17:debug"]
    #[inline(always)]
    pub fn scg(&self) -> SCG_R {
        SCG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: clock gated according to low power flow 10: clock is always off 11: clock is always on bit0-1: fuse bit2-3: sram bit4-5: vad bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart bit16-17:debug"]
    #[inline(always)]
    pub fn scg(&mut self) -> SCG_W<0> {
        SCG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock gate control in PMIC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scg_ctrl](index.html) module"]
pub struct SCG_CTRL_SPEC;
impl crate::RegisterSpec for SCG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scg_ctrl::R](R) reader structure"]
impl crate::Readable for SCG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scg_ctrl::W](W) writer structure"]
impl crate::Writable for SCG_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCG_CTRL to value 0xffff_ffff"]
impl crate::Resettable for SCG_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
