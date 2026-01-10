#[doc = "Register `CSCTL2` reader"]
pub type R = crate::R<Csctl2Spec>;
#[doc = "Register `CSCTL2` writer"]
pub type W = crate::W<Csctl2Spec>;
#[doc = "Field `FLLN` reader - Multiplier bits. These bits set the multiplier value N of the DCO. N must be greater than 0. Writing zero to FLLN causes N to be set to 1."]
pub type FllnR = crate::FieldReader<u16>;
#[doc = "Field `FLLN` writer - Multiplier bits. These bits set the multiplier value N of the DCO. N must be greater than 0. Writing zero to FLLN causes N to be set to 1."]
pub type FllnW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "FLL loop divider. These bits divide f(DCOCLK) in the FLL feedback loop. This results in an additional multiplier for the multiplier bits. See also multiplier bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flld {
    #[doc = "0: fDCOCLK / 1"]
    _1 = 0,
    #[doc = "1: fDCOCLK / 2"]
    _2 = 1,
    #[doc = "2: fDCOCLK / 4"]
    _4 = 2,
    #[doc = "3: fDCOCLK / 8"]
    _8 = 3,
    #[doc = "4: fDCOCLK / 16"]
    _16 = 4,
    #[doc = "5: fDCOCLK / 32"]
    _32 = 5,
    #[doc = "6: fDCOCLK / 40(Only avaliable in 24MHz clock system)"]
    Flld6 = 6,
    #[doc = "7: fDCOCLK / 48(Only avaliable in 24MHz clock system)"]
    Flld7 = 7,
}
impl From<Flld> for u8 {
    #[inline(always)]
    fn from(variant: Flld) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flld {
    type Ux = u8;
}
impl crate::IsEnum for Flld {}
#[doc = "Field `FLLD` reader - FLL loop divider. These bits divide f(DCOCLK) in the FLL feedback loop. This results in an additional multiplier for the multiplier bits. See also multiplier bits."]
pub type FlldR = crate::FieldReader<Flld>;
impl FlldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flld {
        match self.bits {
            0 => Flld::_1,
            1 => Flld::_2,
            2 => Flld::_4,
            3 => Flld::_8,
            4 => Flld::_16,
            5 => Flld::_32,
            6 => Flld::Flld6,
            7 => Flld::Flld7,
            _ => unreachable!(),
        }
    }
    #[doc = "fDCOCLK / 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Flld::_1
    }
    #[doc = "fDCOCLK / 2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Flld::_2
    }
    #[doc = "fDCOCLK / 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Flld::_4
    }
    #[doc = "fDCOCLK / 8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Flld::_8
    }
    #[doc = "fDCOCLK / 16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Flld::_16
    }
    #[doc = "fDCOCLK / 32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Flld::_32
    }
    #[doc = "fDCOCLK / 40(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn is_flld_6(&self) -> bool {
        *self == Flld::Flld6
    }
    #[doc = "fDCOCLK / 48(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn is_flld_7(&self) -> bool {
        *self == Flld::Flld7
    }
}
#[doc = "Field `FLLD` writer - FLL loop divider. These bits divide f(DCOCLK) in the FLL feedback loop. This results in an additional multiplier for the multiplier bits. See also multiplier bits."]
pub type FlldW<'a, REG> = crate::FieldWriter<'a, REG, 3, Flld, crate::Safe>;
impl<'a, REG> FlldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fDCOCLK / 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::_1)
    }
    #[doc = "fDCOCLK / 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::_2)
    }
    #[doc = "fDCOCLK / 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::_4)
    }
    #[doc = "fDCOCLK / 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::_8)
    }
    #[doc = "fDCOCLK / 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::_16)
    }
    #[doc = "fDCOCLK / 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::_32)
    }
    #[doc = "fDCOCLK / 40(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn flld_6(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::Flld6)
    }
    #[doc = "fDCOCLK / 48(Only avaliable in 24MHz clock system)"]
    #[inline(always)]
    pub fn flld_7(self) -> &'a mut crate::W<REG> {
        self.variant(Flld::Flld7)
    }
}
impl R {
    #[doc = "Bits 0:9 - Multiplier bits. These bits set the multiplier value N of the DCO. N must be greater than 0. Writing zero to FLLN causes N to be set to 1."]
    #[inline(always)]
    pub fn flln(&self) -> FllnR {
        FllnR::new(self.bits & 0x03ff)
    }
    #[doc = "Bits 12:14 - FLL loop divider. These bits divide f(DCOCLK) in the FLL feedback loop. This results in an additional multiplier for the multiplier bits. See also multiplier bits."]
    #[inline(always)]
    pub fn flld(&self) -> FlldR {
        FlldR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Multiplier bits. These bits set the multiplier value N of the DCO. N must be greater than 0. Writing zero to FLLN causes N to be set to 1."]
    #[inline(always)]
    pub fn flln(&mut self) -> FllnW<'_, Csctl2Spec> {
        FllnW::new(self, 0)
    }
    #[doc = "Bits 12:14 - FLL loop divider. These bits divide f(DCOCLK) in the FLL feedback loop. This results in an additional multiplier for the multiplier bits. See also multiplier bits."]
    #[inline(always)]
    pub fn flld(&mut self) -> FlldW<'_, Csctl2Spec> {
        FlldW::new(self, 12)
    }
}
#[doc = "Clock System Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`csctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csctl2Spec;
impl crate::RegisterSpec for Csctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csctl2::R`](R) reader structure"]
impl crate::Readable for Csctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`csctl2::W`](W) writer structure"]
impl crate::Writable for Csctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSCTL2 to value 0"]
impl crate::Resettable for Csctl2Spec {}
