#[doc = "Register `PC` reader"]
pub struct R(crate::R<PC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC` writer"]
pub struct W(crate::W<PC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC_SPEC>;
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
impl From<crate::W<PC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DM0_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    INPUT = 1,
    #[doc = "2: `10`"]
    _0_PU = 2,
    #[doc = "3: `11`"]
    PD_1 = 3,
    #[doc = "4: `100`"]
    _0_Z = 4,
    #[doc = "5: `101`"]
    Z_1 = 5,
    #[doc = "6: `110`"]
    _0_1 = 6,
    #[doc = "7: `111`"]
    PD_PU = 7,
}
impl From<DM0_A> for u8 {
    #[inline(always)]
    fn from(variant: DM0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DM0` reader - "]
pub struct DM0_R(crate::FieldReader<u8, DM0_A>);
impl DM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DM0_A {
        match self.bits {
            0 => DM0_A::OFF,
            1 => DM0_A::INPUT,
            2 => DM0_A::_0_PU,
            3 => DM0_A::PD_1,
            4 => DM0_A::_0_Z,
            5 => DM0_A::Z_1,
            6 => DM0_A::_0_1,
            7 => DM0_A::PD_PU,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == DM0_A::OFF
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        **self == DM0_A::INPUT
    }
    #[doc = "Checks if the value of the field is `_0_PU`"]
    #[inline(always)]
    pub fn is_0_pu(&self) -> bool {
        **self == DM0_A::_0_PU
    }
    #[doc = "Checks if the value of the field is `PD_1`"]
    #[inline(always)]
    pub fn is_pd_1(&self) -> bool {
        **self == DM0_A::PD_1
    }
    #[doc = "Checks if the value of the field is `_0_Z`"]
    #[inline(always)]
    pub fn is_0_z(&self) -> bool {
        **self == DM0_A::_0_Z
    }
    #[doc = "Checks if the value of the field is `Z_1`"]
    #[inline(always)]
    pub fn is_z_1(&self) -> bool {
        **self == DM0_A::Z_1
    }
    #[doc = "Checks if the value of the field is `_0_1`"]
    #[inline(always)]
    pub fn is_0_1(&self) -> bool {
        **self == DM0_A::_0_1
    }
    #[doc = "Checks if the value of the field is `PD_PU`"]
    #[inline(always)]
    pub fn is_pd_pu(&self) -> bool {
        **self == DM0_A::PD_PU
    }
}
impl core::ops::Deref for DM0_R {
    type Target = crate::FieldReader<u8, DM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM0` writer - "]
pub struct DM0_W<'a> {
    w: &'a mut W,
}
impl<'a> DM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DM0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DM0_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DM0_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _0_pu(self) -> &'a mut W {
        self.variant(DM0_A::_0_PU)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn pd_1(self) -> &'a mut W {
        self.variant(DM0_A::PD_1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn _0_z(self) -> &'a mut W {
        self.variant(DM0_A::_0_Z)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn z_1(self) -> &'a mut W {
        self.variant(DM0_A::Z_1)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn _0_1(self) -> &'a mut W {
        self.variant(DM0_A::_0_1)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn pd_pu(self) -> &'a mut W {
        self.variant(DM0_A::PD_PU)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `DM1` reader - "]
pub struct DM1_R(crate::FieldReader<u8, u8>);
impl DM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM1` writer - "]
pub struct DM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `DM2` reader - "]
pub struct DM2_R(crate::FieldReader<u8, u8>);
impl DM2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM2` writer - "]
pub struct DM2_W<'a> {
    w: &'a mut W,
}
impl<'a> DM2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `DM3` reader - "]
pub struct DM3_R(crate::FieldReader<u8, u8>);
impl DM3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM3` writer - "]
pub struct DM3_W<'a> {
    w: &'a mut W,
}
impl<'a> DM3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `DM4` reader - "]
pub struct DM4_R(crate::FieldReader<u8, u8>);
impl DM4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM4` writer - "]
pub struct DM4_W<'a> {
    w: &'a mut W,
}
impl<'a> DM4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `DM5` reader - "]
pub struct DM5_R(crate::FieldReader<u8, u8>);
impl DM5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM5` writer - "]
pub struct DM5_W<'a> {
    w: &'a mut W,
}
impl<'a> DM5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "Field `DM6` reader - "]
pub struct DM6_R(crate::FieldReader<u8, u8>);
impl DM6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DM6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM6` writer - "]
pub struct DM6_W<'a> {
    w: &'a mut W,
}
impl<'a> DM6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `DM7` reader - "]
pub struct DM7_R(crate::FieldReader<u8, u8>);
impl DM7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DM7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM7` writer - "]
pub struct DM7_W<'a> {
    w: &'a mut W,
}
impl<'a> DM7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `PORT_VTRIP_SEL` reader - "]
pub struct PORT_VTRIP_SEL_R(crate::FieldReader<bool, bool>);
impl PORT_VTRIP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORT_VTRIP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_VTRIP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_VTRIP_SEL` writer - "]
pub struct PORT_VTRIP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_VTRIP_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PORT_SLOW` reader - "]
pub struct PORT_SLOW_R(crate::FieldReader<bool, bool>);
impl PORT_SLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PORT_SLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_SLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_SLOW` writer - "]
pub struct PORT_SLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_SLOW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PORT_IB_MODE_SEL` reader - "]
pub struct PORT_IB_MODE_SEL_R(crate::FieldReader<u8, u8>);
impl PORT_IB_MODE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PORT_IB_MODE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_IB_MODE_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_IB_MODE_SEL` writer - "]
pub struct PORT_IB_MODE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_IB_MODE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dm0(&self) -> DM0_R {
        DM0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn dm1(&self) -> DM1_R {
        DM1_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn dm2(&self) -> DM2_R {
        DM2_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn dm3(&self) -> DM3_R {
        DM3_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn dm4(&self) -> DM4_R {
        DM4_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn dm5(&self) -> DM5_R {
        DM5_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn dm6(&self) -> DM6_R {
        DM6_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn dm7(&self) -> DM7_R {
        DM7_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn port_vtrip_sel(&self) -> PORT_VTRIP_SEL_R {
        PORT_VTRIP_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn port_slow(&self) -> PORT_SLOW_R {
        PORT_SLOW_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn port_ib_mode_sel(&self) -> PORT_IB_MODE_SEL_R {
        PORT_IB_MODE_SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dm0(&mut self) -> DM0_W {
        DM0_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn dm1(&mut self) -> DM1_W {
        DM1_W { w: self }
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn dm2(&mut self) -> DM2_W {
        DM2_W { w: self }
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn dm3(&mut self) -> DM3_W {
        DM3_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn dm4(&mut self) -> DM4_W {
        DM4_W { w: self }
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn dm5(&mut self) -> DM5_W {
        DM5_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn dm6(&mut self) -> DM6_W {
        DM6_W { w: self }
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn dm7(&mut self) -> DM7_W {
        DM7_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn port_vtrip_sel(&mut self) -> PORT_VTRIP_SEL_W {
        PORT_VTRIP_SEL_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn port_slow(&mut self) -> PORT_SLOW_W {
        PORT_SLOW_W { w: self }
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn port_ib_mode_sel(&mut self) -> PORT_IB_MODE_SEL_W {
        PORT_IB_MODE_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc](index.html) module"]
pub struct PC_SPEC;
impl crate::RegisterSpec for PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc::R](R) reader structure"]
impl crate::Readable for PC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc::W](W) writer structure"]
impl crate::Writable for PC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC to value 0"]
impl crate::Resettable for PC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
