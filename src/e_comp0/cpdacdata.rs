#[doc = "Register `CPDACDATA` reader"]
pub struct R(crate::R<CPDACDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPDACDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPDACDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPDACDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPDACDATA` writer"]
pub struct W(crate::W<CPDACDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPDACDATA_SPEC>;
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
impl From<crate::W<CPDACDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPDACDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPDACBUF1` reader - 1st 6-bit DAC buffer Data"]
pub type CPDACBUF1_R = crate::FieldReader<u8, CPDACBUF1_A>;
#[doc = "1st 6-bit DAC buffer Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPDACBUF1_A {
    #[doc = "0: 0v"]
    CPDACBUF1_0 = 0,
    #[doc = "1: selected reference voltage * 1/64"]
    CPDACBUF1_1 = 1,
    #[doc = "2: selected reference voltage * 2/64"]
    CPDACBUF1_2 = 2,
    #[doc = "3: selected reference voltage * 3/64"]
    CPDACBUF1_3 = 3,
    #[doc = "4: selected reference voltage * 4/64"]
    CPDACBUF1_4 = 4,
    #[doc = "5: selected reference voltage * 5/64"]
    CPDACBUF1_5 = 5,
    #[doc = "6: selected reference voltage * 6/64"]
    CPDACBUF1_6 = 6,
    #[doc = "7: selected reference voltage * 7/64"]
    CPDACBUF1_7 = 7,
    #[doc = "8: selected reference voltage * 8/64"]
    CPDACBUF1_8 = 8,
    #[doc = "9: selected reference voltage *9/64"]
    CPDACBUF1_9 = 9,
    #[doc = "10: selected reference voltage * 10/64"]
    CPDACBUF1_10 = 10,
    #[doc = "11: selected reference voltage * 11/64"]
    CPDACBUF1_11 = 11,
    #[doc = "12: selected reference voltage * 12/64"]
    CPDACBUF1_12 = 12,
    #[doc = "13: selected reference voltage * 13/64"]
    CPDACBUF1_13 = 13,
    #[doc = "14: selected reference voltage * 14/64"]
    CPDACBUF1_14 = 14,
    #[doc = "15: selected reference voltage * 15/64"]
    CPDACBUF1_15 = 15,
    #[doc = "16: selected reference voltage * 16/64"]
    CPDACBUF1_16 = 16,
    #[doc = "17: selected reference voltage * 17/64"]
    CPDACBUF1_17 = 17,
    #[doc = "18: selected reference voltage * 18/64"]
    CPDACBUF1_18 = 18,
    #[doc = "19: selected reference voltage * 19/64"]
    CPDACBUF1_19 = 19,
    #[doc = "20: selected reference voltage * 20/64"]
    CPDACBUF1_20 = 20,
    #[doc = "21: selected reference voltage * 21/64"]
    CPDACBUF1_21 = 21,
    #[doc = "22: selected reference voltage * 22/64"]
    CPDACBUF1_22 = 22,
    #[doc = "23: selected reference voltage * 23/64"]
    CPDACBUF1_23 = 23,
    #[doc = "24: selected reference voltage * 24/64"]
    CPDACBUF1_24 = 24,
    #[doc = "25: selected reference voltage * 25/64"]
    CPDACBUF1_25 = 25,
    #[doc = "26: selected reference voltage * 26/64"]
    CPDACBUF1_26 = 26,
    #[doc = "27: selected reference voltage * 27/64"]
    CPDACBUF1_27 = 27,
    #[doc = "28: selected reference voltage * 28/64"]
    CPDACBUF1_28 = 28,
    #[doc = "29: selected reference voltage * 29/64"]
    CPDACBUF1_29 = 29,
    #[doc = "30: selected reference voltage * 30/64"]
    CPDACBUF1_30 = 30,
    #[doc = "31: selected reference voltage * 31/64"]
    CPDACBUF1_31 = 31,
    #[doc = "32: selected reference voltage * 32/64"]
    CPDACBUF1_32 = 32,
    #[doc = "33: selected reference voltage * 33/64"]
    CPDACBUF1_33 = 33,
    #[doc = "34: selected reference voltage * 34/64"]
    CPDACBUF1_34 = 34,
    #[doc = "35: selected reference voltage * 35/64"]
    CPDACBUF1_35 = 35,
    #[doc = "36: selected reference voltage * 36/64"]
    CPDACBUF1_36 = 36,
    #[doc = "37: selected reference voltage * 37/64"]
    CPDACBUF1_37 = 37,
    #[doc = "38: selected reference voltage * 38/64"]
    CPDACBUF1_38 = 38,
    #[doc = "39: selected reference voltage * 39/64"]
    CPDACBUF1_39 = 39,
    #[doc = "40: selected reference voltage * 40/64"]
    CPDACBUF1_40 = 40,
    #[doc = "41: selected reference voltage * 41/64"]
    CPDACBUF1_41 = 41,
    #[doc = "42: selected reference voltage * 42/64"]
    CPDACBUF1_42 = 42,
    #[doc = "43: selected reference voltage * 43/64"]
    CPDACBUF1_43 = 43,
    #[doc = "44: selected reference voltage * 44/64"]
    CPDACBUF1_44 = 44,
    #[doc = "45: selected reference voltage * 45/64"]
    CPDACBUF1_45 = 45,
    #[doc = "46: selected reference voltage * 46/64"]
    CPDACBUF1_46 = 46,
    #[doc = "47: selected reference voltage * 47/64"]
    CPDACBUF1_47 = 47,
    #[doc = "48: selected reference voltage * 48/64"]
    CPDACBUF1_48 = 48,
    #[doc = "49: selected reference voltage * 49/64"]
    CPDACBUF1_49 = 49,
    #[doc = "50: selected reference voltage * 50/64"]
    CPDACBUF1_50 = 50,
    #[doc = "51: selected reference voltage * 51/64"]
    CPDACBUF1_51 = 51,
    #[doc = "52: selected reference voltage * 52/64"]
    CPDACBUF1_52 = 52,
    #[doc = "53: selected reference voltage * 53/64"]
    CPDACBUF1_53 = 53,
    #[doc = "54: selected reference voltage * 54/64"]
    CPDACBUF1_54 = 54,
    #[doc = "55: selected reference voltage * 55/64"]
    CPDACBUF1_55 = 55,
    #[doc = "56: selected reference voltage * 56/64"]
    CPDACBUF1_56 = 56,
    #[doc = "57: selected reference voltage * 57/64"]
    CPDACBUF1_57 = 57,
    #[doc = "58: selected reference voltage * 58/64"]
    CPDACBUF1_58 = 58,
    #[doc = "59: selected reference voltage * 59/64"]
    CPDACBUF1_59 = 59,
    #[doc = "60: selected reference voltage * 60/64"]
    CPDACBUF1_60 = 60,
    #[doc = "61: selected reference voltage * 61/64"]
    CPDACBUF1_61 = 61,
    #[doc = "62: selected reference voltage * 62/64"]
    CPDACBUF1_62 = 62,
    #[doc = "63: selected reference voltage * 63/64"]
    CPDACBUF1_63 = 63,
}
impl From<CPDACBUF1_A> for u8 {
    #[inline(always)]
    fn from(variant: CPDACBUF1_A) -> Self {
        variant as _
    }
}
impl CPDACBUF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPDACBUF1_A {
        match self.bits {
            0 => CPDACBUF1_A::CPDACBUF1_0,
            1 => CPDACBUF1_A::CPDACBUF1_1,
            2 => CPDACBUF1_A::CPDACBUF1_2,
            3 => CPDACBUF1_A::CPDACBUF1_3,
            4 => CPDACBUF1_A::CPDACBUF1_4,
            5 => CPDACBUF1_A::CPDACBUF1_5,
            6 => CPDACBUF1_A::CPDACBUF1_6,
            7 => CPDACBUF1_A::CPDACBUF1_7,
            8 => CPDACBUF1_A::CPDACBUF1_8,
            9 => CPDACBUF1_A::CPDACBUF1_9,
            10 => CPDACBUF1_A::CPDACBUF1_10,
            11 => CPDACBUF1_A::CPDACBUF1_11,
            12 => CPDACBUF1_A::CPDACBUF1_12,
            13 => CPDACBUF1_A::CPDACBUF1_13,
            14 => CPDACBUF1_A::CPDACBUF1_14,
            15 => CPDACBUF1_A::CPDACBUF1_15,
            16 => CPDACBUF1_A::CPDACBUF1_16,
            17 => CPDACBUF1_A::CPDACBUF1_17,
            18 => CPDACBUF1_A::CPDACBUF1_18,
            19 => CPDACBUF1_A::CPDACBUF1_19,
            20 => CPDACBUF1_A::CPDACBUF1_20,
            21 => CPDACBUF1_A::CPDACBUF1_21,
            22 => CPDACBUF1_A::CPDACBUF1_22,
            23 => CPDACBUF1_A::CPDACBUF1_23,
            24 => CPDACBUF1_A::CPDACBUF1_24,
            25 => CPDACBUF1_A::CPDACBUF1_25,
            26 => CPDACBUF1_A::CPDACBUF1_26,
            27 => CPDACBUF1_A::CPDACBUF1_27,
            28 => CPDACBUF1_A::CPDACBUF1_28,
            29 => CPDACBUF1_A::CPDACBUF1_29,
            30 => CPDACBUF1_A::CPDACBUF1_30,
            31 => CPDACBUF1_A::CPDACBUF1_31,
            32 => CPDACBUF1_A::CPDACBUF1_32,
            33 => CPDACBUF1_A::CPDACBUF1_33,
            34 => CPDACBUF1_A::CPDACBUF1_34,
            35 => CPDACBUF1_A::CPDACBUF1_35,
            36 => CPDACBUF1_A::CPDACBUF1_36,
            37 => CPDACBUF1_A::CPDACBUF1_37,
            38 => CPDACBUF1_A::CPDACBUF1_38,
            39 => CPDACBUF1_A::CPDACBUF1_39,
            40 => CPDACBUF1_A::CPDACBUF1_40,
            41 => CPDACBUF1_A::CPDACBUF1_41,
            42 => CPDACBUF1_A::CPDACBUF1_42,
            43 => CPDACBUF1_A::CPDACBUF1_43,
            44 => CPDACBUF1_A::CPDACBUF1_44,
            45 => CPDACBUF1_A::CPDACBUF1_45,
            46 => CPDACBUF1_A::CPDACBUF1_46,
            47 => CPDACBUF1_A::CPDACBUF1_47,
            48 => CPDACBUF1_A::CPDACBUF1_48,
            49 => CPDACBUF1_A::CPDACBUF1_49,
            50 => CPDACBUF1_A::CPDACBUF1_50,
            51 => CPDACBUF1_A::CPDACBUF1_51,
            52 => CPDACBUF1_A::CPDACBUF1_52,
            53 => CPDACBUF1_A::CPDACBUF1_53,
            54 => CPDACBUF1_A::CPDACBUF1_54,
            55 => CPDACBUF1_A::CPDACBUF1_55,
            56 => CPDACBUF1_A::CPDACBUF1_56,
            57 => CPDACBUF1_A::CPDACBUF1_57,
            58 => CPDACBUF1_A::CPDACBUF1_58,
            59 => CPDACBUF1_A::CPDACBUF1_59,
            60 => CPDACBUF1_A::CPDACBUF1_60,
            61 => CPDACBUF1_A::CPDACBUF1_61,
            62 => CPDACBUF1_A::CPDACBUF1_62,
            63 => CPDACBUF1_A::CPDACBUF1_63,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_0`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_0(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_0
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_1`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_1(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_1
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_2`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_2(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_2
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_3`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_3(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_3
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_4`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_4(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_4
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_5`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_5(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_5
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_6`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_6(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_6
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_7`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_7(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_7
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_8`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_8(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_8
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_9`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_9(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_9
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_10`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_10(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_10
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_11`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_11(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_11
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_12`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_12(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_12
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_13`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_13(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_13
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_14`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_14(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_14
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_15`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_15(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_15
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_16`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_16(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_16
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_17`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_17(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_17
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_18`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_18(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_18
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_19`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_19(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_19
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_20`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_20(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_20
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_21`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_21(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_21
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_22`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_22(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_22
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_23`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_23(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_23
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_24`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_24(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_24
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_25`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_25(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_25
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_26`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_26(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_26
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_27`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_27(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_27
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_28`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_28(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_28
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_29`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_29(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_29
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_30`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_30(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_30
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_31`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_31(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_31
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_32`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_32(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_32
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_33`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_33(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_33
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_34`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_34(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_34
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_35`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_35(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_35
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_36`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_36(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_36
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_37`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_37(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_37
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_38`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_38(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_38
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_39`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_39(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_39
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_40`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_40(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_40
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_41`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_41(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_41
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_42`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_42(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_42
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_43`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_43(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_43
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_44`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_44(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_44
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_45`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_45(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_45
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_46`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_46(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_46
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_47`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_47(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_47
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_48`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_48(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_48
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_49`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_49(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_49
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_50`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_50(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_50
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_51`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_51(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_51
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_52`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_52(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_52
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_53`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_53(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_53
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_54`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_54(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_54
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_55`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_55(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_55
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_56`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_56(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_56
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_57`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_57(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_57
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_58`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_58(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_58
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_59`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_59(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_59
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_60`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_60(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_60
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_61`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_61(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_61
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_62`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_62(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_62
    }
    #[doc = "Checks if the value of the field is `CPDACBUF1_63`"]
    #[inline(always)]
    pub fn is_cpdacbuf1_63(&self) -> bool {
        *self == CPDACBUF1_A::CPDACBUF1_63
    }
}
#[doc = "Field `CPDACBUF1` writer - 1st 6-bit DAC buffer Data"]
pub type CPDACBUF1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CPDACDATA_SPEC, u8, CPDACBUF1_A, 6, O>;
impl<'a, const O: u8> CPDACBUF1_W<'a, O> {
    #[doc = "0v"]
    #[inline(always)]
    pub fn cpdacbuf1_0(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_0)
    }
    #[doc = "selected reference voltage * 1/64"]
    #[inline(always)]
    pub fn cpdacbuf1_1(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_1)
    }
    #[doc = "selected reference voltage * 2/64"]
    #[inline(always)]
    pub fn cpdacbuf1_2(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_2)
    }
    #[doc = "selected reference voltage * 3/64"]
    #[inline(always)]
    pub fn cpdacbuf1_3(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_3)
    }
    #[doc = "selected reference voltage * 4/64"]
    #[inline(always)]
    pub fn cpdacbuf1_4(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_4)
    }
    #[doc = "selected reference voltage * 5/64"]
    #[inline(always)]
    pub fn cpdacbuf1_5(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_5)
    }
    #[doc = "selected reference voltage * 6/64"]
    #[inline(always)]
    pub fn cpdacbuf1_6(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_6)
    }
    #[doc = "selected reference voltage * 7/64"]
    #[inline(always)]
    pub fn cpdacbuf1_7(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_7)
    }
    #[doc = "selected reference voltage * 8/64"]
    #[inline(always)]
    pub fn cpdacbuf1_8(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_8)
    }
    #[doc = "selected reference voltage *9/64"]
    #[inline(always)]
    pub fn cpdacbuf1_9(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_9)
    }
    #[doc = "selected reference voltage * 10/64"]
    #[inline(always)]
    pub fn cpdacbuf1_10(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_10)
    }
    #[doc = "selected reference voltage * 11/64"]
    #[inline(always)]
    pub fn cpdacbuf1_11(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_11)
    }
    #[doc = "selected reference voltage * 12/64"]
    #[inline(always)]
    pub fn cpdacbuf1_12(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_12)
    }
    #[doc = "selected reference voltage * 13/64"]
    #[inline(always)]
    pub fn cpdacbuf1_13(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_13)
    }
    #[doc = "selected reference voltage * 14/64"]
    #[inline(always)]
    pub fn cpdacbuf1_14(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_14)
    }
    #[doc = "selected reference voltage * 15/64"]
    #[inline(always)]
    pub fn cpdacbuf1_15(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_15)
    }
    #[doc = "selected reference voltage * 16/64"]
    #[inline(always)]
    pub fn cpdacbuf1_16(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_16)
    }
    #[doc = "selected reference voltage * 17/64"]
    #[inline(always)]
    pub fn cpdacbuf1_17(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_17)
    }
    #[doc = "selected reference voltage * 18/64"]
    #[inline(always)]
    pub fn cpdacbuf1_18(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_18)
    }
    #[doc = "selected reference voltage * 19/64"]
    #[inline(always)]
    pub fn cpdacbuf1_19(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_19)
    }
    #[doc = "selected reference voltage * 20/64"]
    #[inline(always)]
    pub fn cpdacbuf1_20(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_20)
    }
    #[doc = "selected reference voltage * 21/64"]
    #[inline(always)]
    pub fn cpdacbuf1_21(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_21)
    }
    #[doc = "selected reference voltage * 22/64"]
    #[inline(always)]
    pub fn cpdacbuf1_22(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_22)
    }
    #[doc = "selected reference voltage * 23/64"]
    #[inline(always)]
    pub fn cpdacbuf1_23(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_23)
    }
    #[doc = "selected reference voltage * 24/64"]
    #[inline(always)]
    pub fn cpdacbuf1_24(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_24)
    }
    #[doc = "selected reference voltage * 25/64"]
    #[inline(always)]
    pub fn cpdacbuf1_25(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_25)
    }
    #[doc = "selected reference voltage * 26/64"]
    #[inline(always)]
    pub fn cpdacbuf1_26(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_26)
    }
    #[doc = "selected reference voltage * 27/64"]
    #[inline(always)]
    pub fn cpdacbuf1_27(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_27)
    }
    #[doc = "selected reference voltage * 28/64"]
    #[inline(always)]
    pub fn cpdacbuf1_28(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_28)
    }
    #[doc = "selected reference voltage * 29/64"]
    #[inline(always)]
    pub fn cpdacbuf1_29(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_29)
    }
    #[doc = "selected reference voltage * 30/64"]
    #[inline(always)]
    pub fn cpdacbuf1_30(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_30)
    }
    #[doc = "selected reference voltage * 31/64"]
    #[inline(always)]
    pub fn cpdacbuf1_31(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_31)
    }
    #[doc = "selected reference voltage * 32/64"]
    #[inline(always)]
    pub fn cpdacbuf1_32(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_32)
    }
    #[doc = "selected reference voltage * 33/64"]
    #[inline(always)]
    pub fn cpdacbuf1_33(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_33)
    }
    #[doc = "selected reference voltage * 34/64"]
    #[inline(always)]
    pub fn cpdacbuf1_34(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_34)
    }
    #[doc = "selected reference voltage * 35/64"]
    #[inline(always)]
    pub fn cpdacbuf1_35(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_35)
    }
    #[doc = "selected reference voltage * 36/64"]
    #[inline(always)]
    pub fn cpdacbuf1_36(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_36)
    }
    #[doc = "selected reference voltage * 37/64"]
    #[inline(always)]
    pub fn cpdacbuf1_37(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_37)
    }
    #[doc = "selected reference voltage * 38/64"]
    #[inline(always)]
    pub fn cpdacbuf1_38(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_38)
    }
    #[doc = "selected reference voltage * 39/64"]
    #[inline(always)]
    pub fn cpdacbuf1_39(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_39)
    }
    #[doc = "selected reference voltage * 40/64"]
    #[inline(always)]
    pub fn cpdacbuf1_40(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_40)
    }
    #[doc = "selected reference voltage * 41/64"]
    #[inline(always)]
    pub fn cpdacbuf1_41(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_41)
    }
    #[doc = "selected reference voltage * 42/64"]
    #[inline(always)]
    pub fn cpdacbuf1_42(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_42)
    }
    #[doc = "selected reference voltage * 43/64"]
    #[inline(always)]
    pub fn cpdacbuf1_43(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_43)
    }
    #[doc = "selected reference voltage * 44/64"]
    #[inline(always)]
    pub fn cpdacbuf1_44(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_44)
    }
    #[doc = "selected reference voltage * 45/64"]
    #[inline(always)]
    pub fn cpdacbuf1_45(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_45)
    }
    #[doc = "selected reference voltage * 46/64"]
    #[inline(always)]
    pub fn cpdacbuf1_46(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_46)
    }
    #[doc = "selected reference voltage * 47/64"]
    #[inline(always)]
    pub fn cpdacbuf1_47(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_47)
    }
    #[doc = "selected reference voltage * 48/64"]
    #[inline(always)]
    pub fn cpdacbuf1_48(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_48)
    }
    #[doc = "selected reference voltage * 49/64"]
    #[inline(always)]
    pub fn cpdacbuf1_49(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_49)
    }
    #[doc = "selected reference voltage * 50/64"]
    #[inline(always)]
    pub fn cpdacbuf1_50(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_50)
    }
    #[doc = "selected reference voltage * 51/64"]
    #[inline(always)]
    pub fn cpdacbuf1_51(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_51)
    }
    #[doc = "selected reference voltage * 52/64"]
    #[inline(always)]
    pub fn cpdacbuf1_52(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_52)
    }
    #[doc = "selected reference voltage * 53/64"]
    #[inline(always)]
    pub fn cpdacbuf1_53(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_53)
    }
    #[doc = "selected reference voltage * 54/64"]
    #[inline(always)]
    pub fn cpdacbuf1_54(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_54)
    }
    #[doc = "selected reference voltage * 55/64"]
    #[inline(always)]
    pub fn cpdacbuf1_55(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_55)
    }
    #[doc = "selected reference voltage * 56/64"]
    #[inline(always)]
    pub fn cpdacbuf1_56(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_56)
    }
    #[doc = "selected reference voltage * 57/64"]
    #[inline(always)]
    pub fn cpdacbuf1_57(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_57)
    }
    #[doc = "selected reference voltage * 58/64"]
    #[inline(always)]
    pub fn cpdacbuf1_58(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_58)
    }
    #[doc = "selected reference voltage * 59/64"]
    #[inline(always)]
    pub fn cpdacbuf1_59(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_59)
    }
    #[doc = "selected reference voltage * 60/64"]
    #[inline(always)]
    pub fn cpdacbuf1_60(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_60)
    }
    #[doc = "selected reference voltage * 61/64"]
    #[inline(always)]
    pub fn cpdacbuf1_61(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_61)
    }
    #[doc = "selected reference voltage * 62/64"]
    #[inline(always)]
    pub fn cpdacbuf1_62(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_62)
    }
    #[doc = "selected reference voltage * 63/64"]
    #[inline(always)]
    pub fn cpdacbuf1_63(self) -> &'a mut W {
        self.variant(CPDACBUF1_A::CPDACBUF1_63)
    }
}
#[doc = "Field `CPDACBUF2` reader - 2nd 6-bit DAC buffer Data"]
pub type CPDACBUF2_R = crate::FieldReader<u8, CPDACBUF2_A>;
#[doc = "2nd 6-bit DAC buffer Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPDACBUF2_A {
    #[doc = "0: 0v"]
    CPDACBUF2_0 = 0,
    #[doc = "1: selected reference voltage * 1/64"]
    CPDACBUF2_1 = 1,
    #[doc = "2: selected reference voltage * 2/64"]
    CPDACBUF2_2 = 2,
    #[doc = "3: selected reference voltage * 3/64"]
    CPDACBUF2_3 = 3,
    #[doc = "4: selected reference voltage * 4/64"]
    CPDACBUF2_4 = 4,
    #[doc = "5: selected reference voltage * 5/64"]
    CPDACBUF2_5 = 5,
    #[doc = "6: selected reference voltage * 6/64"]
    CPDACBUF2_6 = 6,
    #[doc = "7: selected reference voltage * 7/64"]
    CPDACBUF2_7 = 7,
    #[doc = "8: selected reference voltage * 8/64"]
    CPDACBUF2_8 = 8,
    #[doc = "9: selected reference voltage * 9/64"]
    CPDACBUF2_9 = 9,
    #[doc = "10: selected reference voltage * 10/64"]
    CPDACBUF2_10 = 10,
    #[doc = "11: selected reference voltage * 11/64"]
    CPDACBUF2_11 = 11,
    #[doc = "12: selected reference voltage * 12/64"]
    CPDACBUF2_12 = 12,
    #[doc = "13: selected reference voltage * 13/64"]
    CPDACBUF2_13 = 13,
    #[doc = "14: selected reference voltage * 14/64"]
    CPDACBUF2_14 = 14,
    #[doc = "15: selected reference voltage * 15/64"]
    CPDACBUF2_15 = 15,
    #[doc = "16: selected reference voltage * 16/64"]
    CPDACBUF2_16 = 16,
    #[doc = "17: selected reference voltage * 17/64"]
    CPDACBUF2_17 = 17,
    #[doc = "18: selected reference voltage * 18/64"]
    CPDACBUF2_18 = 18,
    #[doc = "19: selected reference voltage * 19/64"]
    CPDACBUF2_19 = 19,
    #[doc = "20: selected reference voltage * 20/64"]
    CPDACBUF2_20 = 20,
    #[doc = "21: selected reference voltage * 21/64"]
    CPDACBUF2_21 = 21,
    #[doc = "22: selected reference voltage * 22/64"]
    CPDACBUF2_22 = 22,
    #[doc = "23: selected reference voltage * 23/64"]
    CPDACBUF2_23 = 23,
    #[doc = "24: selected reference voltage * 24/64"]
    CPDACBUF2_24 = 24,
    #[doc = "25: selected reference voltage * 25/64"]
    CPDACBUF2_25 = 25,
    #[doc = "26: selected reference voltage * 26/64"]
    CPDACBUF2_26 = 26,
    #[doc = "27: selected reference voltage * 27/64"]
    CPDACBUF2_27 = 27,
    #[doc = "28: selected reference voltage * 28/64"]
    CPDACBUF2_28 = 28,
    #[doc = "29: selected reference voltage * 29/64"]
    CPDACBUF2_29 = 29,
    #[doc = "30: selected reference voltage * 30/64"]
    CPDACBUF2_30 = 30,
    #[doc = "31: selected reference voltage * 31/64"]
    CPDACBUF2_31 = 31,
    #[doc = "32: selected reference voltage * 32/64"]
    CPDACBUF2_32 = 32,
    #[doc = "33: selected reference voltage * 33/64"]
    CPDACBUF2_33 = 33,
    #[doc = "34: selected reference voltage * 34/64"]
    CPDACBUF2_34 = 34,
    #[doc = "35: selected reference voltage * 35/64"]
    CPDACBUF2_35 = 35,
    #[doc = "36: selected reference voltage * 36/64"]
    CPDACBUF2_36 = 36,
    #[doc = "37: selected reference voltage * 37/64"]
    CPDACBUF2_37 = 37,
    #[doc = "38: selected reference voltage * 38/64"]
    CPDACBUF2_38 = 38,
    #[doc = "39: selected reference voltage * 39/64"]
    CPDACBUF2_39 = 39,
    #[doc = "40: selected reference voltage * 40/64"]
    CPDACBUF2_40 = 40,
    #[doc = "41: selected reference voltage * 41/64"]
    CPDACBUF2_41 = 41,
    #[doc = "42: selected reference voltage * 42/64"]
    CPDACBUF2_42 = 42,
    #[doc = "43: selected reference voltage * 43/64"]
    CPDACBUF2_43 = 43,
    #[doc = "44: selected reference voltage * 44/64"]
    CPDACBUF2_44 = 44,
    #[doc = "45: selected reference voltage * 45/64"]
    CPDACBUF2_45 = 45,
    #[doc = "46: selected reference voltage * 46/64"]
    CPDACBUF2_46 = 46,
    #[doc = "47: selected reference voltage * 47/64"]
    CPDACBUF2_47 = 47,
    #[doc = "48: selected reference voltage * 48/64"]
    CPDACBUF2_48 = 48,
    #[doc = "49: selected reference voltage * 49/64"]
    CPDACBUF2_49 = 49,
    #[doc = "50: selected reference voltage * 50/64"]
    CPDACBUF2_50 = 50,
    #[doc = "51: selected reference voltage * 51/64"]
    CPDACBUF2_51 = 51,
    #[doc = "52: selected reference voltage * 52/64"]
    CPDACBUF2_52 = 52,
    #[doc = "53: selected reference voltage * 53/64"]
    CPDACBUF2_53 = 53,
    #[doc = "54: selected reference voltage * 54/64"]
    CPDACBUF2_54 = 54,
    #[doc = "55: selected reference voltage * 55/64"]
    CPDACBUF2_55 = 55,
    #[doc = "56: selected reference voltage * 56/64"]
    CPDACBUF2_56 = 56,
    #[doc = "57: selected reference voltage * 57/64"]
    CPDACBUF2_57 = 57,
    #[doc = "58: selected reference voltage * 58/64"]
    CPDACBUF2_58 = 58,
    #[doc = "59: selected reference voltage * 59/64"]
    CPDACBUF2_59 = 59,
    #[doc = "60: selected reference voltage * 60/64"]
    CPDACBUF2_60 = 60,
    #[doc = "61: selected reference voltage * 61/64"]
    CPDACBUF2_61 = 61,
    #[doc = "62: selected reference voltage * 62/64"]
    CPDACBUF2_62 = 62,
    #[doc = "63: selected reference voltage * 63/64"]
    CPDACBUF2_63 = 63,
}
impl From<CPDACBUF2_A> for u8 {
    #[inline(always)]
    fn from(variant: CPDACBUF2_A) -> Self {
        variant as _
    }
}
impl CPDACBUF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPDACBUF2_A {
        match self.bits {
            0 => CPDACBUF2_A::CPDACBUF2_0,
            1 => CPDACBUF2_A::CPDACBUF2_1,
            2 => CPDACBUF2_A::CPDACBUF2_2,
            3 => CPDACBUF2_A::CPDACBUF2_3,
            4 => CPDACBUF2_A::CPDACBUF2_4,
            5 => CPDACBUF2_A::CPDACBUF2_5,
            6 => CPDACBUF2_A::CPDACBUF2_6,
            7 => CPDACBUF2_A::CPDACBUF2_7,
            8 => CPDACBUF2_A::CPDACBUF2_8,
            9 => CPDACBUF2_A::CPDACBUF2_9,
            10 => CPDACBUF2_A::CPDACBUF2_10,
            11 => CPDACBUF2_A::CPDACBUF2_11,
            12 => CPDACBUF2_A::CPDACBUF2_12,
            13 => CPDACBUF2_A::CPDACBUF2_13,
            14 => CPDACBUF2_A::CPDACBUF2_14,
            15 => CPDACBUF2_A::CPDACBUF2_15,
            16 => CPDACBUF2_A::CPDACBUF2_16,
            17 => CPDACBUF2_A::CPDACBUF2_17,
            18 => CPDACBUF2_A::CPDACBUF2_18,
            19 => CPDACBUF2_A::CPDACBUF2_19,
            20 => CPDACBUF2_A::CPDACBUF2_20,
            21 => CPDACBUF2_A::CPDACBUF2_21,
            22 => CPDACBUF2_A::CPDACBUF2_22,
            23 => CPDACBUF2_A::CPDACBUF2_23,
            24 => CPDACBUF2_A::CPDACBUF2_24,
            25 => CPDACBUF2_A::CPDACBUF2_25,
            26 => CPDACBUF2_A::CPDACBUF2_26,
            27 => CPDACBUF2_A::CPDACBUF2_27,
            28 => CPDACBUF2_A::CPDACBUF2_28,
            29 => CPDACBUF2_A::CPDACBUF2_29,
            30 => CPDACBUF2_A::CPDACBUF2_30,
            31 => CPDACBUF2_A::CPDACBUF2_31,
            32 => CPDACBUF2_A::CPDACBUF2_32,
            33 => CPDACBUF2_A::CPDACBUF2_33,
            34 => CPDACBUF2_A::CPDACBUF2_34,
            35 => CPDACBUF2_A::CPDACBUF2_35,
            36 => CPDACBUF2_A::CPDACBUF2_36,
            37 => CPDACBUF2_A::CPDACBUF2_37,
            38 => CPDACBUF2_A::CPDACBUF2_38,
            39 => CPDACBUF2_A::CPDACBUF2_39,
            40 => CPDACBUF2_A::CPDACBUF2_40,
            41 => CPDACBUF2_A::CPDACBUF2_41,
            42 => CPDACBUF2_A::CPDACBUF2_42,
            43 => CPDACBUF2_A::CPDACBUF2_43,
            44 => CPDACBUF2_A::CPDACBUF2_44,
            45 => CPDACBUF2_A::CPDACBUF2_45,
            46 => CPDACBUF2_A::CPDACBUF2_46,
            47 => CPDACBUF2_A::CPDACBUF2_47,
            48 => CPDACBUF2_A::CPDACBUF2_48,
            49 => CPDACBUF2_A::CPDACBUF2_49,
            50 => CPDACBUF2_A::CPDACBUF2_50,
            51 => CPDACBUF2_A::CPDACBUF2_51,
            52 => CPDACBUF2_A::CPDACBUF2_52,
            53 => CPDACBUF2_A::CPDACBUF2_53,
            54 => CPDACBUF2_A::CPDACBUF2_54,
            55 => CPDACBUF2_A::CPDACBUF2_55,
            56 => CPDACBUF2_A::CPDACBUF2_56,
            57 => CPDACBUF2_A::CPDACBUF2_57,
            58 => CPDACBUF2_A::CPDACBUF2_58,
            59 => CPDACBUF2_A::CPDACBUF2_59,
            60 => CPDACBUF2_A::CPDACBUF2_60,
            61 => CPDACBUF2_A::CPDACBUF2_61,
            62 => CPDACBUF2_A::CPDACBUF2_62,
            63 => CPDACBUF2_A::CPDACBUF2_63,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_0`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_0(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_0
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_1`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_1(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_1
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_2`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_2(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_2
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_3`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_3(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_3
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_4`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_4(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_4
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_5`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_5(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_5
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_6`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_6(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_6
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_7`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_7(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_7
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_8`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_8(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_8
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_9`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_9(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_9
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_10`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_10(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_10
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_11`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_11(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_11
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_12`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_12(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_12
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_13`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_13(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_13
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_14`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_14(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_14
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_15`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_15(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_15
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_16`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_16(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_16
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_17`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_17(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_17
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_18`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_18(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_18
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_19`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_19(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_19
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_20`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_20(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_20
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_21`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_21(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_21
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_22`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_22(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_22
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_23`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_23(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_23
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_24`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_24(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_24
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_25`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_25(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_25
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_26`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_26(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_26
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_27`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_27(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_27
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_28`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_28(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_28
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_29`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_29(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_29
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_30`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_30(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_30
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_31`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_31(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_31
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_32`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_32(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_32
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_33`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_33(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_33
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_34`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_34(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_34
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_35`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_35(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_35
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_36`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_36(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_36
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_37`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_37(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_37
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_38`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_38(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_38
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_39`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_39(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_39
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_40`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_40(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_40
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_41`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_41(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_41
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_42`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_42(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_42
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_43`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_43(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_43
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_44`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_44(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_44
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_45`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_45(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_45
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_46`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_46(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_46
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_47`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_47(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_47
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_48`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_48(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_48
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_49`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_49(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_49
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_50`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_50(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_50
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_51`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_51(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_51
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_52`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_52(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_52
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_53`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_53(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_53
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_54`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_54(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_54
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_55`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_55(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_55
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_56`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_56(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_56
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_57`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_57(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_57
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_58`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_58(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_58
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_59`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_59(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_59
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_60`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_60(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_60
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_61`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_61(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_61
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_62`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_62(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_62
    }
    #[doc = "Checks if the value of the field is `CPDACBUF2_63`"]
    #[inline(always)]
    pub fn is_cpdacbuf2_63(&self) -> bool {
        *self == CPDACBUF2_A::CPDACBUF2_63
    }
}
#[doc = "Field `CPDACBUF2` writer - 2nd 6-bit DAC buffer Data"]
pub type CPDACBUF2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CPDACDATA_SPEC, u8, CPDACBUF2_A, 6, O>;
impl<'a, const O: u8> CPDACBUF2_W<'a, O> {
    #[doc = "0v"]
    #[inline(always)]
    pub fn cpdacbuf2_0(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_0)
    }
    #[doc = "selected reference voltage * 1/64"]
    #[inline(always)]
    pub fn cpdacbuf2_1(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_1)
    }
    #[doc = "selected reference voltage * 2/64"]
    #[inline(always)]
    pub fn cpdacbuf2_2(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_2)
    }
    #[doc = "selected reference voltage * 3/64"]
    #[inline(always)]
    pub fn cpdacbuf2_3(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_3)
    }
    #[doc = "selected reference voltage * 4/64"]
    #[inline(always)]
    pub fn cpdacbuf2_4(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_4)
    }
    #[doc = "selected reference voltage * 5/64"]
    #[inline(always)]
    pub fn cpdacbuf2_5(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_5)
    }
    #[doc = "selected reference voltage * 6/64"]
    #[inline(always)]
    pub fn cpdacbuf2_6(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_6)
    }
    #[doc = "selected reference voltage * 7/64"]
    #[inline(always)]
    pub fn cpdacbuf2_7(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_7)
    }
    #[doc = "selected reference voltage * 8/64"]
    #[inline(always)]
    pub fn cpdacbuf2_8(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_8)
    }
    #[doc = "selected reference voltage * 9/64"]
    #[inline(always)]
    pub fn cpdacbuf2_9(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_9)
    }
    #[doc = "selected reference voltage * 10/64"]
    #[inline(always)]
    pub fn cpdacbuf2_10(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_10)
    }
    #[doc = "selected reference voltage * 11/64"]
    #[inline(always)]
    pub fn cpdacbuf2_11(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_11)
    }
    #[doc = "selected reference voltage * 12/64"]
    #[inline(always)]
    pub fn cpdacbuf2_12(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_12)
    }
    #[doc = "selected reference voltage * 13/64"]
    #[inline(always)]
    pub fn cpdacbuf2_13(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_13)
    }
    #[doc = "selected reference voltage * 14/64"]
    #[inline(always)]
    pub fn cpdacbuf2_14(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_14)
    }
    #[doc = "selected reference voltage * 15/64"]
    #[inline(always)]
    pub fn cpdacbuf2_15(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_15)
    }
    #[doc = "selected reference voltage * 16/64"]
    #[inline(always)]
    pub fn cpdacbuf2_16(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_16)
    }
    #[doc = "selected reference voltage * 17/64"]
    #[inline(always)]
    pub fn cpdacbuf2_17(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_17)
    }
    #[doc = "selected reference voltage * 18/64"]
    #[inline(always)]
    pub fn cpdacbuf2_18(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_18)
    }
    #[doc = "selected reference voltage * 19/64"]
    #[inline(always)]
    pub fn cpdacbuf2_19(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_19)
    }
    #[doc = "selected reference voltage * 20/64"]
    #[inline(always)]
    pub fn cpdacbuf2_20(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_20)
    }
    #[doc = "selected reference voltage * 21/64"]
    #[inline(always)]
    pub fn cpdacbuf2_21(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_21)
    }
    #[doc = "selected reference voltage * 22/64"]
    #[inline(always)]
    pub fn cpdacbuf2_22(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_22)
    }
    #[doc = "selected reference voltage * 23/64"]
    #[inline(always)]
    pub fn cpdacbuf2_23(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_23)
    }
    #[doc = "selected reference voltage * 24/64"]
    #[inline(always)]
    pub fn cpdacbuf2_24(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_24)
    }
    #[doc = "selected reference voltage * 25/64"]
    #[inline(always)]
    pub fn cpdacbuf2_25(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_25)
    }
    #[doc = "selected reference voltage * 26/64"]
    #[inline(always)]
    pub fn cpdacbuf2_26(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_26)
    }
    #[doc = "selected reference voltage * 27/64"]
    #[inline(always)]
    pub fn cpdacbuf2_27(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_27)
    }
    #[doc = "selected reference voltage * 28/64"]
    #[inline(always)]
    pub fn cpdacbuf2_28(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_28)
    }
    #[doc = "selected reference voltage * 29/64"]
    #[inline(always)]
    pub fn cpdacbuf2_29(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_29)
    }
    #[doc = "selected reference voltage * 30/64"]
    #[inline(always)]
    pub fn cpdacbuf2_30(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_30)
    }
    #[doc = "selected reference voltage * 31/64"]
    #[inline(always)]
    pub fn cpdacbuf2_31(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_31)
    }
    #[doc = "selected reference voltage * 32/64"]
    #[inline(always)]
    pub fn cpdacbuf2_32(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_32)
    }
    #[doc = "selected reference voltage * 33/64"]
    #[inline(always)]
    pub fn cpdacbuf2_33(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_33)
    }
    #[doc = "selected reference voltage * 34/64"]
    #[inline(always)]
    pub fn cpdacbuf2_34(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_34)
    }
    #[doc = "selected reference voltage * 35/64"]
    #[inline(always)]
    pub fn cpdacbuf2_35(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_35)
    }
    #[doc = "selected reference voltage * 36/64"]
    #[inline(always)]
    pub fn cpdacbuf2_36(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_36)
    }
    #[doc = "selected reference voltage * 37/64"]
    #[inline(always)]
    pub fn cpdacbuf2_37(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_37)
    }
    #[doc = "selected reference voltage * 38/64"]
    #[inline(always)]
    pub fn cpdacbuf2_38(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_38)
    }
    #[doc = "selected reference voltage * 39/64"]
    #[inline(always)]
    pub fn cpdacbuf2_39(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_39)
    }
    #[doc = "selected reference voltage * 40/64"]
    #[inline(always)]
    pub fn cpdacbuf2_40(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_40)
    }
    #[doc = "selected reference voltage * 41/64"]
    #[inline(always)]
    pub fn cpdacbuf2_41(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_41)
    }
    #[doc = "selected reference voltage * 42/64"]
    #[inline(always)]
    pub fn cpdacbuf2_42(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_42)
    }
    #[doc = "selected reference voltage * 43/64"]
    #[inline(always)]
    pub fn cpdacbuf2_43(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_43)
    }
    #[doc = "selected reference voltage * 44/64"]
    #[inline(always)]
    pub fn cpdacbuf2_44(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_44)
    }
    #[doc = "selected reference voltage * 45/64"]
    #[inline(always)]
    pub fn cpdacbuf2_45(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_45)
    }
    #[doc = "selected reference voltage * 46/64"]
    #[inline(always)]
    pub fn cpdacbuf2_46(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_46)
    }
    #[doc = "selected reference voltage * 47/64"]
    #[inline(always)]
    pub fn cpdacbuf2_47(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_47)
    }
    #[doc = "selected reference voltage * 48/64"]
    #[inline(always)]
    pub fn cpdacbuf2_48(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_48)
    }
    #[doc = "selected reference voltage * 49/64"]
    #[inline(always)]
    pub fn cpdacbuf2_49(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_49)
    }
    #[doc = "selected reference voltage * 50/64"]
    #[inline(always)]
    pub fn cpdacbuf2_50(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_50)
    }
    #[doc = "selected reference voltage * 51/64"]
    #[inline(always)]
    pub fn cpdacbuf2_51(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_51)
    }
    #[doc = "selected reference voltage * 52/64"]
    #[inline(always)]
    pub fn cpdacbuf2_52(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_52)
    }
    #[doc = "selected reference voltage * 53/64"]
    #[inline(always)]
    pub fn cpdacbuf2_53(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_53)
    }
    #[doc = "selected reference voltage * 54/64"]
    #[inline(always)]
    pub fn cpdacbuf2_54(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_54)
    }
    #[doc = "selected reference voltage * 55/64"]
    #[inline(always)]
    pub fn cpdacbuf2_55(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_55)
    }
    #[doc = "selected reference voltage * 56/64"]
    #[inline(always)]
    pub fn cpdacbuf2_56(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_56)
    }
    #[doc = "selected reference voltage * 57/64"]
    #[inline(always)]
    pub fn cpdacbuf2_57(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_57)
    }
    #[doc = "selected reference voltage * 58/64"]
    #[inline(always)]
    pub fn cpdacbuf2_58(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_58)
    }
    #[doc = "selected reference voltage * 59/64"]
    #[inline(always)]
    pub fn cpdacbuf2_59(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_59)
    }
    #[doc = "selected reference voltage * 60/64"]
    #[inline(always)]
    pub fn cpdacbuf2_60(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_60)
    }
    #[doc = "selected reference voltage * 61/64"]
    #[inline(always)]
    pub fn cpdacbuf2_61(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_61)
    }
    #[doc = "selected reference voltage * 62/64"]
    #[inline(always)]
    pub fn cpdacbuf2_62(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_62)
    }
    #[doc = "selected reference voltage * 63/64"]
    #[inline(always)]
    pub fn cpdacbuf2_63(self) -> &'a mut W {
        self.variant(CPDACBUF2_A::CPDACBUF2_63)
    }
}
impl R {
    #[doc = "Bits 0:5 - 1st 6-bit DAC buffer Data"]
    #[inline(always)]
    pub fn cpdacbuf1(&self) -> CPDACBUF1_R {
        CPDACBUF1_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 2nd 6-bit DAC buffer Data"]
    #[inline(always)]
    pub fn cpdacbuf2(&self) -> CPDACBUF2_R {
        CPDACBUF2_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 1st 6-bit DAC buffer Data"]
    #[inline(always)]
    pub fn cpdacbuf1(&mut self) -> CPDACBUF1_W<0> {
        CPDACBUF1_W::new(self)
    }
    #[doc = "Bits 8:13 - 2nd 6-bit DAC buffer Data"]
    #[inline(always)]
    pub fn cpdacbuf2(&mut self) -> CPDACBUF2_W<8> {
        CPDACBUF2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "6-bit Comparator built-in DAC Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpdacdata](index.html) module"]
pub struct CPDACDATA_SPEC;
impl crate::RegisterSpec for CPDACDATA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cpdacdata::R](R) reader structure"]
impl crate::Readable for CPDACDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpdacdata::W](W) writer structure"]
impl crate::Writable for CPDACDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPDACDATA to value 0"]
impl crate::Resettable for CPDACDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
