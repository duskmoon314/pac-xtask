#[doc = "Register `CE_TDA` reader"]
pub struct R(crate::R<CE_TDA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CE_TDA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CE_TDA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CE_TDA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CE_TDA` writer"]
pub struct W(crate::W<CE_TDA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CE_TDA_SPEC>;
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
impl From<crate::W<CE_TDA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CE_TDA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TASK` reader - Task Descriptor Address\n\nConfigure as the first address of the descriptor structure."]
pub struct TASK_R(crate::FieldReader<u32>);
impl TASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TASK_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TASK` writer - Task Descriptor Address\n\nConfigure as the first address of the descriptor structure."]
pub struct TASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Task Descriptor Address\n\nConfigure as the first address of the descriptor structure."]
    #[inline(always)]
    pub fn task(&self) -> TASK_R {
        TASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Task Descriptor Address\n\nConfigure as the first address of the descriptor structure."]
    #[inline(always)]
    pub fn task(&mut self) -> TASK_W {
        TASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Task Descriptor Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_tda](index.html) module"]
pub struct CE_TDA_SPEC;
impl crate::RegisterSpec for CE_TDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ce_tda::R](R) reader structure"]
impl crate::Readable for CE_TDA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ce_tda::W](W) writer structure"]
impl crate::Writable for CE_TDA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CE_TDA to value 0"]
impl crate::Resettable for CE_TDA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}