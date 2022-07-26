#[doc = "Register `CQISE` reader"]
pub struct R(crate::R<CQISE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQISE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQISE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQISE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQISE` writer"]
pub struct W(crate::W<CQISE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQISE_SPEC>;
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
impl From<crate::W<CQISE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQISE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCL_STE` reader - Task cleared interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.TCL is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.TCL is disabled"]
pub type TCL_STE_R = crate::BitReader<bool>;
#[doc = "Field `TCL_STE` writer - Task cleared interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.TCL is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.TCL is disabled"]
pub type TCL_STE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQISE_SPEC, bool, O>;
#[doc = "Field `RED_STE` reader - Response error detected interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.RED is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.RED is disabled"]
pub type RED_STE_R = crate::BitReader<bool>;
#[doc = "Field `RED_STE` writer - Response error detected interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.RED is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.RED is disabled"]
pub type RED_STE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQISE_SPEC, bool, O>;
#[doc = "Field `TCC_STE` reader - Task complete interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.TCC is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.TCC is disabled"]
pub type TCC_STE_R = crate::BitReader<bool>;
#[doc = "Field `TCC_STE` writer - Task complete interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.TCC is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.TCC is disabled"]
pub type TCC_STE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQISE_SPEC, bool, O>;
#[doc = "Field `HAC_STE` reader - Halt complete interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.HAC is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.HAC is disabled"]
pub type HAC_STE_R = crate::BitReader<bool>;
#[doc = "Field `HAC_STE` writer - Halt complete interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.HAC is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.HAC is disabled"]
pub type HAC_STE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQISE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - Task cleared interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.TCL is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.TCL is disabled"]
    #[inline(always)]
    pub fn tcl_ste(&self) -> TCL_STE_R {
        TCL_STE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Response error detected interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.RED is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.RED is disabled"]
    #[inline(always)]
    pub fn red_ste(&self) -> RED_STE_R {
        RED_STE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Task complete interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.TCC is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.TCC is disabled"]
    #[inline(always)]
    pub fn tcc_ste(&self) -> TCC_STE_R {
        TCC_STE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Halt complete interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.HAC is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.HAC is disabled"]
    #[inline(always)]
    pub fn hac_ste(&self) -> HAC_STE_R {
        HAC_STE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Task cleared interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.TCL is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.TCL is disabled"]
    #[inline(always)]
    pub fn tcl_ste(&mut self) -> TCL_STE_W<3> {
        TCL_STE_W::new(self)
    }
    #[doc = "Bit 2 - Response error detected interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.RED is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.RED is disabled"]
    #[inline(always)]
    pub fn red_ste(&mut self) -> RED_STE_W<2> {
        RED_STE_W::new(self)
    }
    #[doc = "Bit 1 - Task complete interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.TCC is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.TCC is disabled"]
    #[inline(always)]
    pub fn tcc_ste(&mut self) -> TCC_STE_W<1> {
        TCC_STE_W::new(self)
    }
    #[doc = "Bit 0 - Halt complete interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.HAC is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.HAC is disabled"]
    #[inline(always)]
    pub fn hac_ste(&mut self) -> HAC_STE_W<0> {
        HAC_STE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqise](index.html) module"]
pub struct CQISE_SPEC;
impl crate::RegisterSpec for CQISE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqise::R](R) reader structure"]
impl crate::Readable for CQISE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqise::W](W) writer structure"]
impl crate::Writable for CQISE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CQISE to value 0"]
impl crate::Resettable for CQISE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
