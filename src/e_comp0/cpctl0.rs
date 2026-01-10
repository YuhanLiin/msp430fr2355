#[doc = "Register `CPCTL0` reader"]
pub type R = crate::R<Cpctl0Spec>;
#[doc = "Register `CPCTL0` writer"]
pub type W = crate::W<Cpctl0Spec>;
#[doc = "Channel input selected for the V+ terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cppsel {
    #[doc = "0: select external input source"]
    Cppsel0 = 0,
    #[doc = "1: select external input source"]
    Cppsel1 = 1,
    #[doc = "2: select external input source"]
    Cppsel2 = 2,
    #[doc = "3: select external input source"]
    Cppsel3 = 3,
    #[doc = "4: device specific, please refer to device data sheet for details"]
    Cppsel4 = 4,
    #[doc = "5: device specific, please refer to device data sheet for details"]
    Cppsel5 = 5,
    #[doc = "6: 6-bit DAC"]
    Cppsel6 = 6,
    #[doc = "7: Reserved"]
    Cppsel7 = 7,
}
impl From<Cppsel> for u8 {
    #[inline(always)]
    fn from(variant: Cppsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cppsel {
    type Ux = u8;
}
impl crate::IsEnum for Cppsel {}
#[doc = "Field `CPPSEL` reader - Channel input selected for the V+ terminal"]
pub type CppselR = crate::FieldReader<Cppsel>;
impl CppselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cppsel {
        match self.bits {
            0 => Cppsel::Cppsel0,
            1 => Cppsel::Cppsel1,
            2 => Cppsel::Cppsel2,
            3 => Cppsel::Cppsel3,
            4 => Cppsel::Cppsel4,
            5 => Cppsel::Cppsel5,
            6 => Cppsel::Cppsel6,
            7 => Cppsel::Cppsel7,
            _ => unreachable!(),
        }
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn is_cppsel_0(&self) -> bool {
        *self == Cppsel::Cppsel0
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn is_cppsel_1(&self) -> bool {
        *self == Cppsel::Cppsel1
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn is_cppsel_2(&self) -> bool {
        *self == Cppsel::Cppsel2
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn is_cppsel_3(&self) -> bool {
        *self == Cppsel::Cppsel3
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn is_cppsel_4(&self) -> bool {
        *self == Cppsel::Cppsel4
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn is_cppsel_5(&self) -> bool {
        *self == Cppsel::Cppsel5
    }
    #[doc = "6-bit DAC"]
    #[inline(always)]
    pub fn is_cppsel_6(&self) -> bool {
        *self == Cppsel::Cppsel6
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_cppsel_7(&self) -> bool {
        *self == Cppsel::Cppsel7
    }
}
#[doc = "Field `CPPSEL` writer - Channel input selected for the V+ terminal"]
pub type CppselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cppsel, crate::Safe>;
impl<'a, REG> CppselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cppsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cppsel::Cppsel0)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cppsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cppsel::Cppsel1)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cppsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cppsel::Cppsel2)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cppsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cppsel::Cppsel3)
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn cppsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Cppsel::Cppsel4)
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn cppsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Cppsel::Cppsel5)
    }
    #[doc = "6-bit DAC"]
    #[inline(always)]
    pub fn cppsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Cppsel::Cppsel6)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn cppsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Cppsel::Cppsel7)
    }
}
#[doc = "Channel input enable for the V+ terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cppen {
    #[doc = "0: Selected analog input channel for V+ terminal is disabled."]
    Cppen0 = 0,
    #[doc = "1: Selected analog input channel for V+ terminal is enabled."]
    Cppen1 = 1,
}
impl From<Cppen> for bool {
    #[inline(always)]
    fn from(variant: Cppen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPPEN` reader - Channel input enable for the V+ terminal"]
pub type CppenR = crate::BitReader<Cppen>;
impl CppenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cppen {
        match self.bits {
            false => Cppen::Cppen0,
            true => Cppen::Cppen1,
        }
    }
    #[doc = "Selected analog input channel for V+ terminal is disabled."]
    #[inline(always)]
    pub fn is_cppen_0(&self) -> bool {
        *self == Cppen::Cppen0
    }
    #[doc = "Selected analog input channel for V+ terminal is enabled."]
    #[inline(always)]
    pub fn is_cppen_1(&self) -> bool {
        *self == Cppen::Cppen1
    }
}
#[doc = "Field `CPPEN` writer - Channel input enable for the V+ terminal"]
pub type CppenW<'a, REG> = crate::BitWriter<'a, REG, Cppen>;
impl<'a, REG> CppenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selected analog input channel for V+ terminal is disabled."]
    #[inline(always)]
    pub fn cppen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cppen::Cppen0)
    }
    #[doc = "Selected analog input channel for V+ terminal is enabled."]
    #[inline(always)]
    pub fn cppen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cppen::Cppen1)
    }
}
#[doc = "Channel input selected for the - terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpnsel {
    #[doc = "0: select external input source"]
    Cpnsel0 = 0,
    #[doc = "1: select external input source"]
    Cpnsel1 = 1,
    #[doc = "2: select external input source"]
    Cpnsel2 = 2,
    #[doc = "3: select external input source"]
    Cpnsel3 = 3,
    #[doc = "4: device specific, please refer to device data sheet for details"]
    Cpnsel4 = 4,
    #[doc = "5: device specific, please refer to device data sheet for details"]
    Cpnsel5 = 5,
    #[doc = "6: 6-bit DAC"]
    Cpnsel6 = 6,
    #[doc = "7: Reserved"]
    Cpnsel7 = 7,
}
impl From<Cpnsel> for u8 {
    #[inline(always)]
    fn from(variant: Cpnsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpnsel {
    type Ux = u8;
}
impl crate::IsEnum for Cpnsel {}
#[doc = "Field `CPNSEL` reader - Channel input selected for the - terminal"]
pub type CpnselR = crate::FieldReader<Cpnsel>;
impl CpnselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpnsel {
        match self.bits {
            0 => Cpnsel::Cpnsel0,
            1 => Cpnsel::Cpnsel1,
            2 => Cpnsel::Cpnsel2,
            3 => Cpnsel::Cpnsel3,
            4 => Cpnsel::Cpnsel4,
            5 => Cpnsel::Cpnsel5,
            6 => Cpnsel::Cpnsel6,
            7 => Cpnsel::Cpnsel7,
            _ => unreachable!(),
        }
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn is_cpnsel_0(&self) -> bool {
        *self == Cpnsel::Cpnsel0
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn is_cpnsel_1(&self) -> bool {
        *self == Cpnsel::Cpnsel1
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn is_cpnsel_2(&self) -> bool {
        *self == Cpnsel::Cpnsel2
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn is_cpnsel_3(&self) -> bool {
        *self == Cpnsel::Cpnsel3
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn is_cpnsel_4(&self) -> bool {
        *self == Cpnsel::Cpnsel4
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn is_cpnsel_5(&self) -> bool {
        *self == Cpnsel::Cpnsel5
    }
    #[doc = "6-bit DAC"]
    #[inline(always)]
    pub fn is_cpnsel_6(&self) -> bool {
        *self == Cpnsel::Cpnsel6
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_cpnsel_7(&self) -> bool {
        *self == Cpnsel::Cpnsel7
    }
}
#[doc = "Field `CPNSEL` writer - Channel input selected for the - terminal"]
pub type CpnselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cpnsel, crate::Safe>;
impl<'a, REG> CpnselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cpnsel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpnsel::Cpnsel0)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cpnsel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpnsel::Cpnsel1)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cpnsel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Cpnsel::Cpnsel2)
    }
    #[doc = "select external input source"]
    #[inline(always)]
    pub fn cpnsel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Cpnsel::Cpnsel3)
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn cpnsel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Cpnsel::Cpnsel4)
    }
    #[doc = "device specific, please refer to device data sheet for details"]
    #[inline(always)]
    pub fn cpnsel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Cpnsel::Cpnsel5)
    }
    #[doc = "6-bit DAC"]
    #[inline(always)]
    pub fn cpnsel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Cpnsel::Cpnsel6)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn cpnsel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Cpnsel::Cpnsel7)
    }
}
#[doc = "Channel input enable for the - terminal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpnen {
    #[doc = "0: Selected analog input channel for V- terminal is disabled."]
    Cpnen0 = 0,
    #[doc = "1: Selected analog input channel for V- terminal is enabled."]
    Cpnen1 = 1,
}
impl From<Cpnen> for bool {
    #[inline(always)]
    fn from(variant: Cpnen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPNEN` reader - Channel input enable for the - terminal"]
pub type CpnenR = crate::BitReader<Cpnen>;
impl CpnenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpnen {
        match self.bits {
            false => Cpnen::Cpnen0,
            true => Cpnen::Cpnen1,
        }
    }
    #[doc = "Selected analog input channel for V- terminal is disabled."]
    #[inline(always)]
    pub fn is_cpnen_0(&self) -> bool {
        *self == Cpnen::Cpnen0
    }
    #[doc = "Selected analog input channel for V- terminal is enabled."]
    #[inline(always)]
    pub fn is_cpnen_1(&self) -> bool {
        *self == Cpnen::Cpnen1
    }
}
#[doc = "Field `CPNEN` writer - Channel input enable for the - terminal"]
pub type CpnenW<'a, REG> = crate::BitWriter<'a, REG, Cpnen>;
impl<'a, REG> CpnenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selected analog input channel for V- terminal is disabled."]
    #[inline(always)]
    pub fn cpnen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpnen::Cpnen0)
    }
    #[doc = "Selected analog input channel for V- terminal is enabled."]
    #[inline(always)]
    pub fn cpnen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpnen::Cpnen1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Channel input selected for the V+ terminal"]
    #[inline(always)]
    pub fn cppsel(&self) -> CppselR {
        CppselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Channel input enable for the V+ terminal"]
    #[inline(always)]
    pub fn cppen(&self) -> CppenR {
        CppenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Channel input selected for the - terminal"]
    #[inline(always)]
    pub fn cpnsel(&self) -> CpnselR {
        CpnselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Channel input enable for the - terminal"]
    #[inline(always)]
    pub fn cpnen(&self) -> CpnenR {
        CpnenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel input selected for the V+ terminal"]
    #[inline(always)]
    pub fn cppsel(&mut self) -> CppselW<'_, Cpctl0Spec> {
        CppselW::new(self, 0)
    }
    #[doc = "Bit 4 - Channel input enable for the V+ terminal"]
    #[inline(always)]
    pub fn cppen(&mut self) -> CppenW<'_, Cpctl0Spec> {
        CppenW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Channel input selected for the - terminal"]
    #[inline(always)]
    pub fn cpnsel(&mut self) -> CpnselW<'_, Cpctl0Spec> {
        CpnselW::new(self, 8)
    }
    #[doc = "Bit 12 - Channel input enable for the - terminal"]
    #[inline(always)]
    pub fn cpnen(&mut self) -> CpnenW<'_, Cpctl0Spec> {
        CpnenW::new(self, 12)
    }
}
#[doc = "Comparator Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cpctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpctl0Spec;
impl crate::RegisterSpec for Cpctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cpctl0::R`](R) reader structure"]
impl crate::Readable for Cpctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`cpctl0::W`](W) writer structure"]
impl crate::Writable for Cpctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPCTL0 to value 0"]
impl crate::Resettable for Cpctl0Spec {}
