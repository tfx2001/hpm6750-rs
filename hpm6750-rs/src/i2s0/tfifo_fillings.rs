#[doc = "Register `TFIFO_FILLINGS` reader"]
pub struct R(crate::R<TFIFO_FILLINGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFIFO_FILLINGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFIFO_FILLINGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFIFO_FILLINGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX0` reader - TX0 fifo fillings"]
pub type TX0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX1` reader - TX1 fifo fillings"]
pub type TX1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX2` reader - TX2 fifo fillings"]
pub type TX2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX3` reader - TX3 fifo fillings"]
pub type TX3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - TX0 fifo fillings"]
    #[inline(always)]
    pub fn tx0(&self) -> TX0_R {
        TX0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TX1 fifo fillings"]
    #[inline(always)]
    pub fn tx1(&self) -> TX1_R {
        TX1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TX2 fifo fillings"]
    #[inline(always)]
    pub fn tx2(&self) -> TX2_R {
        TX2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TX3 fifo fillings"]
    #[inline(always)]
    pub fn tx3(&self) -> TX3_R {
        TX3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Tx FIFO Filling Level\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfifo_fillings](index.html) module"]
pub struct TFIFO_FILLINGS_SPEC;
impl crate::RegisterSpec for TFIFO_FILLINGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tfifo_fillings::R](R) reader structure"]
impl crate::Readable for TFIFO_FILLINGS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TFIFO_FILLINGS to value 0"]
impl crate::Resettable for TFIFO_FILLINGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
