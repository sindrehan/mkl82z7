#[doc = "Reader of register XACC%s"]
pub type R = crate::R<u8, super::XACC>;
#[doc = "Execute-only access control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XA_A {
    #[doc = "0: Associated segment is accessible in execute mode only (as an instruction fetch)"]
    _0 = 0,
    #[doc = "1: Associated segment is accessible as data or in execute mode"]
    _1 = 1,
}
impl From<XA_A> for u8 {
    #[inline(always)]
    fn from(variant: XA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XA`"]
pub type XA_R = crate::R<u8, XA_A>;
impl XA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, XA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(XA_A::_0),
            1 => Val(XA_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == XA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == XA_A::_1
    }
}
impl R {
    #[doc = "Bits 0:7 - Execute-only access control"]
    #[inline(always)]
    pub fn xa(&self) -> XA_R {
        XA_R::new((self.bits & 0xff) as u8)
    }
}
