#[doc = "Reader of register ICCMVS"]
pub type R = crate::R<u16, super::ICCMVS>;
#[doc = "Interrupt compare mask virtual stack position 0 This field is the virtual stack register for ICM0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICM0_A {}
impl From<ICM0_A> for u8 {
    #[inline(always)]
    fn from(variant: ICM0_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ICM0`"]
pub type ICM0_R = crate::R<u8, ICM0_A>;
impl ICM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ICM0_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Interrupt compare mask virtual stack position 1 This field is the virtual stack register for ICM1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICM1_A {}
impl From<ICM1_A> for u8 {
    #[inline(always)]
    fn from(variant: ICM1_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ICM1`"]
pub type ICM1_R = crate::R<u8, ICM1_A>;
impl ICM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ICM1_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Interrupt compare mask virtual stack position 3 This field is the virtual stack register for ICM3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICM3_A {}
impl From<ICM3_A> for u8 {
    #[inline(always)]
    fn from(variant: ICM3_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ICM3`"]
pub type ICM3_R = crate::R<u8, ICM3_A>;
impl ICM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ICM3_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "MVS stack pointer indicate register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MVSSP_A {
    #[doc = "0: 000b = Stack empty"]
    MVSSP_0,
    #[doc = "1: 001b = ICM0 affected"]
    MVSSP_1,
    #[doc = "2: 010b = ICM0 and ICM1 affected"]
    MVSSP_2,
    #[doc = "3: 011b = ICM0, ICM1, and ICM2 affected"]
    MVSSP_3,
    #[doc = "4: 100b = ICM0, ICM1, ICM2, and ICM3 affected. Also means the stack is full."]
    MVSSP_4,
}
impl From<MVSSP_A> for u8 {
    #[inline(always)]
    fn from(variant: MVSSP_A) -> Self {
        match variant {
            MVSSP_A::MVSSP_0 => 0,
            MVSSP_A::MVSSP_1 => 1,
            MVSSP_A::MVSSP_2 => 2,
            MVSSP_A::MVSSP_3 => 3,
            MVSSP_A::MVSSP_4 => 4,
        }
    }
}
#[doc = "Reader of field `MVSSP`"]
pub type MVSSP_R = crate::R<u8, MVSSP_A>;
impl MVSSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MVSSP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MVSSP_A::MVSSP_0),
            1 => Val(MVSSP_A::MVSSP_1),
            2 => Val(MVSSP_A::MVSSP_2),
            3 => Val(MVSSP_A::MVSSP_3),
            4 => Val(MVSSP_A::MVSSP_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MVSSP_0`"]
    #[inline(always)]
    pub fn is_mvssp_0(&self) -> bool {
        *self == MVSSP_A::MVSSP_0
    }
    #[doc = "Checks if the value of the field is `MVSSP_1`"]
    #[inline(always)]
    pub fn is_mvssp_1(&self) -> bool {
        *self == MVSSP_A::MVSSP_1
    }
    #[doc = "Checks if the value of the field is `MVSSP_2`"]
    #[inline(always)]
    pub fn is_mvssp_2(&self) -> bool {
        *self == MVSSP_A::MVSSP_2
    }
    #[doc = "Checks if the value of the field is `MVSSP_3`"]
    #[inline(always)]
    pub fn is_mvssp_3(&self) -> bool {
        *self == MVSSP_A::MVSSP_3
    }
    #[doc = "Checks if the value of the field is `MVSSP_4`"]
    #[inline(always)]
    pub fn is_mvssp_4(&self) -> bool {
        *self == MVSSP_A::MVSSP_4
    }
}
#[doc = "Interrupt compare mask virtual stack position 2 This field is the virtual stack register for ICM2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICM2_A {}
impl From<ICM2_A> for u8 {
    #[inline(always)]
    fn from(variant: ICM2_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ICM2`"]
pub type ICM2_R = crate::R<u8, ICM2_A>;
impl ICM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ICM2_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt compare mask virtual stack position 0 This field is the virtual stack register for ICM0."]
    #[inline(always)]
    pub fn icm0(&self) -> ICM0_R {
        ICM0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Interrupt compare mask virtual stack position 1 This field is the virtual stack register for ICM1."]
    #[inline(always)]
    pub fn icm1(&self) -> ICM1_R {
        ICM1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Interrupt compare mask virtual stack position 3 This field is the virtual stack register for ICM3."]
    #[inline(always)]
    pub fn icm3(&self) -> ICM3_R {
        ICM3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - MVS stack pointer indicate register"]
    #[inline(always)]
    pub fn mvssp(&self) -> MVSSP_R {
        MVSSP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - Interrupt compare mask virtual stack position 2 This field is the virtual stack register for ICM2."]
    #[inline(always)]
    pub fn icm2(&self) -> ICM2_R {
        ICM2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
