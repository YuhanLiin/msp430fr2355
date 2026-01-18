#[doc = "Register `FRCTL0` reader"]
pub type R = crate::R<Frctl0Spec>;
#[doc = "Register `FRCTL0` writer"]
pub type W = crate::W<Frctl0Spec>;
#[doc = "Wait state numbers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nwaits {
    #[doc = "0: FRAM wait states: 0"]
    Nwaits0 = 0,
    #[doc = "1: FRAM wait states: 1"]
    Nwaits1 = 1,
    #[doc = "2: FRAM wait states: 2"]
    Nwaits2 = 2,
    #[doc = "3: FRAM wait states: 3"]
    Nwaits3 = 3,
    #[doc = "4: FRAM wait states: 4"]
    Nwaits4 = 4,
    #[doc = "5: FRAM wait states: 5"]
    Nwaits5 = 5,
    #[doc = "6: FRAM wait states: 6"]
    Nwaits6 = 6,
    #[doc = "7: FRAM wait states: 7"]
    Nwaits7 = 7,
}
impl From<Nwaits> for u8 {
    #[inline(always)]
    fn from(variant: Nwaits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nwaits {
    type Ux = u8;
}
impl crate::IsEnum for Nwaits {}
#[doc = "Field `NWAITS` reader - Wait state numbers"]
pub type NwaitsR = crate::FieldReader<Nwaits>;
impl NwaitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nwaits {
        match self.bits {
            0 => Nwaits::Nwaits0,
            1 => Nwaits::Nwaits1,
            2 => Nwaits::Nwaits2,
            3 => Nwaits::Nwaits3,
            4 => Nwaits::Nwaits4,
            5 => Nwaits::Nwaits5,
            6 => Nwaits::Nwaits6,
            7 => Nwaits::Nwaits7,
            _ => unreachable!(),
        }
    }
    #[doc = "FRAM wait states: 0"]
    #[inline(always)]
    pub fn is_nwaits_0(&self) -> bool {
        *self == Nwaits::Nwaits0
    }
    #[doc = "FRAM wait states: 1"]
    #[inline(always)]
    pub fn is_nwaits_1(&self) -> bool {
        *self == Nwaits::Nwaits1
    }
    #[doc = "FRAM wait states: 2"]
    #[inline(always)]
    pub fn is_nwaits_2(&self) -> bool {
        *self == Nwaits::Nwaits2
    }
    #[doc = "FRAM wait states: 3"]
    #[inline(always)]
    pub fn is_nwaits_3(&self) -> bool {
        *self == Nwaits::Nwaits3
    }
    #[doc = "FRAM wait states: 4"]
    #[inline(always)]
    pub fn is_nwaits_4(&self) -> bool {
        *self == Nwaits::Nwaits4
    }
    #[doc = "FRAM wait states: 5"]
    #[inline(always)]
    pub fn is_nwaits_5(&self) -> bool {
        *self == Nwaits::Nwaits5
    }
    #[doc = "FRAM wait states: 6"]
    #[inline(always)]
    pub fn is_nwaits_6(&self) -> bool {
        *self == Nwaits::Nwaits6
    }
    #[doc = "FRAM wait states: 7"]
    #[inline(always)]
    pub fn is_nwaits_7(&self) -> bool {
        *self == Nwaits::Nwaits7
    }
}
#[doc = "Field `NWAITS` writer - Wait state numbers"]
pub type NwaitsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Nwaits, crate::Safe>;
impl<'a, REG> NwaitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FRAM wait states: 0"]
    #[inline(always)]
    pub fn nwaits_0(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits0)
    }
    #[doc = "FRAM wait states: 1"]
    #[inline(always)]
    pub fn nwaits_1(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits1)
    }
    #[doc = "FRAM wait states: 2"]
    #[inline(always)]
    pub fn nwaits_2(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits2)
    }
    #[doc = "FRAM wait states: 3"]
    #[inline(always)]
    pub fn nwaits_3(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits3)
    }
    #[doc = "FRAM wait states: 4"]
    #[inline(always)]
    pub fn nwaits_4(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits4)
    }
    #[doc = "FRAM wait states: 5"]
    #[inline(always)]
    pub fn nwaits_5(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits5)
    }
    #[doc = "FRAM wait states: 6"]
    #[inline(always)]
    pub fn nwaits_6(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits6)
    }
    #[doc = "FRAM wait states: 7"]
    #[inline(always)]
    pub fn nwaits_7(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Nwaits7)
    }
}
#[doc = "Field `FRCTLPW` reader - FRCTLPW password"]
pub type FrctlpwR = crate::FieldReader;
#[doc = "Field `FRCTLPW` writer - FRCTLPW password"]
pub type FrctlpwW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 4:6 - Wait state numbers"]
    #[inline(always)]
    pub fn nwaits(&self) -> NwaitsR {
        NwaitsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:15 - FRCTLPW password"]
    #[inline(always)]
    pub fn frctlpw(&self) -> FrctlpwR {
        FrctlpwR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Wait state numbers"]
    #[inline(always)]
    pub fn nwaits(&mut self) -> NwaitsW<'_, Frctl0Spec> {
        NwaitsW::new(self, 4)
    }
    #[doc = "Bits 8:15 - FRCTLPW password"]
    #[inline(always)]
    pub fn frctlpw(&mut self) -> FrctlpwW<'_, Frctl0Spec> {
        FrctlpwW::new(self, 8)
    }
}
#[doc = "FRAM Controller Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`frctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Frctl0Spec;
impl crate::RegisterSpec for Frctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`frctl0::R`](R) reader structure"]
impl crate::Readable for Frctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`frctl0::W`](W) writer structure"]
impl crate::Writable for Frctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRCTL0 to value 0"]
impl crate::Resettable for Frctl0Spec {}
