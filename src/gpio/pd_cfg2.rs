#[doc = "Register `pd_cfg2` reader"]
pub struct R(crate::R<PD_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pd_cfg2` writer"]
pub struct W(crate::W<PD_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_CFG2_SPEC>;
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
impl From<crate::W<PD_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_CFG2_SPEC>) -> Self {
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
#[doc = "PD Configure Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_cfg2](index.html) module"]
pub struct PD_CFG2_SPEC;
impl crate::RegisterSpec for PD_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_cfg2::R](R) reader structure"]
impl crate::Readable for PD_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_cfg2::W](W) writer structure"]
impl crate::Writable for PD_CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pd_cfg2 to value 0"]
impl crate::Resettable for PD_CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}