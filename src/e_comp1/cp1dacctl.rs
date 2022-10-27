#[doc = "Register `CP1DACCTL` reader"]
pub struct R(crate::R<CP1DACCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CP1DACCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CP1DACCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CP1DACCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CP1DACCTL` writer"]
pub struct W(crate::W<CP1DACCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CP1DACCTL_SPEC>;
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
impl From<crate::W<CP1DACCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CP1DACCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPDACSW` reader - This bit is only valid when CPDACBUFS is set to 1."]
pub type CPDACSW_R = crate::BitReader<CPDACSW_A>;
#[doc = "This bit is only valid when CPDACBUFS is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPDACSW_A {
    #[doc = "0: CPDACBUF1 selected"]
    CPDACSW_0 = 0,
    #[doc = "1: CPDACBUF2 selected"]
    CPDACSW_1 = 1,
}
impl From<CPDACSW_A> for bool {
    #[inline(always)]
    fn from(variant: CPDACSW_A) -> Self {
        variant as u8 != 0
    }
}
impl CPDACSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPDACSW_A {
        match self.bits {
            false => CPDACSW_A::CPDACSW_0,
            true => CPDACSW_A::CPDACSW_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPDACSW_0`"]
    #[inline(always)]
    pub fn is_cpdacsw_0(&self) -> bool {
        *self == CPDACSW_A::CPDACSW_0
    }
    #[doc = "Checks if the value of the field is `CPDACSW_1`"]
    #[inline(always)]
    pub fn is_cpdacsw_1(&self) -> bool {
        *self == CPDACSW_A::CPDACSW_1
    }
}
#[doc = "Field `CPDACSW` writer - This bit is only valid when CPDACBUFS is set to 1."]
pub type CPDACSW_W<'a, const O: u8> = crate::BitWriter<'a, u16, CP1DACCTL_SPEC, CPDACSW_A, O>;
impl<'a, const O: u8> CPDACSW_W<'a, O> {
    #[doc = "CPDACBUF1 selected"]
    #[inline(always)]
    pub fn cpdacsw_0(self) -> &'a mut W {
        self.variant(CPDACSW_A::CPDACSW_0)
    }
    #[doc = "CPDACBUF2 selected"]
    #[inline(always)]
    pub fn cpdacsw_1(self) -> &'a mut W {
        self.variant(CPDACSW_A::CPDACSW_1)
    }
}
#[doc = "Field `CPDACBUFS` reader - Comparator built-in DAC buffer controlled source selection."]
pub type CPDACBUFS_R = crate::BitReader<CPDACBUFS_A>;
#[doc = "Comparator built-in DAC buffer controlled source selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPDACBUFS_A {
    #[doc = "0: Comparator output is selected as the buffer control source"]
    CPDACBUFS_0 = 0,
    #[doc = "1: CPDACSW bit is selected as the buffer control source"]
    CPDACBUFS_1 = 1,
}
impl From<CPDACBUFS_A> for bool {
    #[inline(always)]
    fn from(variant: CPDACBUFS_A) -> Self {
        variant as u8 != 0
    }
}
impl CPDACBUFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPDACBUFS_A {
        match self.bits {
            false => CPDACBUFS_A::CPDACBUFS_0,
            true => CPDACBUFS_A::CPDACBUFS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPDACBUFS_0`"]
    #[inline(always)]
    pub fn is_cpdacbufs_0(&self) -> bool {
        *self == CPDACBUFS_A::CPDACBUFS_0
    }
    #[doc = "Checks if the value of the field is `CPDACBUFS_1`"]
    #[inline(always)]
    pub fn is_cpdacbufs_1(&self) -> bool {
        *self == CPDACBUFS_A::CPDACBUFS_1
    }
}
#[doc = "Field `CPDACBUFS` writer - Comparator built-in DAC buffer controlled source selection."]
pub type CPDACBUFS_W<'a, const O: u8> = crate::BitWriter<'a, u16, CP1DACCTL_SPEC, CPDACBUFS_A, O>;
impl<'a, const O: u8> CPDACBUFS_W<'a, O> {
    #[doc = "Comparator output is selected as the buffer control source"]
    #[inline(always)]
    pub fn cpdacbufs_0(self) -> &'a mut W {
        self.variant(CPDACBUFS_A::CPDACBUFS_0)
    }
    #[doc = "CPDACSW bit is selected as the buffer control source"]
    #[inline(always)]
    pub fn cpdacbufs_1(self) -> &'a mut W {
        self.variant(CPDACBUFS_A::CPDACBUFS_1)
    }
}
#[doc = "Field `CPDACREFS` reader - Comparator built-in DAC reference voltage selection"]
pub type CPDACREFS_R = crate::BitReader<CPDACREFS_A>;
#[doc = "Comparator built-in DAC reference voltage selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPDACREFS_A {
    #[doc = "0: VDD selected"]
    CPDACREFS_0 = 0,
    #[doc = "1: on-chip VREF selected"]
    CPDACREFS_1 = 1,
}
impl From<CPDACREFS_A> for bool {
    #[inline(always)]
    fn from(variant: CPDACREFS_A) -> Self {
        variant as u8 != 0
    }
}
impl CPDACREFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPDACREFS_A {
        match self.bits {
            false => CPDACREFS_A::CPDACREFS_0,
            true => CPDACREFS_A::CPDACREFS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPDACREFS_0`"]
    #[inline(always)]
    pub fn is_cpdacrefs_0(&self) -> bool {
        *self == CPDACREFS_A::CPDACREFS_0
    }
    #[doc = "Checks if the value of the field is `CPDACREFS_1`"]
    #[inline(always)]
    pub fn is_cpdacrefs_1(&self) -> bool {
        *self == CPDACREFS_A::CPDACREFS_1
    }
}
#[doc = "Field `CPDACREFS` writer - Comparator built-in DAC reference voltage selection"]
pub type CPDACREFS_W<'a, const O: u8> = crate::BitWriter<'a, u16, CP1DACCTL_SPEC, CPDACREFS_A, O>;
impl<'a, const O: u8> CPDACREFS_W<'a, O> {
    #[doc = "VDD selected"]
    #[inline(always)]
    pub fn cpdacrefs_0(self) -> &'a mut W {
        self.variant(CPDACREFS_A::CPDACREFS_0)
    }
    #[doc = "on-chip VREF selected"]
    #[inline(always)]
    pub fn cpdacrefs_1(self) -> &'a mut W {
        self.variant(CPDACREFS_A::CPDACREFS_1)
    }
}
#[doc = "Field `CPDACEN` reader - Comparator built-in DAC output control bit."]
pub type CPDACEN_R = crate::BitReader<CPDACEN_A>;
#[doc = "Comparator built-in DAC output control bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPDACEN_A {
    #[doc = "0: DAC output is disabled."]
    CPDACEN_0 = 0,
    #[doc = "1: DAC output is enabled."]
    CPDACEN_1 = 1,
}
impl From<CPDACEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPDACEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CPDACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPDACEN_A {
        match self.bits {
            false => CPDACEN_A::CPDACEN_0,
            true => CPDACEN_A::CPDACEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPDACEN_0`"]
    #[inline(always)]
    pub fn is_cpdacen_0(&self) -> bool {
        *self == CPDACEN_A::CPDACEN_0
    }
    #[doc = "Checks if the value of the field is `CPDACEN_1`"]
    #[inline(always)]
    pub fn is_cpdacen_1(&self) -> bool {
        *self == CPDACEN_A::CPDACEN_1
    }
}
#[doc = "Field `CPDACEN` writer - Comparator built-in DAC output control bit."]
pub type CPDACEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CP1DACCTL_SPEC, CPDACEN_A, O>;
impl<'a, const O: u8> CPDACEN_W<'a, O> {
    #[doc = "DAC output is disabled."]
    #[inline(always)]
    pub fn cpdacen_0(self) -> &'a mut W {
        self.variant(CPDACEN_A::CPDACEN_0)
    }
    #[doc = "DAC output is enabled."]
    #[inline(always)]
    pub fn cpdacen_1(self) -> &'a mut W {
        self.variant(CPDACEN_A::CPDACEN_1)
    }
}
impl R {
    #[doc = "Bit 0 - This bit is only valid when CPDACBUFS is set to 1."]
    #[inline(always)]
    pub fn cpdacsw(&self) -> CPDACSW_R {
        CPDACSW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator built-in DAC buffer controlled source selection."]
    #[inline(always)]
    pub fn cpdacbufs(&self) -> CPDACBUFS_R {
        CPDACBUFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator built-in DAC reference voltage selection"]
    #[inline(always)]
    pub fn cpdacrefs(&self) -> CPDACREFS_R {
        CPDACREFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparator built-in DAC output control bit."]
    #[inline(always)]
    pub fn cpdacen(&self) -> CPDACEN_R {
        CPDACEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is only valid when CPDACBUFS is set to 1."]
    #[inline(always)]
    pub fn cpdacsw(&mut self) -> CPDACSW_W<0> {
        CPDACSW_W::new(self)
    }
    #[doc = "Bit 1 - Comparator built-in DAC buffer controlled source selection."]
    #[inline(always)]
    pub fn cpdacbufs(&mut self) -> CPDACBUFS_W<1> {
        CPDACBUFS_W::new(self)
    }
    #[doc = "Bit 2 - Comparator built-in DAC reference voltage selection"]
    #[inline(always)]
    pub fn cpdacrefs(&mut self) -> CPDACREFS_W<2> {
        CPDACREFS_W::new(self)
    }
    #[doc = "Bit 7 - Comparator built-in DAC output control bit."]
    #[inline(always)]
    pub fn cpdacen(&mut self) -> CPDACEN_W<7> {
        CPDACEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "6-bit Comparator built-in DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp1dacctl](index.html) module"]
pub struct CP1DACCTL_SPEC;
impl crate::RegisterSpec for CP1DACCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cp1dacctl::R](R) reader structure"]
impl crate::Readable for CP1DACCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cp1dacctl::W](W) writer structure"]
impl crate::Writable for CP1DACCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CP1DACCTL to value 0"]
impl crate::Resettable for CP1DACCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
