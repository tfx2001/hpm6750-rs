#[doc = "Register `BUS_RESULT_CHN15` reader"]
pub struct R(crate::R<BUS_RESULT_CHN15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_RESULT_CHN15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_RESULT_CHN15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_RESULT_CHN15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHAN_RESULT` reader - read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
pub type CHAN_RESULT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VALID` reader - set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
pub type VALID_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    #[inline(always)]
    pub fn chan_result(&self) -> CHAN_RESULT_R {
        CHAN_RESULT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_result_chn15](index.html) module"]
pub struct BUS_RESULT_CHN15_SPEC;
impl crate::RegisterSpec for BUS_RESULT_CHN15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_result_chn15::R](R) reader structure"]
impl crate::Readable for BUS_RESULT_CHN15_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUS_RESULT_CHN15 to value 0"]
impl crate::Resettable for BUS_RESULT_CHN15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
