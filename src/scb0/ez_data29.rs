#[doc = "Register `EZ_DATA29` reader"]
pub struct R(crate::R<EZ_DATA29_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EZ_DATA29_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EZ_DATA29_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EZ_DATA29_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EZ_DATA29` writer"]
pub struct W(crate::W<EZ_DATA29_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EZ_DATA29_SPEC>;
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
impl From<crate::W<EZ_DATA29_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EZ_DATA29_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EZ_DATA` reader - "]
pub struct EZ_DATA_R(crate::FieldReader<u8, u8>);
impl EZ_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EZ_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EZ_DATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EZ_DATA` writer - "]
pub struct EZ_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> EZ_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ez_data(&self) -> EZ_DATA_R {
        EZ_DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ez_data(&mut self) -> EZ_DATA_W {
        EZ_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ez_data29](index.html) module"]
pub struct EZ_DATA29_SPEC;
impl crate::RegisterSpec for EZ_DATA29_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ez_data29::R](R) reader structure"]
impl crate::Readable for EZ_DATA29_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ez_data29::W](W) writer structure"]
impl crate::Writable for EZ_DATA29_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EZ_DATA29 to value 0"]
impl crate::Resettable for EZ_DATA29_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
