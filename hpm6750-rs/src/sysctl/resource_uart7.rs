#[doc = "Register `RESOURCE_UART7` reader"]
pub struct R(crate::R<RESOURCE_UART7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESOURCE_UART7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESOURCE_UART7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESOURCE_UART7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESOURCE_UART7` writer"]
pub struct W(crate::W<RESOURCE_UART7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESOURCE_UART7_SPEC>;
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
impl From<crate::W<RESOURCE_UART7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESOURCE_UART7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESOURCE_UART7_SPEC, u8, u8, 2, O>;
#[doc = "Field `LOC_BUSY` reader - local busy 0: no change is pending for current node 1: current node is changing status"]
pub type LOC_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `GLB_BUSY` reader - global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
pub type GLB_BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 30 - local busy 0: no change is pending for current node 1: current node is changing status"]
    #[inline(always)]
    pub fn loc_busy(&self) -> LOC_BUSY_R {
        LOC_BUSY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    #[inline(always)]
    pub fn glb_busy(&self) -> GLB_BUSY_R {
        GLB_BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Resource control register for urt7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resource_uart7](index.html) module"]
pub struct RESOURCE_UART7_SPEC;
impl crate::RegisterSpec for RESOURCE_UART7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resource_uart7::R](R) reader structure"]
impl crate::Readable for RESOURCE_UART7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resource_uart7::W](W) writer structure"]
impl crate::Writable for RESOURCE_UART7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESOURCE_UART7 to value 0"]
impl crate::Resettable for RESOURCE_UART7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
