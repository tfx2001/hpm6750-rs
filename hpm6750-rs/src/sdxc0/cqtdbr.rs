#[doc = "Register `CQTDBR` reader"]
pub struct R(crate::R<CQTDBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQTDBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQTDBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQTDBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQTDBR` writer"]
pub struct W(crate::W<CQTDBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQTDBR_SPEC>;
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
impl From<crate::W<CQTDBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQTDBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBR` reader - The software configures TDLBA and TDLBAU, and enable CQE in CQCFG before using this register. Writing 1 to bit n of this register triggers CQE to start processing the task encoded in slot n of the TDL. Writing 0 by the software does not have any impact on the hardware, and does not change the value of the register bit. CQE always processes tasks according to the order submitted to the list by CQTDBR write transactions. CQE processes Data Transfer tasks by reading the Task Descriptor and sending QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) commands to the device. CQE processes DCMD tasks (in slot #31, when enabled) by reading the Task Descriptor, and generating the command encoded by its index and argument. The corresponding bit is cleared to 0 by CQE in one of the following events: A task execution is completed (with success or error). The task is cleared using CQTCLR register. All tasks are cleared using CQCTL register. CQE is disabled using CQCFG register. Software may initiate multiple tasks at the same time (batch submission) by writing 1 to multiple bits of this register in the same transaction. In the case of batch submission, CQE processes the tasks in order of the task index, starting with the lowest index. If one or more tasks in the batch are marked with QBR, the ordering of execution is based on said processing order."]
pub type DBR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DBR` writer - The software configures TDLBA and TDLBAU, and enable CQE in CQCFG before using this register. Writing 1 to bit n of this register triggers CQE to start processing the task encoded in slot n of the TDL. Writing 0 by the software does not have any impact on the hardware, and does not change the value of the register bit. CQE always processes tasks according to the order submitted to the list by CQTDBR write transactions. CQE processes Data Transfer tasks by reading the Task Descriptor and sending QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) commands to the device. CQE processes DCMD tasks (in slot #31, when enabled) by reading the Task Descriptor, and generating the command encoded by its index and argument. The corresponding bit is cleared to 0 by CQE in one of the following events: A task execution is completed (with success or error). The task is cleared using CQTCLR register. All tasks are cleared using CQCTL register. CQE is disabled using CQCFG register. Software may initiate multiple tasks at the same time (batch submission) by writing 1 to multiple bits of this register in the same transaction. In the case of batch submission, CQE processes the tasks in order of the task index, starting with the lowest index. If one or more tasks in the batch are marked with QBR, the ordering of execution is based on said processing order."]
pub type DBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CQTDBR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The software configures TDLBA and TDLBAU, and enable CQE in CQCFG before using this register. Writing 1 to bit n of this register triggers CQE to start processing the task encoded in slot n of the TDL. Writing 0 by the software does not have any impact on the hardware, and does not change the value of the register bit. CQE always processes tasks according to the order submitted to the list by CQTDBR write transactions. CQE processes Data Transfer tasks by reading the Task Descriptor and sending QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) commands to the device. CQE processes DCMD tasks (in slot #31, when enabled) by reading the Task Descriptor, and generating the command encoded by its index and argument. The corresponding bit is cleared to 0 by CQE in one of the following events: A task execution is completed (with success or error). The task is cleared using CQTCLR register. All tasks are cleared using CQCTL register. CQE is disabled using CQCFG register. Software may initiate multiple tasks at the same time (batch submission) by writing 1 to multiple bits of this register in the same transaction. In the case of batch submission, CQE processes the tasks in order of the task index, starting with the lowest index. If one or more tasks in the batch are marked with QBR, the ordering of execution is based on said processing order."]
    #[inline(always)]
    pub fn dbr(&self) -> DBR_R {
        DBR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The software configures TDLBA and TDLBAU, and enable CQE in CQCFG before using this register. Writing 1 to bit n of this register triggers CQE to start processing the task encoded in slot n of the TDL. Writing 0 by the software does not have any impact on the hardware, and does not change the value of the register bit. CQE always processes tasks according to the order submitted to the list by CQTDBR write transactions. CQE processes Data Transfer tasks by reading the Task Descriptor and sending QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) commands to the device. CQE processes DCMD tasks (in slot #31, when enabled) by reading the Task Descriptor, and generating the command encoded by its index and argument. The corresponding bit is cleared to 0 by CQE in one of the following events: A task execution is completed (with success or error). The task is cleared using CQTCLR register. All tasks are cleared using CQCTL register. CQE is disabled using CQCFG register. Software may initiate multiple tasks at the same time (batch submission) by writing 1 to multiple bits of this register in the same transaction. In the case of batch submission, CQE processes the tasks in order of the task index, starting with the lowest index. If one or more tasks in the batch are marked with QBR, the ordering of execution is based on said processing order."]
    #[inline(always)]
    pub fn dbr(&mut self) -> DBR_W<0> {
        DBR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqtdbr](index.html) module"]
pub struct CQTDBR_SPEC;
impl crate::RegisterSpec for CQTDBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqtdbr::R](R) reader structure"]
impl crate::Readable for CQTDBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqtdbr::W](W) writer structure"]
impl crate::Writable for CQTDBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CQTDBR to value 0"]
impl crate::Resettable for CQTDBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
