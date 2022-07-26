#[doc = "Register `IIR` reader"]
pub struct R(crate::R<IIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFOED` reader - FIFOs enabled These two bits are 1 when bit 0 of the FIFO Control Register (FIFOE) is set to 1."]
pub type FIFOED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTRID` reader - Interrupt ID"]
pub type INTRID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 6:7 - FIFOs enabled These two bits are 1 when bit 0 of the FIFO Control Register (FIFOE) is set to 1."]
    #[inline(always)]
    pub fn fifoed(&self) -> FIFOED_R {
        FIFOED_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 0:3 - Interrupt ID"]
    #[inline(always)]
    pub fn intrid(&self) -> INTRID_R {
        INTRID_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Interrupt Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iir](index.html) module"]
pub struct IIR_SPEC;
impl crate::RegisterSpec for IIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iir::R](R) reader structure"]
impl crate::Readable for IIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IIR to value 0x01"]
impl crate::Resettable for IIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
