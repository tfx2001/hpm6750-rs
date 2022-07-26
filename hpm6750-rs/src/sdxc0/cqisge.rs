#[doc = "Register `CQISGE` reader"]
pub struct R(crate::R<CQISGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQISGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQISGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQISGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQISGE` writer"]
pub struct W(crate::W<CQISGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQISGE_SPEC>;
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
impl From<crate::W<CQISGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQISGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCL_SGE` reader - Task cleared interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.TCL interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.TCL interrupt signal generation is disabled"]
pub type TCL_SGE_R = crate::BitReader<bool>;
#[doc = "Field `TCL_SGE` writer - Task cleared interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.TCL interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.TCL interrupt signal generation is disabled"]
pub type TCL_SGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQISGE_SPEC, bool, O>;
#[doc = "Field `RED_SGE` reader - Response error detected interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.RED interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.RED interrupt signal generation is disabled"]
pub type RED_SGE_R = crate::BitReader<bool>;
#[doc = "Field `RED_SGE` writer - Response error detected interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.RED interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.RED interrupt signal generation is disabled"]
pub type RED_SGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQISGE_SPEC, bool, O>;
#[doc = "Field `TCC_SGE` reader - Task complete interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.TCC interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.TCC interrupt signal generation is disabled"]
pub type TCC_SGE_R = crate::BitReader<bool>;
#[doc = "Field `TCC_SGE` writer - Task complete interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.TCC interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.TCC interrupt signal generation is disabled"]
pub type TCC_SGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQISGE_SPEC, bool, O>;
#[doc = "Field `HAC_SGE` reader - Halt complete interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.HAC interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.HAC interrupt signal generation is disabled"]
pub type HAC_SGE_R = crate::BitReader<bool>;
#[doc = "Field `HAC_SGE` writer - Halt complete interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.HAC interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.HAC interrupt signal generation is disabled"]
pub type HAC_SGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQISGE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - Task cleared interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.TCL interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.TCL interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn tcl_sge(&self) -> TCL_SGE_R {
        TCL_SGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Response error detected interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.RED interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.RED interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn red_sge(&self) -> RED_SGE_R {
        RED_SGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Task complete interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.TCC interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.TCC interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn tcc_sge(&self) -> TCC_SGE_R {
        TCC_SGE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Halt complete interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.HAC interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.HAC interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn hac_sge(&self) -> HAC_SGE_R {
        HAC_SGE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Task cleared interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.TCL interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.TCL interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn tcl_sge(&mut self) -> TCL_SGE_W<3> {
        TCL_SGE_W::new(self)
    }
    #[doc = "Bit 2 - Response error detected interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.RED interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.RED interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn red_sge(&mut self) -> RED_SGE_W<2> {
        RED_SGE_W::new(self)
    }
    #[doc = "Bit 1 - Task complete interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.TCC interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.TCC interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn tcc_sge(&mut self) -> TCC_SGE_W<1> {
        TCC_SGE_W::new(self)
    }
    #[doc = "Bit 0 - Halt complete interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.HAC interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.HAC interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn hac_sge(&mut self) -> HAC_SGE_W<0> {
        HAC_SGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqisge](index.html) module"]
pub struct CQISGE_SPEC;
impl crate::RegisterSpec for CQISGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqisge::R](R) reader structure"]
impl crate::Readable for CQISGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqisge::W](W) writer structure"]
impl crate::Writable for CQISGE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CQISGE to value 0"]
impl crate::Resettable for CQISGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
