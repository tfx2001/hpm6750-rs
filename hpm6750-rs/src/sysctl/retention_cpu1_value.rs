#[doc = "Register `RETENTION_CPU1_VALUE` reader"]
pub struct R(crate::R<RETENTION_CPU1_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETENTION_CPU1_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETENTION_CPU1_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETENTION_CPU1_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETENTION_CPU1_VALUE` writer"]
pub struct W(crate::W<RETENTION_CPU1_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETENTION_CPU1_VALUE_SPEC>;
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
impl From<crate::W<RETENTION_CPU1_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETENTION_CPU1_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINK` reader - retention setting while system sleep, each bit represents a resource bit0: soc_pow bit1: soc_rst bit2: cpu0_pow bit3: cpu0_rst bit4: cpu1_pow bit5: cpu1_rst bit6: con_pow bit7: con_rst bit8: vis_pow bit9: vis_rst bit10: xtal bit11: pll0 bit12: pll1 bit13: pll2 bit14: pll3 bit15: pll4"]
pub type LINK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LINK` writer - retention setting while system sleep, each bit represents a resource bit0: soc_pow bit1: soc_rst bit2: cpu0_pow bit3: cpu0_rst bit4: cpu1_pow bit5: cpu1_rst bit6: con_pow bit7: con_rst bit8: vis_pow bit9: vis_rst bit10: xtal bit11: pll0 bit12: pll1 bit13: pll2 bit14: pll3 bit15: pll4"]
pub type LINK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RETENTION_CPU1_VALUE_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:17 - retention setting while system sleep, each bit represents a resource bit0: soc_pow bit1: soc_rst bit2: cpu0_pow bit3: cpu0_rst bit4: cpu1_pow bit5: cpu1_rst bit6: con_pow bit7: con_rst bit8: vis_pow bit9: vis_rst bit10: xtal bit11: pll0 bit12: pll1 bit13: pll2 bit14: pll3 bit15: pll4"]
    #[inline(always)]
    pub fn link(&self) -> LINK_R {
        LINK_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - retention setting while system sleep, each bit represents a resource bit0: soc_pow bit1: soc_rst bit2: cpu0_pow bit3: cpu0_rst bit4: cpu1_pow bit5: cpu1_rst bit6: con_pow bit7: con_rst bit8: vis_pow bit9: vis_rst bit10: xtal bit11: pll0 bit12: pll1 bit13: pll2 bit14: pll3 bit15: pll4"]
    #[inline(always)]
    pub fn link(&mut self) -> LINK_W<0> {
        LINK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Retention Contol\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_cpu1_value](index.html) module"]
pub struct RETENTION_CPU1_VALUE_SPEC;
impl crate::RegisterSpec for RETENTION_CPU1_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retention_cpu1_value::R](R) reader structure"]
impl crate::Readable for RETENTION_CPU1_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retention_cpu1_value::W](W) writer structure"]
impl crate::Writable for RETENTION_CPU1_VALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RETENTION_CPU1_VALUE to value 0"]
impl crate::Resettable for RETENTION_CPU1_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
