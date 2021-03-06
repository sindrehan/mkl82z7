#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Error Status Register"]
    pub es: ES,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Enable Request Register"]
    pub erq: ERQ,
    _reserved3: [u8; 4usize],
    #[doc = "0x14 - Enable Error Interrupt Register"]
    pub eei: EEI,
    #[doc = "0x18 - Clear Enable Error Interrupt Register"]
    pub ceei: CEEI,
    #[doc = "0x19 - Set Enable Error Interrupt Register"]
    pub seei: SEEI,
    #[doc = "0x1a - Clear Enable Request Register"]
    pub cerq: CERQ,
    #[doc = "0x1b - Set Enable Request Register"]
    pub serq: SERQ,
    #[doc = "0x1c - Clear DONE Status Bit Register"]
    pub cdne: CDNE,
    #[doc = "0x1d - Set START Bit Register"]
    pub ssrt: SSRT,
    #[doc = "0x1e - Clear Error Register"]
    pub cerr: CERR,
    #[doc = "0x1f - Clear Interrupt Request Register"]
    pub cint: CINT,
    _reserved12: [u8; 4usize],
    #[doc = "0x24 - Interrupt Request Register"]
    pub int: INT,
    _reserved13: [u8; 4usize],
    #[doc = "0x2c - Error Register"]
    pub err: ERR,
    _reserved14: [u8; 4usize],
    #[doc = "0x34 - Hardware Request Status Register"]
    pub hrs: HRS,
    _reserved15: [u8; 12usize],
    #[doc = "0x44 - Enable Asynchronous Request in Stop Register"]
    pub ears: EARS,
    _reserved16: [u8; 184usize],
    #[doc = "0x100 - Channel n Priority Register"]
    pub dchpri3: DCHPRI,
    #[doc = "0x101 - Channel n Priority Register"]
    pub dchpri2: DCHPRI,
    #[doc = "0x102 - Channel n Priority Register"]
    pub dchpri1: DCHPRI,
    #[doc = "0x103 - Channel n Priority Register"]
    pub dchpri0: DCHPRI,
    #[doc = "0x104 - Channel n Priority Register"]
    pub dchpri7: DCHPRI,
    #[doc = "0x105 - Channel n Priority Register"]
    pub dchpri6: DCHPRI,
    #[doc = "0x106 - Channel n Priority Register"]
    pub dchpri5: DCHPRI,
    #[doc = "0x107 - Channel n Priority Register"]
    pub dchpri4: DCHPRI,
    _reserved24: [u8; 3832usize],
    #[doc = "0x1000 - TCD Source Address"]
    pub tcd0_saddr: TCD_SADDR,
    #[doc = "0x1004 - TCD Signed Source Address Offset"]
    pub tcd0_soff: TCD_SOFF,
    #[doc = "0x1006 - TCD Transfer Attributes"]
    pub tcd0_attr: TCD_ATTR,
    _reserved_27_tcd0_nbytes: [u8; 4usize],
    #[doc = "0x100c - TCD Last Source Address Adjustment"]
    pub tcd0_slast: TCD_SLAST,
    #[doc = "0x1010 - TCD Destination Address"]
    pub tcd0_daddr: TCD_DADDR,
    #[doc = "0x1014 - TCD Signed Destination Address Offset"]
    pub tcd0_doff: TCD_DOFF,
    _reserved_31_tcd0_citer: [u8; 2usize],
    #[doc = "0x1018 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd0_dlastsga: TCD_DLASTSGA,
    #[doc = "0x101c - TCD Control and Status"]
    pub tcd0_csr: TCD_CSR,
    _reserved_34_tcd0_biter: [u8; 2usize],
    #[doc = "0x1020 - TCD Source Address"]
    pub tcd1_saddr: TCD_SADDR,
    #[doc = "0x1024 - TCD Signed Source Address Offset"]
    pub tcd1_soff: TCD_SOFF,
    #[doc = "0x1026 - TCD Transfer Attributes"]
    pub tcd1_attr: TCD_ATTR,
    _reserved_38_tcd1_nbytes: [u8; 4usize],
    #[doc = "0x102c - TCD Last Source Address Adjustment"]
    pub tcd1_slast: TCD_SLAST,
    #[doc = "0x1030 - TCD Destination Address"]
    pub tcd1_daddr: TCD_DADDR,
    #[doc = "0x1034 - TCD Signed Destination Address Offset"]
    pub tcd1_doff: TCD_DOFF,
    _reserved_42_tcd1_citer: [u8; 2usize],
    #[doc = "0x1038 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd1_dlastsga: TCD_DLASTSGA,
    #[doc = "0x103c - TCD Control and Status"]
    pub tcd1_csr: TCD_CSR,
    _reserved_45_tcd1_biter: [u8; 2usize],
    #[doc = "0x1040 - TCD Source Address"]
    pub tcd2_saddr: TCD_SADDR,
    #[doc = "0x1044 - TCD Signed Source Address Offset"]
    pub tcd2_soff: TCD_SOFF,
    #[doc = "0x1046 - TCD Transfer Attributes"]
    pub tcd2_attr: TCD_ATTR,
    _reserved_49_tcd2_nbytes: [u8; 4usize],
    #[doc = "0x104c - TCD Last Source Address Adjustment"]
    pub tcd2_slast: TCD_SLAST,
    #[doc = "0x1050 - TCD Destination Address"]
    pub tcd2_daddr: TCD_DADDR,
    #[doc = "0x1054 - TCD Signed Destination Address Offset"]
    pub tcd2_doff: TCD_DOFF,
    _reserved_53_tcd2_citer: [u8; 2usize],
    #[doc = "0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd2_dlastsga: TCD_DLASTSGA,
    #[doc = "0x105c - TCD Control and Status"]
    pub tcd2_csr: TCD_CSR,
    _reserved_56_tcd2_biter: [u8; 2usize],
    #[doc = "0x1060 - TCD Source Address"]
    pub tcd3_saddr: TCD_SADDR,
    #[doc = "0x1064 - TCD Signed Source Address Offset"]
    pub tcd3_soff: TCD_SOFF,
    #[doc = "0x1066 - TCD Transfer Attributes"]
    pub tcd3_attr: TCD_ATTR,
    _reserved_60_tcd3_nbytes: [u8; 4usize],
    #[doc = "0x106c - TCD Last Source Address Adjustment"]
    pub tcd3_slast: TCD_SLAST,
    #[doc = "0x1070 - TCD Destination Address"]
    pub tcd3_daddr: TCD_DADDR,
    #[doc = "0x1074 - TCD Signed Destination Address Offset"]
    pub tcd3_doff: TCD_DOFF,
    _reserved_64_tcd3_citer: [u8; 2usize],
    #[doc = "0x1078 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd3_dlastsga: TCD_DLASTSGA,
    #[doc = "0x107c - TCD Control and Status"]
    pub tcd3_csr: TCD_CSR,
    _reserved_67_tcd3_biter: [u8; 2usize],
    #[doc = "0x1080 - TCD Source Address"]
    pub tcd4_saddr: TCD_SADDR,
    #[doc = "0x1084 - TCD Signed Source Address Offset"]
    pub tcd4_soff: TCD_SOFF,
    #[doc = "0x1086 - TCD Transfer Attributes"]
    pub tcd4_attr: TCD_ATTR,
    _reserved_71_tcd4_nbytes: [u8; 4usize],
    #[doc = "0x108c - TCD Last Source Address Adjustment"]
    pub tcd4_slast: TCD_SLAST,
    #[doc = "0x1090 - TCD Destination Address"]
    pub tcd4_daddr: TCD_DADDR,
    #[doc = "0x1094 - TCD Signed Destination Address Offset"]
    pub tcd4_doff: TCD_DOFF,
    _reserved_75_tcd4_citer: [u8; 2usize],
    #[doc = "0x1098 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd4_dlastsga: TCD_DLASTSGA,
    #[doc = "0x109c - TCD Control and Status"]
    pub tcd4_csr: TCD_CSR,
    _reserved_78_tcd4_biter: [u8; 2usize],
    #[doc = "0x10a0 - TCD Source Address"]
    pub tcd5_saddr: TCD_SADDR,
    #[doc = "0x10a4 - TCD Signed Source Address Offset"]
    pub tcd5_soff: TCD_SOFF,
    #[doc = "0x10a6 - TCD Transfer Attributes"]
    pub tcd5_attr: TCD_ATTR,
    _reserved_82_tcd5_nbytes: [u8; 4usize],
    #[doc = "0x10ac - TCD Last Source Address Adjustment"]
    pub tcd5_slast: TCD_SLAST,
    #[doc = "0x10b0 - TCD Destination Address"]
    pub tcd5_daddr: TCD_DADDR,
    #[doc = "0x10b4 - TCD Signed Destination Address Offset"]
    pub tcd5_doff: TCD_DOFF,
    _reserved_86_tcd5_citer: [u8; 2usize],
    #[doc = "0x10b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd5_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10bc - TCD Control and Status"]
    pub tcd5_csr: TCD_CSR,
    _reserved_89_tcd5_biter: [u8; 2usize],
    #[doc = "0x10c0 - TCD Source Address"]
    pub tcd6_saddr: TCD_SADDR,
    #[doc = "0x10c4 - TCD Signed Source Address Offset"]
    pub tcd6_soff: TCD_SOFF,
    #[doc = "0x10c6 - TCD Transfer Attributes"]
    pub tcd6_attr: TCD_ATTR,
    _reserved_93_tcd6_nbytes: [u8; 4usize],
    #[doc = "0x10cc - TCD Last Source Address Adjustment"]
    pub tcd6_slast: TCD_SLAST,
    #[doc = "0x10d0 - TCD Destination Address"]
    pub tcd6_daddr: TCD_DADDR,
    #[doc = "0x10d4 - TCD Signed Destination Address Offset"]
    pub tcd6_doff: TCD_DOFF,
    _reserved_97_tcd6_citer: [u8; 2usize],
    #[doc = "0x10d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd6_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10dc - TCD Control and Status"]
    pub tcd6_csr: TCD_CSR,
    _reserved_100_tcd6_biter: [u8; 2usize],
    #[doc = "0x10e0 - TCD Source Address"]
    pub tcd7_saddr: TCD_SADDR,
    #[doc = "0x10e4 - TCD Signed Source Address Offset"]
    pub tcd7_soff: TCD_SOFF,
    #[doc = "0x10e6 - TCD Transfer Attributes"]
    pub tcd7_attr: TCD_ATTR,
    _reserved_104_tcd7_nbytes: [u8; 4usize],
    #[doc = "0x10ec - TCD Last Source Address Adjustment"]
    pub tcd7_slast: TCD_SLAST,
    #[doc = "0x10f0 - TCD Destination Address"]
    pub tcd7_daddr: TCD_DADDR,
    #[doc = "0x10f4 - TCD Signed Destination Address Offset"]
    pub tcd7_doff: TCD_DOFF,
    _reserved_108_tcd7_citer: [u8; 2usize],
    #[doc = "0x10f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd7_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10fc - TCD Control and Status"]
    pub tcd7_csr: TCD_CSR,
    _reserved_111_tcd7_biter: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4104usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4104usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4104usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4104usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4104usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4104usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4118usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4118usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4118usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4118usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4126usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4126usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4126usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4126usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4136usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4136usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4136usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4136usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4136usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4136usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4150usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4150usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4150usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4150usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4158usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4158usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4158usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4158usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4168usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4168usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4168usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4168usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4168usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4168usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4182usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4182usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4182usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4182usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4190usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4190usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4190usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4190usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4200usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4200usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4200usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4200usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4200usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4200usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4214usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4214usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4214usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4214usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4222usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4222usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4222usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4222usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4232usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4232usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4232usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4232usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x1088 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4232usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1088 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4232usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4246usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4246usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4246usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4246usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4254usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4254usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4254usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4254usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4264usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4264usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4264usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4264usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x10a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4264usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4264usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4278usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4278usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4278usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4278usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4286usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4286usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4286usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4286usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4296usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4296usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4296usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4296usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x10c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4296usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4296usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4310usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4310usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4310usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4310usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4318usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4318usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4318usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4318usize) as *mut TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4328usize) as *const TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4328usize) as *mut TCD_NBYTES_MLOFFYES)
        }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4328usize) as *const TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4328usize) as *mut TCD_NBYTES_MLOFFNO)
        }
    }
    #[doc = "0x10e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4328usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4328usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4342usize) as *const TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4342usize) as *mut TCD_CITER_ELINKYES)
        }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4342usize) as *const TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4342usize) as *mut TCD_CITER_ELINKNO)
        }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4350usize) as *const TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4350usize) as *mut TCD_BITER_ELINKYES)
        }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4350usize) as *const TCD_BITER_ELINKNO)
        }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(4350usize) as *mut TCD_BITER_ELINKNO)
        }
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [es](es) module"]
pub type ES = crate::Reg<u32, _ES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ES;
#[doc = "`read()` method returns [es::R](es::R) reader structure"]
impl crate::Readable for ES {}
#[doc = "Error Status Register"]
pub mod es;
#[doc = "Enable Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erq](erq) module"]
pub type ERQ = crate::Reg<u32, _ERQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERQ;
#[doc = "`read()` method returns [erq::R](erq::R) reader structure"]
impl crate::Readable for ERQ {}
#[doc = "`write(|w| ..)` method takes [erq::W](erq::W) writer structure"]
impl crate::Writable for ERQ {}
#[doc = "Enable Request Register"]
pub mod erq;
#[doc = "Enable Error Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eei](eei) module"]
pub type EEI = crate::Reg<u32, _EEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEI;
#[doc = "`read()` method returns [eei::R](eei::R) reader structure"]
impl crate::Readable for EEI {}
#[doc = "`write(|w| ..)` method takes [eei::W](eei::W) writer structure"]
impl crate::Writable for EEI {}
#[doc = "Enable Error Interrupt Register"]
pub mod eei;
#[doc = "Clear Enable Error Interrupt Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ceei](ceei) module"]
pub type CEEI = crate::Reg<u8, _CEEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEEI;
#[doc = "`write(|w| ..)` method takes [ceei::W](ceei::W) writer structure"]
impl crate::Writable for CEEI {}
#[doc = "Clear Enable Error Interrupt Register"]
pub mod ceei;
#[doc = "Set Enable Error Interrupt Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seei](seei) module"]
pub type SEEI = crate::Reg<u8, _SEEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEEI;
#[doc = "`write(|w| ..)` method takes [seei::W](seei::W) writer structure"]
impl crate::Writable for SEEI {}
#[doc = "Set Enable Error Interrupt Register"]
pub mod seei;
#[doc = "Clear Enable Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cerq](cerq) module"]
pub type CERQ = crate::Reg<u8, _CERQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CERQ;
#[doc = "`write(|w| ..)` method takes [cerq::W](cerq::W) writer structure"]
impl crate::Writable for CERQ {}
#[doc = "Clear Enable Request Register"]
pub mod cerq;
#[doc = "Set Enable Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [serq](serq) module"]
pub type SERQ = crate::Reg<u8, _SERQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SERQ;
#[doc = "`write(|w| ..)` method takes [serq::W](serq::W) writer structure"]
impl crate::Writable for SERQ {}
#[doc = "Set Enable Request Register"]
pub mod serq;
#[doc = "Clear DONE Status Bit Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdne](cdne) module"]
pub type CDNE = crate::Reg<u8, _CDNE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDNE;
#[doc = "`write(|w| ..)` method takes [cdne::W](cdne::W) writer structure"]
impl crate::Writable for CDNE {}
#[doc = "Clear DONE Status Bit Register"]
pub mod cdne;
#[doc = "Set START Bit Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssrt](ssrt) module"]
pub type SSRT = crate::Reg<u8, _SSRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSRT;
#[doc = "`write(|w| ..)` method takes [ssrt::W](ssrt::W) writer structure"]
impl crate::Writable for SSRT {}
#[doc = "Set START Bit Register"]
pub mod ssrt;
#[doc = "Clear Error Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cerr](cerr) module"]
pub type CERR = crate::Reg<u8, _CERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CERR;
#[doc = "`write(|w| ..)` method takes [cerr::W](cerr::W) writer structure"]
impl crate::Writable for CERR {}
#[doc = "Clear Error Register"]
pub mod cerr;
#[doc = "Clear Interrupt Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cint](cint) module"]
pub type CINT = crate::Reg<u8, _CINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CINT;
#[doc = "`write(|w| ..)` method takes [cint::W](cint::W) writer structure"]
impl crate::Writable for CINT {}
#[doc = "Clear Interrupt Request Register"]
pub mod cint;
#[doc = "Interrupt Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int](int) module"]
pub type INT = crate::Reg<u32, _INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT;
#[doc = "`read()` method returns [int::R](int::R) reader structure"]
impl crate::Readable for INT {}
#[doc = "`write(|w| ..)` method takes [int::W](int::W) writer structure"]
impl crate::Writable for INT {}
#[doc = "Interrupt Request Register"]
pub mod int;
#[doc = "Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err](err) module"]
pub type ERR = crate::Reg<u32, _ERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERR;
#[doc = "`read()` method returns [err::R](err::R) reader structure"]
impl crate::Readable for ERR {}
#[doc = "`write(|w| ..)` method takes [err::W](err::W) writer structure"]
impl crate::Writable for ERR {}
#[doc = "Error Register"]
pub mod err;
#[doc = "Hardware Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrs](hrs) module"]
pub type HRS = crate::Reg<u32, _HRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRS;
#[doc = "`read()` method returns [hrs::R](hrs::R) reader structure"]
impl crate::Readable for HRS {}
#[doc = "Hardware Request Status Register"]
pub mod hrs;
#[doc = "Enable Asynchronous Request in Stop Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ears](ears) module"]
pub type EARS = crate::Reg<u32, _EARS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EARS;
#[doc = "`read()` method returns [ears::R](ears::R) reader structure"]
impl crate::Readable for EARS {}
#[doc = "`write(|w| ..)` method takes [ears::W](ears::W) writer structure"]
impl crate::Writable for EARS {}
#[doc = "Enable Asynchronous Request in Stop Register"]
pub mod ears;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri](dchpri) module"]
pub type DCHPRI = crate::Reg<u8, _DCHPRI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI;
#[doc = "`read()` method returns [dchpri::R](dchpri::R) reader structure"]
impl crate::Readable for DCHPRI {}
#[doc = "`write(|w| ..)` method takes [dchpri::W](dchpri::W) writer structure"]
impl crate::Writable for DCHPRI {}
#[doc = "Channel n Priority Register"]
pub mod dchpri;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_saddr](tcd_saddr) module"]
pub type TCD_SADDR = crate::Reg<u32, _TCD_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_SADDR;
#[doc = "`read()` method returns [tcd_saddr::R](tcd_saddr::R) reader structure"]
impl crate::Readable for TCD_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd_saddr::W](tcd_saddr::W) writer structure"]
impl crate::Writable for TCD_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_soff](tcd_soff) module"]
pub type TCD_SOFF = crate::Reg<u16, _TCD_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_SOFF;
#[doc = "`read()` method returns [tcd_soff::R](tcd_soff::R) reader structure"]
impl crate::Readable for TCD_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd_soff::W](tcd_soff::W) writer structure"]
impl crate::Writable for TCD_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_attr](tcd_attr) module"]
pub type TCD_ATTR = crate::Reg<u16, _TCD_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_ATTR;
#[doc = "`read()` method returns [tcd_attr::R](tcd_attr::R) reader structure"]
impl crate::Readable for TCD_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd_attr::W](tcd_attr::W) writer structure"]
impl crate::Writable for TCD_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_nbytes_mlno](tcd_nbytes_mlno) module"]
pub type TCD_NBYTES_MLNO = crate::Reg<u32, _TCD_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd_nbytes_mlno::R](tcd_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd_nbytes_mlno::W](tcd_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_nbytes_mloffno](tcd_nbytes_mloffno) module"]
pub type TCD_NBYTES_MLOFFNO = crate::Reg<u32, _TCD_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd_nbytes_mloffno::R](tcd_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd_nbytes_mloffno::W](tcd_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_nbytes_mloffyes](tcd_nbytes_mloffyes) module"]
pub type TCD_NBYTES_MLOFFYES = crate::Reg<u32, _TCD_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd_nbytes_mloffyes::R](tcd_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd_nbytes_mloffyes::W](tcd_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_slast](tcd_slast) module"]
pub type TCD_SLAST = crate::Reg<u32, _TCD_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_SLAST;
#[doc = "`read()` method returns [tcd_slast::R](tcd_slast::R) reader structure"]
impl crate::Readable for TCD_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd_slast::W](tcd_slast::W) writer structure"]
impl crate::Writable for TCD_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_daddr](tcd_daddr) module"]
pub type TCD_DADDR = crate::Reg<u32, _TCD_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_DADDR;
#[doc = "`read()` method returns [tcd_daddr::R](tcd_daddr::R) reader structure"]
impl crate::Readable for TCD_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd_daddr::W](tcd_daddr::W) writer structure"]
impl crate::Writable for TCD_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_doff](tcd_doff) module"]
pub type TCD_DOFF = crate::Reg<u16, _TCD_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_DOFF;
#[doc = "`read()` method returns [tcd_doff::R](tcd_doff::R) reader structure"]
impl crate::Readable for TCD_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd_doff::W](tcd_doff::W) writer structure"]
impl crate::Writable for TCD_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_citer_elinkno](tcd_citer_elinkno) module"]
pub type TCD_CITER_ELINKNO = crate::Reg<u16, _TCD_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd_citer_elinkno::R](tcd_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd_citer_elinkno::W](tcd_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_citer_elinkyes](tcd_citer_elinkyes) module"]
pub type TCD_CITER_ELINKYES = crate::Reg<u16, _TCD_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd_citer_elinkyes::R](tcd_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd_citer_elinkyes::W](tcd_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_dlastsga](tcd_dlastsga) module"]
pub type TCD_DLASTSGA = crate::Reg<u32, _TCD_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_DLASTSGA;
#[doc = "`read()` method returns [tcd_dlastsga::R](tcd_dlastsga::R) reader structure"]
impl crate::Readable for TCD_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd_dlastsga::W](tcd_dlastsga::W) writer structure"]
impl crate::Writable for TCD_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_csr](tcd_csr) module"]
pub type TCD_CSR = crate::Reg<u16, _TCD_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_CSR;
#[doc = "`read()` method returns [tcd_csr::R](tcd_csr::R) reader structure"]
impl crate::Readable for TCD_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd_csr::W](tcd_csr::W) writer structure"]
impl crate::Writable for TCD_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_biter_elinkno](tcd_biter_elinkno) module"]
pub type TCD_BITER_ELINKNO = crate::Reg<u16, _TCD_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd_biter_elinkno::R](tcd_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd_biter_elinkno::W](tcd_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_biter_elinkyes](tcd_biter_elinkyes) module"]
pub type TCD_BITER_ELINKYES = crate::Reg<u16, _TCD_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd_biter_elinkyes::R](tcd_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd_biter_elinkyes::W](tcd_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elinkyes;
