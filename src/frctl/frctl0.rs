#[doc = "Register `FRCTL0` reader"]
pub type R = crate::R<Frctl0Spec>;
#[doc = "Register `FRCTL0` writer"]
pub type W = crate::W<Frctl0Spec>;
#[doc = "Wait state numbers\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nwaits {
    #[doc = "0: No wait state"]
    Wait0 = 0,
    #[doc = "1: 1 wait state"]
    Wait1 = 1,
    #[doc = "2: 2 wait states"]
    Wait2 = 2,
    #[doc = "3: 3 wait states"]
    Wait3 = 3,
    #[doc = "4: 4 wait states"]
    Wait4 = 4,
    #[doc = "5: 5 wait states"]
    Wait5 = 5,
    #[doc = "6: 6 wait states"]
    Wait6 = 6,
    #[doc = "7: 7 wait states"]
    Wait7 = 7,
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
            0 => Nwaits::Wait0,
            1 => Nwaits::Wait1,
            2 => Nwaits::Wait2,
            3 => Nwaits::Wait3,
            4 => Nwaits::Wait4,
            5 => Nwaits::Wait5,
            6 => Nwaits::Wait6,
            7 => Nwaits::Wait7,
            _ => unreachable!(),
        }
    }
    #[doc = "No wait state"]
    #[inline(always)]
    pub fn is_wait0(&self) -> bool {
        *self == Nwaits::Wait0
    }
    #[doc = "1 wait state"]
    #[inline(always)]
    pub fn is_wait1(&self) -> bool {
        *self == Nwaits::Wait1
    }
    #[doc = "2 wait states"]
    #[inline(always)]
    pub fn is_wait2(&self) -> bool {
        *self == Nwaits::Wait2
    }
    #[doc = "3 wait states"]
    #[inline(always)]
    pub fn is_wait3(&self) -> bool {
        *self == Nwaits::Wait3
    }
    #[doc = "4 wait states"]
    #[inline(always)]
    pub fn is_wait4(&self) -> bool {
        *self == Nwaits::Wait4
    }
    #[doc = "5 wait states"]
    #[inline(always)]
    pub fn is_wait5(&self) -> bool {
        *self == Nwaits::Wait5
    }
    #[doc = "6 wait states"]
    #[inline(always)]
    pub fn is_wait6(&self) -> bool {
        *self == Nwaits::Wait6
    }
    #[doc = "7 wait states"]
    #[inline(always)]
    pub fn is_wait7(&self) -> bool {
        *self == Nwaits::Wait7
    }
}
#[doc = "Field `NWAITS` writer - Wait state numbers"]
pub type NwaitsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Nwaits, crate::Safe>;
impl<'a, REG> NwaitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No wait state"]
    #[inline(always)]
    pub fn wait0(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Wait0)
    }
    #[doc = "1 wait state"]
    #[inline(always)]
    pub fn wait1(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Wait1)
    }
    #[doc = "2 wait states"]
    #[inline(always)]
    pub fn wait2(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Wait2)
    }
    #[doc = "3 wait states"]
    #[inline(always)]
    pub fn wait3(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Wait3)
    }
    #[doc = "4 wait states"]
    #[inline(always)]
    pub fn wait4(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Wait4)
    }
    #[doc = "5 wait states"]
    #[inline(always)]
    pub fn wait5(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Wait5)
    }
    #[doc = "6 wait states"]
    #[inline(always)]
    pub fn wait6(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Wait6)
    }
    #[doc = "7 wait states"]
    #[inline(always)]
    pub fn wait7(self) -> &'a mut crate::W<REG> {
        self.variant(Nwaits::Wait7)
    }
}
#[doc = "FRCTLPW password\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frctlpwr {
    #[doc = "150: Value always read from the FRCTL0 password"]
    Password = 150,
}
impl From<Frctlpwr> for u8 {
    #[inline(always)]
    fn from(variant: Frctlpwr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frctlpwr {
    type Ux = u8;
}
impl crate::IsEnum for Frctlpwr {}
#[doc = "Field `FRCTLPW` reader - FRCTLPW password"]
pub type FrctlpwR = crate::FieldReader<Frctlpwr>;
impl FrctlpwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frctlpwr> {
        match self.bits {
            150 => Some(Frctlpwr::Password),
            _ => None,
        }
    }
    #[doc = "Value always read from the FRCTL0 password"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == Frctlpwr::Password
    }
}
#[doc = "FRCTLPW password\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FrctlpwwWO {
    #[doc = "165: Value which must be written to the FRCTL0 password"]
    Password = 165,
}
impl From<FrctlpwwWO> for u8 {
    #[inline(always)]
    fn from(variant: FrctlpwwWO) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FrctlpwwWO {
    type Ux = u8;
}
impl crate::IsEnum for FrctlpwwWO {}
#[doc = "Field `FRCTLPW` writer - FRCTLPW password"]
pub type FrctlpwW<'a, REG> = crate::FieldWriter<'a, REG, 8, FrctlpwwWO>;
impl<'a, REG> FrctlpwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Value which must be written to the FRCTL0 password"]
    #[inline(always)]
    pub fn password(self) -> &'a mut crate::W<REG> {
        self.variant(FrctlpwwWO::Password)
    }
}
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
