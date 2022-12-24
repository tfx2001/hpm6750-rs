#[doc = "Register `CQCTL` reader"]
pub struct R(crate::R<CQCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQCTL` writer"]
pub struct W(crate::W<CQCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQCTL_SPEC>;
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
impl From<crate::W<CQCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HALT` reader - Halt request and resume Values: 0x1 (HALT_CQE): Software writes 1 to this bit when it wants to acquire software control over the eMMC bus and to disable CQE from issuing command on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When the software writes 1, CQE completes the ongoing task (if any in progress). After the task is completed and the CQE is in idle state, CQE does not issue new commands and indicates to the software by setting this bit to 1. The software can poll on this bit until it is set to 1 and only then send commands on the eMMC bus. 0x0 (RESUME_CQE): Software writes 0 to this bit to exit from the halt state and resume CQE activity"]
pub type HALT_R = crate::BitReader<bool>;
#[doc = "Field `HALT` writer - Halt request and resume Values: 0x1 (HALT_CQE): Software writes 1 to this bit when it wants to acquire software control over the eMMC bus and to disable CQE from issuing command on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When the software writes 1, CQE completes the ongoing task (if any in progress). After the task is completed and the CQE is in idle state, CQE does not issue new commands and indicates to the software by setting this bit to 1. The software can poll on this bit until it is set to 1 and only then send commands on the eMMC bus. 0x0 (RESUME_CQE): Software writes 0 to this bit to exit from the halt state and resume CQE activity"]
pub type HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQCTL_SPEC, bool, O>;
#[doc = "Field `CLR_ALL_TASKS` reader - Clear all tasks This bit can only be written when the controller is halted. This bit does not clear tasks in the device. The software has to use the CMDQ_TASK_MGMT command to clear device's queue. Values: 0x1 (CLEAR_ALL_TASKS): Clears all the tasks in the controller 0x0 (NO_EFFECT): Programming 0 has no effect"]
pub type CLR_ALL_TASKS_R = crate::BitReader<bool>;
#[doc = "Field `CLR_ALL_TASKS` writer - Clear all tasks This bit can only be written when the controller is halted. This bit does not clear tasks in the device. The software has to use the CMDQ_TASK_MGMT command to clear device's queue. Values: 0x1 (CLEAR_ALL_TASKS): Clears all the tasks in the controller 0x0 (NO_EFFECT): Programming 0 has no effect"]
pub type CLR_ALL_TASKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Halt request and resume Values: 0x1 (HALT_CQE): Software writes 1 to this bit when it wants to acquire software control over the eMMC bus and to disable CQE from issuing command on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When the software writes 1, CQE completes the ongoing task (if any in progress). After the task is completed and the CQE is in idle state, CQE does not issue new commands and indicates to the software by setting this bit to 1. The software can poll on this bit until it is set to 1 and only then send commands on the eMMC bus. 0x0 (RESUME_CQE): Software writes 0 to this bit to exit from the halt state and resume CQE activity"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Clear all tasks This bit can only be written when the controller is halted. This bit does not clear tasks in the device. The software has to use the CMDQ_TASK_MGMT command to clear device's queue. Values: 0x1 (CLEAR_ALL_TASKS): Clears all the tasks in the controller 0x0 (NO_EFFECT): Programming 0 has no effect"]
    #[inline(always)]
    pub fn clr_all_tasks(&self) -> CLR_ALL_TASKS_R {
        CLR_ALL_TASKS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt request and resume Values: 0x1 (HALT_CQE): Software writes 1 to this bit when it wants to acquire software control over the eMMC bus and to disable CQE from issuing command on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When the software writes 1, CQE completes the ongoing task (if any in progress). After the task is completed and the CQE is in idle state, CQE does not issue new commands and indicates to the software by setting this bit to 1. The software can poll on this bit until it is set to 1 and only then send commands on the eMMC bus. 0x0 (RESUME_CQE): Software writes 0 to this bit to exit from the halt state and resume CQE activity"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<0> {
        HALT_W::new(self)
    }
    #[doc = "Bit 8 - Clear all tasks This bit can only be written when the controller is halted. This bit does not clear tasks in the device. The software has to use the CMDQ_TASK_MGMT command to clear device's queue. Values: 0x1 (CLEAR_ALL_TASKS): Clears all the tasks in the controller 0x0 (NO_EFFECT): Programming 0 has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn clr_all_tasks(&mut self) -> CLR_ALL_TASKS_W<8> {
        CLR_ALL_TASKS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqctl](index.html) module"]
pub struct CQCTL_SPEC;
impl crate::RegisterSpec for CQCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqctl::R](R) reader structure"]
impl crate::Readable for CQCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqctl::W](W) writer structure"]
impl crate::Writable for CQCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CQCTL to value 0"]
impl crate::Resettable for CQCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
