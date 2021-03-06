#[doc = "Writer for register SERQ"]
pub type W = crate::W<u8, super::SERQ>;
#[doc = "Register SERQ `reset()`'s with value 0"]
impl crate::ResetValue for super::SERQ {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SERQ`"]
pub struct SERQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SERQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "Set All Enable Requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAER_AW {
    #[doc = "0: Set only the ERQ bit specified in the SERQ field"]
    _0 = 0,
    #[doc = "1: Set all bits in ERQ"]
    _1 = 1,
}
impl From<SAER_AW> for bool {
    #[inline(always)]
    fn from(variant: SAER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SAER`"]
pub struct SAER_W<'a> {
    w: &'a mut W,
}
impl<'a> SAER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAER_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set only the ERQ bit specified in the SERQ field"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAER_AW::_0)
    }
    #[doc = "Set all bits in ERQ"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAER_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "No Op enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOP_AW {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: No operation, ignore the other bits in this register"]
    _1 = 1,
}
impl From<NOP_AW> for bool {
    #[inline(always)]
    fn from(variant: NOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `NOP`"]
pub struct NOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NOP_AW::_0)
    }
    #[doc = "No operation, ignore the other bits in this register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NOP_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:2 - Set Enable Request"]
    #[inline(always)]
    pub fn serq(&mut self) -> SERQ_W {
        SERQ_W { w: self }
    }
    #[doc = "Bit 6 - Set All Enable Requests"]
    #[inline(always)]
    pub fn saer(&mut self) -> SAER_W {
        SAER_W { w: self }
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&mut self) -> NOP_W {
        NOP_W { w: self }
    }
}
