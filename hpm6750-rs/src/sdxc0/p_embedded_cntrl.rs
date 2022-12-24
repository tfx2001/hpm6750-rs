#[doc = "Register `P_EMBEDDED_CNTRL` reader"]
pub struct R(crate::R<P_EMBEDDED_CNTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P_EMBEDDED_CNTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P_EMBEDDED_CNTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P_EMBEDDED_CNTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REG_OFFSET_ADDR` reader - Offset Address of Embedded Control register."]
pub type REG_OFFSET_ADDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Offset Address of Embedded Control register."]
    #[inline(always)]
    pub fn reg_offset_addr(&self) -> REG_OFFSET_ADDR_R {
        REG_OFFSET_ADDR_R::new(self.bits & 0x0fff)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p_embedded_cntrl](index.html) module"]
pub struct P_EMBEDDED_CNTRL_SPEC;
impl crate::RegisterSpec for P_EMBEDDED_CNTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p_embedded_cntrl::R](R) reader structure"]
impl crate::Readable for P_EMBEDDED_CNTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets P_EMBEDDED_CNTRL to value 0"]
impl crate::Resettable for P_EMBEDDED_CNTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
