#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Options Register 1"]
    pub sopt1: SOPT1,
    _reserved1: [u8; 4096usize],
    #[doc = "0x1004 - System Options Register 2"]
    pub sopt2: SOPT2,
    _reserved2: [u8; 8usize],
    #[doc = "0x1010 - System Options Register 5"]
    pub sopt5: SOPT5,
    _reserved3: [u8; 4usize],
    #[doc = "0x1018 - System Options Register 7"]
    pub sopt7: SOPT7,
    _reserved4: [u8; 4usize],
    #[doc = "0x1020 - System Options Register 9"]
    pub sopt9: SOPT9,
    #[doc = "0x1024 - System Device Identification Register"]
    pub sdid: SDID,
    _reserved6: [u8; 12usize],
    #[doc = "0x1034 - System Clock Gating Control Register 4"]
    pub scgc4: SCGC4,
    #[doc = "0x1038 - System Clock Gating Control Register 5"]
    pub scgc5: SCGC5,
    #[doc = "0x103c - System Clock Gating Control Register 6"]
    pub scgc6: SCGC6,
    #[doc = "0x1040 - System Clock Gating Control Register 7"]
    pub scgc7: SCGC7,
    #[doc = "0x1044 - System Clock Divider Register 1"]
    pub clkdiv1: CLKDIV1,
    #[doc = "0x1048 - System Clock Divider Register 2"]
    pub clkdiv2: CLKDIV2,
    #[doc = "0x104c - Flash Configuration Register 1"]
    pub fcfg1: FCFG1,
    #[doc = "0x1050 - Flash Configuration Register 2"]
    pub fcfg2: FCFG2,
    #[doc = "0x1054 - Unique Identification Register High"]
    pub uidh: UIDH,
    #[doc = "0x1058 - Unique Identification Register Mid-High"]
    pub uidmh: UIDMH,
    #[doc = "0x105c - Unique Identification Register Mid Low"]
    pub uidml: UIDML,
    #[doc = "0x1060 - Unique Identification Register Low"]
    pub uidl: UIDL,
    #[doc = "0x1064 - System Clock Divider Register 3"]
    pub clkdiv3: CLKDIV3,
    _reserved19: [u8; 4usize],
    #[doc = "0x106c - Misc Control Register"]
    pub miscctrl: MISCCTRL,
    _reserved20: [u8; 32usize],
    #[doc = "0x1090 - Secure Key Register 0"]
    pub seckey0: SECKEY0,
    #[doc = "0x1094 - Secure Key Register 1"]
    pub seckey1: SECKEY1,
    #[doc = "0x1098 - Secure Key Register 2"]
    pub seckey2: SECKEY2,
    #[doc = "0x109c - Secure Key Register 3"]
    pub seckey3: SECKEY3,
}
#[doc = "System Options Register 1"]
pub struct SOPT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Options Register 1"]
pub mod sopt1;
#[doc = "System Options Register 2"]
pub struct SOPT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Options Register 2"]
pub mod sopt2;
#[doc = "System Options Register 5"]
pub struct SOPT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Options Register 5"]
pub mod sopt5;
#[doc = "System Options Register 7"]
pub struct SOPT7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Options Register 7"]
pub mod sopt7;
#[doc = "System Options Register 9"]
pub struct SOPT9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Options Register 9"]
pub mod sopt9;
#[doc = "System Device Identification Register"]
pub struct SDID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Device Identification Register"]
pub mod sdid;
#[doc = "System Clock Gating Control Register 4"]
pub struct SCGC4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Clock Gating Control Register 4"]
pub mod scgc4;
#[doc = "System Clock Gating Control Register 5"]
pub struct SCGC5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Clock Gating Control Register 5"]
pub mod scgc5;
#[doc = "System Clock Gating Control Register 6"]
pub struct SCGC6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Clock Gating Control Register 6"]
pub mod scgc6;
#[doc = "System Clock Gating Control Register 7"]
pub struct SCGC7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Clock Gating Control Register 7"]
pub mod scgc7;
#[doc = "System Clock Divider Register 1"]
pub struct CLKDIV1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Clock Divider Register 1"]
pub mod clkdiv1;
#[doc = "System Clock Divider Register 2"]
pub struct CLKDIV2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Clock Divider Register 2"]
pub mod clkdiv2;
#[doc = "Flash Configuration Register 1"]
pub struct FCFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Configuration Register 1"]
pub mod fcfg1;
#[doc = "Flash Configuration Register 2"]
pub struct FCFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Configuration Register 2"]
pub mod fcfg2;
#[doc = "Unique Identification Register High"]
pub struct UIDH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unique Identification Register High"]
pub mod uidh;
#[doc = "Unique Identification Register Mid-High"]
pub struct UIDMH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unique Identification Register Mid-High"]
pub mod uidmh;
#[doc = "Unique Identification Register Mid Low"]
pub struct UIDML {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unique Identification Register Mid Low"]
pub mod uidml;
#[doc = "Unique Identification Register Low"]
pub struct UIDL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unique Identification Register Low"]
pub mod uidl;
#[doc = "System Clock Divider Register 3"]
pub struct CLKDIV3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Clock Divider Register 3"]
pub mod clkdiv3;
#[doc = "Misc Control Register"]
pub struct MISCCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Misc Control Register"]
pub mod miscctrl;
#[doc = "Secure Key Register 0"]
pub struct SECKEY0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure Key Register 0"]
pub mod seckey0;
#[doc = "Secure Key Register 1"]
pub struct SECKEY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure Key Register 1"]
pub mod seckey1;
#[doc = "Secure Key Register 2"]
pub struct SECKEY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure Key Register 2"]
pub mod seckey2;
#[doc = "Secure Key Register 3"]
pub struct SECKEY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure Key Register 3"]
pub mod seckey3;
