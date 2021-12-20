#[doc = "Register `pf_drv0` reader"]
pub struct R(crate::R<PF_DRV0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_DRV0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_DRV0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_DRV0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pf_drv0` writer"]
pub struct W(crate::W<PF_DRV0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_DRV0_SPEC>;
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
impl From<crate::W<PF_DRV0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF_DRV0_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PF Multi_Driving Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_drv0](index.html) module"]
pub struct PF_DRV0_SPEC;
impl crate::RegisterSpec for PF_DRV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_drv0::R](R) reader structure"]
impl crate::Readable for PF_DRV0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_drv0::W](W) writer structure"]
impl crate::Writable for PF_DRV0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pf_drv0 to value 0"]
impl crate::Resettable for PF_DRV0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}