#[doc = "Register `data0_data1_vol_ctr` reader"]
pub struct R(crate::R<DATA0_DATA1_VOL_CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA0_DATA1_VOL_CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA0_DATA1_VOL_CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA0_DATA1_VOL_CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `data0_data1_vol_ctr` writer"]
pub struct W(crate::W<DATA0_DATA1_VOL_CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA0_DATA1_VOL_CTR_SPEC>;
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
impl From<crate::W<DATA0_DATA1_VOL_CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA0_DATA1_VOL_CTR_SPEC>) -> Self {
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
#[doc = "Data0 and Data1 Volume Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data0_data1_vol_ctr](index.html) module"]
pub struct DATA0_DATA1_VOL_CTR_SPEC;
impl crate::RegisterSpec for DATA0_DATA1_VOL_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data0_data1_vol_ctr::R](R) reader structure"]
impl crate::Readable for DATA0_DATA1_VOL_CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data0_data1_vol_ctr::W](W) writer structure"]
impl crate::Writable for DATA0_DATA1_VOL_CTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets data0_data1_vol_ctr to value 0"]
impl crate::Resettable for DATA0_DATA1_VOL_CTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}