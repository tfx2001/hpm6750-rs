#[doc = "Register `CQIS` reader"]
pub struct R(crate::R<CQIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQIS` writer"]
pub struct W(crate::W<CQIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQIS_SPEC>;
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
impl From<crate::W<CQIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCL` reader - Task cleared interrupt This status bit is asserted (if CQISE.TCL_STE=1) when a task clear operation is completed by CQE. The completed task clear operation is either an individual task clear (by writing CQTCLR) or clearing of all tasks (by writing CQCTL). A value of 1 clears this status bit. Values: 0x1 (SET): TCL Interrupt is set 0x0 (NOTSET): TCL Interrupt is not set"]
pub type TCL_R = crate::BitReader<bool>;
#[doc = "Field `TCL` writer - Task cleared interrupt This status bit is asserted (if CQISE.TCL_STE=1) when a task clear operation is completed by CQE. The completed task clear operation is either an individual task clear (by writing CQTCLR) or clearing of all tasks (by writing CQCTL). A value of 1 clears this status bit. Values: 0x1 (SET): TCL Interrupt is set 0x0 (NOTSET): TCL Interrupt is not set"]
pub type TCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQIS_SPEC, bool, O>;
#[doc = "Field `RED` reader - Response error detected interrupt This status bit is asserted (if CQISE.RED_STE=1) when a response is received with an error bit set in the device status field. Configure the CQRMEM register to identify device status bit fields that may trigger an interrupt and that are masked. A value of 1 clears this status bit. Values: 0x1 (SET): RED Interrupt is set 0x0 (NOTSET): RED Interrupt is not set"]
pub type RED_R = crate::BitReader<bool>;
#[doc = "Field `RED` writer - Response error detected interrupt This status bit is asserted (if CQISE.RED_STE=1) when a response is received with an error bit set in the device status field. Configure the CQRMEM register to identify device status bit fields that may trigger an interrupt and that are masked. A value of 1 clears this status bit. Values: 0x1 (SET): RED Interrupt is set 0x0 (NOTSET): RED Interrupt is not set"]
pub type RED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQIS_SPEC, bool, O>;
#[doc = "Field `TCC` reader - Task complete interrupt This status bit is asserted (if CQISE.TCC_STE=1) when at least one of the following conditions are met: A task is completed and the INT bit is set in its Task Descriptor Interrupt caused by Interrupt Coalescing logic due to timeout Interrupt Coalescing logic reached the configured threshold A value of 1 clears this status bit"]
pub type TCC_R = crate::BitReader<bool>;
#[doc = "Field `TCC` writer - Task complete interrupt This status bit is asserted (if CQISE.TCC_STE=1) when at least one of the following conditions are met: A task is completed and the INT bit is set in its Task Descriptor Interrupt caused by Interrupt Coalescing logic due to timeout Interrupt Coalescing logic reached the configured threshold A value of 1 clears this status bit"]
pub type TCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQIS_SPEC, bool, O>;
#[doc = "Field `HAC` reader - Halt complete interrupt This status bit is asserted (only if CQISE.HAC_STE=1) when halt bit in the CQCTL register transitions from 0 to 1 indicating that the host controller has completed its current ongoing task and has entered halt state. A value of 1 clears this status bit. Values: 0x1 (SET): HAC Interrupt is set 0x0 (NOTSET): HAC Interrupt is not set"]
pub type HAC_R = crate::BitReader<bool>;
#[doc = "Field `HAC` writer - Halt complete interrupt This status bit is asserted (only if CQISE.HAC_STE=1) when halt bit in the CQCTL register transitions from 0 to 1 indicating that the host controller has completed its current ongoing task and has entered halt state. A value of 1 clears this status bit. Values: 0x1 (SET): HAC Interrupt is set 0x0 (NOTSET): HAC Interrupt is not set"]
pub type HAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQIS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - Task cleared interrupt This status bit is asserted (if CQISE.TCL_STE=1) when a task clear operation is completed by CQE. The completed task clear operation is either an individual task clear (by writing CQTCLR) or clearing of all tasks (by writing CQCTL). A value of 1 clears this status bit. Values: 0x1 (SET): TCL Interrupt is set 0x0 (NOTSET): TCL Interrupt is not set"]
    #[inline(always)]
    pub fn tcl(&self) -> TCL_R {
        TCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Response error detected interrupt This status bit is asserted (if CQISE.RED_STE=1) when a response is received with an error bit set in the device status field. Configure the CQRMEM register to identify device status bit fields that may trigger an interrupt and that are masked. A value of 1 clears this status bit. Values: 0x1 (SET): RED Interrupt is set 0x0 (NOTSET): RED Interrupt is not set"]
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Task complete interrupt This status bit is asserted (if CQISE.TCC_STE=1) when at least one of the following conditions are met: A task is completed and the INT bit is set in its Task Descriptor Interrupt caused by Interrupt Coalescing logic due to timeout Interrupt Coalescing logic reached the configured threshold A value of 1 clears this status bit"]
    #[inline(always)]
    pub fn tcc(&self) -> TCC_R {
        TCC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Halt complete interrupt This status bit is asserted (only if CQISE.HAC_STE=1) when halt bit in the CQCTL register transitions from 0 to 1 indicating that the host controller has completed its current ongoing task and has entered halt state. A value of 1 clears this status bit. Values: 0x1 (SET): HAC Interrupt is set 0x0 (NOTSET): HAC Interrupt is not set"]
    #[inline(always)]
    pub fn hac(&self) -> HAC_R {
        HAC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Task cleared interrupt This status bit is asserted (if CQISE.TCL_STE=1) when a task clear operation is completed by CQE. The completed task clear operation is either an individual task clear (by writing CQTCLR) or clearing of all tasks (by writing CQCTL). A value of 1 clears this status bit. Values: 0x1 (SET): TCL Interrupt is set 0x0 (NOTSET): TCL Interrupt is not set"]
    #[inline(always)]
    pub fn tcl(&mut self) -> TCL_W<3> {
        TCL_W::new(self)
    }
    #[doc = "Bit 2 - Response error detected interrupt This status bit is asserted (if CQISE.RED_STE=1) when a response is received with an error bit set in the device status field. Configure the CQRMEM register to identify device status bit fields that may trigger an interrupt and that are masked. A value of 1 clears this status bit. Values: 0x1 (SET): RED Interrupt is set 0x0 (NOTSET): RED Interrupt is not set"]
    #[inline(always)]
    pub fn red(&mut self) -> RED_W<2> {
        RED_W::new(self)
    }
    #[doc = "Bit 1 - Task complete interrupt This status bit is asserted (if CQISE.TCC_STE=1) when at least one of the following conditions are met: A task is completed and the INT bit is set in its Task Descriptor Interrupt caused by Interrupt Coalescing logic due to timeout Interrupt Coalescing logic reached the configured threshold A value of 1 clears this status bit"]
    #[inline(always)]
    pub fn tcc(&mut self) -> TCC_W<1> {
        TCC_W::new(self)
    }
    #[doc = "Bit 0 - Halt complete interrupt This status bit is asserted (only if CQISE.HAC_STE=1) when halt bit in the CQCTL register transitions from 0 to 1 indicating that the host controller has completed its current ongoing task and has entered halt state. A value of 1 clears this status bit. Values: 0x1 (SET): HAC Interrupt is set 0x0 (NOTSET): HAC Interrupt is not set"]
    #[inline(always)]
    pub fn hac(&mut self) -> HAC_W<0> {
        HAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqis](index.html) module"]
pub struct CQIS_SPEC;
impl crate::RegisterSpec for CQIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqis::R](R) reader structure"]
impl crate::Readable for CQIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqis::W](W) writer structure"]
impl crate::Writable for CQIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CQIS to value 0"]
impl crate::Resettable for CQIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
