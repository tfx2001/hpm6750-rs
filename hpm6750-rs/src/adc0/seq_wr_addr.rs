#[doc = "Register `SEQ_WR_ADDR` reader"]
pub struct R(crate::R<SEQ_WR_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ_WR_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ_WR_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ_WR_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEQ_WR_POINTER` reader - HW update this field after each dma write, it indicate the next dma write pointer. dma write address is (tar_addr+seq_wr_pointer)*4"]
pub type SEQ_WR_POINTER_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - HW update this field after each dma write, it indicate the next dma write pointer. dma write address is (tar_addr+seq_wr_pointer)*4"]
    #[inline(always)]
    pub fn seq_wr_pointer(&self) -> SEQ_WR_POINTER_R {
        SEQ_WR_POINTER_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_wr_addr](index.html) module"]
pub struct SEQ_WR_ADDR_SPEC;
impl crate::RegisterSpec for SEQ_WR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq_wr_addr::R](R) reader structure"]
impl crate::Readable for SEQ_WR_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SEQ_WR_ADDR to value 0"]
impl crate::Resettable for SEQ_WR_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
