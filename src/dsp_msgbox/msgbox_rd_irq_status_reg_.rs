#[doc = "Register `MSGBOX_RD_IRQ_STATUS_REG_%s` reader"]
pub struct R(crate::R<MSGBOX_RD_IRQ_STATUS_REG__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSGBOX_RD_IRQ_STATUS_REG__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSGBOX_RD_IRQ_STATUS_REG__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSGBOX_RD_IRQ_STATUS_REG__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSGBOX_RD_IRQ_STATUS_REG_%s` writer"]
pub struct W(crate::W<MSGBOX_RD_IRQ_STATUS_REG__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSGBOX_RD_IRQ_STATUS_REG__SPEC>;
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
impl From<crate::W<MSGBOX_RD_IRQ_STATUS_REG__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSGBOX_RD_IRQ_STATUS_REG__SPEC>) -> Self {
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
#[doc = "MSGBOX Read IRQ Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msgbox_rd_irq_status_reg_](index.html) module"]
pub struct MSGBOX_RD_IRQ_STATUS_REG__SPEC;
impl crate::RegisterSpec for MSGBOX_RD_IRQ_STATUS_REG__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msgbox_rd_irq_status_reg_::R](R) reader structure"]
impl crate::Readable for MSGBOX_RD_IRQ_STATUS_REG__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msgbox_rd_irq_status_reg_::W](W) writer structure"]
impl crate::Writable for MSGBOX_RD_IRQ_STATUS_REG__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSGBOX_RD_IRQ_STATUS_REG_%s to value 0"]
impl crate::Resettable for MSGBOX_RD_IRQ_STATUS_REG__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}