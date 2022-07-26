#[doc = "Register `RESOURCE_MOT0` reader"]
pub struct R(crate::R<RESOURCE_MOT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESOURCE_MOT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESOURCE_MOT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESOURCE_MOT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESOURCE_MOT0` writer"]
pub struct W(crate::W<RESOURCE_MOT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESOURCE_MOT0_SPEC>;
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
impl From<crate::W<RESOURCE_MOT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESOURCE_MOT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GLB_BUSY` reader - global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
pub type GLB_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `LOC_BUSY` reader - local busy 0: no change is pending for current node 1: current node is changing status"]
pub type LOC_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `MODE` reader - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESOURCE_MOT0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 31 - global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    #[inline(always)]
    pub fn glb_busy(&self) -> GLB_BUSY_R {
        GLB_BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - local busy 0: no change is pending for current node 1: current node is changing status"]
    #[inline(always)]
    pub fn loc_busy(&self) -> LOC_BUSY_R {
        LOC_BUSY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 0:1 - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    #[inline(always)]
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
#[doc = "Resource control register for mot0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resource_mot0](index.html) module"]
pub struct RESOURCE_MOT0_SPEC;
impl crate::RegisterSpec for RESOURCE_MOT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resource_mot0::R](R) reader structure"]
impl crate::Readable for RESOURCE_MOT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resource_mot0::W](W) writer structure"]
impl crate::Writable for RESOURCE_MOT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESOURCE_MOT0 to value 0"]
impl crate::Resettable for RESOURCE_MOT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
