#[doc = "Register `ICCMVS` reader"]
pub type R = crate::R<IccmvsSpec>;
#[doc = "Register `ICCMVS` writer"]
pub type W = crate::W<IccmvsSpec>;
#[doc = "Field `ICM0` reader - Interrupt compare mask virtual stack position 0 This field is the virtual stack register for ICM0."]
pub type Icm0R = crate::FieldReader;
#[doc = "Field `ICM1` reader - Interrupt compare mask virtual stack position 1 This field is the virtual stack register for ICM1."]
pub type Icm1R = crate::FieldReader;
#[doc = "Field `ICM2` reader - Interrupt compare mask virtual stack position 2 This field is the virtual stack register for ICM2."]
pub type Icm2R = crate::FieldReader;
#[doc = "Field `ICM3` reader - Interrupt compare mask virtual stack position 3 This field is the virtual stack register for ICM3."]
pub type Icm3R = crate::FieldReader;
#[doc = "MVS stack pointer indicate register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mvssp {
    #[doc = "0: 000b = Stack empty"]
    Mvssp0 = 0,
    #[doc = "1: 001b = ICM0 affected"]
    Mvssp1 = 1,
    #[doc = "2: 010b = ICM0 and ICM1 affected"]
    Mvssp2 = 2,
    #[doc = "3: 011b = ICM0, ICM1, and ICM2 affected"]
    Mvssp3 = 3,
    #[doc = "4: 100b = ICM0, ICM1, ICM2, and ICM3 affected. Also means the stack is full."]
    Mvssp4 = 4,
}
impl From<Mvssp> for u8 {
    #[inline(always)]
    fn from(variant: Mvssp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mvssp {
    type Ux = u8;
}
impl crate::IsEnum for Mvssp {}
#[doc = "Field `MVSSP` reader - MVS stack pointer indicate register"]
pub type MvsspR = crate::FieldReader<Mvssp>;
impl MvsspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mvssp> {
        match self.bits {
            0 => Some(Mvssp::Mvssp0),
            1 => Some(Mvssp::Mvssp1),
            2 => Some(Mvssp::Mvssp2),
            3 => Some(Mvssp::Mvssp3),
            4 => Some(Mvssp::Mvssp4),
            _ => None,
        }
    }
    #[doc = "000b = Stack empty"]
    #[inline(always)]
    pub fn is_mvssp_0(&self) -> bool {
        *self == Mvssp::Mvssp0
    }
    #[doc = "001b = ICM0 affected"]
    #[inline(always)]
    pub fn is_mvssp_1(&self) -> bool {
        *self == Mvssp::Mvssp1
    }
    #[doc = "010b = ICM0 and ICM1 affected"]
    #[inline(always)]
    pub fn is_mvssp_2(&self) -> bool {
        *self == Mvssp::Mvssp2
    }
    #[doc = "011b = ICM0, ICM1, and ICM2 affected"]
    #[inline(always)]
    pub fn is_mvssp_3(&self) -> bool {
        *self == Mvssp::Mvssp3
    }
    #[doc = "100b = ICM0, ICM1, ICM2, and ICM3 affected. Also means the stack is full."]
    #[inline(always)]
    pub fn is_mvssp_4(&self) -> bool {
        *self == Mvssp::Mvssp4
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt compare mask virtual stack position 0 This field is the virtual stack register for ICM0."]
    #[inline(always)]
    pub fn icm0(&self) -> Icm0R {
        Icm0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Interrupt compare mask virtual stack position 1 This field is the virtual stack register for ICM1."]
    #[inline(always)]
    pub fn icm1(&self) -> Icm1R {
        Icm1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Interrupt compare mask virtual stack position 2 This field is the virtual stack register for ICM2."]
    #[inline(always)]
    pub fn icm2(&self) -> Icm2R {
        Icm2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Interrupt compare mask virtual stack position 3 This field is the virtual stack register for ICM3."]
    #[inline(always)]
    pub fn icm3(&self) -> Icm3R {
        Icm3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - MVS stack pointer indicate register"]
    #[inline(always)]
    pub fn mvssp(&self) -> MvsspR {
        MvsspR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {}
#[doc = "ICCMVS\n\nYou can [`read`](crate::Reg::read) this register and get [`iccmvs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iccmvs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IccmvsSpec;
impl crate::RegisterSpec for IccmvsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`iccmvs::R`](R) reader structure"]
impl crate::Readable for IccmvsSpec {}
#[doc = "`write(|w| ..)` method takes [`iccmvs::W`](W) writer structure"]
impl crate::Writable for IccmvsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICCMVS to value 0"]
impl crate::Resettable for IccmvsSpec {}
