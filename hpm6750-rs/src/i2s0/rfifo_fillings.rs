#[doc = "Register `RFIFO_FILLINGS` reader"]
pub struct R(crate::R<RFIFO_FILLINGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIFO_FILLINGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIFO_FILLINGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIFO_FILLINGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX0` reader - RX0 fifo fillings"]
pub type RX0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX1` reader - RX1 fifo fillings"]
pub type RX1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX2` reader - RX2 fifo fillings"]
pub type RX2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX3` reader - RX3 fifo fillings"]
pub type RX3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX0 fifo fillings"]
    #[inline(always)]
    pub fn rx0(&self) -> RX0_R {
        RX0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX1 fifo fillings"]
    #[inline(always)]
    pub fn rx1(&self) -> RX1_R {
        RX1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RX2 fifo fillings"]
    #[inline(always)]
    pub fn rx2(&self) -> RX2_R {
        RX2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RX3 fifo fillings"]
    #[inline(always)]
    pub fn rx3(&self) -> RX3_R {
        RX3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Rx FIFO Filling Level\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifo_fillings](index.html) module"]
pub struct RFIFO_FILLINGS_SPEC;
impl crate::RegisterSpec for RFIFO_FILLINGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfifo_fillings::R](R) reader structure"]
impl crate::Readable for RFIFO_FILLINGS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RFIFO_FILLINGS to value 0"]
impl crate::Resettable for RFIFO_FILLINGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
