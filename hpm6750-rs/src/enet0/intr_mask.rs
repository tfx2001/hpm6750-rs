#[doc = "Register `INTR_MASK` reader"]
pub struct R(crate::R<INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_MASK` writer"]
pub struct W(crate::W<INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_MASK_SPEC>;
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
impl From<crate::W<INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPIIM` reader - LPI Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register 14 (Interrupt Status Register)."]
pub type LPIIM_R = crate::BitReader<bool>;
#[doc = "Field `LPIIM` writer - LPI Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register 14 (Interrupt Status Register)."]
pub type LPIIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `TSIM` reader - Timestamp Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of Timestamp Interrupt Status bit in Register 14 (Interrupt Status Register)."]
pub type TSIM_R = crate::BitReader<bool>;
#[doc = "Field `TSIM` writer - Timestamp Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of Timestamp Interrupt Status bit in Register 14 (Interrupt Status Register)."]
pub type TSIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `PMTIM` reader - PMT Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register 14 (Interrupt Status Register)."]
pub type PMTIM_R = crate::BitReader<bool>;
#[doc = "Field `PMTIM` writer - PMT Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register 14 (Interrupt Status Register)."]
pub type PMTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `PCSANCIM` reader - PCS AN Completion Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PCS Auto-negotiation complete bit in Register 14 (Interrupt Status Register)."]
pub type PCSANCIM_R = crate::BitReader<bool>;
#[doc = "Field `PCSANCIM` writer - PCS AN Completion Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PCS Auto-negotiation complete bit in Register 14 (Interrupt Status Register)."]
pub type PCSANCIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `PCSLCHGIM` reader - PCS Link Status Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the PCS Link-status changed bit in Register 14 (Interrupt Status Register)."]
pub type PCSLCHGIM_R = crate::BitReader<bool>;
#[doc = "Field `PCSLCHGIM` writer - PCS Link Status Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the PCS Link-status changed bit in Register 14 (Interrupt Status Register)."]
pub type PCSLCHGIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
#[doc = "Field `RGSMIIIM` reader - RGMII or SMII Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the RGMII or SMII Interrupt Status bit in Register 14 (Interrupt Status Register)."]
pub type RGSMIIIM_R = crate::BitReader<bool>;
#[doc = "Field `RGSMIIIM` writer - RGMII or SMII Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the RGMII or SMII Interrupt Status bit in Register 14 (Interrupt Status Register)."]
pub type RGSMIIIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 10 - LPI Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register 14 (Interrupt Status Register)."]
    #[inline(always)]
    pub fn lpiim(&self) -> LPIIM_R {
        LPIIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of Timestamp Interrupt Status bit in Register 14 (Interrupt Status Register)."]
    #[inline(always)]
    pub fn tsim(&self) -> TSIM_R {
        TSIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 3 - PMT Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register 14 (Interrupt Status Register)."]
    #[inline(always)]
    pub fn pmtim(&self) -> PMTIM_R {
        PMTIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - PCS AN Completion Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PCS Auto-negotiation complete bit in Register 14 (Interrupt Status Register)."]
    #[inline(always)]
    pub fn pcsancim(&self) -> PCSANCIM_R {
        PCSANCIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - PCS Link Status Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the PCS Link-status changed bit in Register 14 (Interrupt Status Register)."]
    #[inline(always)]
    pub fn pcslchgim(&self) -> PCSLCHGIM_R {
        PCSLCHGIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - RGMII or SMII Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the RGMII or SMII Interrupt Status bit in Register 14 (Interrupt Status Register)."]
    #[inline(always)]
    pub fn rgsmiiim(&self) -> RGSMIIIM_R {
        RGSMIIIM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - LPI Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the LPI Interrupt Status bit in Register 14 (Interrupt Status Register)."]
    #[inline(always)]
    pub fn lpiim(&mut self) -> LPIIM_W<10> {
        LPIIM_W::new(self)
    }
    #[doc = "Bit 9 - Timestamp Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of Timestamp Interrupt Status bit in Register 14 (Interrupt Status Register)."]
    #[inline(always)]
    pub fn tsim(&mut self) -> TSIM_W<9> {
        TSIM_W::new(self)
    }
    #[doc = "Bit 3 - PMT Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PMT Interrupt Status bit in Register 14 (Interrupt Status Register)."]
    #[inline(always)]
    pub fn pmtim(&mut self) -> PMTIM_W<3> {
        PMTIM_W::new(self)
    }
    #[doc = "Bit 2 - PCS AN Completion Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of PCS Auto-negotiation complete bit in Register 14 (Interrupt Status Register)."]
    #[inline(always)]
    pub fn pcsancim(&mut self) -> PCSANCIM_W<2> {
        PCSANCIM_W::new(self)
    }
    #[doc = "Bit 1 - PCS Link Status Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the PCS Link-status changed bit in Register 14 (Interrupt Status Register)."]
    #[inline(always)]
    pub fn pcslchgim(&mut self) -> PCSLCHGIM_W<1> {
        PCSLCHGIM_W::new(self)
    }
    #[doc = "Bit 0 - RGMII or SMII Interrupt Mask When set, this bit disables the assertion of the interrupt signal because of the setting of the RGMII or SMII Interrupt Status bit in Register 14 (Interrupt Status Register)."]
    #[inline(always)]
    pub fn rgsmiiim(&mut self) -> RGSMIIIM_W<0> {
        RGSMIIIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_mask](index.html) module"]
pub struct INTR_MASK_SPEC;
impl crate::RegisterSpec for INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_mask::R](R) reader structure"]
impl crate::Readable for INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](W) writer structure"]
impl crate::Writable for INTR_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for INTR_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
