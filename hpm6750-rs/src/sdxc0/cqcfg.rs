#[doc = "Register `CQCFG` reader"]
pub struct R(crate::R<CQCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CQCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CQCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CQCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CQCFG` writer"]
pub struct W(crate::W<CQCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CQCFG_SPEC>;
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
impl From<crate::W<CQCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CQCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CQ_EN` reader - No description avaiable"]
pub type CQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `CQ_EN` writer - No description avaiable"]
pub type CQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQCFG_SPEC, bool, O>;
#[doc = "Field `TASK_DESC_SIZE` reader - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits 0x0 (TASK_DESC_64b): Task descriptor size is 64 bit"]
pub type TASK_DESC_SIZE_R = crate::BitReader<bool>;
#[doc = "Field `TASK_DESC_SIZE` writer - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits 0x0 (TASK_DESC_64b): Task descriptor size is 64 bit"]
pub type TASK_DESC_SIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQCFG_SPEC, bool, O>;
#[doc = "Field `DCMD_EN` reader - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
pub type DCMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `DCMD_EN` writer - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
pub type DCMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CQCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn cq_en(&self) -> CQ_EN_R {
        CQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits 0x0 (TASK_DESC_64b): Task descriptor size is 64 bit"]
    #[inline(always)]
    pub fn task_desc_size(&self) -> TASK_DESC_SIZE_R {
        TASK_DESC_SIZE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
    #[inline(always)]
    pub fn dcmd_en(&self) -> DCMD_EN_R {
        DCMD_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn cq_en(&mut self) -> CQ_EN_W<0> {
        CQ_EN_W::new(self)
    }
    #[doc = "Bit 8 - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits 0x0 (TASK_DESC_64b): Task descriptor size is 64 bit"]
    #[inline(always)]
    #[must_use]
    pub fn task_desc_size(&mut self) -> TASK_DESC_SIZE_W<8> {
        TASK_DESC_SIZE_W::new(self)
    }
    #[doc = "Bit 12 - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
    #[inline(always)]
    #[must_use]
    pub fn dcmd_en(&mut self) -> DCMD_EN_W<12> {
        DCMD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cqcfg](index.html) module"]
pub struct CQCFG_SPEC;
impl crate::RegisterSpec for CQCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cqcfg::R](R) reader structure"]
impl crate::Readable for CQCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cqcfg::W](W) writer structure"]
impl crate::Writable for CQCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CQCFG to value 0"]
impl crate::Resettable for CQCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
