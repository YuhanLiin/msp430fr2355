#[doc = "Register `ADCMCTL0` reader"]
pub type R = crate::R<Adcmctl0Spec>;
#[doc = "Register `ADCMCTL0` writer"]
pub type W = crate::W<Adcmctl0Spec>;
#[doc = "Input channel select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcinch {
    #[doc = "0: A0 - see device-specific data sheet"]
    Adcinch0 = 0,
    #[doc = "1: A1 - see device-specific data sheet"]
    Adcinch1 = 1,
    #[doc = "2: A2 - see device-specific data sheet"]
    Adcinch2 = 2,
    #[doc = "3: A3 - see device-specific data sheet"]
    Adcinch3 = 3,
    #[doc = "4: A4 - see device-specific data sheet"]
    Adcinch4 = 4,
    #[doc = "5: A5 - see device-specific data sheet"]
    Adcinch5 = 5,
    #[doc = "6: A2 - see device-specific data sheet"]
    Adcinch6 = 6,
    #[doc = "7: A7 - see device-specific data sheet"]
    Adcinch7 = 7,
    #[doc = "8: A8 - see device-specific data sheet"]
    Adcinch8 = 8,
    #[doc = "9: A9 - see device-specific data sheet"]
    Adcinch9 = 9,
    #[doc = "10: A10 - see device-specific data sheet"]
    Adcinch10 = 10,
    #[doc = "11: A11 - see device-specific data sheet"]
    Adcinch11 = 11,
    #[doc = "12: A12 - see device-specific data sheet"]
    Adcinch12 = 12,
    #[doc = "13: A13 - see device-specific data sheet"]
    Adcinch13 = 13,
    #[doc = "14: A14 - see device-specific data sheet"]
    Adcinch14 = 14,
    #[doc = "15: A15 - see device-specific data sheet"]
    Adcinch15 = 15,
}
impl From<Adcinch> for u8 {
    #[inline(always)]
    fn from(variant: Adcinch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcinch {
    type Ux = u8;
}
impl crate::IsEnum for Adcinch {}
#[doc = "Field `ADCINCH` reader - Input channel select"]
pub type AdcinchR = crate::FieldReader<Adcinch>;
impl AdcinchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcinch {
        match self.bits {
            0 => Adcinch::Adcinch0,
            1 => Adcinch::Adcinch1,
            2 => Adcinch::Adcinch2,
            3 => Adcinch::Adcinch3,
            4 => Adcinch::Adcinch4,
            5 => Adcinch::Adcinch5,
            6 => Adcinch::Adcinch6,
            7 => Adcinch::Adcinch7,
            8 => Adcinch::Adcinch8,
            9 => Adcinch::Adcinch9,
            10 => Adcinch::Adcinch10,
            11 => Adcinch::Adcinch11,
            12 => Adcinch::Adcinch12,
            13 => Adcinch::Adcinch13,
            14 => Adcinch::Adcinch14,
            15 => Adcinch::Adcinch15,
            _ => unreachable!(),
        }
    }
    #[doc = "A0 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_0(&self) -> bool {
        *self == Adcinch::Adcinch0
    }
    #[doc = "A1 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_1(&self) -> bool {
        *self == Adcinch::Adcinch1
    }
    #[doc = "A2 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_2(&self) -> bool {
        *self == Adcinch::Adcinch2
    }
    #[doc = "A3 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_3(&self) -> bool {
        *self == Adcinch::Adcinch3
    }
    #[doc = "A4 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_4(&self) -> bool {
        *self == Adcinch::Adcinch4
    }
    #[doc = "A5 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_5(&self) -> bool {
        *self == Adcinch::Adcinch5
    }
    #[doc = "A2 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_6(&self) -> bool {
        *self == Adcinch::Adcinch6
    }
    #[doc = "A7 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_7(&self) -> bool {
        *self == Adcinch::Adcinch7
    }
    #[doc = "A8 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_8(&self) -> bool {
        *self == Adcinch::Adcinch8
    }
    #[doc = "A9 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_9(&self) -> bool {
        *self == Adcinch::Adcinch9
    }
    #[doc = "A10 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_10(&self) -> bool {
        *self == Adcinch::Adcinch10
    }
    #[doc = "A11 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_11(&self) -> bool {
        *self == Adcinch::Adcinch11
    }
    #[doc = "A12 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_12(&self) -> bool {
        *self == Adcinch::Adcinch12
    }
    #[doc = "A13 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_13(&self) -> bool {
        *self == Adcinch::Adcinch13
    }
    #[doc = "A14 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_14(&self) -> bool {
        *self == Adcinch::Adcinch14
    }
    #[doc = "A15 - see device-specific data sheet"]
    #[inline(always)]
    pub fn is_adcinch_15(&self) -> bool {
        *self == Adcinch::Adcinch15
    }
}
#[doc = "Field `ADCINCH` writer - Input channel select"]
pub type AdcinchW<'a, REG> = crate::FieldWriter<'a, REG, 4, Adcinch, crate::Safe>;
impl<'a, REG> AdcinchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "A0 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch0)
    }
    #[doc = "A1 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch1)
    }
    #[doc = "A2 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch2)
    }
    #[doc = "A3 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch3)
    }
    #[doc = "A4 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch4)
    }
    #[doc = "A5 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch5)
    }
    #[doc = "A2 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch6)
    }
    #[doc = "A7 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch7)
    }
    #[doc = "A8 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_8(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch8)
    }
    #[doc = "A9 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_9(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch9)
    }
    #[doc = "A10 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_10(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch10)
    }
    #[doc = "A11 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_11(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch11)
    }
    #[doc = "A12 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_12(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch12)
    }
    #[doc = "A13 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_13(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch13)
    }
    #[doc = "A14 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_14(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch14)
    }
    #[doc = "A15 - see device-specific data sheet"]
    #[inline(always)]
    pub fn adcinch_15(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinch::Adcinch15)
    }
}
#[doc = "Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcsref {
    #[doc = "0: 000b = V(R+) = AVCC and V(R-) = AVSS"]
    Adcsref0 = 0,
    #[doc = "1: 001b = V(R+) = VREF and V(R-) = AVSS"]
    Adcsref1 = 1,
    #[doc = "2: 010b = V(R+) = VEREF+ buffered and V(R-) = AVSS"]
    Adcsref2 = 2,
    #[doc = "3: 011b =V(R+) = VEREF+ and V(R-) = AVSS"]
    Adcsref3 = 3,
    #[doc = "4: 100b = V(R+) = AVCC and V(R-) = VEREF-"]
    Adcsref4 = 4,
    #[doc = "5: 101b = V(R+) = VREF and V(R-) = VEREF-"]
    Adcsref5 = 5,
    #[doc = "6: 110b = V(R+) = VEREF+ buffered and V(R-) = VEREF-"]
    Adcsref6 = 6,
    #[doc = "7: 111b = V(R+) = VEREF+ and V(R-) = VEREF-"]
    Adcsref7 = 7,
}
impl From<Adcsref> for u8 {
    #[inline(always)]
    fn from(variant: Adcsref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcsref {
    type Ux = u8;
}
impl crate::IsEnum for Adcsref {}
#[doc = "Field `ADCSREF` reader - Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active."]
pub type AdcsrefR = crate::FieldReader<Adcsref>;
impl AdcsrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcsref {
        match self.bits {
            0 => Adcsref::Adcsref0,
            1 => Adcsref::Adcsref1,
            2 => Adcsref::Adcsref2,
            3 => Adcsref::Adcsref3,
            4 => Adcsref::Adcsref4,
            5 => Adcsref::Adcsref5,
            6 => Adcsref::Adcsref6,
            7 => Adcsref::Adcsref7,
            _ => unreachable!(),
        }
    }
    #[doc = "000b = V(R+) = AVCC and V(R-) = AVSS"]
    #[inline(always)]
    pub fn is_adcsref_0(&self) -> bool {
        *self == Adcsref::Adcsref0
    }
    #[doc = "001b = V(R+) = VREF and V(R-) = AVSS"]
    #[inline(always)]
    pub fn is_adcsref_1(&self) -> bool {
        *self == Adcsref::Adcsref1
    }
    #[doc = "010b = V(R+) = VEREF+ buffered and V(R-) = AVSS"]
    #[inline(always)]
    pub fn is_adcsref_2(&self) -> bool {
        *self == Adcsref::Adcsref2
    }
    #[doc = "011b =V(R+) = VEREF+ and V(R-) = AVSS"]
    #[inline(always)]
    pub fn is_adcsref_3(&self) -> bool {
        *self == Adcsref::Adcsref3
    }
    #[doc = "100b = V(R+) = AVCC and V(R-) = VEREF-"]
    #[inline(always)]
    pub fn is_adcsref_4(&self) -> bool {
        *self == Adcsref::Adcsref4
    }
    #[doc = "101b = V(R+) = VREF and V(R-) = VEREF-"]
    #[inline(always)]
    pub fn is_adcsref_5(&self) -> bool {
        *self == Adcsref::Adcsref5
    }
    #[doc = "110b = V(R+) = VEREF+ buffered and V(R-) = VEREF-"]
    #[inline(always)]
    pub fn is_adcsref_6(&self) -> bool {
        *self == Adcsref::Adcsref6
    }
    #[doc = "111b = V(R+) = VEREF+ and V(R-) = VEREF-"]
    #[inline(always)]
    pub fn is_adcsref_7(&self) -> bool {
        *self == Adcsref::Adcsref7
    }
}
#[doc = "Field `ADCSREF` writer - Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active."]
pub type AdcsrefW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adcsref, crate::Safe>;
impl<'a, REG> AdcsrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "000b = V(R+) = AVCC and V(R-) = AVSS"]
    #[inline(always)]
    pub fn adcsref_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref0)
    }
    #[doc = "001b = V(R+) = VREF and V(R-) = AVSS"]
    #[inline(always)]
    pub fn adcsref_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref1)
    }
    #[doc = "010b = V(R+) = VEREF+ buffered and V(R-) = AVSS"]
    #[inline(always)]
    pub fn adcsref_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref2)
    }
    #[doc = "011b =V(R+) = VEREF+ and V(R-) = AVSS"]
    #[inline(always)]
    pub fn adcsref_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref3)
    }
    #[doc = "100b = V(R+) = AVCC and V(R-) = VEREF-"]
    #[inline(always)]
    pub fn adcsref_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref4)
    }
    #[doc = "101b = V(R+) = VREF and V(R-) = VEREF-"]
    #[inline(always)]
    pub fn adcsref_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref5)
    }
    #[doc = "110b = V(R+) = VEREF+ buffered and V(R-) = VEREF-"]
    #[inline(always)]
    pub fn adcsref_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref6)
    }
    #[doc = "111b = V(R+) = VEREF+ and V(R-) = VEREF-"]
    #[inline(always)]
    pub fn adcsref_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsref::Adcsref7)
    }
}
#[doc = "ADC input channels expanded\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Expchen {
    #[doc = "0: ADC channel expanded disable"]
    Expchen0 = 0,
    #[doc = "1: ADC channel expanded enable"]
    Expchen1 = 1,
}
impl From<Expchen> for bool {
    #[inline(always)]
    fn from(variant: Expchen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXPCHEN` reader - ADC input channels expanded"]
pub type ExpchenR = crate::BitReader<Expchen>;
impl ExpchenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Expchen {
        match self.bits {
            false => Expchen::Expchen0,
            true => Expchen::Expchen1,
        }
    }
    #[doc = "ADC channel expanded disable"]
    #[inline(always)]
    pub fn is_expchen_0(&self) -> bool {
        *self == Expchen::Expchen0
    }
    #[doc = "ADC channel expanded enable"]
    #[inline(always)]
    pub fn is_expchen_1(&self) -> bool {
        *self == Expchen::Expchen1
    }
}
#[doc = "Field `EXPCHEN` writer - ADC input channels expanded"]
pub type ExpchenW<'a, REG> = crate::BitWriter<'a, REG, Expchen>;
impl<'a, REG> ExpchenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC channel expanded disable"]
    #[inline(always)]
    pub fn expchen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Expchen::Expchen0)
    }
    #[doc = "ADC channel expanded enable"]
    #[inline(always)]
    pub fn expchen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Expchen::Expchen1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Input channel select"]
    #[inline(always)]
    pub fn adcinch(&self) -> AdcinchR {
        AdcinchR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active."]
    #[inline(always)]
    pub fn adcsref(&self) -> AdcsrefR {
        AdcsrefR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - ADC input channels expanded"]
    #[inline(always)]
    pub fn expchen(&self) -> ExpchenR {
        ExpchenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input channel select"]
    #[inline(always)]
    pub fn adcinch(&mut self) -> AdcinchW<'_, Adcmctl0Spec> {
        AdcinchW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Select reference. It is not recommended to change this setting while a conversion is ongoing. Can be modified only when ADCENC = 0. Resetting ADCENC = 0 by software and changing these fields immediately shows an effect when a conversion is active."]
    #[inline(always)]
    pub fn adcsref(&mut self) -> AdcsrefW<'_, Adcmctl0Spec> {
        AdcsrefW::new(self, 4)
    }
    #[doc = "Bit 8 - ADC input channels expanded"]
    #[inline(always)]
    pub fn expchen(&mut self) -> ExpchenW<'_, Adcmctl0Spec> {
        ExpchenW::new(self, 8)
    }
}
#[doc = "ADC Conversion Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcmctl0Spec;
impl crate::RegisterSpec for Adcmctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmctl0::R`](R) reader structure"]
impl crate::Readable for Adcmctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`adcmctl0::W`](W) writer structure"]
impl crate::Writable for Adcmctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCMCTL0 to value 0"]
impl crate::Resettable for Adcmctl0Spec {}
