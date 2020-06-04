#[doc = "Reader of register CH%s_IER_31_0"]
pub type R = crate::R<u32, super::CH_IER_31_0>;
#[doc = "Writer for register CH%s_IER_31_0"]
pub type W = crate::W<u32, super::CH_IER_31_0>;
#[doc = "Register CH%s_IER_31_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CH_IER_31_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum INTE_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<INTE_A> for u32 {
    #[inline(always)]
    fn from(variant: INTE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INTE`"]
pub type INTE_R = crate::R<u32, INTE_A>;
impl INTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, INTE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INTE_A::_0),
            1 => Val(INTE_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTE_A::_1
    }
}
#[doc = "Write proxy for field `INTE`"]
pub struct INTE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTE_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt Enable"]
    #[inline(always)]
    pub fn inte(&self) -> INTE_R {
        INTE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Enable"]
    #[inline(always)]
    pub fn inte(&mut self) -> INTE_W {
        INTE_W { w: self }
    }
}
