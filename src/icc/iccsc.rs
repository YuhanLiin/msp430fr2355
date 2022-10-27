#[doc = "Register `ICCSC` reader"]
pub struct R(crate::R<ICCSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICCSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICCSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICCSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICCSC` writer"]
pub struct W(crate::W<ICCSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICCSC_SPEC>;
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
impl From<crate::W<ICCSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICCSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICMC` reader - Current Interrupt Compare Mask of virtual stack specifies the current ICM at the top of virtual stack If ICM\\[1:0\\]
is less than the priority level (ILSRx\\[1:0\\]) of the new interrupt, the corresponding source is sent to the CPU. Note that the ICMC is the element stack that the stack pointer is pointing to."]
pub type ICMC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VSFFLG` reader - Virtual stack full flag This bit indicates whether or not the virtual stack is full. It is automatically updated when the stack is pushed or popped."]
pub type VSFFLG_R = crate::BitReader<VSFFLG_A>;
#[doc = "Virtual stack full flag This bit indicates whether or not the virtual stack is full. It is automatically updated when the stack is pushed or popped.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSFFLG_A {
    #[doc = "0: ICCMVS register is not full"]
    VSFFLG_0 = 0,
    #[doc = "1: ICCMVS register is full"]
    VSFFLG_1 = 1,
}
impl From<VSFFLG_A> for bool {
    #[inline(always)]
    fn from(variant: VSFFLG_A) -> Self {
        variant as u8 != 0
    }
}
impl VSFFLG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSFFLG_A {
        match self.bits {
            false => VSFFLG_A::VSFFLG_0,
            true => VSFFLG_A::VSFFLG_1,
        }
    }
    #[doc = "Checks if the value of the field is `VSFFLG_0`"]
    #[inline(always)]
    pub fn is_vsfflg_0(&self) -> bool {
        *self == VSFFLG_A::VSFFLG_0
    }
    #[doc = "Checks if the value of the field is `VSFFLG_1`"]
    #[inline(always)]
    pub fn is_vsfflg_1(&self) -> bool {
        *self == VSFFLG_A::VSFFLG_1
    }
}
#[doc = "Field `VSEFLG` reader - Virtual stack empty flag.This bit indicates whether or not the virtual stack is empty. It is automatically updated when the stack is pushed or popped."]
pub type VSEFLG_R = crate::BitReader<VSEFLG_A>;
#[doc = "Virtual stack empty flag.This bit indicates whether or not the virtual stack is empty. It is automatically updated when the stack is pushed or popped.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSEFLG_A {
    #[doc = "0: Stack has valid data"]
    VSEFLG_0 = 0,
    #[doc = "1: Stack has no valid data"]
    VSEFLG_1 = 1,
}
impl From<VSEFLG_A> for bool {
    #[inline(always)]
    fn from(variant: VSEFLG_A) -> Self {
        variant as u8 != 0
    }
}
impl VSEFLG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSEFLG_A {
        match self.bits {
            false => VSEFLG_A::VSEFLG_0,
            true => VSEFLG_A::VSEFLG_1,
        }
    }
    #[doc = "Checks if the value of the field is `VSEFLG_0`"]
    #[inline(always)]
    pub fn is_vseflg_0(&self) -> bool {
        *self == VSEFLG_A::VSEFLG_0
    }
    #[doc = "Checks if the value of the field is `VSEFLG_1`"]
    #[inline(always)]
    pub fn is_vseflg_1(&self) -> bool {
        *self == VSEFLG_A::VSEFLG_1
    }
}
#[doc = "Field `ICCEN` reader - ICC enable"]
pub type ICCEN_R = crate::BitReader<ICCEN_A>;
#[doc = "ICC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICCEN_A {
    #[doc = "0: ICC module disabled"]
    ICCEN_0 = 0,
    #[doc = "1: ICC module enabled"]
    ICCEN_1 = 1,
}
impl From<ICCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ICCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ICCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICCEN_A {
        match self.bits {
            false => ICCEN_A::ICCEN_0,
            true => ICCEN_A::ICCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ICCEN_0`"]
    #[inline(always)]
    pub fn is_iccen_0(&self) -> bool {
        *self == ICCEN_A::ICCEN_0
    }
    #[doc = "Checks if the value of the field is `ICCEN_1`"]
    #[inline(always)]
    pub fn is_iccen_1(&self) -> bool {
        *self == ICCEN_A::ICCEN_1
    }
}
#[doc = "Field `ICCEN` writer - ICC enable"]
pub type ICCEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, ICCSC_SPEC, ICCEN_A, O>;
impl<'a, const O: u8> ICCEN_W<'a, O> {
    #[doc = "ICC module disabled"]
    #[inline(always)]
    pub fn iccen_0(self) -> &'a mut W {
        self.variant(ICCEN_A::ICCEN_0)
    }
    #[doc = "ICC module enabled"]
    #[inline(always)]
    pub fn iccen_1(self) -> &'a mut W {
        self.variant(ICCEN_A::ICCEN_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Current Interrupt Compare Mask of virtual stack specifies the current ICM at the top of virtual stack If ICM\\[1:0\\]
is less than the priority level (ILSRx\\[1:0\\]) of the new interrupt, the corresponding source is sent to the CPU. Note that the ICMC is the element stack that the stack pointer is pointing to."]
    #[inline(always)]
    pub fn icmc(&self) -> ICMC_R {
        ICMC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Virtual stack full flag This bit indicates whether or not the virtual stack is full. It is automatically updated when the stack is pushed or popped."]
    #[inline(always)]
    pub fn vsfflg(&self) -> VSFFLG_R {
        VSFFLG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Virtual stack empty flag.This bit indicates whether or not the virtual stack is empty. It is automatically updated when the stack is pushed or popped."]
    #[inline(always)]
    pub fn vseflg(&self) -> VSEFLG_R {
        VSEFLG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - ICC enable"]
    #[inline(always)]
    pub fn iccen(&self) -> ICCEN_R {
        ICCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - ICC enable"]
    #[inline(always)]
    pub fn iccen(&mut self) -> ICCEN_W<7> {
        ICCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICCSC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iccsc](index.html) module"]
pub struct ICCSC_SPEC;
impl crate::RegisterSpec for ICCSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [iccsc::R](R) reader structure"]
impl crate::Readable for ICCSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iccsc::W](W) writer structure"]
impl crate::Writable for ICCSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICCSC to value 0"]
impl crate::Resettable for ICCSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
