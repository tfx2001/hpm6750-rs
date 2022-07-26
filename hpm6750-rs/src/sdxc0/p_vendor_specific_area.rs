#[doc = "Register `P_VENDOR_SPECIFIC_AREA` reader"]
pub struct R(crate::R<P_VENDOR_SPECIFIC_AREA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P_VENDOR_SPECIFIC_AREA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P_VENDOR_SPECIFIC_AREA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P_VENDOR_SPECIFIC_AREA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REG_OFFSET_ADDR` reader - Base offset Address for Vendor-Specific registers."]
pub type REG_OFFSET_ADDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Base offset Address for Vendor-Specific registers."]
    #[inline(always)]
    pub fn reg_offset_addr(&self) -> REG_OFFSET_ADDR_R {
        REG_OFFSET_ADDR_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p_vendor_specific_area](index.html) module"]
pub struct P_VENDOR_SPECIFIC_AREA_SPEC;
impl crate::RegisterSpec for P_VENDOR_SPECIFIC_AREA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p_vendor_specific_area::R](R) reader structure"]
impl crate::Readable for P_VENDOR_SPECIFIC_AREA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets P_VENDOR_SPECIFIC_AREA to value 0"]
impl crate::Resettable for P_VENDOR_SPECIFIC_AREA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
