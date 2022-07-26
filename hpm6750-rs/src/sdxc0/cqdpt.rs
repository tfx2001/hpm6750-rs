#[doc = "Register `CQDPT` reader"]
pub struct R(crate::R<CQDPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQDPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQDPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQDPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQDPT` writer"]
pub struct W(crate::W<CQDPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQDPT_SPEC>;
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
impl From<crate::W<CQDPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQDPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPT` reader - Device-Pending Tasks Each of the 32 bits are bit mapped to the 32 tasks. Bit-N(1): Task-N has been successfully queued into the device and is awaiting execution Bit-N(0): Task-N is not yet queued. Bit n of this register is set if and only if QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) were sent for this specific task and if this task has not been executed. The controller sets this bit after receiving a successful response for CMD45. CQE clears this bit after the task has completed execution. Software reads this register in the task-discard procedure to determine if the task is queued in the device"]
pub type DPT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DPT` writer - Device-Pending Tasks Each of the 32 bits are bit mapped to the 32 tasks. Bit-N(1): Task-N has been successfully queued into the device and is awaiting execution Bit-N(0): Task-N is not yet queued. Bit n of this register is set if and only if QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) were sent for this specific task and if this task has not been executed. The controller sets this bit after receiving a successful response for CMD45. CQE clears this bit after the task has completed execution. Software reads this register in the task-discard procedure to determine if the task is queued in the device"]
pub type DPT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CQDPT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Device-Pending Tasks Each of the 32 bits are bit mapped to the 32 tasks. Bit-N(1): Task-N has been successfully queued into the device and is awaiting execution Bit-N(0): Task-N is not yet queued. Bit n of this register is set if and only if QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) were sent for this specific task and if this task has not been executed. The controller sets this bit after receiving a successful response for CMD45. CQE clears this bit after the task has completed execution. Software reads this register in the task-discard procedure to determine if the task is queued in the device"]
    #[inline(always)]
    pub fn dpt(&self) -> DPT_R {
        DPT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Device-Pending Tasks Each of the 32 bits are bit mapped to the 32 tasks. Bit-N(1): Task-N has been successfully queued into the device and is awaiting execution Bit-N(0): Task-N is not yet queued. Bit n of this register is set if and only if QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) were sent for this specific task and if this task has not been executed. The controller sets this bit after receiving a successful response for CMD45. CQE clears this bit after the task has completed execution. Software reads this register in the task-discard procedure to determine if the task is queued in the device"]
    #[inline(always)]
    pub fn dpt(&mut self) -> DPT_W<0> {
        DPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqdpt](index.html) module"]
pub struct CQDPT_SPEC;
impl crate::RegisterSpec for CQDPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqdpt::R](R) reader structure"]
impl crate::Readable for CQDPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqdpt::W](W) writer structure"]
impl crate::Writable for CQDPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CQDPT to value 0"]
impl crate::Resettable for CQDPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
