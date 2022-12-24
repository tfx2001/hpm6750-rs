#[doc = "Register `ADMA_ERR_STAT` reader"]
pub struct R(crate::R<ADMA_ERR_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADMA_ERR_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADMA_ERR_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADMA_ERR_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADMA_ERR_STATES` reader - ADMA Error States These bits indicate the state of ADMA when an error occurs during ADMA data transfer. Values: 0x0 (ST_STOP): Stop DMA - SYS_ADR register points to a location next to the error descriptor 0x1 (ST_FDS): Fetch Descriptor - SYS_ADR register points to the error descriptor 0x2 (UNUSED): Never set this state 0x3 (ST_TFR): Transfer Data - SYS_ADR register points to a location next to the error descriptor"]
pub type ADMA_ERR_STATES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADMA_LEN_ERR` reader - ADMA Length Mismatch Error States This error occurs in the following instances: While the Block Count Enable is being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length When the total data length cannot be divided by the block length Values: 0x0 (NO_ERR): No Error 0x1 (ERROR): Error"]
pub type ADMA_LEN_ERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - ADMA Error States These bits indicate the state of ADMA when an error occurs during ADMA data transfer. Values: 0x0 (ST_STOP): Stop DMA - SYS_ADR register points to a location next to the error descriptor 0x1 (ST_FDS): Fetch Descriptor - SYS_ADR register points to the error descriptor 0x2 (UNUSED): Never set this state 0x3 (ST_TFR): Transfer Data - SYS_ADR register points to a location next to the error descriptor"]
    #[inline(always)]
    pub fn adma_err_states(&self) -> ADMA_ERR_STATES_R {
        ADMA_ERR_STATES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error States This error occurs in the following instances: While the Block Count Enable is being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length When the total data length cannot be divided by the block length Values: 0x0 (NO_ERR): No Error 0x1 (ERROR): Error"]
    #[inline(always)]
    pub fn adma_len_err(&self) -> ADMA_LEN_ERR_R {
        ADMA_LEN_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adma_err_stat](index.html) module"]
pub struct ADMA_ERR_STAT_SPEC;
impl crate::RegisterSpec for ADMA_ERR_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adma_err_stat::R](R) reader structure"]
impl crate::Readable for ADMA_ERR_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADMA_ERR_STAT to value 0"]
impl crate::Resettable for ADMA_ERR_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
