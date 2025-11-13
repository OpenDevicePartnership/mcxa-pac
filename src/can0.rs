#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcr: Mcr,
    ctrl1: Ctrl1,
    timer: Timer,
    _reserved3: [u8; 0x04],
    rxmgmask: Rxmgmask,
    rx14mask: Rx14mask,
    rx15mask: Rx15mask,
    ecr: Ecr,
    esr1: Esr1,
    _reserved8: [u8; 0x04],
    imask1: Imask1,
    _reserved9: [u8; 0x04],
    iflag1: Iflag1,
    ctrl2: Ctrl2,
    esr2: Esr2,
    _reserved12: [u8; 0x08],
    crcr: Crcr,
    rxfgmask: Rxfgmask,
    rxfir: Rxfir,
    cbt: Cbt,
    _reserved16: [u8; 0x2c],
    _reserved_16_cs0: [u8; 0x04],
    _reserved_17_id0: [u8; 0x04],
    _reserved_18_word00: [u8; 0x04],
    _reserved_19_word10: [u8; 0x04],
    _reserved_20_cs1: [u8; 0x04],
    _reserved_21_id1: [u8; 0x04],
    _reserved_22_word01: [u8; 0x04],
    _reserved_23_word11: [u8; 0x04],
    _reserved_24_cs2: [u8; 0x04],
    _reserved_25_id2: [u8; 0x04],
    _reserved_26_word02: [u8; 0x04],
    _reserved_27_word12: [u8; 0x04],
    _reserved_28_cs3: [u8; 0x04],
    _reserved_29_id3: [u8; 0x04],
    _reserved_30_word03: [u8; 0x04],
    _reserved_31_word13: [u8; 0x04],
    _reserved_32_cs4: [u8; 0x04],
    _reserved_33_id4: [u8; 0x04],
    _reserved_34_word04: [u8; 0x04],
    _reserved_35_word14: [u8; 0x04],
    _reserved_36_cs5: [u8; 0x04],
    _reserved_37_id5: [u8; 0x04],
    _reserved_38_word05: [u8; 0x04],
    _reserved_39_word15: [u8; 0x04],
    _reserved_40_cs6: [u8; 0x04],
    _reserved_41_id6: [u8; 0x04],
    _reserved_42_word06: [u8; 0x04],
    _reserved_43_word16: [u8; 0x04],
    _reserved_44_cs7: [u8; 0x04],
    _reserved_45_id7: [u8; 0x04],
    _reserved_46_word07: [u8; 0x04],
    _reserved_47_word17: [u8; 0x04],
    _reserved_48_cs8: [u8; 0x04],
    _reserved_49_id8: [u8; 0x04],
    _reserved_50_word08: [u8; 0x04],
    _reserved_51_word18: [u8; 0x04],
    _reserved_52_cs9: [u8; 0x04],
    _reserved_53_id9: [u8; 0x04],
    _reserved_54_word09: [u8; 0x04],
    _reserved_55_word19: [u8; 0x04],
    _reserved_56_cs10: [u8; 0x04],
    _reserved_57_id10: [u8; 0x04],
    _reserved_58_word010: [u8; 0x04],
    _reserved_59_word110: [u8; 0x04],
    _reserved_60_cs11: [u8; 0x04],
    _reserved_61_id11: [u8; 0x04],
    _reserved_62_word011: [u8; 0x04],
    _reserved_63_word111: [u8; 0x04],
    _reserved_64_cs12: [u8; 0x04],
    _reserved_65_id12: [u8; 0x04],
    _reserved_66_word012: [u8; 0x04],
    _reserved_67_word112: [u8; 0x04],
    _reserved_68_cs13: [u8; 0x04],
    _reserved_69_id13: [u8; 0x04],
    _reserved_70_word013: [u8; 0x04],
    _reserved_71_word113: [u8; 0x04],
    _reserved_72_cs14: [u8; 0x04],
    _reserved_73_id14: [u8; 0x04],
    _reserved_74_word014: [u8; 0x04],
    _reserved_75_word114: [u8; 0x04],
    _reserved_76_cs15: [u8; 0x04],
    _reserved_77_id15: [u8; 0x04],
    _reserved_78_word015: [u8; 0x04],
    _reserved_79_word115: [u8; 0x04],
    _reserved_80_cs16: [u8; 0x04],
    _reserved_81_id16: [u8; 0x04],
    _reserved_82_word016: [u8; 0x04],
    _reserved_83_word116: [u8; 0x04],
    _reserved_84_cs17: [u8; 0x04],
    _reserved_85_id17: [u8; 0x04],
    _reserved_86_word017: [u8; 0x04],
    _reserved_87_word117: [u8; 0x04],
    _reserved_88_cs18: [u8; 0x04],
    _reserved_89_id18: [u8; 0x04],
    _reserved_90_word018: [u8; 0x04],
    _reserved_91_word118: [u8; 0x04],
    _reserved_92_cs19: [u8; 0x04],
    _reserved_93_id19: [u8; 0x04],
    _reserved_94_word019: [u8; 0x04],
    _reserved_95_word119: [u8; 0x04],
    _reserved_96_cs20: [u8; 0x04],
    _reserved_97_id20: [u8; 0x04],
    _reserved_98_word020: [u8; 0x04],
    _reserved_99_word120: [u8; 0x04],
    _reserved_100_cs21: [u8; 0x04],
    _reserved_101_id21: [u8; 0x04],
    _reserved_102_word021: [u8; 0x04],
    _reserved_103_word121: [u8; 0x04],
    _reserved_104_cs22: [u8; 0x04],
    _reserved_105_id22: [u8; 0x04],
    _reserved_106_word022: [u8; 0x04],
    _reserved_107_word122: [u8; 0x04],
    _reserved_108_cs23: [u8; 0x04],
    _reserved_109_id23: [u8; 0x04],
    _reserved_110_word023: [u8; 0x04],
    _reserved_111_word123: [u8; 0x04],
    _reserved_112_cs24: [u8; 0x04],
    _reserved_113_id24: [u8; 0x04],
    _reserved_114_word024: [u8; 0x04],
    _reserved_115_word124: [u8; 0x04],
    _reserved_116_cs25: [u8; 0x04],
    _reserved_117_id25: [u8; 0x04],
    _reserved_118_word025: [u8; 0x04],
    _reserved_119_word125: [u8; 0x04],
    _reserved_120_cs26: [u8; 0x04],
    _reserved_121_id26: [u8; 0x04],
    _reserved_122_word026: [u8; 0x04],
    _reserved_123_word126: [u8; 0x04],
    _reserved_124_cs27: [u8; 0x04],
    _reserved_125_id27: [u8; 0x04],
    _reserved_126_word027: [u8; 0x04],
    _reserved_127_word127: [u8; 0x04],
    _reserved_128_cs28: [u8; 0x04],
    _reserved_129_id28: [u8; 0x04],
    _reserved_130_word028: [u8; 0x04],
    _reserved_131_word128: [u8; 0x04],
    _reserved_132_cs29: [u8; 0x04],
    _reserved_133_id29: [u8; 0x04],
    _reserved_134_word029: [u8; 0x04],
    _reserved_135_word129: [u8; 0x04],
    _reserved_136_cs30: [u8; 0x04],
    _reserved_137_id30: [u8; 0x04],
    _reserved_138_word030: [u8; 0x04],
    _reserved_139_word130: [u8; 0x04],
    _reserved_140_cs31: [u8; 0x04],
    _reserved_141_id31: [u8; 0x04],
    _reserved_142_word031: [u8; 0x04],
    _reserved_143_word131: [u8; 0x04],
    _reserved144: [u8; 0x0600],
    rximr: [Rximr; 32],
    _reserved145: [u8; 0x0200],
    ctrl1_pn: Ctrl1Pn,
    ctrl2_pn: Ctrl2Pn,
    wu_mtc: WuMtc,
    flt_id1: FltId1,
    flt_dlc: FltDlc,
    pl1_lo: Pl1Lo,
    pl1_hi: Pl1Hi,
    flt_id2_idmask: FltId2Idmask,
    pl2_plmask_lo: Pl2PlmaskLo,
    pl2_plmask_hi: Pl2PlmaskHi,
    _reserved155: [u8; 0x18],
    wmb: [Wmb; 4],
    _reserved156: [u8; 0x70],
    eprs: Eprs,
    encbt: Encbt,
    edcbt: Edcbt,
    etdc: Etdc,
    fdctrl: Fdctrl,
    fdcbt: Fdcbt,
    fdcrc: Fdcrc,
    erfcr: Erfcr,
    erfier: Erfier,
    erfsr: Erfsr,
    _reserved166: [u8; 0x23e8],
    erffel: [Erffel; 32],
}
impl RegisterBlock {
    #[doc = "0x00 - Module Configuration"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x04 - Control 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x08 - Free-Running Timer"]
    #[inline(always)]
    pub const fn timer(&self) -> &Timer {
        &self.timer
    }
    #[doc = "0x10 - RX Message Buffers Global Mask"]
    #[inline(always)]
    pub const fn rxmgmask(&self) -> &Rxmgmask {
        &self.rxmgmask
    }
    #[doc = "0x14 - Receive 14 Mask"]
    #[inline(always)]
    pub const fn rx14mask(&self) -> &Rx14mask {
        &self.rx14mask
    }
    #[doc = "0x18 - Receive 15 Mask"]
    #[inline(always)]
    pub const fn rx15mask(&self) -> &Rx15mask {
        &self.rx15mask
    }
    #[doc = "0x1c - Error Counter"]
    #[inline(always)]
    pub const fn ecr(&self) -> &Ecr {
        &self.ecr
    }
    #[doc = "0x20 - Error and Status 1"]
    #[inline(always)]
    pub const fn esr1(&self) -> &Esr1 {
        &self.esr1
    }
    #[doc = "0x28 - Interrupt Masks 1"]
    #[inline(always)]
    pub const fn imask1(&self) -> &Imask1 {
        &self.imask1
    }
    #[doc = "0x30 - Interrupt Flags 1"]
    #[inline(always)]
    pub const fn iflag1(&self) -> &Iflag1 {
        &self.iflag1
    }
    #[doc = "0x34 - Control 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x38 - Error and Status 2"]
    #[inline(always)]
    pub const fn esr2(&self) -> &Esr2 {
        &self.esr2
    }
    #[doc = "0x44 - Cyclic Redundancy Check"]
    #[inline(always)]
    pub const fn crcr(&self) -> &Crcr {
        &self.crcr
    }
    #[doc = "0x48 - Legacy RX FIFO Global Mask"]
    #[inline(always)]
    pub const fn rxfgmask(&self) -> &Rxfgmask {
        &self.rxfgmask
    }
    #[doc = "0x4c - Legacy RX FIFO Information"]
    #[inline(always)]
    pub const fn rxfir(&self) -> &Rxfir {
        &self.rxfir
    }
    #[doc = "0x50 - CAN Bit Timing"]
    #[inline(always)]
    pub const fn cbt(&self) -> &Cbt {
        &self.cbt
    }
    #[doc = "0x80 - Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb0_8b_cs(&self) -> &Mb0_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x80 - Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb0_64b_cs(&self) -> &Mb0_64bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x80 - Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb0_32b_cs(&self) -> &Mb0_32bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x80 - Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn mb0_16b_cs(&self) -> &Mb0_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x80 - Message Buffer 0 CS Register"]
    #[inline(always)]
    pub const fn cs0(&self) -> &Cs0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x84 - Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb0_8b_id(&self) -> &Mb0_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x84 - Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb0_64b_id(&self) -> &Mb0_64bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x84 - Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb0_32b_id(&self) -> &Mb0_32bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x84 - Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn mb0_16b_id(&self) -> &Mb0_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x84 - Message Buffer 0 ID Register"]
    #[inline(always)]
    pub const fn id0(&self) -> &Id0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0x88 - Message Buffer 0 WORD0 Register"]
    #[inline(always)]
    pub const fn word00(&self) -> &Word00 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x88 - Message Buffer 0 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb0_8b_word0(&self) -> &Mb0_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x88 - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word0(&self) -> &Mb0_64bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x88 - Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word0(&self) -> &Mb0_32bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x88 - Message Buffer 0 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb0_16b_word0(&self) -> &Mb0_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(136).cast() }
    }
    #[doc = "0x8c - Message Buffer 0 WORD1 Register"]
    #[inline(always)]
    pub const fn word10(&self) -> &Word10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x8c - Message Buffer 0 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb0_8b_word1(&self) -> &Mb0_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x8c - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word1(&self) -> &Mb0_64bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x8c - Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word1(&self) -> &Mb0_32bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x8c - Message Buffer 0 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb0_16b_word1(&self) -> &Mb0_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(140).cast() }
    }
    #[doc = "0x90 - Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn mb1_8b_cs(&self) -> &Mb1_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x90 - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word2(&self) -> &Mb0_64bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x90 - Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word2(&self) -> &Mb0_32bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x90 - Message Buffer 0 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb0_16b_word2(&self) -> &Mb0_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x90 - Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn cs1(&self) -> &Cs1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x94 - Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn mb1_8b_id(&self) -> &Mb1_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(148).cast() }
    }
    #[doc = "0x94 - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word3(&self) -> &Mb0_64bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(148).cast() }
    }
    #[doc = "0x94 - Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word3(&self) -> &Mb0_32bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(148).cast() }
    }
    #[doc = "0x94 - Message Buffer 0 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb0_16b_word3(&self) -> &Mb0_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(148).cast() }
    }
    #[doc = "0x94 - Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn id1(&self) -> &Id1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(148).cast() }
    }
    #[doc = "0x98 - Message Buffer 1 WORD0 Register"]
    #[inline(always)]
    pub const fn word01(&self) -> &Word01 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(152).cast() }
    }
    #[doc = "0x98 - Message Buffer 1 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb1_8b_word0(&self) -> &Mb1_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(152).cast() }
    }
    #[doc = "0x98 - Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn mb1_16b_cs(&self) -> &Mb1_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(152).cast() }
    }
    #[doc = "0x98 - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word4(&self) -> &Mb0_64bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(152).cast() }
    }
    #[doc = "0x98 - Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word4(&self) -> &Mb0_32bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(152).cast() }
    }
    #[doc = "0x9c - Message Buffer 1 WORD1 Register"]
    #[inline(always)]
    pub const fn word11(&self) -> &Word11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(156).cast() }
    }
    #[doc = "0x9c - Message Buffer 1 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb1_8b_word1(&self) -> &Mb1_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(156).cast() }
    }
    #[doc = "0x9c - Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn mb1_16b_id(&self) -> &Mb1_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(156).cast() }
    }
    #[doc = "0x9c - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word5(&self) -> &Mb0_64bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(156).cast() }
    }
    #[doc = "0x9c - Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word5(&self) -> &Mb0_32bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(156).cast() }
    }
    #[doc = "0xa0 - Message Buffer 2 CS Register"]
    #[inline(always)]
    pub const fn mb2_8b_cs(&self) -> &Mb2_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(160).cast() }
    }
    #[doc = "0xa0 - Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word0(&self) -> &Mb1_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(160).cast() }
    }
    #[doc = "0xa0 - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word6(&self) -> &Mb0_64bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(160).cast() }
    }
    #[doc = "0xa0 - Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word6(&self) -> &Mb0_32bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(160).cast() }
    }
    #[doc = "0xa0 - Message Buffer 2 CS Register"]
    #[inline(always)]
    pub const fn cs2(&self) -> &Cs2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(160).cast() }
    }
    #[doc = "0xa4 - Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn mb2_8b_id(&self) -> &Mb2_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(164).cast() }
    }
    #[doc = "0xa4 - Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word1(&self) -> &Mb1_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(164).cast() }
    }
    #[doc = "0xa4 - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word7(&self) -> &Mb0_64bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(164).cast() }
    }
    #[doc = "0xa4 - Message Buffer 0 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb0_32b_word7(&self) -> &Mb0_32bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(164).cast() }
    }
    #[doc = "0xa4 - Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn id2(&self) -> &Id2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(164).cast() }
    }
    #[doc = "0xa8 - Message Buffer 2 WORD0 Register"]
    #[inline(always)]
    pub const fn word02(&self) -> &Word02 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(168).cast() }
    }
    #[doc = "0xa8 - Message Buffer 2 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb2_8b_word0(&self) -> &Mb2_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(168).cast() }
    }
    #[doc = "0xa8 - Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn mb1_32b_cs(&self) -> &Mb1_32bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(168).cast() }
    }
    #[doc = "0xa8 - Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word2(&self) -> &Mb1_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(168).cast() }
    }
    #[doc = "0xa8 - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word8(&self) -> &Mb0_64bWord8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(168).cast() }
    }
    #[doc = "0xac - Message Buffer 2 WORD1 Register"]
    #[inline(always)]
    pub const fn word12(&self) -> &Word12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(172).cast() }
    }
    #[doc = "0xac - Message Buffer 2 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb2_8b_word1(&self) -> &Mb2_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(172).cast() }
    }
    #[doc = "0xac - Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn mb1_32b_id(&self) -> &Mb1_32bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(172).cast() }
    }
    #[doc = "0xac - Message Buffer 1 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb1_16b_word3(&self) -> &Mb1_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(172).cast() }
    }
    #[doc = "0xac - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word9(&self) -> &Mb0_64bWord9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(172).cast() }
    }
    #[doc = "0xb0 - Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn mb3_8b_cs(&self) -> &Mb3_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb0 - Message Buffer 2 CS Register"]
    #[inline(always)]
    pub const fn mb2_16b_cs(&self) -> &Mb2_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb0 - Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word0(&self) -> &Mb1_32bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb0 - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word10(&self) -> &Mb0_64bWord10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb0 - Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn cs3(&self) -> &Cs3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb4 - Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn mb3_8b_id(&self) -> &Mb3_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(180).cast() }
    }
    #[doc = "0xb4 - Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn mb2_16b_id(&self) -> &Mb2_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(180).cast() }
    }
    #[doc = "0xb4 - Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word1(&self) -> &Mb1_32bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(180).cast() }
    }
    #[doc = "0xb4 - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word11(&self) -> &Mb0_64bWord11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(180).cast() }
    }
    #[doc = "0xb4 - Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn id3(&self) -> &Id3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(180).cast() }
    }
    #[doc = "0xb8 - Message Buffer 3 WORD0 Register"]
    #[inline(always)]
    pub const fn word03(&self) -> &Word03 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(184).cast() }
    }
    #[doc = "0xb8 - Message Buffer 3 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb3_8b_word0(&self) -> &Mb3_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(184).cast() }
    }
    #[doc = "0xb8 - Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word0(&self) -> &Mb2_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(184).cast() }
    }
    #[doc = "0xb8 - Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word2(&self) -> &Mb1_32bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(184).cast() }
    }
    #[doc = "0xb8 - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word12(&self) -> &Mb0_64bWord12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(184).cast() }
    }
    #[doc = "0xbc - Message Buffer 3 WORD1 Register"]
    #[inline(always)]
    pub const fn word13(&self) -> &Word13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(188).cast() }
    }
    #[doc = "0xbc - Message Buffer 3 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb3_8b_word1(&self) -> &Mb3_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(188).cast() }
    }
    #[doc = "0xbc - Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word1(&self) -> &Mb2_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(188).cast() }
    }
    #[doc = "0xbc - Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word3(&self) -> &Mb1_32bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(188).cast() }
    }
    #[doc = "0xbc - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word13(&self) -> &Mb0_64bWord13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(188).cast() }
    }
    #[doc = "0xc0 - Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn mb4_8b_cs(&self) -> &Mb4_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(192).cast() }
    }
    #[doc = "0xc0 - Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word2(&self) -> &Mb2_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(192).cast() }
    }
    #[doc = "0xc0 - Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word4(&self) -> &Mb1_32bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(192).cast() }
    }
    #[doc = "0xc0 - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word14(&self) -> &Mb0_64bWord14 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(192).cast() }
    }
    #[doc = "0xc0 - Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn cs4(&self) -> &Cs4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(192).cast() }
    }
    #[doc = "0xc4 - Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn mb4_8b_id(&self) -> &Mb4_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc4 - Message Buffer 2 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb2_16b_word3(&self) -> &Mb2_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc4 - Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word5(&self) -> &Mb1_32bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc4 - Message Buffer 0 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb0_64b_word15(&self) -> &Mb0_64bWord15 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc4 - Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn id4(&self) -> &Id4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(196).cast() }
    }
    #[doc = "0xc8 - Message Buffer 4 WORD0 Register"]
    #[inline(always)]
    pub const fn word04(&self) -> &Word04 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(200).cast() }
    }
    #[doc = "0xc8 - Message Buffer 4 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb4_8b_word0(&self) -> &Mb4_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(200).cast() }
    }
    #[doc = "0xc8 - Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn mb3_16b_cs(&self) -> &Mb3_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(200).cast() }
    }
    #[doc = "0xc8 - Message Buffer 1 CS Register"]
    #[inline(always)]
    pub const fn mb1_64b_cs(&self) -> &Mb1_64bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(200).cast() }
    }
    #[doc = "0xc8 - Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word6(&self) -> &Mb1_32bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(200).cast() }
    }
    #[doc = "0xcc - Message Buffer 4 WORD1 Register"]
    #[inline(always)]
    pub const fn word14(&self) -> &Word14 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(204).cast() }
    }
    #[doc = "0xcc - Message Buffer 4 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb4_8b_word1(&self) -> &Mb4_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(204).cast() }
    }
    #[doc = "0xcc - Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn mb3_16b_id(&self) -> &Mb3_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(204).cast() }
    }
    #[doc = "0xcc - Message Buffer 1 ID Register"]
    #[inline(always)]
    pub const fn mb1_64b_id(&self) -> &Mb1_64bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(204).cast() }
    }
    #[doc = "0xcc - Message Buffer 1 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb1_32b_word7(&self) -> &Mb1_32bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(204).cast() }
    }
    #[doc = "0xd0 - Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn mb5_8b_cs(&self) -> &Mb5_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(208).cast() }
    }
    #[doc = "0xd0 - Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word0(&self) -> &Mb3_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(208).cast() }
    }
    #[doc = "0xd0 - Message Buffer 2 CS Register"]
    #[inline(always)]
    pub const fn mb2_32b_cs(&self) -> &Mb2_32bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(208).cast() }
    }
    #[doc = "0xd0 - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word0(&self) -> &Mb1_64bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(208).cast() }
    }
    #[doc = "0xd0 - Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn cs5(&self) -> &Cs5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(208).cast() }
    }
    #[doc = "0xd4 - Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn mb5_8b_id(&self) -> &Mb5_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(212).cast() }
    }
    #[doc = "0xd4 - Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word1(&self) -> &Mb3_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(212).cast() }
    }
    #[doc = "0xd4 - Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn mb2_32b_id(&self) -> &Mb2_32bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(212).cast() }
    }
    #[doc = "0xd4 - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word1(&self) -> &Mb1_64bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(212).cast() }
    }
    #[doc = "0xd4 - Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn id5(&self) -> &Id5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(212).cast() }
    }
    #[doc = "0xd8 - Message Buffer 5 WORD0 Register"]
    #[inline(always)]
    pub const fn word05(&self) -> &Word05 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(216).cast() }
    }
    #[doc = "0xd8 - Message Buffer 5 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb5_8b_word0(&self) -> &Mb5_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(216).cast() }
    }
    #[doc = "0xd8 - Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word2(&self) -> &Mb3_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(216).cast() }
    }
    #[doc = "0xd8 - Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word0(&self) -> &Mb2_32bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(216).cast() }
    }
    #[doc = "0xd8 - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word2(&self) -> &Mb1_64bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(216).cast() }
    }
    #[doc = "0xdc - Message Buffer 5 WORD1 Register"]
    #[inline(always)]
    pub const fn word15(&self) -> &Word15 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(220).cast() }
    }
    #[doc = "0xdc - Message Buffer 5 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb5_8b_word1(&self) -> &Mb5_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(220).cast() }
    }
    #[doc = "0xdc - Message Buffer 3 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb3_16b_word3(&self) -> &Mb3_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(220).cast() }
    }
    #[doc = "0xdc - Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word1(&self) -> &Mb2_32bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(220).cast() }
    }
    #[doc = "0xdc - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word3(&self) -> &Mb1_64bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(220).cast() }
    }
    #[doc = "0xe0 - Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn mb6_8b_cs(&self) -> &Mb6_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(224).cast() }
    }
    #[doc = "0xe0 - Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn mb4_16b_cs(&self) -> &Mb4_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(224).cast() }
    }
    #[doc = "0xe0 - Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word2(&self) -> &Mb2_32bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(224).cast() }
    }
    #[doc = "0xe0 - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word4(&self) -> &Mb1_64bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(224).cast() }
    }
    #[doc = "0xe0 - Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn cs6(&self) -> &Cs6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(224).cast() }
    }
    #[doc = "0xe4 - Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn mb6_8b_id(&self) -> &Mb6_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(228).cast() }
    }
    #[doc = "0xe4 - Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn mb4_16b_id(&self) -> &Mb4_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(228).cast() }
    }
    #[doc = "0xe4 - Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word3(&self) -> &Mb2_32bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(228).cast() }
    }
    #[doc = "0xe4 - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word5(&self) -> &Mb1_64bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(228).cast() }
    }
    #[doc = "0xe4 - Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn id6(&self) -> &Id6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(228).cast() }
    }
    #[doc = "0xe8 - Message Buffer 6 WORD0 Register"]
    #[inline(always)]
    pub const fn word06(&self) -> &Word06 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(232).cast() }
    }
    #[doc = "0xe8 - Message Buffer 6 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb6_8b_word0(&self) -> &Mb6_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(232).cast() }
    }
    #[doc = "0xe8 - Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word0(&self) -> &Mb4_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(232).cast() }
    }
    #[doc = "0xe8 - Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word4(&self) -> &Mb2_32bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(232).cast() }
    }
    #[doc = "0xe8 - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word6(&self) -> &Mb1_64bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(232).cast() }
    }
    #[doc = "0xec - Message Buffer 6 WORD1 Register"]
    #[inline(always)]
    pub const fn word16(&self) -> &Word16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(236).cast() }
    }
    #[doc = "0xec - Message Buffer 6 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb6_8b_word1(&self) -> &Mb6_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(236).cast() }
    }
    #[doc = "0xec - Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word1(&self) -> &Mb4_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(236).cast() }
    }
    #[doc = "0xec - Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word5(&self) -> &Mb2_32bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(236).cast() }
    }
    #[doc = "0xec - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word7(&self) -> &Mb1_64bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(236).cast() }
    }
    #[doc = "0xf0 - Message Buffer 7 CS Register"]
    #[inline(always)]
    pub const fn mb7_8b_cs(&self) -> &Mb7_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(240).cast() }
    }
    #[doc = "0xf0 - Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word2(&self) -> &Mb4_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(240).cast() }
    }
    #[doc = "0xf0 - Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word6(&self) -> &Mb2_32bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(240).cast() }
    }
    #[doc = "0xf0 - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word8(&self) -> &Mb1_64bWord8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(240).cast() }
    }
    #[doc = "0xf0 - Message Buffer 7 CS Register"]
    #[inline(always)]
    pub const fn cs7(&self) -> &Cs7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(240).cast() }
    }
    #[doc = "0xf4 - Message Buffer 7 ID Register"]
    #[inline(always)]
    pub const fn mb7_8b_id(&self) -> &Mb7_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(244).cast() }
    }
    #[doc = "0xf4 - Message Buffer 4 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb4_16b_word3(&self) -> &Mb4_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(244).cast() }
    }
    #[doc = "0xf4 - Message Buffer 2 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb2_32b_word7(&self) -> &Mb2_32bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(244).cast() }
    }
    #[doc = "0xf4 - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word9(&self) -> &Mb1_64bWord9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(244).cast() }
    }
    #[doc = "0xf4 - Message Buffer 7 ID Register"]
    #[inline(always)]
    pub const fn id7(&self) -> &Id7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(244).cast() }
    }
    #[doc = "0xf8 - Message Buffer 7 WORD0 Register"]
    #[inline(always)]
    pub const fn word07(&self) -> &Word07 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(248).cast() }
    }
    #[doc = "0xf8 - Message Buffer 7 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb7_8b_word0(&self) -> &Mb7_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(248).cast() }
    }
    #[doc = "0xf8 - Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn mb5_16b_cs(&self) -> &Mb5_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(248).cast() }
    }
    #[doc = "0xf8 - Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn mb3_32b_cs(&self) -> &Mb3_32bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(248).cast() }
    }
    #[doc = "0xf8 - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word10(&self) -> &Mb1_64bWord10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(248).cast() }
    }
    #[doc = "0xfc - Message Buffer 7 WORD1 Register"]
    #[inline(always)]
    pub const fn word17(&self) -> &Word17 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(252).cast() }
    }
    #[doc = "0xfc - Message Buffer 7 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb7_8b_word1(&self) -> &Mb7_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(252).cast() }
    }
    #[doc = "0xfc - Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn mb5_16b_id(&self) -> &Mb5_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(252).cast() }
    }
    #[doc = "0xfc - Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn mb3_32b_id(&self) -> &Mb3_32bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(252).cast() }
    }
    #[doc = "0xfc - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word11(&self) -> &Mb1_64bWord11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(252).cast() }
    }
    #[doc = "0x100 - Message Buffer 8 CS Register"]
    #[inline(always)]
    pub const fn mb8_8b_cs(&self) -> &Mb8_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x100 - Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word0(&self) -> &Mb5_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x100 - Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word0(&self) -> &Mb3_32bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x100 - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word12(&self) -> &Mb1_64bWord12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x100 - Message Buffer 8 CS Register"]
    #[inline(always)]
    pub const fn cs8(&self) -> &Cs8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x104 - Message Buffer 8 ID Register"]
    #[inline(always)]
    pub const fn mb8_8b_id(&self) -> &Mb8_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x104 - Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word1(&self) -> &Mb5_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x104 - Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word1(&self) -> &Mb3_32bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x104 - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word13(&self) -> &Mb1_64bWord13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x104 - Message Buffer 8 ID Register"]
    #[inline(always)]
    pub const fn id8(&self) -> &Id8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x108 - Message Buffer 8 WORD0 Register"]
    #[inline(always)]
    pub const fn word08(&self) -> &Word08 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x108 - Message Buffer 8 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb8_8b_word0(&self) -> &Mb8_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x108 - Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word2(&self) -> &Mb5_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x108 - Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word2(&self) -> &Mb3_32bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x108 - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word14(&self) -> &Mb1_64bWord14 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x10c - Message Buffer 8 WORD1 Register"]
    #[inline(always)]
    pub const fn word18(&self) -> &Word18 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x10c - Message Buffer 8 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb8_8b_word1(&self) -> &Mb8_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x10c - Message Buffer 5 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb5_16b_word3(&self) -> &Mb5_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x10c - Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word3(&self) -> &Mb3_32bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x10c - Message Buffer 1 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb1_64b_word15(&self) -> &Mb1_64bWord15 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x110 - Message Buffer 9 CS Register"]
    #[inline(always)]
    pub const fn mb9_8b_cs(&self) -> &Mb9_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(272).cast() }
    }
    #[doc = "0x110 - Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn mb6_16b_cs(&self) -> &Mb6_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(272).cast() }
    }
    #[doc = "0x110 - Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word4(&self) -> &Mb3_32bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(272).cast() }
    }
    #[doc = "0x110 - Message Buffer 2 CS Register"]
    #[inline(always)]
    pub const fn mb2_64b_cs(&self) -> &Mb2_64bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(272).cast() }
    }
    #[doc = "0x110 - Message Buffer 9 CS Register"]
    #[inline(always)]
    pub const fn cs9(&self) -> &Cs9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(272).cast() }
    }
    #[doc = "0x114 - Message Buffer 9 ID Register"]
    #[inline(always)]
    pub const fn mb9_8b_id(&self) -> &Mb9_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x114 - Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn mb6_16b_id(&self) -> &Mb6_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x114 - Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word5(&self) -> &Mb3_32bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x114 - Message Buffer 2 ID Register"]
    #[inline(always)]
    pub const fn mb2_64b_id(&self) -> &Mb2_64bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x114 - Message Buffer 9 ID Register"]
    #[inline(always)]
    pub const fn id9(&self) -> &Id9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x118 - Message Buffer 9 WORD0 Register"]
    #[inline(always)]
    pub const fn word09(&self) -> &Word09 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x118 - Message Buffer 9 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb9_8b_word0(&self) -> &Mb9_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x118 - Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word0(&self) -> &Mb6_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x118 - Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word6(&self) -> &Mb3_32bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x118 - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word0(&self) -> &Mb2_64bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x11c - Message Buffer 9 WORD1 Register"]
    #[inline(always)]
    pub const fn word19(&self) -> &Word19 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x11c - Message Buffer 9 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb9_8b_word1(&self) -> &Mb9_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x11c - Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word1(&self) -> &Mb6_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x11c - Message Buffer 3 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb3_32b_word7(&self) -> &Mb3_32bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x11c - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word1(&self) -> &Mb2_64bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x120 - Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word2(&self) -> &Mb6_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(288).cast() }
    }
    #[doc = "0x120 - Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn mb4_32b_cs(&self) -> &Mb4_32bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(288).cast() }
    }
    #[doc = "0x120 - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word2(&self) -> &Mb2_64bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(288).cast() }
    }
    #[doc = "0x120 - Message Buffer 10 CS Register"]
    #[inline(always)]
    pub const fn mb10_8b_cs(&self) -> &Mb10_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(288).cast() }
    }
    #[doc = "0x120 - Message Buffer 10 CS Register"]
    #[inline(always)]
    pub const fn cs10(&self) -> &Cs10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(288).cast() }
    }
    #[doc = "0x124 - Message Buffer 6 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb6_16b_word3(&self) -> &Mb6_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(292).cast() }
    }
    #[doc = "0x124 - Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn mb4_32b_id(&self) -> &Mb4_32bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(292).cast() }
    }
    #[doc = "0x124 - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word3(&self) -> &Mb2_64bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(292).cast() }
    }
    #[doc = "0x124 - Message Buffer 10 ID Register"]
    #[inline(always)]
    pub const fn mb10_8b_id(&self) -> &Mb10_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(292).cast() }
    }
    #[doc = "0x124 - Message Buffer 10 ID Register"]
    #[inline(always)]
    pub const fn id10(&self) -> &Id10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(292).cast() }
    }
    #[doc = "0x128 - Message Buffer 10 WORD0 Register"]
    #[inline(always)]
    pub const fn word010(&self) -> &Word010 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x128 - Message Buffer 7 CS Register"]
    #[inline(always)]
    pub const fn mb7_16b_cs(&self) -> &Mb7_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x128 - Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word0(&self) -> &Mb4_32bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x128 - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word4(&self) -> &Mb2_64bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x128 - Message Buffer 10 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb10_8b_word0(&self) -> &Mb10_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x12c - Message Buffer 10 WORD1 Register"]
    #[inline(always)]
    pub const fn word110(&self) -> &Word110 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(300).cast() }
    }
    #[doc = "0x12c - Message Buffer 7 ID Register"]
    #[inline(always)]
    pub const fn mb7_16b_id(&self) -> &Mb7_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(300).cast() }
    }
    #[doc = "0x12c - Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word1(&self) -> &Mb4_32bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(300).cast() }
    }
    #[doc = "0x12c - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word5(&self) -> &Mb2_64bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(300).cast() }
    }
    #[doc = "0x12c - Message Buffer 10 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb10_8b_word1(&self) -> &Mb10_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(300).cast() }
    }
    #[doc = "0x130 - Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word0(&self) -> &Mb7_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(304).cast() }
    }
    #[doc = "0x130 - Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word2(&self) -> &Mb4_32bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(304).cast() }
    }
    #[doc = "0x130 - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word6(&self) -> &Mb2_64bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(304).cast() }
    }
    #[doc = "0x130 - Message Buffer 11 CS Register"]
    #[inline(always)]
    pub const fn mb11_8b_cs(&self) -> &Mb11_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(304).cast() }
    }
    #[doc = "0x130 - Message Buffer 11 CS Register"]
    #[inline(always)]
    pub const fn cs11(&self) -> &Cs11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(304).cast() }
    }
    #[doc = "0x134 - Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word1(&self) -> &Mb7_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x134 - Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word3(&self) -> &Mb4_32bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x134 - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word7(&self) -> &Mb2_64bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x134 - Message Buffer 11 ID Register"]
    #[inline(always)]
    pub const fn mb11_8b_id(&self) -> &Mb11_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x134 - Message Buffer 11 ID Register"]
    #[inline(always)]
    pub const fn id11(&self) -> &Id11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x138 - Message Buffer 11 WORD0 Register"]
    #[inline(always)]
    pub const fn word011(&self) -> &Word011 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x138 - Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word2(&self) -> &Mb7_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x138 - Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word4(&self) -> &Mb4_32bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x138 - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word8(&self) -> &Mb2_64bWord8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x138 - Message Buffer 11 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb11_8b_word0(&self) -> &Mb11_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x13c - Message Buffer 11 WORD1 Register"]
    #[inline(always)]
    pub const fn word111(&self) -> &Word111 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(316).cast() }
    }
    #[doc = "0x13c - Message Buffer 7 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb7_16b_word3(&self) -> &Mb7_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(316).cast() }
    }
    #[doc = "0x13c - Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word5(&self) -> &Mb4_32bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(316).cast() }
    }
    #[doc = "0x13c - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word9(&self) -> &Mb2_64bWord9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(316).cast() }
    }
    #[doc = "0x13c - Message Buffer 11 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb11_8b_word1(&self) -> &Mb11_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(316).cast() }
    }
    #[doc = "0x140 - Message Buffer 8 CS Register"]
    #[inline(always)]
    pub const fn mb8_16b_cs(&self) -> &Mb8_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    #[doc = "0x140 - Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word6(&self) -> &Mb4_32bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    #[doc = "0x140 - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word10(&self) -> &Mb2_64bWord10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    #[doc = "0x140 - Message Buffer 12 CS Register"]
    #[inline(always)]
    pub const fn mb12_8b_cs(&self) -> &Mb12_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    #[doc = "0x140 - Message Buffer 12 CS Register"]
    #[inline(always)]
    pub const fn cs12(&self) -> &Cs12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(320).cast() }
    }
    #[doc = "0x144 - Message Buffer 8 ID Register"]
    #[inline(always)]
    pub const fn mb8_16b_id(&self) -> &Mb8_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(324).cast() }
    }
    #[doc = "0x144 - Message Buffer 4 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb4_32b_word7(&self) -> &Mb4_32bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(324).cast() }
    }
    #[doc = "0x144 - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word11(&self) -> &Mb2_64bWord11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(324).cast() }
    }
    #[doc = "0x144 - Message Buffer 12 ID Register"]
    #[inline(always)]
    pub const fn mb12_8b_id(&self) -> &Mb12_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(324).cast() }
    }
    #[doc = "0x144 - Message Buffer 12 ID Register"]
    #[inline(always)]
    pub const fn id12(&self) -> &Id12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(324).cast() }
    }
    #[doc = "0x148 - Message Buffer 12 WORD0 Register"]
    #[inline(always)]
    pub const fn word012(&self) -> &Word012 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(328).cast() }
    }
    #[doc = "0x148 - Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word0(&self) -> &Mb8_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(328).cast() }
    }
    #[doc = "0x148 - Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn mb5_32b_cs(&self) -> &Mb5_32bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(328).cast() }
    }
    #[doc = "0x148 - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word12(&self) -> &Mb2_64bWord12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(328).cast() }
    }
    #[doc = "0x148 - Message Buffer 12 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb12_8b_word0(&self) -> &Mb12_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(328).cast() }
    }
    #[doc = "0x14c - Message Buffer 12 WORD1 Register"]
    #[inline(always)]
    pub const fn word112(&self) -> &Word112 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(332).cast() }
    }
    #[doc = "0x14c - Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word1(&self) -> &Mb8_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(332).cast() }
    }
    #[doc = "0x14c - Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn mb5_32b_id(&self) -> &Mb5_32bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(332).cast() }
    }
    #[doc = "0x14c - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word13(&self) -> &Mb2_64bWord13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(332).cast() }
    }
    #[doc = "0x14c - Message Buffer 12 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb12_8b_word1(&self) -> &Mb12_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(332).cast() }
    }
    #[doc = "0x150 - Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word2(&self) -> &Mb8_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(336).cast() }
    }
    #[doc = "0x150 - Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word0(&self) -> &Mb5_32bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(336).cast() }
    }
    #[doc = "0x150 - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word14(&self) -> &Mb2_64bWord14 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(336).cast() }
    }
    #[doc = "0x150 - Message Buffer 13 CS Register"]
    #[inline(always)]
    pub const fn mb13_8b_cs(&self) -> &Mb13_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(336).cast() }
    }
    #[doc = "0x150 - Message Buffer 13 CS Register"]
    #[inline(always)]
    pub const fn cs13(&self) -> &Cs13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(336).cast() }
    }
    #[doc = "0x154 - Message Buffer 8 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb8_16b_word3(&self) -> &Mb8_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(340).cast() }
    }
    #[doc = "0x154 - Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word1(&self) -> &Mb5_32bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(340).cast() }
    }
    #[doc = "0x154 - Message Buffer 2 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb2_64b_word15(&self) -> &Mb2_64bWord15 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(340).cast() }
    }
    #[doc = "0x154 - Message Buffer 13 ID Register"]
    #[inline(always)]
    pub const fn mb13_8b_id(&self) -> &Mb13_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(340).cast() }
    }
    #[doc = "0x154 - Message Buffer 13 ID Register"]
    #[inline(always)]
    pub const fn id13(&self) -> &Id13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(340).cast() }
    }
    #[doc = "0x158 - Message Buffer 13 WORD0 Register"]
    #[inline(always)]
    pub const fn word013(&self) -> &Word013 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(344).cast() }
    }
    #[doc = "0x158 - Message Buffer 9 CS Register"]
    #[inline(always)]
    pub const fn mb9_16b_cs(&self) -> &Mb9_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(344).cast() }
    }
    #[doc = "0x158 - Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word2(&self) -> &Mb5_32bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(344).cast() }
    }
    #[doc = "0x158 - Message Buffer 3 CS Register"]
    #[inline(always)]
    pub const fn mb3_64b_cs(&self) -> &Mb3_64bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(344).cast() }
    }
    #[doc = "0x158 - Message Buffer 13 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb13_8b_word0(&self) -> &Mb13_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(344).cast() }
    }
    #[doc = "0x15c - Message Buffer 13 WORD1 Register"]
    #[inline(always)]
    pub const fn word113(&self) -> &Word113 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(348).cast() }
    }
    #[doc = "0x15c - Message Buffer 9 ID Register"]
    #[inline(always)]
    pub const fn mb9_16b_id(&self) -> &Mb9_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(348).cast() }
    }
    #[doc = "0x15c - Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word3(&self) -> &Mb5_32bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(348).cast() }
    }
    #[doc = "0x15c - Message Buffer 3 ID Register"]
    #[inline(always)]
    pub const fn mb3_64b_id(&self) -> &Mb3_64bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(348).cast() }
    }
    #[doc = "0x15c - Message Buffer 13 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb13_8b_word1(&self) -> &Mb13_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(348).cast() }
    }
    #[doc = "0x160 - Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word0(&self) -> &Mb9_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(352).cast() }
    }
    #[doc = "0x160 - Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word4(&self) -> &Mb5_32bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(352).cast() }
    }
    #[doc = "0x160 - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word0(&self) -> &Mb3_64bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(352).cast() }
    }
    #[doc = "0x160 - Message Buffer 14 CS Register"]
    #[inline(always)]
    pub const fn mb14_8b_cs(&self) -> &Mb14_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(352).cast() }
    }
    #[doc = "0x160 - Message Buffer 14 CS Register"]
    #[inline(always)]
    pub const fn cs14(&self) -> &Cs14 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(352).cast() }
    }
    #[doc = "0x164 - Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word1(&self) -> &Mb9_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(356).cast() }
    }
    #[doc = "0x164 - Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word5(&self) -> &Mb5_32bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(356).cast() }
    }
    #[doc = "0x164 - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word1(&self) -> &Mb3_64bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(356).cast() }
    }
    #[doc = "0x164 - Message Buffer 14 ID Register"]
    #[inline(always)]
    pub const fn mb14_8b_id(&self) -> &Mb14_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(356).cast() }
    }
    #[doc = "0x164 - Message Buffer 14 ID Register"]
    #[inline(always)]
    pub const fn id14(&self) -> &Id14 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(356).cast() }
    }
    #[doc = "0x168 - Message Buffer 14 WORD0 Register"]
    #[inline(always)]
    pub const fn word014(&self) -> &Word014 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(360).cast() }
    }
    #[doc = "0x168 - Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word2(&self) -> &Mb9_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(360).cast() }
    }
    #[doc = "0x168 - Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word6(&self) -> &Mb5_32bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(360).cast() }
    }
    #[doc = "0x168 - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word2(&self) -> &Mb3_64bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(360).cast() }
    }
    #[doc = "0x168 - Message Buffer 14 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb14_8b_word0(&self) -> &Mb14_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(360).cast() }
    }
    #[doc = "0x16c - Message Buffer 14 WORD1 Register"]
    #[inline(always)]
    pub const fn word114(&self) -> &Word114 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(364).cast() }
    }
    #[doc = "0x16c - Message Buffer 9 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb9_16b_word3(&self) -> &Mb9_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(364).cast() }
    }
    #[doc = "0x16c - Message Buffer 5 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb5_32b_word7(&self) -> &Mb5_32bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(364).cast() }
    }
    #[doc = "0x16c - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word3(&self) -> &Mb3_64bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(364).cast() }
    }
    #[doc = "0x16c - Message Buffer 14 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb14_8b_word1(&self) -> &Mb14_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(364).cast() }
    }
    #[doc = "0x170 - Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn mb6_32b_cs(&self) -> &Mb6_32bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(368).cast() }
    }
    #[doc = "0x170 - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word4(&self) -> &Mb3_64bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(368).cast() }
    }
    #[doc = "0x170 - Message Buffer 15 CS Register"]
    #[inline(always)]
    pub const fn mb15_8b_cs(&self) -> &Mb15_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(368).cast() }
    }
    #[doc = "0x170 - Message Buffer 10 CS Register"]
    #[inline(always)]
    pub const fn mb10_16b_cs(&self) -> &Mb10_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(368).cast() }
    }
    #[doc = "0x170 - Message Buffer 15 CS Register"]
    #[inline(always)]
    pub const fn cs15(&self) -> &Cs15 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(368).cast() }
    }
    #[doc = "0x174 - Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn mb6_32b_id(&self) -> &Mb6_32bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(372).cast() }
    }
    #[doc = "0x174 - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word5(&self) -> &Mb3_64bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(372).cast() }
    }
    #[doc = "0x174 - Message Buffer 15 ID Register"]
    #[inline(always)]
    pub const fn mb15_8b_id(&self) -> &Mb15_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(372).cast() }
    }
    #[doc = "0x174 - Message Buffer 10 ID Register"]
    #[inline(always)]
    pub const fn mb10_16b_id(&self) -> &Mb10_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(372).cast() }
    }
    #[doc = "0x174 - Message Buffer 15 ID Register"]
    #[inline(always)]
    pub const fn id15(&self) -> &Id15 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(372).cast() }
    }
    #[doc = "0x178 - Message Buffer 15 WORD0 Register"]
    #[inline(always)]
    pub const fn word015(&self) -> &Word015 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(376).cast() }
    }
    #[doc = "0x178 - Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word0(&self) -> &Mb6_32bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(376).cast() }
    }
    #[doc = "0x178 - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word6(&self) -> &Mb3_64bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(376).cast() }
    }
    #[doc = "0x178 - Message Buffer 15 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb15_8b_word0(&self) -> &Mb15_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(376).cast() }
    }
    #[doc = "0x178 - Message Buffer 10 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb10_16b_word0(&self) -> &Mb10_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(376).cast() }
    }
    #[doc = "0x17c - Message Buffer 15 WORD1 Register"]
    #[inline(always)]
    pub const fn word115(&self) -> &Word115 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(380).cast() }
    }
    #[doc = "0x17c - Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word1(&self) -> &Mb6_32bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(380).cast() }
    }
    #[doc = "0x17c - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word7(&self) -> &Mb3_64bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(380).cast() }
    }
    #[doc = "0x17c - Message Buffer 15 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb15_8b_word1(&self) -> &Mb15_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(380).cast() }
    }
    #[doc = "0x17c - Message Buffer 10 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb10_16b_word1(&self) -> &Mb10_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(380).cast() }
    }
    #[doc = "0x180 - Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word2(&self) -> &Mb6_32bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    #[doc = "0x180 - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word8(&self) -> &Mb3_64bWord8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    #[doc = "0x180 - Message Buffer 16 CS Register"]
    #[inline(always)]
    pub const fn mb16_8b_cs(&self) -> &Mb16_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    #[doc = "0x180 - Message Buffer 10 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb10_16b_word2(&self) -> &Mb10_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    #[doc = "0x180 - Message Buffer 16 CS Register"]
    #[inline(always)]
    pub const fn cs16(&self) -> &Cs16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    #[doc = "0x184 - Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word3(&self) -> &Mb6_32bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(388).cast() }
    }
    #[doc = "0x184 - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word9(&self) -> &Mb3_64bWord9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(388).cast() }
    }
    #[doc = "0x184 - Message Buffer 16 ID Register"]
    #[inline(always)]
    pub const fn mb16_8b_id(&self) -> &Mb16_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(388).cast() }
    }
    #[doc = "0x184 - Message Buffer 10 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb10_16b_word3(&self) -> &Mb10_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(388).cast() }
    }
    #[doc = "0x184 - Message Buffer 16 ID Register"]
    #[inline(always)]
    pub const fn id16(&self) -> &Id16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(388).cast() }
    }
    #[doc = "0x188 - Message Buffer 16 WORD0 Register"]
    #[inline(always)]
    pub const fn word016(&self) -> &Word016 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(392).cast() }
    }
    #[doc = "0x188 - Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word4(&self) -> &Mb6_32bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(392).cast() }
    }
    #[doc = "0x188 - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word10(&self) -> &Mb3_64bWord10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(392).cast() }
    }
    #[doc = "0x188 - Message Buffer 16 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb16_8b_word0(&self) -> &Mb16_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(392).cast() }
    }
    #[doc = "0x188 - Message Buffer 11 CS Register"]
    #[inline(always)]
    pub const fn mb11_16b_cs(&self) -> &Mb11_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(392).cast() }
    }
    #[doc = "0x18c - Message Buffer 16 WORD1 Register"]
    #[inline(always)]
    pub const fn word116(&self) -> &Word116 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(396).cast() }
    }
    #[doc = "0x18c - Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word5(&self) -> &Mb6_32bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(396).cast() }
    }
    #[doc = "0x18c - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word11(&self) -> &Mb3_64bWord11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(396).cast() }
    }
    #[doc = "0x18c - Message Buffer 16 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb16_8b_word1(&self) -> &Mb16_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(396).cast() }
    }
    #[doc = "0x18c - Message Buffer 11 ID Register"]
    #[inline(always)]
    pub const fn mb11_16b_id(&self) -> &Mb11_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(396).cast() }
    }
    #[doc = "0x190 - Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word6(&self) -> &Mb6_32bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(400).cast() }
    }
    #[doc = "0x190 - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word12(&self) -> &Mb3_64bWord12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(400).cast() }
    }
    #[doc = "0x190 - Message Buffer 17 CS Register"]
    #[inline(always)]
    pub const fn mb17_8b_cs(&self) -> &Mb17_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(400).cast() }
    }
    #[doc = "0x190 - Message Buffer 11 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb11_16b_word0(&self) -> &Mb11_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(400).cast() }
    }
    #[doc = "0x190 - Message Buffer 17 CS Register"]
    #[inline(always)]
    pub const fn cs17(&self) -> &Cs17 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(400).cast() }
    }
    #[doc = "0x194 - Message Buffer 6 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb6_32b_word7(&self) -> &Mb6_32bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(404).cast() }
    }
    #[doc = "0x194 - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word13(&self) -> &Mb3_64bWord13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(404).cast() }
    }
    #[doc = "0x194 - Message Buffer 17 ID Register"]
    #[inline(always)]
    pub const fn mb17_8b_id(&self) -> &Mb17_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(404).cast() }
    }
    #[doc = "0x194 - Message Buffer 11 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb11_16b_word1(&self) -> &Mb11_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(404).cast() }
    }
    #[doc = "0x194 - Message Buffer 17 ID Register"]
    #[inline(always)]
    pub const fn id17(&self) -> &Id17 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(404).cast() }
    }
    #[doc = "0x198 - Message Buffer 17 WORD0 Register"]
    #[inline(always)]
    pub const fn word017(&self) -> &Word017 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(408).cast() }
    }
    #[doc = "0x198 - Message Buffer 7 CS Register"]
    #[inline(always)]
    pub const fn mb7_32b_cs(&self) -> &Mb7_32bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(408).cast() }
    }
    #[doc = "0x198 - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word14(&self) -> &Mb3_64bWord14 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(408).cast() }
    }
    #[doc = "0x198 - Message Buffer 17 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb17_8b_word0(&self) -> &Mb17_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(408).cast() }
    }
    #[doc = "0x198 - Message Buffer 11 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb11_16b_word2(&self) -> &Mb11_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(408).cast() }
    }
    #[doc = "0x19c - Message Buffer 17 WORD1 Register"]
    #[inline(always)]
    pub const fn word117(&self) -> &Word117 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(412).cast() }
    }
    #[doc = "0x19c - Message Buffer 7 ID Register"]
    #[inline(always)]
    pub const fn mb7_32b_id(&self) -> &Mb7_32bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(412).cast() }
    }
    #[doc = "0x19c - Message Buffer 3 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb3_64b_word15(&self) -> &Mb3_64bWord15 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(412).cast() }
    }
    #[doc = "0x19c - Message Buffer 17 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb17_8b_word1(&self) -> &Mb17_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(412).cast() }
    }
    #[doc = "0x19c - Message Buffer 11 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb11_16b_word3(&self) -> &Mb11_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(412).cast() }
    }
    #[doc = "0x1a0 - Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word0(&self) -> &Mb7_32bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(416).cast() }
    }
    #[doc = "0x1a0 - Message Buffer 4 CS Register"]
    #[inline(always)]
    pub const fn mb4_64b_cs(&self) -> &Mb4_64bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(416).cast() }
    }
    #[doc = "0x1a0 - Message Buffer 18 CS Register"]
    #[inline(always)]
    pub const fn mb18_8b_cs(&self) -> &Mb18_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(416).cast() }
    }
    #[doc = "0x1a0 - Message Buffer 12 CS Register"]
    #[inline(always)]
    pub const fn mb12_16b_cs(&self) -> &Mb12_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(416).cast() }
    }
    #[doc = "0x1a0 - Message Buffer 18 CS Register"]
    #[inline(always)]
    pub const fn cs18(&self) -> &Cs18 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(416).cast() }
    }
    #[doc = "0x1a4 - Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word1(&self) -> &Mb7_32bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(420).cast() }
    }
    #[doc = "0x1a4 - Message Buffer 4 ID Register"]
    #[inline(always)]
    pub const fn mb4_64b_id(&self) -> &Mb4_64bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(420).cast() }
    }
    #[doc = "0x1a4 - Message Buffer 18 ID Register"]
    #[inline(always)]
    pub const fn mb18_8b_id(&self) -> &Mb18_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(420).cast() }
    }
    #[doc = "0x1a4 - Message Buffer 12 ID Register"]
    #[inline(always)]
    pub const fn mb12_16b_id(&self) -> &Mb12_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(420).cast() }
    }
    #[doc = "0x1a4 - Message Buffer 18 ID Register"]
    #[inline(always)]
    pub const fn id18(&self) -> &Id18 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(420).cast() }
    }
    #[doc = "0x1a8 - Message Buffer 18 WORD0 Register"]
    #[inline(always)]
    pub const fn word018(&self) -> &Word018 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(424).cast() }
    }
    #[doc = "0x1a8 - Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word2(&self) -> &Mb7_32bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(424).cast() }
    }
    #[doc = "0x1a8 - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word0(&self) -> &Mb4_64bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(424).cast() }
    }
    #[doc = "0x1a8 - Message Buffer 18 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb18_8b_word0(&self) -> &Mb18_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(424).cast() }
    }
    #[doc = "0x1a8 - Message Buffer 12 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb12_16b_word0(&self) -> &Mb12_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(424).cast() }
    }
    #[doc = "0x1ac - Message Buffer 18 WORD1 Register"]
    #[inline(always)]
    pub const fn word118(&self) -> &Word118 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(428).cast() }
    }
    #[doc = "0x1ac - Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word3(&self) -> &Mb7_32bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(428).cast() }
    }
    #[doc = "0x1ac - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word1(&self) -> &Mb4_64bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(428).cast() }
    }
    #[doc = "0x1ac - Message Buffer 18 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb18_8b_word1(&self) -> &Mb18_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(428).cast() }
    }
    #[doc = "0x1ac - Message Buffer 12 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb12_16b_word1(&self) -> &Mb12_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(428).cast() }
    }
    #[doc = "0x1b0 - Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word4(&self) -> &Mb7_32bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(432).cast() }
    }
    #[doc = "0x1b0 - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word2(&self) -> &Mb4_64bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(432).cast() }
    }
    #[doc = "0x1b0 - Message Buffer 19 CS Register"]
    #[inline(always)]
    pub const fn mb19_8b_cs(&self) -> &Mb19_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(432).cast() }
    }
    #[doc = "0x1b0 - Message Buffer 12 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb12_16b_word2(&self) -> &Mb12_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(432).cast() }
    }
    #[doc = "0x1b0 - Message Buffer 19 CS Register"]
    #[inline(always)]
    pub const fn cs19(&self) -> &Cs19 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(432).cast() }
    }
    #[doc = "0x1b4 - Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word5(&self) -> &Mb7_32bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(436).cast() }
    }
    #[doc = "0x1b4 - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word3(&self) -> &Mb4_64bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(436).cast() }
    }
    #[doc = "0x1b4 - Message Buffer 19 ID Register"]
    #[inline(always)]
    pub const fn mb19_8b_id(&self) -> &Mb19_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(436).cast() }
    }
    #[doc = "0x1b4 - Message Buffer 12 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb12_16b_word3(&self) -> &Mb12_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(436).cast() }
    }
    #[doc = "0x1b4 - Message Buffer 19 ID Register"]
    #[inline(always)]
    pub const fn id19(&self) -> &Id19 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(436).cast() }
    }
    #[doc = "0x1b8 - Message Buffer 19 WORD0 Register"]
    #[inline(always)]
    pub const fn word019(&self) -> &Word019 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(440).cast() }
    }
    #[doc = "0x1b8 - Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word6(&self) -> &Mb7_32bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(440).cast() }
    }
    #[doc = "0x1b8 - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word4(&self) -> &Mb4_64bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(440).cast() }
    }
    #[doc = "0x1b8 - Message Buffer 19 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb19_8b_word0(&self) -> &Mb19_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(440).cast() }
    }
    #[doc = "0x1b8 - Message Buffer 13 CS Register"]
    #[inline(always)]
    pub const fn mb13_16b_cs(&self) -> &Mb13_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(440).cast() }
    }
    #[doc = "0x1bc - Message Buffer 19 WORD1 Register"]
    #[inline(always)]
    pub const fn word119(&self) -> &Word119 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(444).cast() }
    }
    #[doc = "0x1bc - Message Buffer 7 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb7_32b_word7(&self) -> &Mb7_32bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(444).cast() }
    }
    #[doc = "0x1bc - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word5(&self) -> &Mb4_64bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(444).cast() }
    }
    #[doc = "0x1bc - Message Buffer 19 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb19_8b_word1(&self) -> &Mb19_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(444).cast() }
    }
    #[doc = "0x1bc - Message Buffer 13 ID Register"]
    #[inline(always)]
    pub const fn mb13_16b_id(&self) -> &Mb13_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(444).cast() }
    }
    #[doc = "0x1c0 - Message Buffer 8 CS Register"]
    #[inline(always)]
    pub const fn mb8_32b_cs(&self) -> &Mb8_32bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(448).cast() }
    }
    #[doc = "0x1c0 - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word6(&self) -> &Mb4_64bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(448).cast() }
    }
    #[doc = "0x1c0 - Message Buffer 20 CS Register"]
    #[inline(always)]
    pub const fn mb20_8b_cs(&self) -> &Mb20_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(448).cast() }
    }
    #[doc = "0x1c0 - Message Buffer 13 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb13_16b_word0(&self) -> &Mb13_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(448).cast() }
    }
    #[doc = "0x1c0 - Message Buffer 20 CS Register"]
    #[inline(always)]
    pub const fn cs20(&self) -> &Cs20 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(448).cast() }
    }
    #[doc = "0x1c4 - Message Buffer 8 ID Register"]
    #[inline(always)]
    pub const fn mb8_32b_id(&self) -> &Mb8_32bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(452).cast() }
    }
    #[doc = "0x1c4 - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word7(&self) -> &Mb4_64bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(452).cast() }
    }
    #[doc = "0x1c4 - Message Buffer 20 ID Register"]
    #[inline(always)]
    pub const fn mb20_8b_id(&self) -> &Mb20_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(452).cast() }
    }
    #[doc = "0x1c4 - Message Buffer 13 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb13_16b_word1(&self) -> &Mb13_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(452).cast() }
    }
    #[doc = "0x1c4 - Message Buffer 20 ID Register"]
    #[inline(always)]
    pub const fn id20(&self) -> &Id20 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(452).cast() }
    }
    #[doc = "0x1c8 - Message Buffer 20 WORD0 Register"]
    #[inline(always)]
    pub const fn word020(&self) -> &Word020 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(456).cast() }
    }
    #[doc = "0x1c8 - Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word0(&self) -> &Mb8_32bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(456).cast() }
    }
    #[doc = "0x1c8 - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word8(&self) -> &Mb4_64bWord8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(456).cast() }
    }
    #[doc = "0x1c8 - Message Buffer 20 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb20_8b_word0(&self) -> &Mb20_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(456).cast() }
    }
    #[doc = "0x1c8 - Message Buffer 13 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb13_16b_word2(&self) -> &Mb13_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(456).cast() }
    }
    #[doc = "0x1cc - Message Buffer 20 WORD1 Register"]
    #[inline(always)]
    pub const fn word120(&self) -> &Word120 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(460).cast() }
    }
    #[doc = "0x1cc - Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word1(&self) -> &Mb8_32bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(460).cast() }
    }
    #[doc = "0x1cc - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word9(&self) -> &Mb4_64bWord9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(460).cast() }
    }
    #[doc = "0x1cc - Message Buffer 20 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb20_8b_word1(&self) -> &Mb20_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(460).cast() }
    }
    #[doc = "0x1cc - Message Buffer 13 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb13_16b_word3(&self) -> &Mb13_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(460).cast() }
    }
    #[doc = "0x1d0 - Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word2(&self) -> &Mb8_32bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(464).cast() }
    }
    #[doc = "0x1d0 - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word10(&self) -> &Mb4_64bWord10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(464).cast() }
    }
    #[doc = "0x1d0 - Message Buffer 21 CS Register"]
    #[inline(always)]
    pub const fn mb21_8b_cs(&self) -> &Mb21_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(464).cast() }
    }
    #[doc = "0x1d0 - Message Buffer 14 CS Register"]
    #[inline(always)]
    pub const fn mb14_16b_cs(&self) -> &Mb14_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(464).cast() }
    }
    #[doc = "0x1d0 - Message Buffer 21 CS Register"]
    #[inline(always)]
    pub const fn cs21(&self) -> &Cs21 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(464).cast() }
    }
    #[doc = "0x1d4 - Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word3(&self) -> &Mb8_32bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(468).cast() }
    }
    #[doc = "0x1d4 - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word11(&self) -> &Mb4_64bWord11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(468).cast() }
    }
    #[doc = "0x1d4 - Message Buffer 21 ID Register"]
    #[inline(always)]
    pub const fn mb21_8b_id(&self) -> &Mb21_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(468).cast() }
    }
    #[doc = "0x1d4 - Message Buffer 14 ID Register"]
    #[inline(always)]
    pub const fn mb14_16b_id(&self) -> &Mb14_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(468).cast() }
    }
    #[doc = "0x1d4 - Message Buffer 21 ID Register"]
    #[inline(always)]
    pub const fn id21(&self) -> &Id21 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(468).cast() }
    }
    #[doc = "0x1d8 - Message Buffer 21 WORD0 Register"]
    #[inline(always)]
    pub const fn word021(&self) -> &Word021 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(472).cast() }
    }
    #[doc = "0x1d8 - Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word4(&self) -> &Mb8_32bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(472).cast() }
    }
    #[doc = "0x1d8 - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word12(&self) -> &Mb4_64bWord12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(472).cast() }
    }
    #[doc = "0x1d8 - Message Buffer 21 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb21_8b_word0(&self) -> &Mb21_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(472).cast() }
    }
    #[doc = "0x1d8 - Message Buffer 14 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb14_16b_word0(&self) -> &Mb14_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(472).cast() }
    }
    #[doc = "0x1dc - Message Buffer 21 WORD1 Register"]
    #[inline(always)]
    pub const fn word121(&self) -> &Word121 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(476).cast() }
    }
    #[doc = "0x1dc - Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word5(&self) -> &Mb8_32bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(476).cast() }
    }
    #[doc = "0x1dc - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word13(&self) -> &Mb4_64bWord13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(476).cast() }
    }
    #[doc = "0x1dc - Message Buffer 21 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb21_8b_word1(&self) -> &Mb21_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(476).cast() }
    }
    #[doc = "0x1dc - Message Buffer 14 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb14_16b_word1(&self) -> &Mb14_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(476).cast() }
    }
    #[doc = "0x1e0 - Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word6(&self) -> &Mb8_32bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(480).cast() }
    }
    #[doc = "0x1e0 - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word14(&self) -> &Mb4_64bWord14 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(480).cast() }
    }
    #[doc = "0x1e0 - Message Buffer 22 CS Register"]
    #[inline(always)]
    pub const fn mb22_8b_cs(&self) -> &Mb22_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(480).cast() }
    }
    #[doc = "0x1e0 - Message Buffer 14 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb14_16b_word2(&self) -> &Mb14_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(480).cast() }
    }
    #[doc = "0x1e0 - Message Buffer 22 CS Register"]
    #[inline(always)]
    pub const fn cs22(&self) -> &Cs22 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(480).cast() }
    }
    #[doc = "0x1e4 - Message Buffer 8 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb8_32b_word7(&self) -> &Mb8_32bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(484).cast() }
    }
    #[doc = "0x1e4 - Message Buffer 4 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb4_64b_word15(&self) -> &Mb4_64bWord15 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(484).cast() }
    }
    #[doc = "0x1e4 - Message Buffer 22 ID Register"]
    #[inline(always)]
    pub const fn mb22_8b_id(&self) -> &Mb22_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(484).cast() }
    }
    #[doc = "0x1e4 - Message Buffer 14 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb14_16b_word3(&self) -> &Mb14_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(484).cast() }
    }
    #[doc = "0x1e4 - Message Buffer 22 ID Register"]
    #[inline(always)]
    pub const fn id22(&self) -> &Id22 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(484).cast() }
    }
    #[doc = "0x1e8 - Message Buffer 22 WORD0 Register"]
    #[inline(always)]
    pub const fn word022(&self) -> &Word022 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(488).cast() }
    }
    #[doc = "0x1e8 - Message Buffer 9 CS Register"]
    #[inline(always)]
    pub const fn mb9_32b_cs(&self) -> &Mb9_32bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(488).cast() }
    }
    #[doc = "0x1e8 - Message Buffer 5 CS Register"]
    #[inline(always)]
    pub const fn mb5_64b_cs(&self) -> &Mb5_64bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(488).cast() }
    }
    #[doc = "0x1e8 - Message Buffer 22 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb22_8b_word0(&self) -> &Mb22_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(488).cast() }
    }
    #[doc = "0x1e8 - Message Buffer 15 CS Register"]
    #[inline(always)]
    pub const fn mb15_16b_cs(&self) -> &Mb15_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(488).cast() }
    }
    #[doc = "0x1ec - Message Buffer 22 WORD1 Register"]
    #[inline(always)]
    pub const fn word122(&self) -> &Word122 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(492).cast() }
    }
    #[doc = "0x1ec - Message Buffer 9 ID Register"]
    #[inline(always)]
    pub const fn mb9_32b_id(&self) -> &Mb9_32bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(492).cast() }
    }
    #[doc = "0x1ec - Message Buffer 5 ID Register"]
    #[inline(always)]
    pub const fn mb5_64b_id(&self) -> &Mb5_64bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(492).cast() }
    }
    #[doc = "0x1ec - Message Buffer 22 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb22_8b_word1(&self) -> &Mb22_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(492).cast() }
    }
    #[doc = "0x1ec - Message Buffer 15 ID Register"]
    #[inline(always)]
    pub const fn mb15_16b_id(&self) -> &Mb15_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(492).cast() }
    }
    #[doc = "0x1f0 - Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word0(&self) -> &Mb9_32bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(496).cast() }
    }
    #[doc = "0x1f0 - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word0(&self) -> &Mb5_64bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(496).cast() }
    }
    #[doc = "0x1f0 - Message Buffer 23 CS Register"]
    #[inline(always)]
    pub const fn mb23_8b_cs(&self) -> &Mb23_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(496).cast() }
    }
    #[doc = "0x1f0 - Message Buffer 15 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb15_16b_word0(&self) -> &Mb15_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(496).cast() }
    }
    #[doc = "0x1f0 - Message Buffer 23 CS Register"]
    #[inline(always)]
    pub const fn cs23(&self) -> &Cs23 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(496).cast() }
    }
    #[doc = "0x1f4 - Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word1(&self) -> &Mb9_32bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(500).cast() }
    }
    #[doc = "0x1f4 - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word1(&self) -> &Mb5_64bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(500).cast() }
    }
    #[doc = "0x1f4 - Message Buffer 23 ID Register"]
    #[inline(always)]
    pub const fn mb23_8b_id(&self) -> &Mb23_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(500).cast() }
    }
    #[doc = "0x1f4 - Message Buffer 15 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb15_16b_word1(&self) -> &Mb15_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(500).cast() }
    }
    #[doc = "0x1f4 - Message Buffer 23 ID Register"]
    #[inline(always)]
    pub const fn id23(&self) -> &Id23 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(500).cast() }
    }
    #[doc = "0x1f8 - Message Buffer 23 WORD0 Register"]
    #[inline(always)]
    pub const fn word023(&self) -> &Word023 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(504).cast() }
    }
    #[doc = "0x1f8 - Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word2(&self) -> &Mb9_32bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(504).cast() }
    }
    #[doc = "0x1f8 - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word2(&self) -> &Mb5_64bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(504).cast() }
    }
    #[doc = "0x1f8 - Message Buffer 23 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb23_8b_word0(&self) -> &Mb23_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(504).cast() }
    }
    #[doc = "0x1f8 - Message Buffer 15 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb15_16b_word2(&self) -> &Mb15_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(504).cast() }
    }
    #[doc = "0x1fc - Message Buffer 23 WORD1 Register"]
    #[inline(always)]
    pub const fn word123(&self) -> &Word123 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(508).cast() }
    }
    #[doc = "0x1fc - Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word3(&self) -> &Mb9_32bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(508).cast() }
    }
    #[doc = "0x1fc - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word3(&self) -> &Mb5_64bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(508).cast() }
    }
    #[doc = "0x1fc - Message Buffer 23 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb23_8b_word1(&self) -> &Mb23_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(508).cast() }
    }
    #[doc = "0x1fc - Message Buffer 15 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb15_16b_word3(&self) -> &Mb15_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(508).cast() }
    }
    #[doc = "0x200 - Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word4(&self) -> &Mb9_32bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x200 - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word4(&self) -> &Mb5_64bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x200 - Message Buffer 24 CS Register"]
    #[inline(always)]
    pub const fn mb24_8b_cs(&self) -> &Mb24_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x200 - Message Buffer 16 CS Register"]
    #[inline(always)]
    pub const fn mb16_16b_cs(&self) -> &Mb16_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x200 - Message Buffer 24 CS Register"]
    #[inline(always)]
    pub const fn cs24(&self) -> &Cs24 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x204 - Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word5(&self) -> &Mb9_32bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(516).cast() }
    }
    #[doc = "0x204 - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word5(&self) -> &Mb5_64bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(516).cast() }
    }
    #[doc = "0x204 - Message Buffer 24 ID Register"]
    #[inline(always)]
    pub const fn mb24_8b_id(&self) -> &Mb24_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(516).cast() }
    }
    #[doc = "0x204 - Message Buffer 16 ID Register"]
    #[inline(always)]
    pub const fn mb16_16b_id(&self) -> &Mb16_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(516).cast() }
    }
    #[doc = "0x204 - Message Buffer 24 ID Register"]
    #[inline(always)]
    pub const fn id24(&self) -> &Id24 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(516).cast() }
    }
    #[doc = "0x208 - Message Buffer 24 WORD0 Register"]
    #[inline(always)]
    pub const fn word024(&self) -> &Word024 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x208 - Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word6(&self) -> &Mb9_32bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x208 - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word6(&self) -> &Mb5_64bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x208 - Message Buffer 24 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb24_8b_word0(&self) -> &Mb24_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x208 - Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word0(&self) -> &Mb16_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x20c - Message Buffer 24 WORD1 Register"]
    #[inline(always)]
    pub const fn word124(&self) -> &Word124 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(524).cast() }
    }
    #[doc = "0x20c - Message Buffer 9 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb9_32b_word7(&self) -> &Mb9_32bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(524).cast() }
    }
    #[doc = "0x20c - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word7(&self) -> &Mb5_64bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(524).cast() }
    }
    #[doc = "0x20c - Message Buffer 24 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb24_8b_word1(&self) -> &Mb24_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(524).cast() }
    }
    #[doc = "0x20c - Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word1(&self) -> &Mb16_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(524).cast() }
    }
    #[doc = "0x210 - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word8(&self) -> &Mb5_64bWord8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(528).cast() }
    }
    #[doc = "0x210 - Message Buffer 25 CS Register"]
    #[inline(always)]
    pub const fn mb25_8b_cs(&self) -> &Mb25_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(528).cast() }
    }
    #[doc = "0x210 - Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word2(&self) -> &Mb16_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(528).cast() }
    }
    #[doc = "0x210 - Message Buffer 10 CS Register"]
    #[inline(always)]
    pub const fn mb10_32b_cs(&self) -> &Mb10_32bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(528).cast() }
    }
    #[doc = "0x210 - Message Buffer 25 CS Register"]
    #[inline(always)]
    pub const fn cs25(&self) -> &Cs25 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(528).cast() }
    }
    #[doc = "0x214 - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word9(&self) -> &Mb5_64bWord9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(532).cast() }
    }
    #[doc = "0x214 - Message Buffer 25 ID Register"]
    #[inline(always)]
    pub const fn mb25_8b_id(&self) -> &Mb25_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(532).cast() }
    }
    #[doc = "0x214 - Message Buffer 16 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb16_16b_word3(&self) -> &Mb16_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(532).cast() }
    }
    #[doc = "0x214 - Message Buffer 10 ID Register"]
    #[inline(always)]
    pub const fn mb10_32b_id(&self) -> &Mb10_32bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(532).cast() }
    }
    #[doc = "0x214 - Message Buffer 25 ID Register"]
    #[inline(always)]
    pub const fn id25(&self) -> &Id25 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(532).cast() }
    }
    #[doc = "0x218 - Message Buffer 25 WORD0 Register"]
    #[inline(always)]
    pub const fn word025(&self) -> &Word025 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x218 - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word10(&self) -> &Mb5_64bWord10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x218 - Message Buffer 25 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb25_8b_word0(&self) -> &Mb25_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x218 - Message Buffer 17 CS Register"]
    #[inline(always)]
    pub const fn mb17_16b_cs(&self) -> &Mb17_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x218 - Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word0(&self) -> &Mb10_32bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x21c - Message Buffer 25 WORD1 Register"]
    #[inline(always)]
    pub const fn word125(&self) -> &Word125 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(540).cast() }
    }
    #[doc = "0x21c - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word11(&self) -> &Mb5_64bWord11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(540).cast() }
    }
    #[doc = "0x21c - Message Buffer 25 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb25_8b_word1(&self) -> &Mb25_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(540).cast() }
    }
    #[doc = "0x21c - Message Buffer 17 ID Register"]
    #[inline(always)]
    pub const fn mb17_16b_id(&self) -> &Mb17_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(540).cast() }
    }
    #[doc = "0x21c - Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word1(&self) -> &Mb10_32bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(540).cast() }
    }
    #[doc = "0x220 - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word12(&self) -> &Mb5_64bWord12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(544).cast() }
    }
    #[doc = "0x220 - Message Buffer 26 CS Register"]
    #[inline(always)]
    pub const fn mb26_8b_cs(&self) -> &Mb26_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(544).cast() }
    }
    #[doc = "0x220 - Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word0(&self) -> &Mb17_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(544).cast() }
    }
    #[doc = "0x220 - Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word2(&self) -> &Mb10_32bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(544).cast() }
    }
    #[doc = "0x220 - Message Buffer 26 CS Register"]
    #[inline(always)]
    pub const fn cs26(&self) -> &Cs26 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(544).cast() }
    }
    #[doc = "0x224 - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word13(&self) -> &Mb5_64bWord13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(548).cast() }
    }
    #[doc = "0x224 - Message Buffer 26 ID Register"]
    #[inline(always)]
    pub const fn mb26_8b_id(&self) -> &Mb26_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(548).cast() }
    }
    #[doc = "0x224 - Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word1(&self) -> &Mb17_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(548).cast() }
    }
    #[doc = "0x224 - Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word3(&self) -> &Mb10_32bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(548).cast() }
    }
    #[doc = "0x224 - Message Buffer 26 ID Register"]
    #[inline(always)]
    pub const fn id26(&self) -> &Id26 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(548).cast() }
    }
    #[doc = "0x228 - Message Buffer 26 WORD0 Register"]
    #[inline(always)]
    pub const fn word026(&self) -> &Word026 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(552).cast() }
    }
    #[doc = "0x228 - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word14(&self) -> &Mb5_64bWord14 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(552).cast() }
    }
    #[doc = "0x228 - Message Buffer 26 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb26_8b_word0(&self) -> &Mb26_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(552).cast() }
    }
    #[doc = "0x228 - Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word2(&self) -> &Mb17_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(552).cast() }
    }
    #[doc = "0x228 - Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word4(&self) -> &Mb10_32bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(552).cast() }
    }
    #[doc = "0x22c - Message Buffer 26 WORD1 Register"]
    #[inline(always)]
    pub const fn word126(&self) -> &Word126 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(556).cast() }
    }
    #[doc = "0x22c - Message Buffer 5 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb5_64b_word15(&self) -> &Mb5_64bWord15 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(556).cast() }
    }
    #[doc = "0x22c - Message Buffer 26 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb26_8b_word1(&self) -> &Mb26_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(556).cast() }
    }
    #[doc = "0x22c - Message Buffer 17 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb17_16b_word3(&self) -> &Mb17_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(556).cast() }
    }
    #[doc = "0x22c - Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word5(&self) -> &Mb10_32bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(556).cast() }
    }
    #[doc = "0x230 - Message Buffer 6 CS Register"]
    #[inline(always)]
    pub const fn mb6_64b_cs(&self) -> &Mb6_64bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(560).cast() }
    }
    #[doc = "0x230 - Message Buffer 27 CS Register"]
    #[inline(always)]
    pub const fn mb27_8b_cs(&self) -> &Mb27_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(560).cast() }
    }
    #[doc = "0x230 - Message Buffer 18 CS Register"]
    #[inline(always)]
    pub const fn mb18_16b_cs(&self) -> &Mb18_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(560).cast() }
    }
    #[doc = "0x230 - Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word6(&self) -> &Mb10_32bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(560).cast() }
    }
    #[doc = "0x230 - Message Buffer 27 CS Register"]
    #[inline(always)]
    pub const fn cs27(&self) -> &Cs27 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(560).cast() }
    }
    #[doc = "0x234 - Message Buffer 6 ID Register"]
    #[inline(always)]
    pub const fn mb6_64b_id(&self) -> &Mb6_64bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(564).cast() }
    }
    #[doc = "0x234 - Message Buffer 27 ID Register"]
    #[inline(always)]
    pub const fn mb27_8b_id(&self) -> &Mb27_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(564).cast() }
    }
    #[doc = "0x234 - Message Buffer 18 ID Register"]
    #[inline(always)]
    pub const fn mb18_16b_id(&self) -> &Mb18_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(564).cast() }
    }
    #[doc = "0x234 - Message Buffer 10 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb10_32b_word7(&self) -> &Mb10_32bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(564).cast() }
    }
    #[doc = "0x234 - Message Buffer 27 ID Register"]
    #[inline(always)]
    pub const fn id27(&self) -> &Id27 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(564).cast() }
    }
    #[doc = "0x238 - Message Buffer 27 WORD0 Register"]
    #[inline(always)]
    pub const fn word027(&self) -> &Word027 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(568).cast() }
    }
    #[doc = "0x238 - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word0(&self) -> &Mb6_64bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(568).cast() }
    }
    #[doc = "0x238 - Message Buffer 27 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb27_8b_word0(&self) -> &Mb27_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(568).cast() }
    }
    #[doc = "0x238 - Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word0(&self) -> &Mb18_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(568).cast() }
    }
    #[doc = "0x238 - Message Buffer 11 CS Register"]
    #[inline(always)]
    pub const fn mb11_32b_cs(&self) -> &Mb11_32bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(568).cast() }
    }
    #[doc = "0x23c - Message Buffer 27 WORD1 Register"]
    #[inline(always)]
    pub const fn word127(&self) -> &Word127 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(572).cast() }
    }
    #[doc = "0x23c - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word1(&self) -> &Mb6_64bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(572).cast() }
    }
    #[doc = "0x23c - Message Buffer 27 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb27_8b_word1(&self) -> &Mb27_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(572).cast() }
    }
    #[doc = "0x23c - Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word1(&self) -> &Mb18_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(572).cast() }
    }
    #[doc = "0x23c - Message Buffer 11 ID Register"]
    #[inline(always)]
    pub const fn mb11_32b_id(&self) -> &Mb11_32bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(572).cast() }
    }
    #[doc = "0x240 - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word2(&self) -> &Mb6_64bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(576).cast() }
    }
    #[doc = "0x240 - Message Buffer 28 CS Register"]
    #[inline(always)]
    pub const fn mb28_8b_cs(&self) -> &Mb28_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(576).cast() }
    }
    #[doc = "0x240 - Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word2(&self) -> &Mb18_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(576).cast() }
    }
    #[doc = "0x240 - Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word0(&self) -> &Mb11_32bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(576).cast() }
    }
    #[doc = "0x240 - Message Buffer 28 CS Register"]
    #[inline(always)]
    pub const fn cs28(&self) -> &Cs28 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(576).cast() }
    }
    #[doc = "0x244 - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word3(&self) -> &Mb6_64bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(580).cast() }
    }
    #[doc = "0x244 - Message Buffer 28 ID Register"]
    #[inline(always)]
    pub const fn mb28_8b_id(&self) -> &Mb28_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(580).cast() }
    }
    #[doc = "0x244 - Message Buffer 18 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb18_16b_word3(&self) -> &Mb18_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(580).cast() }
    }
    #[doc = "0x244 - Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word1(&self) -> &Mb11_32bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(580).cast() }
    }
    #[doc = "0x244 - Message Buffer 28 ID Register"]
    #[inline(always)]
    pub const fn id28(&self) -> &Id28 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(580).cast() }
    }
    #[doc = "0x248 - Message Buffer 28 WORD0 Register"]
    #[inline(always)]
    pub const fn word028(&self) -> &Word028 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(584).cast() }
    }
    #[doc = "0x248 - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word4(&self) -> &Mb6_64bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(584).cast() }
    }
    #[doc = "0x248 - Message Buffer 28 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb28_8b_word0(&self) -> &Mb28_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(584).cast() }
    }
    #[doc = "0x248 - Message Buffer 19 CS Register"]
    #[inline(always)]
    pub const fn mb19_16b_cs(&self) -> &Mb19_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(584).cast() }
    }
    #[doc = "0x248 - Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word2(&self) -> &Mb11_32bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(584).cast() }
    }
    #[doc = "0x24c - Message Buffer 28 WORD1 Register"]
    #[inline(always)]
    pub const fn word128(&self) -> &Word128 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(588).cast() }
    }
    #[doc = "0x24c - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word5(&self) -> &Mb6_64bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(588).cast() }
    }
    #[doc = "0x24c - Message Buffer 28 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb28_8b_word1(&self) -> &Mb28_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(588).cast() }
    }
    #[doc = "0x24c - Message Buffer 19 ID Register"]
    #[inline(always)]
    pub const fn mb19_16b_id(&self) -> &Mb19_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(588).cast() }
    }
    #[doc = "0x24c - Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word3(&self) -> &Mb11_32bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(588).cast() }
    }
    #[doc = "0x250 - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word6(&self) -> &Mb6_64bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(592).cast() }
    }
    #[doc = "0x250 - Message Buffer 29 CS Register"]
    #[inline(always)]
    pub const fn mb29_8b_cs(&self) -> &Mb29_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(592).cast() }
    }
    #[doc = "0x250 - Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word0(&self) -> &Mb19_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(592).cast() }
    }
    #[doc = "0x250 - Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word4(&self) -> &Mb11_32bWord4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(592).cast() }
    }
    #[doc = "0x250 - Message Buffer 29 CS Register"]
    #[inline(always)]
    pub const fn cs29(&self) -> &Cs29 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(592).cast() }
    }
    #[doc = "0x254 - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word7(&self) -> &Mb6_64bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(596).cast() }
    }
    #[doc = "0x254 - Message Buffer 29 ID Register"]
    #[inline(always)]
    pub const fn mb29_8b_id(&self) -> &Mb29_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(596).cast() }
    }
    #[doc = "0x254 - Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word1(&self) -> &Mb19_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(596).cast() }
    }
    #[doc = "0x254 - Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word5(&self) -> &Mb11_32bWord5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(596).cast() }
    }
    #[doc = "0x254 - Message Buffer 29 ID Register"]
    #[inline(always)]
    pub const fn id29(&self) -> &Id29 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(596).cast() }
    }
    #[doc = "0x258 - Message Buffer 29 WORD0 Register"]
    #[inline(always)]
    pub const fn word029(&self) -> &Word029 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(600).cast() }
    }
    #[doc = "0x258 - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word8(&self) -> &Mb6_64bWord8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(600).cast() }
    }
    #[doc = "0x258 - Message Buffer 29 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb29_8b_word0(&self) -> &Mb29_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(600).cast() }
    }
    #[doc = "0x258 - Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word2(&self) -> &Mb19_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(600).cast() }
    }
    #[doc = "0x258 - Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word6(&self) -> &Mb11_32bWord6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(600).cast() }
    }
    #[doc = "0x25c - Message Buffer 29 WORD1 Register"]
    #[inline(always)]
    pub const fn word129(&self) -> &Word129 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(604).cast() }
    }
    #[doc = "0x25c - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word9(&self) -> &Mb6_64bWord9 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(604).cast() }
    }
    #[doc = "0x25c - Message Buffer 29 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb29_8b_word1(&self) -> &Mb29_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(604).cast() }
    }
    #[doc = "0x25c - Message Buffer 19 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb19_16b_word3(&self) -> &Mb19_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(604).cast() }
    }
    #[doc = "0x25c - Message Buffer 11 WORD_32B Register"]
    #[inline(always)]
    pub const fn mb11_32b_word7(&self) -> &Mb11_32bWord7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(604).cast() }
    }
    #[doc = "0x260 - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word10(&self) -> &Mb6_64bWord10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(608).cast() }
    }
    #[doc = "0x260 - Message Buffer 30 CS Register"]
    #[inline(always)]
    pub const fn mb30_8b_cs(&self) -> &Mb30_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(608).cast() }
    }
    #[doc = "0x260 - Message Buffer 20 CS Register"]
    #[inline(always)]
    pub const fn mb20_16b_cs(&self) -> &Mb20_16bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(608).cast() }
    }
    #[doc = "0x260 - Message Buffer 30 CS Register"]
    #[inline(always)]
    pub const fn cs30(&self) -> &Cs30 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(608).cast() }
    }
    #[doc = "0x264 - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word11(&self) -> &Mb6_64bWord11 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(612).cast() }
    }
    #[doc = "0x264 - Message Buffer 30 ID Register"]
    #[inline(always)]
    pub const fn mb30_8b_id(&self) -> &Mb30_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(612).cast() }
    }
    #[doc = "0x264 - Message Buffer 20 ID Register"]
    #[inline(always)]
    pub const fn mb20_16b_id(&self) -> &Mb20_16bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(612).cast() }
    }
    #[doc = "0x264 - Message Buffer 30 ID Register"]
    #[inline(always)]
    pub const fn id30(&self) -> &Id30 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(612).cast() }
    }
    #[doc = "0x268 - Message Buffer 30 WORD0 Register"]
    #[inline(always)]
    pub const fn word030(&self) -> &Word030 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(616).cast() }
    }
    #[doc = "0x268 - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word12(&self) -> &Mb6_64bWord12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(616).cast() }
    }
    #[doc = "0x268 - Message Buffer 30 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb30_8b_word0(&self) -> &Mb30_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(616).cast() }
    }
    #[doc = "0x268 - Message Buffer 20 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb20_16b_word0(&self) -> &Mb20_16bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(616).cast() }
    }
    #[doc = "0x26c - Message Buffer 30 WORD1 Register"]
    #[inline(always)]
    pub const fn word130(&self) -> &Word130 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(620).cast() }
    }
    #[doc = "0x26c - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word13(&self) -> &Mb6_64bWord13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(620).cast() }
    }
    #[doc = "0x26c - Message Buffer 30 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb30_8b_word1(&self) -> &Mb30_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(620).cast() }
    }
    #[doc = "0x26c - Message Buffer 20 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb20_16b_word1(&self) -> &Mb20_16bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(620).cast() }
    }
    #[doc = "0x270 - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word14(&self) -> &Mb6_64bWord14 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(624).cast() }
    }
    #[doc = "0x270 - Message Buffer 31 CS Register"]
    #[inline(always)]
    pub const fn mb31_8b_cs(&self) -> &Mb31_8bCs {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(624).cast() }
    }
    #[doc = "0x270 - Message Buffer 20 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb20_16b_word2(&self) -> &Mb20_16bWord2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(624).cast() }
    }
    #[doc = "0x270 - Message Buffer 31 CS Register"]
    #[inline(always)]
    pub const fn cs31(&self) -> &Cs31 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(624).cast() }
    }
    #[doc = "0x274 - Message Buffer 6 WORD_64B Register"]
    #[inline(always)]
    pub const fn mb6_64b_word15(&self) -> &Mb6_64bWord15 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(628).cast() }
    }
    #[doc = "0x274 - Message Buffer 31 ID Register"]
    #[inline(always)]
    pub const fn mb31_8b_id(&self) -> &Mb31_8bId {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(628).cast() }
    }
    #[doc = "0x274 - Message Buffer 20 WORD_16B Register"]
    #[inline(always)]
    pub const fn mb20_16b_word3(&self) -> &Mb20_16bWord3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(628).cast() }
    }
    #[doc = "0x274 - Message Buffer 31 ID Register"]
    #[inline(always)]
    pub const fn id31(&self) -> &Id31 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(628).cast() }
    }
    #[doc = "0x278 - Message Buffer 31 WORD0 Register"]
    #[inline(always)]
    pub const fn word031(&self) -> &Word031 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(632).cast() }
    }
    #[doc = "0x278 - Message Buffer 31 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb31_8b_word0(&self) -> &Mb31_8bWord0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(632).cast() }
    }
    #[doc = "0x27c - Message Buffer 31 WORD1 Register"]
    #[inline(always)]
    pub const fn word131(&self) -> &Word131 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(636).cast() }
    }
    #[doc = "0x27c - Message Buffer 31 WORD_8B Register"]
    #[inline(always)]
    pub const fn mb31_8b_word1(&self) -> &Mb31_8bWord1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(636).cast() }
    }
    #[doc = "0x880..0x900 - Receive Individual Mask"]
    #[inline(always)]
    pub const fn rximr(&self, n: usize) -> &Rximr {
        &self.rximr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x880..0x900 - Receive Individual Mask"]
    #[inline(always)]
    pub fn rximr_iter(&self) -> impl Iterator<Item = &Rximr> {
        self.rximr.iter()
    }
    #[doc = "0xb00 - Pretended Networking Control 1"]
    #[inline(always)]
    pub const fn ctrl1_pn(&self) -> &Ctrl1Pn {
        &self.ctrl1_pn
    }
    #[doc = "0xb04 - Pretended Networking Control 2"]
    #[inline(always)]
    pub const fn ctrl2_pn(&self) -> &Ctrl2Pn {
        &self.ctrl2_pn
    }
    #[doc = "0xb08 - Pretended Networking Wake-Up Match"]
    #[inline(always)]
    pub const fn wu_mtc(&self) -> &WuMtc {
        &self.wu_mtc
    }
    #[doc = "0xb0c - Pretended Networking ID Filter 1"]
    #[inline(always)]
    pub const fn flt_id1(&self) -> &FltId1 {
        &self.flt_id1
    }
    #[doc = "0xb10 - Pretended Networking Data Length Code (DLC) Filter"]
    #[inline(always)]
    pub const fn flt_dlc(&self) -> &FltDlc {
        &self.flt_dlc
    }
    #[doc = "0xb14 - Pretended Networking Payload Low Filter 1"]
    #[inline(always)]
    pub const fn pl1_lo(&self) -> &Pl1Lo {
        &self.pl1_lo
    }
    #[doc = "0xb18 - Pretended Networking Payload High Filter 1"]
    #[inline(always)]
    pub const fn pl1_hi(&self) -> &Pl1Hi {
        &self.pl1_hi
    }
    #[doc = "0xb1c - Pretended Networking ID Filter 2 or ID Mask"]
    #[inline(always)]
    pub const fn flt_id2_idmask(&self) -> &FltId2Idmask {
        &self.flt_id2_idmask
    }
    #[doc = "0xb20 - Pretended Networking Payload Low Filter 2 and Payload Low Mask"]
    #[inline(always)]
    pub const fn pl2_plmask_lo(&self) -> &Pl2PlmaskLo {
        &self.pl2_plmask_lo
    }
    #[doc = "0xb24 - Pretended Networking Payload High Filter 2 and Payload High Mask"]
    #[inline(always)]
    pub const fn pl2_plmask_hi(&self) -> &Pl2PlmaskHi {
        &self.pl2_plmask_hi
    }
    #[doc = "0xb40..0xb80 - Array of registers: WMB_CS, WMB_D03, WMB_D47, WMB_ID"]
    #[inline(always)]
    pub const fn wmb(&self, n: usize) -> &Wmb {
        &self.wmb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xb40..0xb80 - Array of registers: WMB_CS, WMB_D03, WMB_D47, WMB_ID"]
    #[inline(always)]
    pub fn wmb_iter(&self) -> impl Iterator<Item = &Wmb> {
        self.wmb.iter()
    }
    #[doc = "0xbf0 - Enhanced CAN Bit Timing Prescalers"]
    #[inline(always)]
    pub const fn eprs(&self) -> &Eprs {
        &self.eprs
    }
    #[doc = "0xbf4 - Enhanced Nominal CAN Bit Timing"]
    #[inline(always)]
    pub const fn encbt(&self) -> &Encbt {
        &self.encbt
    }
    #[doc = "0xbf8 - Enhanced Data Phase CAN Bit Timing"]
    #[inline(always)]
    pub const fn edcbt(&self) -> &Edcbt {
        &self.edcbt
    }
    #[doc = "0xbfc - Enhanced Transceiver Delay Compensation"]
    #[inline(always)]
    pub const fn etdc(&self) -> &Etdc {
        &self.etdc
    }
    #[doc = "0xc00 - CAN FD Control"]
    #[inline(always)]
    pub const fn fdctrl(&self) -> &Fdctrl {
        &self.fdctrl
    }
    #[doc = "0xc04 - CAN FD Bit Timing"]
    #[inline(always)]
    pub const fn fdcbt(&self) -> &Fdcbt {
        &self.fdcbt
    }
    #[doc = "0xc08 - CAN FD CRC"]
    #[inline(always)]
    pub const fn fdcrc(&self) -> &Fdcrc {
        &self.fdcrc
    }
    #[doc = "0xc0c - Enhanced RX FIFO Control"]
    #[inline(always)]
    pub const fn erfcr(&self) -> &Erfcr {
        &self.erfcr
    }
    #[doc = "0xc10 - Enhanced RX FIFO Interrupt Enable"]
    #[inline(always)]
    pub const fn erfier(&self) -> &Erfier {
        &self.erfier
    }
    #[doc = "0xc14 - Enhanced RX FIFO Status"]
    #[inline(always)]
    pub const fn erfsr(&self) -> &Erfsr {
        &self.erfsr
    }
    #[doc = "0x3000..0x3080 - Enhanced RX FIFO Filter Element"]
    #[inline(always)]
    pub const fn erffel(&self, n: usize) -> &Erffel {
        &self.erffel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3000..0x3080 - Enhanced RX FIFO Filter Element"]
    #[inline(always)]
    pub fn erffel_iter(&self) -> impl Iterator<Item = &Erffel> {
        self.erffel.iter()
    }
}
#[doc = "MCR (rw) register accessor: Module Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`] module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "Module Configuration"]
pub mod mcr;
#[doc = "CTRL1 (rw) register accessor: Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "Control 1"]
pub mod ctrl1;
#[doc = "TIMER (rw) register accessor: Free-Running Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer`] module"]
#[doc(alias = "TIMER")]
pub type Timer = crate::Reg<timer::TimerSpec>;
#[doc = "Free-Running Timer"]
pub mod timer;
#[doc = "RXMGMASK (rw) register accessor: RX Message Buffers Global Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmgmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxmgmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmgmask`] module"]
#[doc(alias = "RXMGMASK")]
pub type Rxmgmask = crate::Reg<rxmgmask::RxmgmaskSpec>;
#[doc = "RX Message Buffers Global Mask"]
pub mod rxmgmask;
#[doc = "RX14MASK (rw) register accessor: Receive 14 Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`rx14mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx14mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx14mask`] module"]
#[doc(alias = "RX14MASK")]
pub type Rx14mask = crate::Reg<rx14mask::Rx14maskSpec>;
#[doc = "Receive 14 Mask"]
pub mod rx14mask;
#[doc = "RX15MASK (rw) register accessor: Receive 15 Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`rx15mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx15mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx15mask`] module"]
#[doc(alias = "RX15MASK")]
pub type Rx15mask = crate::Reg<rx15mask::Rx15maskSpec>;
#[doc = "Receive 15 Mask"]
pub mod rx15mask;
#[doc = "ECR (rw) register accessor: Error Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`] module"]
#[doc(alias = "ECR")]
pub type Ecr = crate::Reg<ecr::EcrSpec>;
#[doc = "Error Counter"]
pub mod ecr;
#[doc = "ESR1 (rw) register accessor: Error and Status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`esr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esr1`] module"]
#[doc(alias = "ESR1")]
pub type Esr1 = crate::Reg<esr1::Esr1Spec>;
#[doc = "Error and Status 1"]
pub mod esr1;
#[doc = "IMASK1 (rw) register accessor: Interrupt Masks 1\n\nYou can [`read`](crate::Reg::read) this register and get [`imask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask1`] module"]
#[doc(alias = "IMASK1")]
pub type Imask1 = crate::Reg<imask1::Imask1Spec>;
#[doc = "Interrupt Masks 1"]
pub mod imask1;
#[doc = "IFLAG1 (rw) register accessor: Interrupt Flags 1\n\nYou can [`read`](crate::Reg::read) this register and get [`iflag1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iflag1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iflag1`] module"]
#[doc(alias = "IFLAG1")]
pub type Iflag1 = crate::Reg<iflag1::Iflag1Spec>;
#[doc = "Interrupt Flags 1"]
pub mod iflag1;
#[doc = "CTRL2 (rw) register accessor: Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "Control 2"]
pub mod ctrl2;
#[doc = "ESR2 (r) register accessor: Error and Status 2\n\nYou can [`read`](crate::Reg::read) this register and get [`esr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esr2`] module"]
#[doc(alias = "ESR2")]
pub type Esr2 = crate::Reg<esr2::Esr2Spec>;
#[doc = "Error and Status 2"]
pub mod esr2;
#[doc = "CRCR (r) register accessor: Cyclic Redundancy Check\n\nYou can [`read`](crate::Reg::read) this register and get [`crcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcr`] module"]
#[doc(alias = "CRCR")]
pub type Crcr = crate::Reg<crcr::CrcrSpec>;
#[doc = "Cyclic Redundancy Check"]
pub mod crcr;
#[doc = "RXFGMASK (rw) register accessor: Legacy RX FIFO Global Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfgmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfgmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfgmask`] module"]
#[doc(alias = "RXFGMASK")]
pub type Rxfgmask = crate::Reg<rxfgmask::RxfgmaskSpec>;
#[doc = "Legacy RX FIFO Global Mask"]
pub mod rxfgmask;
#[doc = "RXFIR (r) register accessor: Legacy RX FIFO Information\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfir`] module"]
#[doc(alias = "RXFIR")]
pub type Rxfir = crate::Reg<rxfir::RxfirSpec>;
#[doc = "Legacy RX FIFO Information"]
pub mod rxfir;
#[doc = "CBT (rw) register accessor: CAN Bit Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`cbt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cbt`] module"]
#[doc(alias = "CBT")]
pub type Cbt = crate::Reg<cbt::CbtSpec>;
#[doc = "CAN Bit Timing"]
pub mod cbt;
#[doc = "CS0 (rw) register accessor: Message Buffer 0 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs0`] module"]
#[doc(alias = "CS0")]
pub type Cs0 = crate::Reg<cs0::Cs0Spec>;
#[doc = "Message Buffer 0 CS Register"]
pub mod cs0;
#[doc = "MB0_16B_CS (rw) register accessor: Message Buffer 0 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_16b_cs`] module"]
#[doc(alias = "MB0_16B_CS")]
pub type Mb0_16bCs = crate::Reg<mb0_16b_cs::Mb0_16bCsSpec>;
#[doc = "Message Buffer 0 CS Register"]
pub mod mb0_16b_cs;
#[doc = "MB0_32B_CS (rw) register accessor: Message Buffer 0 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_32b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_32b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_32b_cs`] module"]
#[doc(alias = "MB0_32B_CS")]
pub type Mb0_32bCs = crate::Reg<mb0_32b_cs::Mb0_32bCsSpec>;
#[doc = "Message Buffer 0 CS Register"]
pub mod mb0_32b_cs;
#[doc = "MB0_64B_CS (rw) register accessor: Message Buffer 0 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_cs`] module"]
#[doc(alias = "MB0_64B_CS")]
pub type Mb0_64bCs = crate::Reg<mb0_64b_cs::Mb0_64bCsSpec>;
#[doc = "Message Buffer 0 CS Register"]
pub mod mb0_64b_cs;
#[doc = "MB0_8B_CS (rw) register accessor: Message Buffer 0 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_8b_cs`] module"]
#[doc(alias = "MB0_8B_CS")]
pub type Mb0_8bCs = crate::Reg<mb0_8b_cs::Mb0_8bCsSpec>;
#[doc = "Message Buffer 0 CS Register"]
pub mod mb0_8b_cs;
#[doc = "ID0 (rw) register accessor: Message Buffer 0 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0`] module"]
#[doc(alias = "ID0")]
pub type Id0 = crate::Reg<id0::Id0Spec>;
#[doc = "Message Buffer 0 ID Register"]
pub mod id0;
#[doc = "MB0_16B_ID (rw) register accessor: Message Buffer 0 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_16b_id`] module"]
#[doc(alias = "MB0_16B_ID")]
pub type Mb0_16bId = crate::Reg<mb0_16b_id::Mb0_16bIdSpec>;
#[doc = "Message Buffer 0 ID Register"]
pub mod mb0_16b_id;
#[doc = "MB0_32B_ID (rw) register accessor: Message Buffer 0 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_32b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_32b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_32b_id`] module"]
#[doc(alias = "MB0_32B_ID")]
pub type Mb0_32bId = crate::Reg<mb0_32b_id::Mb0_32bIdSpec>;
#[doc = "Message Buffer 0 ID Register"]
pub mod mb0_32b_id;
#[doc = "MB0_64B_ID (rw) register accessor: Message Buffer 0 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_id`] module"]
#[doc(alias = "MB0_64B_ID")]
pub type Mb0_64bId = crate::Reg<mb0_64b_id::Mb0_64bIdSpec>;
#[doc = "Message Buffer 0 ID Register"]
pub mod mb0_64b_id;
#[doc = "MB0_8B_ID (rw) register accessor: Message Buffer 0 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_8b_id`] module"]
#[doc(alias = "MB0_8B_ID")]
pub type Mb0_8bId = crate::Reg<mb0_8b_id::Mb0_8bIdSpec>;
#[doc = "Message Buffer 0 ID Register"]
pub mod mb0_8b_id;
#[doc = "MB0_16B_WORD0 (rw) register accessor: Message Buffer 0 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_16b_word0`] module"]
#[doc(alias = "MB0_16B_WORD0")]
pub type Mb0_16bWord0 = crate::Reg<mb0_16b_word0::Mb0_16bWord0Spec>;
#[doc = "Message Buffer 0 WORD_16B Register"]
pub mod mb0_16b_word0;
#[doc = "MB0_32B_WORD0 (rw) register accessor: Message Buffer 0 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_32b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_32b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_32b_word0`] module"]
#[doc(alias = "MB0_32B_WORD0")]
pub type Mb0_32bWord0 = crate::Reg<mb0_32b_word0::Mb0_32bWord0Spec>;
#[doc = "Message Buffer 0 WORD_32B Register"]
pub mod mb0_32b_word0;
#[doc = "MB0_64B_WORD0 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word0`] module"]
#[doc(alias = "MB0_64B_WORD0")]
pub type Mb0_64bWord0 = crate::Reg<mb0_64b_word0::Mb0_64bWord0Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word0;
#[doc = "MB0_8B_WORD0 (rw) register accessor: Message Buffer 0 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_8b_word0`] module"]
#[doc(alias = "MB0_8B_WORD0")]
pub type Mb0_8bWord0 = crate::Reg<mb0_8b_word0::Mb0_8bWord0Spec>;
#[doc = "Message Buffer 0 WORD_8B Register"]
pub mod mb0_8b_word0;
#[doc = "WORD00 (rw) register accessor: Message Buffer 0 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word00`] module"]
#[doc(alias = "WORD00")]
pub type Word00 = crate::Reg<word00::Word00Spec>;
#[doc = "Message Buffer 0 WORD0 Register"]
pub mod word00;
#[doc = "MB0_16B_WORD1 (rw) register accessor: Message Buffer 0 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_16b_word1`] module"]
#[doc(alias = "MB0_16B_WORD1")]
pub type Mb0_16bWord1 = crate::Reg<mb0_16b_word1::Mb0_16bWord1Spec>;
#[doc = "Message Buffer 0 WORD_16B Register"]
pub mod mb0_16b_word1;
#[doc = "MB0_32B_WORD1 (rw) register accessor: Message Buffer 0 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_32b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_32b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_32b_word1`] module"]
#[doc(alias = "MB0_32B_WORD1")]
pub type Mb0_32bWord1 = crate::Reg<mb0_32b_word1::Mb0_32bWord1Spec>;
#[doc = "Message Buffer 0 WORD_32B Register"]
pub mod mb0_32b_word1;
#[doc = "MB0_64B_WORD1 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word1`] module"]
#[doc(alias = "MB0_64B_WORD1")]
pub type Mb0_64bWord1 = crate::Reg<mb0_64b_word1::Mb0_64bWord1Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word1;
#[doc = "MB0_8B_WORD1 (rw) register accessor: Message Buffer 0 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_8b_word1`] module"]
#[doc(alias = "MB0_8B_WORD1")]
pub type Mb0_8bWord1 = crate::Reg<mb0_8b_word1::Mb0_8bWord1Spec>;
#[doc = "Message Buffer 0 WORD_8B Register"]
pub mod mb0_8b_word1;
#[doc = "WORD10 (rw) register accessor: Message Buffer 0 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word10`] module"]
#[doc(alias = "WORD10")]
pub type Word10 = crate::Reg<word10::Word10Spec>;
#[doc = "Message Buffer 0 WORD1 Register"]
pub mod word10;
#[doc = "CS1 (rw) register accessor: Message Buffer 1 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs1`] module"]
#[doc(alias = "CS1")]
pub type Cs1 = crate::Reg<cs1::Cs1Spec>;
#[doc = "Message Buffer 1 CS Register"]
pub mod cs1;
#[doc = "MB0_16B_WORD2 (rw) register accessor: Message Buffer 0 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_16b_word2`] module"]
#[doc(alias = "MB0_16B_WORD2")]
pub type Mb0_16bWord2 = crate::Reg<mb0_16b_word2::Mb0_16bWord2Spec>;
#[doc = "Message Buffer 0 WORD_16B Register"]
pub mod mb0_16b_word2;
#[doc = "MB0_32B_WORD2 (rw) register accessor: Message Buffer 0 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_32b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_32b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_32b_word2`] module"]
#[doc(alias = "MB0_32B_WORD2")]
pub type Mb0_32bWord2 = crate::Reg<mb0_32b_word2::Mb0_32bWord2Spec>;
#[doc = "Message Buffer 0 WORD_32B Register"]
pub mod mb0_32b_word2;
#[doc = "MB0_64B_WORD2 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word2`] module"]
#[doc(alias = "MB0_64B_WORD2")]
pub type Mb0_64bWord2 = crate::Reg<mb0_64b_word2::Mb0_64bWord2Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word2;
#[doc = "MB1_8B_CS (rw) register accessor: Message Buffer 1 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_8b_cs`] module"]
#[doc(alias = "MB1_8B_CS")]
pub type Mb1_8bCs = crate::Reg<mb1_8b_cs::Mb1_8bCsSpec>;
#[doc = "Message Buffer 1 CS Register"]
pub mod mb1_8b_cs;
#[doc = "ID1 (rw) register accessor: Message Buffer 1 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id1`] module"]
#[doc(alias = "ID1")]
pub type Id1 = crate::Reg<id1::Id1Spec>;
#[doc = "Message Buffer 1 ID Register"]
pub mod id1;
#[doc = "MB0_16B_WORD3 (rw) register accessor: Message Buffer 0 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_16b_word3`] module"]
#[doc(alias = "MB0_16B_WORD3")]
pub type Mb0_16bWord3 = crate::Reg<mb0_16b_word3::Mb0_16bWord3Spec>;
#[doc = "Message Buffer 0 WORD_16B Register"]
pub mod mb0_16b_word3;
#[doc = "MB0_32B_WORD3 (rw) register accessor: Message Buffer 0 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_32b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_32b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_32b_word3`] module"]
#[doc(alias = "MB0_32B_WORD3")]
pub type Mb0_32bWord3 = crate::Reg<mb0_32b_word3::Mb0_32bWord3Spec>;
#[doc = "Message Buffer 0 WORD_32B Register"]
pub mod mb0_32b_word3;
#[doc = "MB0_64B_WORD3 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word3`] module"]
#[doc(alias = "MB0_64B_WORD3")]
pub type Mb0_64bWord3 = crate::Reg<mb0_64b_word3::Mb0_64bWord3Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word3;
#[doc = "MB1_8B_ID (rw) register accessor: Message Buffer 1 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_8b_id`] module"]
#[doc(alias = "MB1_8B_ID")]
pub type Mb1_8bId = crate::Reg<mb1_8b_id::Mb1_8bIdSpec>;
#[doc = "Message Buffer 1 ID Register"]
pub mod mb1_8b_id;
#[doc = "MB0_32B_WORD4 (rw) register accessor: Message Buffer 0 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_32b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_32b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_32b_word4`] module"]
#[doc(alias = "MB0_32B_WORD4")]
pub type Mb0_32bWord4 = crate::Reg<mb0_32b_word4::Mb0_32bWord4Spec>;
#[doc = "Message Buffer 0 WORD_32B Register"]
pub mod mb0_32b_word4;
#[doc = "MB0_64B_WORD4 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word4`] module"]
#[doc(alias = "MB0_64B_WORD4")]
pub type Mb0_64bWord4 = crate::Reg<mb0_64b_word4::Mb0_64bWord4Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word4;
#[doc = "MB1_16B_CS (rw) register accessor: Message Buffer 1 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_16b_cs`] module"]
#[doc(alias = "MB1_16B_CS")]
pub type Mb1_16bCs = crate::Reg<mb1_16b_cs::Mb1_16bCsSpec>;
#[doc = "Message Buffer 1 CS Register"]
pub mod mb1_16b_cs;
#[doc = "MB1_8B_WORD0 (rw) register accessor: Message Buffer 1 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_8b_word0`] module"]
#[doc(alias = "MB1_8B_WORD0")]
pub type Mb1_8bWord0 = crate::Reg<mb1_8b_word0::Mb1_8bWord0Spec>;
#[doc = "Message Buffer 1 WORD_8B Register"]
pub mod mb1_8b_word0;
#[doc = "WORD01 (rw) register accessor: Message Buffer 1 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word01`] module"]
#[doc(alias = "WORD01")]
pub type Word01 = crate::Reg<word01::Word01Spec>;
#[doc = "Message Buffer 1 WORD0 Register"]
pub mod word01;
#[doc = "MB0_32B_WORD5 (rw) register accessor: Message Buffer 0 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_32b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_32b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_32b_word5`] module"]
#[doc(alias = "MB0_32B_WORD5")]
pub type Mb0_32bWord5 = crate::Reg<mb0_32b_word5::Mb0_32bWord5Spec>;
#[doc = "Message Buffer 0 WORD_32B Register"]
pub mod mb0_32b_word5;
#[doc = "MB0_64B_WORD5 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word5`] module"]
#[doc(alias = "MB0_64B_WORD5")]
pub type Mb0_64bWord5 = crate::Reg<mb0_64b_word5::Mb0_64bWord5Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word5;
#[doc = "MB1_16B_ID (rw) register accessor: Message Buffer 1 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_16b_id`] module"]
#[doc(alias = "MB1_16B_ID")]
pub type Mb1_16bId = crate::Reg<mb1_16b_id::Mb1_16bIdSpec>;
#[doc = "Message Buffer 1 ID Register"]
pub mod mb1_16b_id;
#[doc = "MB1_8B_WORD1 (rw) register accessor: Message Buffer 1 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_8b_word1`] module"]
#[doc(alias = "MB1_8B_WORD1")]
pub type Mb1_8bWord1 = crate::Reg<mb1_8b_word1::Mb1_8bWord1Spec>;
#[doc = "Message Buffer 1 WORD_8B Register"]
pub mod mb1_8b_word1;
#[doc = "WORD11 (rw) register accessor: Message Buffer 1 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word11`] module"]
#[doc(alias = "WORD11")]
pub type Word11 = crate::Reg<word11::Word11Spec>;
#[doc = "Message Buffer 1 WORD1 Register"]
pub mod word11;
#[doc = "CS2 (rw) register accessor: Message Buffer 2 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs2`] module"]
#[doc(alias = "CS2")]
pub type Cs2 = crate::Reg<cs2::Cs2Spec>;
#[doc = "Message Buffer 2 CS Register"]
pub mod cs2;
#[doc = "MB0_32B_WORD6 (rw) register accessor: Message Buffer 0 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_32b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_32b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_32b_word6`] module"]
#[doc(alias = "MB0_32B_WORD6")]
pub type Mb0_32bWord6 = crate::Reg<mb0_32b_word6::Mb0_32bWord6Spec>;
#[doc = "Message Buffer 0 WORD_32B Register"]
pub mod mb0_32b_word6;
#[doc = "MB0_64B_WORD6 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word6`] module"]
#[doc(alias = "MB0_64B_WORD6")]
pub type Mb0_64bWord6 = crate::Reg<mb0_64b_word6::Mb0_64bWord6Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word6;
#[doc = "MB1_16B_WORD0 (rw) register accessor: Message Buffer 1 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_16b_word0`] module"]
#[doc(alias = "MB1_16B_WORD0")]
pub type Mb1_16bWord0 = crate::Reg<mb1_16b_word0::Mb1_16bWord0Spec>;
#[doc = "Message Buffer 1 WORD_16B Register"]
pub mod mb1_16b_word0;
#[doc = "MB2_8B_CS (rw) register accessor: Message Buffer 2 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_8b_cs`] module"]
#[doc(alias = "MB2_8B_CS")]
pub type Mb2_8bCs = crate::Reg<mb2_8b_cs::Mb2_8bCsSpec>;
#[doc = "Message Buffer 2 CS Register"]
pub mod mb2_8b_cs;
#[doc = "ID2 (rw) register accessor: Message Buffer 2 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id2`] module"]
#[doc(alias = "ID2")]
pub type Id2 = crate::Reg<id2::Id2Spec>;
#[doc = "Message Buffer 2 ID Register"]
pub mod id2;
#[doc = "MB0_32B_WORD7 (rw) register accessor: Message Buffer 0 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_32b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_32b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_32b_word7`] module"]
#[doc(alias = "MB0_32B_WORD7")]
pub type Mb0_32bWord7 = crate::Reg<mb0_32b_word7::Mb0_32bWord7Spec>;
#[doc = "Message Buffer 0 WORD_32B Register"]
pub mod mb0_32b_word7;
#[doc = "MB0_64B_WORD7 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word7`] module"]
#[doc(alias = "MB0_64B_WORD7")]
pub type Mb0_64bWord7 = crate::Reg<mb0_64b_word7::Mb0_64bWord7Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word7;
#[doc = "MB1_16B_WORD1 (rw) register accessor: Message Buffer 1 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_16b_word1`] module"]
#[doc(alias = "MB1_16B_WORD1")]
pub type Mb1_16bWord1 = crate::Reg<mb1_16b_word1::Mb1_16bWord1Spec>;
#[doc = "Message Buffer 1 WORD_16B Register"]
pub mod mb1_16b_word1;
#[doc = "MB2_8B_ID (rw) register accessor: Message Buffer 2 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_8b_id`] module"]
#[doc(alias = "MB2_8B_ID")]
pub type Mb2_8bId = crate::Reg<mb2_8b_id::Mb2_8bIdSpec>;
#[doc = "Message Buffer 2 ID Register"]
pub mod mb2_8b_id;
#[doc = "MB0_64B_WORD8 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word8`] module"]
#[doc(alias = "MB0_64B_WORD8")]
pub type Mb0_64bWord8 = crate::Reg<mb0_64b_word8::Mb0_64bWord8Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word8;
#[doc = "MB1_16B_WORD2 (rw) register accessor: Message Buffer 1 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_16b_word2`] module"]
#[doc(alias = "MB1_16B_WORD2")]
pub type Mb1_16bWord2 = crate::Reg<mb1_16b_word2::Mb1_16bWord2Spec>;
#[doc = "Message Buffer 1 WORD_16B Register"]
pub mod mb1_16b_word2;
#[doc = "MB1_32B_CS (rw) register accessor: Message Buffer 1 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_32b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_32b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_32b_cs`] module"]
#[doc(alias = "MB1_32B_CS")]
pub type Mb1_32bCs = crate::Reg<mb1_32b_cs::Mb1_32bCsSpec>;
#[doc = "Message Buffer 1 CS Register"]
pub mod mb1_32b_cs;
#[doc = "MB2_8B_WORD0 (rw) register accessor: Message Buffer 2 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_8b_word0`] module"]
#[doc(alias = "MB2_8B_WORD0")]
pub type Mb2_8bWord0 = crate::Reg<mb2_8b_word0::Mb2_8bWord0Spec>;
#[doc = "Message Buffer 2 WORD_8B Register"]
pub mod mb2_8b_word0;
#[doc = "WORD02 (rw) register accessor: Message Buffer 2 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word02`] module"]
#[doc(alias = "WORD02")]
pub type Word02 = crate::Reg<word02::Word02Spec>;
#[doc = "Message Buffer 2 WORD0 Register"]
pub mod word02;
#[doc = "MB0_64B_WORD9 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word9`] module"]
#[doc(alias = "MB0_64B_WORD9")]
pub type Mb0_64bWord9 = crate::Reg<mb0_64b_word9::Mb0_64bWord9Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word9;
#[doc = "MB1_16B_WORD3 (rw) register accessor: Message Buffer 1 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_16b_word3`] module"]
#[doc(alias = "MB1_16B_WORD3")]
pub type Mb1_16bWord3 = crate::Reg<mb1_16b_word3::Mb1_16bWord3Spec>;
#[doc = "Message Buffer 1 WORD_16B Register"]
pub mod mb1_16b_word3;
#[doc = "MB1_32B_ID (rw) register accessor: Message Buffer 1 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_32b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_32b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_32b_id`] module"]
#[doc(alias = "MB1_32B_ID")]
pub type Mb1_32bId = crate::Reg<mb1_32b_id::Mb1_32bIdSpec>;
#[doc = "Message Buffer 1 ID Register"]
pub mod mb1_32b_id;
#[doc = "MB2_8B_WORD1 (rw) register accessor: Message Buffer 2 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_8b_word1`] module"]
#[doc(alias = "MB2_8B_WORD1")]
pub type Mb2_8bWord1 = crate::Reg<mb2_8b_word1::Mb2_8bWord1Spec>;
#[doc = "Message Buffer 2 WORD_8B Register"]
pub mod mb2_8b_word1;
#[doc = "WORD12 (rw) register accessor: Message Buffer 2 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word12`] module"]
#[doc(alias = "WORD12")]
pub type Word12 = crate::Reg<word12::Word12Spec>;
#[doc = "Message Buffer 2 WORD1 Register"]
pub mod word12;
#[doc = "CS3 (rw) register accessor: Message Buffer 3 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs3`] module"]
#[doc(alias = "CS3")]
pub type Cs3 = crate::Reg<cs3::Cs3Spec>;
#[doc = "Message Buffer 3 CS Register"]
pub mod cs3;
#[doc = "MB0_64B_WORD10 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word10`] module"]
#[doc(alias = "MB0_64B_WORD10")]
pub type Mb0_64bWord10 = crate::Reg<mb0_64b_word10::Mb0_64bWord10Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word10;
#[doc = "MB1_32B_WORD0 (rw) register accessor: Message Buffer 1 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_32b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_32b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_32b_word0`] module"]
#[doc(alias = "MB1_32B_WORD0")]
pub type Mb1_32bWord0 = crate::Reg<mb1_32b_word0::Mb1_32bWord0Spec>;
#[doc = "Message Buffer 1 WORD_32B Register"]
pub mod mb1_32b_word0;
#[doc = "MB2_16B_CS (rw) register accessor: Message Buffer 2 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_16b_cs`] module"]
#[doc(alias = "MB2_16B_CS")]
pub type Mb2_16bCs = crate::Reg<mb2_16b_cs::Mb2_16bCsSpec>;
#[doc = "Message Buffer 2 CS Register"]
pub mod mb2_16b_cs;
#[doc = "MB3_8B_CS (rw) register accessor: Message Buffer 3 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_8b_cs`] module"]
#[doc(alias = "MB3_8B_CS")]
pub type Mb3_8bCs = crate::Reg<mb3_8b_cs::Mb3_8bCsSpec>;
#[doc = "Message Buffer 3 CS Register"]
pub mod mb3_8b_cs;
#[doc = "ID3 (rw) register accessor: Message Buffer 3 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id3`] module"]
#[doc(alias = "ID3")]
pub type Id3 = crate::Reg<id3::Id3Spec>;
#[doc = "Message Buffer 3 ID Register"]
pub mod id3;
#[doc = "MB0_64B_WORD11 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word11`] module"]
#[doc(alias = "MB0_64B_WORD11")]
pub type Mb0_64bWord11 = crate::Reg<mb0_64b_word11::Mb0_64bWord11Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word11;
#[doc = "MB1_32B_WORD1 (rw) register accessor: Message Buffer 1 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_32b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_32b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_32b_word1`] module"]
#[doc(alias = "MB1_32B_WORD1")]
pub type Mb1_32bWord1 = crate::Reg<mb1_32b_word1::Mb1_32bWord1Spec>;
#[doc = "Message Buffer 1 WORD_32B Register"]
pub mod mb1_32b_word1;
#[doc = "MB2_16B_ID (rw) register accessor: Message Buffer 2 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_16b_id`] module"]
#[doc(alias = "MB2_16B_ID")]
pub type Mb2_16bId = crate::Reg<mb2_16b_id::Mb2_16bIdSpec>;
#[doc = "Message Buffer 2 ID Register"]
pub mod mb2_16b_id;
#[doc = "MB3_8B_ID (rw) register accessor: Message Buffer 3 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_8b_id`] module"]
#[doc(alias = "MB3_8B_ID")]
pub type Mb3_8bId = crate::Reg<mb3_8b_id::Mb3_8bIdSpec>;
#[doc = "Message Buffer 3 ID Register"]
pub mod mb3_8b_id;
#[doc = "MB0_64B_WORD12 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word12`] module"]
#[doc(alias = "MB0_64B_WORD12")]
pub type Mb0_64bWord12 = crate::Reg<mb0_64b_word12::Mb0_64bWord12Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word12;
#[doc = "MB1_32B_WORD2 (rw) register accessor: Message Buffer 1 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_32b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_32b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_32b_word2`] module"]
#[doc(alias = "MB1_32B_WORD2")]
pub type Mb1_32bWord2 = crate::Reg<mb1_32b_word2::Mb1_32bWord2Spec>;
#[doc = "Message Buffer 1 WORD_32B Register"]
pub mod mb1_32b_word2;
#[doc = "MB2_16B_WORD0 (rw) register accessor: Message Buffer 2 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_16b_word0`] module"]
#[doc(alias = "MB2_16B_WORD0")]
pub type Mb2_16bWord0 = crate::Reg<mb2_16b_word0::Mb2_16bWord0Spec>;
#[doc = "Message Buffer 2 WORD_16B Register"]
pub mod mb2_16b_word0;
#[doc = "MB3_8B_WORD0 (rw) register accessor: Message Buffer 3 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_8b_word0`] module"]
#[doc(alias = "MB3_8B_WORD0")]
pub type Mb3_8bWord0 = crate::Reg<mb3_8b_word0::Mb3_8bWord0Spec>;
#[doc = "Message Buffer 3 WORD_8B Register"]
pub mod mb3_8b_word0;
#[doc = "WORD03 (rw) register accessor: Message Buffer 3 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word03`] module"]
#[doc(alias = "WORD03")]
pub type Word03 = crate::Reg<word03::Word03Spec>;
#[doc = "Message Buffer 3 WORD0 Register"]
pub mod word03;
#[doc = "MB0_64B_WORD13 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word13`] module"]
#[doc(alias = "MB0_64B_WORD13")]
pub type Mb0_64bWord13 = crate::Reg<mb0_64b_word13::Mb0_64bWord13Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word13;
#[doc = "MB1_32B_WORD3 (rw) register accessor: Message Buffer 1 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_32b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_32b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_32b_word3`] module"]
#[doc(alias = "MB1_32B_WORD3")]
pub type Mb1_32bWord3 = crate::Reg<mb1_32b_word3::Mb1_32bWord3Spec>;
#[doc = "Message Buffer 1 WORD_32B Register"]
pub mod mb1_32b_word3;
#[doc = "MB2_16B_WORD1 (rw) register accessor: Message Buffer 2 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_16b_word1`] module"]
#[doc(alias = "MB2_16B_WORD1")]
pub type Mb2_16bWord1 = crate::Reg<mb2_16b_word1::Mb2_16bWord1Spec>;
#[doc = "Message Buffer 2 WORD_16B Register"]
pub mod mb2_16b_word1;
#[doc = "MB3_8B_WORD1 (rw) register accessor: Message Buffer 3 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_8b_word1`] module"]
#[doc(alias = "MB3_8B_WORD1")]
pub type Mb3_8bWord1 = crate::Reg<mb3_8b_word1::Mb3_8bWord1Spec>;
#[doc = "Message Buffer 3 WORD_8B Register"]
pub mod mb3_8b_word1;
#[doc = "WORD13 (rw) register accessor: Message Buffer 3 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word13`] module"]
#[doc(alias = "WORD13")]
pub type Word13 = crate::Reg<word13::Word13Spec>;
#[doc = "Message Buffer 3 WORD1 Register"]
pub mod word13;
#[doc = "CS4 (rw) register accessor: Message Buffer 4 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs4`] module"]
#[doc(alias = "CS4")]
pub type Cs4 = crate::Reg<cs4::Cs4Spec>;
#[doc = "Message Buffer 4 CS Register"]
pub mod cs4;
#[doc = "MB0_64B_WORD14 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word14`] module"]
#[doc(alias = "MB0_64B_WORD14")]
pub type Mb0_64bWord14 = crate::Reg<mb0_64b_word14::Mb0_64bWord14Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word14;
#[doc = "MB1_32B_WORD4 (rw) register accessor: Message Buffer 1 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_32b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_32b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_32b_word4`] module"]
#[doc(alias = "MB1_32B_WORD4")]
pub type Mb1_32bWord4 = crate::Reg<mb1_32b_word4::Mb1_32bWord4Spec>;
#[doc = "Message Buffer 1 WORD_32B Register"]
pub mod mb1_32b_word4;
#[doc = "MB2_16B_WORD2 (rw) register accessor: Message Buffer 2 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_16b_word2`] module"]
#[doc(alias = "MB2_16B_WORD2")]
pub type Mb2_16bWord2 = crate::Reg<mb2_16b_word2::Mb2_16bWord2Spec>;
#[doc = "Message Buffer 2 WORD_16B Register"]
pub mod mb2_16b_word2;
#[doc = "MB4_8B_CS (rw) register accessor: Message Buffer 4 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_8b_cs`] module"]
#[doc(alias = "MB4_8B_CS")]
pub type Mb4_8bCs = crate::Reg<mb4_8b_cs::Mb4_8bCsSpec>;
#[doc = "Message Buffer 4 CS Register"]
pub mod mb4_8b_cs;
#[doc = "ID4 (rw) register accessor: Message Buffer 4 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id4`] module"]
#[doc(alias = "ID4")]
pub type Id4 = crate::Reg<id4::Id4Spec>;
#[doc = "Message Buffer 4 ID Register"]
pub mod id4;
#[doc = "MB0_64B_WORD15 (rw) register accessor: Message Buffer 0 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb0_64b_word15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb0_64b_word15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb0_64b_word15`] module"]
#[doc(alias = "MB0_64B_WORD15")]
pub type Mb0_64bWord15 = crate::Reg<mb0_64b_word15::Mb0_64bWord15Spec>;
#[doc = "Message Buffer 0 WORD_64B Register"]
pub mod mb0_64b_word15;
#[doc = "MB1_32B_WORD5 (rw) register accessor: Message Buffer 1 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_32b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_32b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_32b_word5`] module"]
#[doc(alias = "MB1_32B_WORD5")]
pub type Mb1_32bWord5 = crate::Reg<mb1_32b_word5::Mb1_32bWord5Spec>;
#[doc = "Message Buffer 1 WORD_32B Register"]
pub mod mb1_32b_word5;
#[doc = "MB2_16B_WORD3 (rw) register accessor: Message Buffer 2 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_16b_word3`] module"]
#[doc(alias = "MB2_16B_WORD3")]
pub type Mb2_16bWord3 = crate::Reg<mb2_16b_word3::Mb2_16bWord3Spec>;
#[doc = "Message Buffer 2 WORD_16B Register"]
pub mod mb2_16b_word3;
#[doc = "MB4_8B_ID (rw) register accessor: Message Buffer 4 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_8b_id`] module"]
#[doc(alias = "MB4_8B_ID")]
pub type Mb4_8bId = crate::Reg<mb4_8b_id::Mb4_8bIdSpec>;
#[doc = "Message Buffer 4 ID Register"]
pub mod mb4_8b_id;
#[doc = "MB1_32B_WORD6 (rw) register accessor: Message Buffer 1 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_32b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_32b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_32b_word6`] module"]
#[doc(alias = "MB1_32B_WORD6")]
pub type Mb1_32bWord6 = crate::Reg<mb1_32b_word6::Mb1_32bWord6Spec>;
#[doc = "Message Buffer 1 WORD_32B Register"]
pub mod mb1_32b_word6;
#[doc = "MB1_64B_CS (rw) register accessor: Message Buffer 1 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_cs`] module"]
#[doc(alias = "MB1_64B_CS")]
pub type Mb1_64bCs = crate::Reg<mb1_64b_cs::Mb1_64bCsSpec>;
#[doc = "Message Buffer 1 CS Register"]
pub mod mb1_64b_cs;
#[doc = "MB3_16B_CS (rw) register accessor: Message Buffer 3 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_16b_cs`] module"]
#[doc(alias = "MB3_16B_CS")]
pub type Mb3_16bCs = crate::Reg<mb3_16b_cs::Mb3_16bCsSpec>;
#[doc = "Message Buffer 3 CS Register"]
pub mod mb3_16b_cs;
#[doc = "MB4_8B_WORD0 (rw) register accessor: Message Buffer 4 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_8b_word0`] module"]
#[doc(alias = "MB4_8B_WORD0")]
pub type Mb4_8bWord0 = crate::Reg<mb4_8b_word0::Mb4_8bWord0Spec>;
#[doc = "Message Buffer 4 WORD_8B Register"]
pub mod mb4_8b_word0;
#[doc = "WORD04 (rw) register accessor: Message Buffer 4 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word04`] module"]
#[doc(alias = "WORD04")]
pub type Word04 = crate::Reg<word04::Word04Spec>;
#[doc = "Message Buffer 4 WORD0 Register"]
pub mod word04;
#[doc = "MB1_32B_WORD7 (rw) register accessor: Message Buffer 1 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_32b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_32b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_32b_word7`] module"]
#[doc(alias = "MB1_32B_WORD7")]
pub type Mb1_32bWord7 = crate::Reg<mb1_32b_word7::Mb1_32bWord7Spec>;
#[doc = "Message Buffer 1 WORD_32B Register"]
pub mod mb1_32b_word7;
#[doc = "MB1_64B_ID (rw) register accessor: Message Buffer 1 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_id`] module"]
#[doc(alias = "MB1_64B_ID")]
pub type Mb1_64bId = crate::Reg<mb1_64b_id::Mb1_64bIdSpec>;
#[doc = "Message Buffer 1 ID Register"]
pub mod mb1_64b_id;
#[doc = "MB3_16B_ID (rw) register accessor: Message Buffer 3 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_16b_id`] module"]
#[doc(alias = "MB3_16B_ID")]
pub type Mb3_16bId = crate::Reg<mb3_16b_id::Mb3_16bIdSpec>;
#[doc = "Message Buffer 3 ID Register"]
pub mod mb3_16b_id;
#[doc = "MB4_8B_WORD1 (rw) register accessor: Message Buffer 4 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_8b_word1`] module"]
#[doc(alias = "MB4_8B_WORD1")]
pub type Mb4_8bWord1 = crate::Reg<mb4_8b_word1::Mb4_8bWord1Spec>;
#[doc = "Message Buffer 4 WORD_8B Register"]
pub mod mb4_8b_word1;
#[doc = "WORD14 (rw) register accessor: Message Buffer 4 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word14`] module"]
#[doc(alias = "WORD14")]
pub type Word14 = crate::Reg<word14::Word14Spec>;
#[doc = "Message Buffer 4 WORD1 Register"]
pub mod word14;
#[doc = "CS5 (rw) register accessor: Message Buffer 5 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs5`] module"]
#[doc(alias = "CS5")]
pub type Cs5 = crate::Reg<cs5::Cs5Spec>;
#[doc = "Message Buffer 5 CS Register"]
pub mod cs5;
#[doc = "MB1_64B_WORD0 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word0`] module"]
#[doc(alias = "MB1_64B_WORD0")]
pub type Mb1_64bWord0 = crate::Reg<mb1_64b_word0::Mb1_64bWord0Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word0;
#[doc = "MB2_32B_CS (rw) register accessor: Message Buffer 2 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_32b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_32b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_32b_cs`] module"]
#[doc(alias = "MB2_32B_CS")]
pub type Mb2_32bCs = crate::Reg<mb2_32b_cs::Mb2_32bCsSpec>;
#[doc = "Message Buffer 2 CS Register"]
pub mod mb2_32b_cs;
#[doc = "MB3_16B_WORD0 (rw) register accessor: Message Buffer 3 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_16b_word0`] module"]
#[doc(alias = "MB3_16B_WORD0")]
pub type Mb3_16bWord0 = crate::Reg<mb3_16b_word0::Mb3_16bWord0Spec>;
#[doc = "Message Buffer 3 WORD_16B Register"]
pub mod mb3_16b_word0;
#[doc = "MB5_8B_CS (rw) register accessor: Message Buffer 5 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_8b_cs`] module"]
#[doc(alias = "MB5_8B_CS")]
pub type Mb5_8bCs = crate::Reg<mb5_8b_cs::Mb5_8bCsSpec>;
#[doc = "Message Buffer 5 CS Register"]
pub mod mb5_8b_cs;
#[doc = "ID5 (rw) register accessor: Message Buffer 5 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id5`] module"]
#[doc(alias = "ID5")]
pub type Id5 = crate::Reg<id5::Id5Spec>;
#[doc = "Message Buffer 5 ID Register"]
pub mod id5;
#[doc = "MB1_64B_WORD1 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word1`] module"]
#[doc(alias = "MB1_64B_WORD1")]
pub type Mb1_64bWord1 = crate::Reg<mb1_64b_word1::Mb1_64bWord1Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word1;
#[doc = "MB2_32B_ID (rw) register accessor: Message Buffer 2 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_32b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_32b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_32b_id`] module"]
#[doc(alias = "MB2_32B_ID")]
pub type Mb2_32bId = crate::Reg<mb2_32b_id::Mb2_32bIdSpec>;
#[doc = "Message Buffer 2 ID Register"]
pub mod mb2_32b_id;
#[doc = "MB3_16B_WORD1 (rw) register accessor: Message Buffer 3 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_16b_word1`] module"]
#[doc(alias = "MB3_16B_WORD1")]
pub type Mb3_16bWord1 = crate::Reg<mb3_16b_word1::Mb3_16bWord1Spec>;
#[doc = "Message Buffer 3 WORD_16B Register"]
pub mod mb3_16b_word1;
#[doc = "MB5_8B_ID (rw) register accessor: Message Buffer 5 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_8b_id`] module"]
#[doc(alias = "MB5_8B_ID")]
pub type Mb5_8bId = crate::Reg<mb5_8b_id::Mb5_8bIdSpec>;
#[doc = "Message Buffer 5 ID Register"]
pub mod mb5_8b_id;
#[doc = "MB1_64B_WORD2 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word2`] module"]
#[doc(alias = "MB1_64B_WORD2")]
pub type Mb1_64bWord2 = crate::Reg<mb1_64b_word2::Mb1_64bWord2Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word2;
#[doc = "MB2_32B_WORD0 (rw) register accessor: Message Buffer 2 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_32b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_32b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_32b_word0`] module"]
#[doc(alias = "MB2_32B_WORD0")]
pub type Mb2_32bWord0 = crate::Reg<mb2_32b_word0::Mb2_32bWord0Spec>;
#[doc = "Message Buffer 2 WORD_32B Register"]
pub mod mb2_32b_word0;
#[doc = "MB3_16B_WORD2 (rw) register accessor: Message Buffer 3 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_16b_word2`] module"]
#[doc(alias = "MB3_16B_WORD2")]
pub type Mb3_16bWord2 = crate::Reg<mb3_16b_word2::Mb3_16bWord2Spec>;
#[doc = "Message Buffer 3 WORD_16B Register"]
pub mod mb3_16b_word2;
#[doc = "MB5_8B_WORD0 (rw) register accessor: Message Buffer 5 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_8b_word0`] module"]
#[doc(alias = "MB5_8B_WORD0")]
pub type Mb5_8bWord0 = crate::Reg<mb5_8b_word0::Mb5_8bWord0Spec>;
#[doc = "Message Buffer 5 WORD_8B Register"]
pub mod mb5_8b_word0;
#[doc = "WORD05 (rw) register accessor: Message Buffer 5 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word05::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word05::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word05`] module"]
#[doc(alias = "WORD05")]
pub type Word05 = crate::Reg<word05::Word05Spec>;
#[doc = "Message Buffer 5 WORD0 Register"]
pub mod word05;
#[doc = "MB1_64B_WORD3 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word3`] module"]
#[doc(alias = "MB1_64B_WORD3")]
pub type Mb1_64bWord3 = crate::Reg<mb1_64b_word3::Mb1_64bWord3Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word3;
#[doc = "MB2_32B_WORD1 (rw) register accessor: Message Buffer 2 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_32b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_32b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_32b_word1`] module"]
#[doc(alias = "MB2_32B_WORD1")]
pub type Mb2_32bWord1 = crate::Reg<mb2_32b_word1::Mb2_32bWord1Spec>;
#[doc = "Message Buffer 2 WORD_32B Register"]
pub mod mb2_32b_word1;
#[doc = "MB3_16B_WORD3 (rw) register accessor: Message Buffer 3 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_16b_word3`] module"]
#[doc(alias = "MB3_16B_WORD3")]
pub type Mb3_16bWord3 = crate::Reg<mb3_16b_word3::Mb3_16bWord3Spec>;
#[doc = "Message Buffer 3 WORD_16B Register"]
pub mod mb3_16b_word3;
#[doc = "MB5_8B_WORD1 (rw) register accessor: Message Buffer 5 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_8b_word1`] module"]
#[doc(alias = "MB5_8B_WORD1")]
pub type Mb5_8bWord1 = crate::Reg<mb5_8b_word1::Mb5_8bWord1Spec>;
#[doc = "Message Buffer 5 WORD_8B Register"]
pub mod mb5_8b_word1;
#[doc = "WORD15 (rw) register accessor: Message Buffer 5 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word15`] module"]
#[doc(alias = "WORD15")]
pub type Word15 = crate::Reg<word15::Word15Spec>;
#[doc = "Message Buffer 5 WORD1 Register"]
pub mod word15;
#[doc = "CS6 (rw) register accessor: Message Buffer 6 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs6`] module"]
#[doc(alias = "CS6")]
pub type Cs6 = crate::Reg<cs6::Cs6Spec>;
#[doc = "Message Buffer 6 CS Register"]
pub mod cs6;
#[doc = "MB1_64B_WORD4 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word4`] module"]
#[doc(alias = "MB1_64B_WORD4")]
pub type Mb1_64bWord4 = crate::Reg<mb1_64b_word4::Mb1_64bWord4Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word4;
#[doc = "MB2_32B_WORD2 (rw) register accessor: Message Buffer 2 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_32b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_32b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_32b_word2`] module"]
#[doc(alias = "MB2_32B_WORD2")]
pub type Mb2_32bWord2 = crate::Reg<mb2_32b_word2::Mb2_32bWord2Spec>;
#[doc = "Message Buffer 2 WORD_32B Register"]
pub mod mb2_32b_word2;
#[doc = "MB4_16B_CS (rw) register accessor: Message Buffer 4 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_16b_cs`] module"]
#[doc(alias = "MB4_16B_CS")]
pub type Mb4_16bCs = crate::Reg<mb4_16b_cs::Mb4_16bCsSpec>;
#[doc = "Message Buffer 4 CS Register"]
pub mod mb4_16b_cs;
#[doc = "MB6_8B_CS (rw) register accessor: Message Buffer 6 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_8b_cs`] module"]
#[doc(alias = "MB6_8B_CS")]
pub type Mb6_8bCs = crate::Reg<mb6_8b_cs::Mb6_8bCsSpec>;
#[doc = "Message Buffer 6 CS Register"]
pub mod mb6_8b_cs;
#[doc = "ID6 (rw) register accessor: Message Buffer 6 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id6`] module"]
#[doc(alias = "ID6")]
pub type Id6 = crate::Reg<id6::Id6Spec>;
#[doc = "Message Buffer 6 ID Register"]
pub mod id6;
#[doc = "MB1_64B_WORD5 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word5`] module"]
#[doc(alias = "MB1_64B_WORD5")]
pub type Mb1_64bWord5 = crate::Reg<mb1_64b_word5::Mb1_64bWord5Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word5;
#[doc = "MB2_32B_WORD3 (rw) register accessor: Message Buffer 2 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_32b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_32b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_32b_word3`] module"]
#[doc(alias = "MB2_32B_WORD3")]
pub type Mb2_32bWord3 = crate::Reg<mb2_32b_word3::Mb2_32bWord3Spec>;
#[doc = "Message Buffer 2 WORD_32B Register"]
pub mod mb2_32b_word3;
#[doc = "MB4_16B_ID (rw) register accessor: Message Buffer 4 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_16b_id`] module"]
#[doc(alias = "MB4_16B_ID")]
pub type Mb4_16bId = crate::Reg<mb4_16b_id::Mb4_16bIdSpec>;
#[doc = "Message Buffer 4 ID Register"]
pub mod mb4_16b_id;
#[doc = "MB6_8B_ID (rw) register accessor: Message Buffer 6 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_8b_id`] module"]
#[doc(alias = "MB6_8B_ID")]
pub type Mb6_8bId = crate::Reg<mb6_8b_id::Mb6_8bIdSpec>;
#[doc = "Message Buffer 6 ID Register"]
pub mod mb6_8b_id;
#[doc = "MB1_64B_WORD6 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word6`] module"]
#[doc(alias = "MB1_64B_WORD6")]
pub type Mb1_64bWord6 = crate::Reg<mb1_64b_word6::Mb1_64bWord6Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word6;
#[doc = "MB2_32B_WORD4 (rw) register accessor: Message Buffer 2 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_32b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_32b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_32b_word4`] module"]
#[doc(alias = "MB2_32B_WORD4")]
pub type Mb2_32bWord4 = crate::Reg<mb2_32b_word4::Mb2_32bWord4Spec>;
#[doc = "Message Buffer 2 WORD_32B Register"]
pub mod mb2_32b_word4;
#[doc = "MB4_16B_WORD0 (rw) register accessor: Message Buffer 4 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_16b_word0`] module"]
#[doc(alias = "MB4_16B_WORD0")]
pub type Mb4_16bWord0 = crate::Reg<mb4_16b_word0::Mb4_16bWord0Spec>;
#[doc = "Message Buffer 4 WORD_16B Register"]
pub mod mb4_16b_word0;
#[doc = "MB6_8B_WORD0 (rw) register accessor: Message Buffer 6 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_8b_word0`] module"]
#[doc(alias = "MB6_8B_WORD0")]
pub type Mb6_8bWord0 = crate::Reg<mb6_8b_word0::Mb6_8bWord0Spec>;
#[doc = "Message Buffer 6 WORD_8B Register"]
pub mod mb6_8b_word0;
#[doc = "WORD06 (rw) register accessor: Message Buffer 6 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word06::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word06::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word06`] module"]
#[doc(alias = "WORD06")]
pub type Word06 = crate::Reg<word06::Word06Spec>;
#[doc = "Message Buffer 6 WORD0 Register"]
pub mod word06;
#[doc = "MB1_64B_WORD7 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word7`] module"]
#[doc(alias = "MB1_64B_WORD7")]
pub type Mb1_64bWord7 = crate::Reg<mb1_64b_word7::Mb1_64bWord7Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word7;
#[doc = "MB2_32B_WORD5 (rw) register accessor: Message Buffer 2 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_32b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_32b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_32b_word5`] module"]
#[doc(alias = "MB2_32B_WORD5")]
pub type Mb2_32bWord5 = crate::Reg<mb2_32b_word5::Mb2_32bWord5Spec>;
#[doc = "Message Buffer 2 WORD_32B Register"]
pub mod mb2_32b_word5;
#[doc = "MB4_16B_WORD1 (rw) register accessor: Message Buffer 4 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_16b_word1`] module"]
#[doc(alias = "MB4_16B_WORD1")]
pub type Mb4_16bWord1 = crate::Reg<mb4_16b_word1::Mb4_16bWord1Spec>;
#[doc = "Message Buffer 4 WORD_16B Register"]
pub mod mb4_16b_word1;
#[doc = "MB6_8B_WORD1 (rw) register accessor: Message Buffer 6 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_8b_word1`] module"]
#[doc(alias = "MB6_8B_WORD1")]
pub type Mb6_8bWord1 = crate::Reg<mb6_8b_word1::Mb6_8bWord1Spec>;
#[doc = "Message Buffer 6 WORD_8B Register"]
pub mod mb6_8b_word1;
#[doc = "WORD16 (rw) register accessor: Message Buffer 6 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word16`] module"]
#[doc(alias = "WORD16")]
pub type Word16 = crate::Reg<word16::Word16Spec>;
#[doc = "Message Buffer 6 WORD1 Register"]
pub mod word16;
#[doc = "CS7 (rw) register accessor: Message Buffer 7 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs7`] module"]
#[doc(alias = "CS7")]
pub type Cs7 = crate::Reg<cs7::Cs7Spec>;
#[doc = "Message Buffer 7 CS Register"]
pub mod cs7;
#[doc = "MB1_64B_WORD8 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word8`] module"]
#[doc(alias = "MB1_64B_WORD8")]
pub type Mb1_64bWord8 = crate::Reg<mb1_64b_word8::Mb1_64bWord8Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word8;
#[doc = "MB2_32B_WORD6 (rw) register accessor: Message Buffer 2 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_32b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_32b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_32b_word6`] module"]
#[doc(alias = "MB2_32B_WORD6")]
pub type Mb2_32bWord6 = crate::Reg<mb2_32b_word6::Mb2_32bWord6Spec>;
#[doc = "Message Buffer 2 WORD_32B Register"]
pub mod mb2_32b_word6;
#[doc = "MB4_16B_WORD2 (rw) register accessor: Message Buffer 4 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_16b_word2`] module"]
#[doc(alias = "MB4_16B_WORD2")]
pub type Mb4_16bWord2 = crate::Reg<mb4_16b_word2::Mb4_16bWord2Spec>;
#[doc = "Message Buffer 4 WORD_16B Register"]
pub mod mb4_16b_word2;
#[doc = "MB7_8B_CS (rw) register accessor: Message Buffer 7 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_8b_cs`] module"]
#[doc(alias = "MB7_8B_CS")]
pub type Mb7_8bCs = crate::Reg<mb7_8b_cs::Mb7_8bCsSpec>;
#[doc = "Message Buffer 7 CS Register"]
pub mod mb7_8b_cs;
#[doc = "ID7 (rw) register accessor: Message Buffer 7 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id7`] module"]
#[doc(alias = "ID7")]
pub type Id7 = crate::Reg<id7::Id7Spec>;
#[doc = "Message Buffer 7 ID Register"]
pub mod id7;
#[doc = "MB1_64B_WORD9 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word9`] module"]
#[doc(alias = "MB1_64B_WORD9")]
pub type Mb1_64bWord9 = crate::Reg<mb1_64b_word9::Mb1_64bWord9Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word9;
#[doc = "MB2_32B_WORD7 (rw) register accessor: Message Buffer 2 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_32b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_32b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_32b_word7`] module"]
#[doc(alias = "MB2_32B_WORD7")]
pub type Mb2_32bWord7 = crate::Reg<mb2_32b_word7::Mb2_32bWord7Spec>;
#[doc = "Message Buffer 2 WORD_32B Register"]
pub mod mb2_32b_word7;
#[doc = "MB4_16B_WORD3 (rw) register accessor: Message Buffer 4 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_16b_word3`] module"]
#[doc(alias = "MB4_16B_WORD3")]
pub type Mb4_16bWord3 = crate::Reg<mb4_16b_word3::Mb4_16bWord3Spec>;
#[doc = "Message Buffer 4 WORD_16B Register"]
pub mod mb4_16b_word3;
#[doc = "MB7_8B_ID (rw) register accessor: Message Buffer 7 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_8b_id`] module"]
#[doc(alias = "MB7_8B_ID")]
pub type Mb7_8bId = crate::Reg<mb7_8b_id::Mb7_8bIdSpec>;
#[doc = "Message Buffer 7 ID Register"]
pub mod mb7_8b_id;
#[doc = "MB1_64B_WORD10 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word10`] module"]
#[doc(alias = "MB1_64B_WORD10")]
pub type Mb1_64bWord10 = crate::Reg<mb1_64b_word10::Mb1_64bWord10Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word10;
#[doc = "MB3_32B_CS (rw) register accessor: Message Buffer 3 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_32b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_32b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_32b_cs`] module"]
#[doc(alias = "MB3_32B_CS")]
pub type Mb3_32bCs = crate::Reg<mb3_32b_cs::Mb3_32bCsSpec>;
#[doc = "Message Buffer 3 CS Register"]
pub mod mb3_32b_cs;
#[doc = "MB5_16B_CS (rw) register accessor: Message Buffer 5 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_16b_cs`] module"]
#[doc(alias = "MB5_16B_CS")]
pub type Mb5_16bCs = crate::Reg<mb5_16b_cs::Mb5_16bCsSpec>;
#[doc = "Message Buffer 5 CS Register"]
pub mod mb5_16b_cs;
#[doc = "MB7_8B_WORD0 (rw) register accessor: Message Buffer 7 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_8b_word0`] module"]
#[doc(alias = "MB7_8B_WORD0")]
pub type Mb7_8bWord0 = crate::Reg<mb7_8b_word0::Mb7_8bWord0Spec>;
#[doc = "Message Buffer 7 WORD_8B Register"]
pub mod mb7_8b_word0;
#[doc = "WORD07 (rw) register accessor: Message Buffer 7 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word07::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word07::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word07`] module"]
#[doc(alias = "WORD07")]
pub type Word07 = crate::Reg<word07::Word07Spec>;
#[doc = "Message Buffer 7 WORD0 Register"]
pub mod word07;
#[doc = "MB1_64B_WORD11 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word11`] module"]
#[doc(alias = "MB1_64B_WORD11")]
pub type Mb1_64bWord11 = crate::Reg<mb1_64b_word11::Mb1_64bWord11Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word11;
#[doc = "MB3_32B_ID (rw) register accessor: Message Buffer 3 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_32b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_32b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_32b_id`] module"]
#[doc(alias = "MB3_32B_ID")]
pub type Mb3_32bId = crate::Reg<mb3_32b_id::Mb3_32bIdSpec>;
#[doc = "Message Buffer 3 ID Register"]
pub mod mb3_32b_id;
#[doc = "MB5_16B_ID (rw) register accessor: Message Buffer 5 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_16b_id`] module"]
#[doc(alias = "MB5_16B_ID")]
pub type Mb5_16bId = crate::Reg<mb5_16b_id::Mb5_16bIdSpec>;
#[doc = "Message Buffer 5 ID Register"]
pub mod mb5_16b_id;
#[doc = "MB7_8B_WORD1 (rw) register accessor: Message Buffer 7 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_8b_word1`] module"]
#[doc(alias = "MB7_8B_WORD1")]
pub type Mb7_8bWord1 = crate::Reg<mb7_8b_word1::Mb7_8bWord1Spec>;
#[doc = "Message Buffer 7 WORD_8B Register"]
pub mod mb7_8b_word1;
#[doc = "WORD17 (rw) register accessor: Message Buffer 7 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word17`] module"]
#[doc(alias = "WORD17")]
pub type Word17 = crate::Reg<word17::Word17Spec>;
#[doc = "Message Buffer 7 WORD1 Register"]
pub mod word17;
#[doc = "CS8 (rw) register accessor: Message Buffer 8 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs8`] module"]
#[doc(alias = "CS8")]
pub type Cs8 = crate::Reg<cs8::Cs8Spec>;
#[doc = "Message Buffer 8 CS Register"]
pub mod cs8;
#[doc = "MB1_64B_WORD12 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word12`] module"]
#[doc(alias = "MB1_64B_WORD12")]
pub type Mb1_64bWord12 = crate::Reg<mb1_64b_word12::Mb1_64bWord12Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word12;
#[doc = "MB3_32B_WORD0 (rw) register accessor: Message Buffer 3 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_32b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_32b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_32b_word0`] module"]
#[doc(alias = "MB3_32B_WORD0")]
pub type Mb3_32bWord0 = crate::Reg<mb3_32b_word0::Mb3_32bWord0Spec>;
#[doc = "Message Buffer 3 WORD_32B Register"]
pub mod mb3_32b_word0;
#[doc = "MB5_16B_WORD0 (rw) register accessor: Message Buffer 5 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_16b_word0`] module"]
#[doc(alias = "MB5_16B_WORD0")]
pub type Mb5_16bWord0 = crate::Reg<mb5_16b_word0::Mb5_16bWord0Spec>;
#[doc = "Message Buffer 5 WORD_16B Register"]
pub mod mb5_16b_word0;
#[doc = "MB8_8B_CS (rw) register accessor: Message Buffer 8 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_8b_cs`] module"]
#[doc(alias = "MB8_8B_CS")]
pub type Mb8_8bCs = crate::Reg<mb8_8b_cs::Mb8_8bCsSpec>;
#[doc = "Message Buffer 8 CS Register"]
pub mod mb8_8b_cs;
#[doc = "ID8 (rw) register accessor: Message Buffer 8 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id8`] module"]
#[doc(alias = "ID8")]
pub type Id8 = crate::Reg<id8::Id8Spec>;
#[doc = "Message Buffer 8 ID Register"]
pub mod id8;
#[doc = "MB1_64B_WORD13 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word13`] module"]
#[doc(alias = "MB1_64B_WORD13")]
pub type Mb1_64bWord13 = crate::Reg<mb1_64b_word13::Mb1_64bWord13Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word13;
#[doc = "MB3_32B_WORD1 (rw) register accessor: Message Buffer 3 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_32b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_32b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_32b_word1`] module"]
#[doc(alias = "MB3_32B_WORD1")]
pub type Mb3_32bWord1 = crate::Reg<mb3_32b_word1::Mb3_32bWord1Spec>;
#[doc = "Message Buffer 3 WORD_32B Register"]
pub mod mb3_32b_word1;
#[doc = "MB5_16B_WORD1 (rw) register accessor: Message Buffer 5 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_16b_word1`] module"]
#[doc(alias = "MB5_16B_WORD1")]
pub type Mb5_16bWord1 = crate::Reg<mb5_16b_word1::Mb5_16bWord1Spec>;
#[doc = "Message Buffer 5 WORD_16B Register"]
pub mod mb5_16b_word1;
#[doc = "MB8_8B_ID (rw) register accessor: Message Buffer 8 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_8b_id`] module"]
#[doc(alias = "MB8_8B_ID")]
pub type Mb8_8bId = crate::Reg<mb8_8b_id::Mb8_8bIdSpec>;
#[doc = "Message Buffer 8 ID Register"]
pub mod mb8_8b_id;
#[doc = "MB1_64B_WORD14 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word14`] module"]
#[doc(alias = "MB1_64B_WORD14")]
pub type Mb1_64bWord14 = crate::Reg<mb1_64b_word14::Mb1_64bWord14Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word14;
#[doc = "MB3_32B_WORD2 (rw) register accessor: Message Buffer 3 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_32b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_32b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_32b_word2`] module"]
#[doc(alias = "MB3_32B_WORD2")]
pub type Mb3_32bWord2 = crate::Reg<mb3_32b_word2::Mb3_32bWord2Spec>;
#[doc = "Message Buffer 3 WORD_32B Register"]
pub mod mb3_32b_word2;
#[doc = "MB5_16B_WORD2 (rw) register accessor: Message Buffer 5 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_16b_word2`] module"]
#[doc(alias = "MB5_16B_WORD2")]
pub type Mb5_16bWord2 = crate::Reg<mb5_16b_word2::Mb5_16bWord2Spec>;
#[doc = "Message Buffer 5 WORD_16B Register"]
pub mod mb5_16b_word2;
#[doc = "MB8_8B_WORD0 (rw) register accessor: Message Buffer 8 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_8b_word0`] module"]
#[doc(alias = "MB8_8B_WORD0")]
pub type Mb8_8bWord0 = crate::Reg<mb8_8b_word0::Mb8_8bWord0Spec>;
#[doc = "Message Buffer 8 WORD_8B Register"]
pub mod mb8_8b_word0;
#[doc = "WORD08 (rw) register accessor: Message Buffer 8 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word08`] module"]
#[doc(alias = "WORD08")]
pub type Word08 = crate::Reg<word08::Word08Spec>;
#[doc = "Message Buffer 8 WORD0 Register"]
pub mod word08;
#[doc = "MB1_64B_WORD15 (rw) register accessor: Message Buffer 1 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb1_64b_word15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb1_64b_word15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb1_64b_word15`] module"]
#[doc(alias = "MB1_64B_WORD15")]
pub type Mb1_64bWord15 = crate::Reg<mb1_64b_word15::Mb1_64bWord15Spec>;
#[doc = "Message Buffer 1 WORD_64B Register"]
pub mod mb1_64b_word15;
#[doc = "MB3_32B_WORD3 (rw) register accessor: Message Buffer 3 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_32b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_32b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_32b_word3`] module"]
#[doc(alias = "MB3_32B_WORD3")]
pub type Mb3_32bWord3 = crate::Reg<mb3_32b_word3::Mb3_32bWord3Spec>;
#[doc = "Message Buffer 3 WORD_32B Register"]
pub mod mb3_32b_word3;
#[doc = "MB5_16B_WORD3 (rw) register accessor: Message Buffer 5 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_16b_word3`] module"]
#[doc(alias = "MB5_16B_WORD3")]
pub type Mb5_16bWord3 = crate::Reg<mb5_16b_word3::Mb5_16bWord3Spec>;
#[doc = "Message Buffer 5 WORD_16B Register"]
pub mod mb5_16b_word3;
#[doc = "MB8_8B_WORD1 (rw) register accessor: Message Buffer 8 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_8b_word1`] module"]
#[doc(alias = "MB8_8B_WORD1")]
pub type Mb8_8bWord1 = crate::Reg<mb8_8b_word1::Mb8_8bWord1Spec>;
#[doc = "Message Buffer 8 WORD_8B Register"]
pub mod mb8_8b_word1;
#[doc = "WORD18 (rw) register accessor: Message Buffer 8 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word18`] module"]
#[doc(alias = "WORD18")]
pub type Word18 = crate::Reg<word18::Word18Spec>;
#[doc = "Message Buffer 8 WORD1 Register"]
pub mod word18;
#[doc = "CS9 (rw) register accessor: Message Buffer 9 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs9`] module"]
#[doc(alias = "CS9")]
pub type Cs9 = crate::Reg<cs9::Cs9Spec>;
#[doc = "Message Buffer 9 CS Register"]
pub mod cs9;
#[doc = "MB2_64B_CS (rw) register accessor: Message Buffer 2 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_cs`] module"]
#[doc(alias = "MB2_64B_CS")]
pub type Mb2_64bCs = crate::Reg<mb2_64b_cs::Mb2_64bCsSpec>;
#[doc = "Message Buffer 2 CS Register"]
pub mod mb2_64b_cs;
#[doc = "MB3_32B_WORD4 (rw) register accessor: Message Buffer 3 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_32b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_32b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_32b_word4`] module"]
#[doc(alias = "MB3_32B_WORD4")]
pub type Mb3_32bWord4 = crate::Reg<mb3_32b_word4::Mb3_32bWord4Spec>;
#[doc = "Message Buffer 3 WORD_32B Register"]
pub mod mb3_32b_word4;
#[doc = "MB6_16B_CS (rw) register accessor: Message Buffer 6 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_16b_cs`] module"]
#[doc(alias = "MB6_16B_CS")]
pub type Mb6_16bCs = crate::Reg<mb6_16b_cs::Mb6_16bCsSpec>;
#[doc = "Message Buffer 6 CS Register"]
pub mod mb6_16b_cs;
#[doc = "MB9_8B_CS (rw) register accessor: Message Buffer 9 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_8b_cs`] module"]
#[doc(alias = "MB9_8B_CS")]
pub type Mb9_8bCs = crate::Reg<mb9_8b_cs::Mb9_8bCsSpec>;
#[doc = "Message Buffer 9 CS Register"]
pub mod mb9_8b_cs;
#[doc = "ID9 (rw) register accessor: Message Buffer 9 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id9`] module"]
#[doc(alias = "ID9")]
pub type Id9 = crate::Reg<id9::Id9Spec>;
#[doc = "Message Buffer 9 ID Register"]
pub mod id9;
#[doc = "MB2_64B_ID (rw) register accessor: Message Buffer 2 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_id`] module"]
#[doc(alias = "MB2_64B_ID")]
pub type Mb2_64bId = crate::Reg<mb2_64b_id::Mb2_64bIdSpec>;
#[doc = "Message Buffer 2 ID Register"]
pub mod mb2_64b_id;
#[doc = "MB3_32B_WORD5 (rw) register accessor: Message Buffer 3 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_32b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_32b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_32b_word5`] module"]
#[doc(alias = "MB3_32B_WORD5")]
pub type Mb3_32bWord5 = crate::Reg<mb3_32b_word5::Mb3_32bWord5Spec>;
#[doc = "Message Buffer 3 WORD_32B Register"]
pub mod mb3_32b_word5;
#[doc = "MB6_16B_ID (rw) register accessor: Message Buffer 6 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_16b_id`] module"]
#[doc(alias = "MB6_16B_ID")]
pub type Mb6_16bId = crate::Reg<mb6_16b_id::Mb6_16bIdSpec>;
#[doc = "Message Buffer 6 ID Register"]
pub mod mb6_16b_id;
#[doc = "MB9_8B_ID (rw) register accessor: Message Buffer 9 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_8b_id`] module"]
#[doc(alias = "MB9_8B_ID")]
pub type Mb9_8bId = crate::Reg<mb9_8b_id::Mb9_8bIdSpec>;
#[doc = "Message Buffer 9 ID Register"]
pub mod mb9_8b_id;
#[doc = "MB2_64B_WORD0 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word0`] module"]
#[doc(alias = "MB2_64B_WORD0")]
pub type Mb2_64bWord0 = crate::Reg<mb2_64b_word0::Mb2_64bWord0Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word0;
#[doc = "MB3_32B_WORD6 (rw) register accessor: Message Buffer 3 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_32b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_32b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_32b_word6`] module"]
#[doc(alias = "MB3_32B_WORD6")]
pub type Mb3_32bWord6 = crate::Reg<mb3_32b_word6::Mb3_32bWord6Spec>;
#[doc = "Message Buffer 3 WORD_32B Register"]
pub mod mb3_32b_word6;
#[doc = "MB6_16B_WORD0 (rw) register accessor: Message Buffer 6 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_16b_word0`] module"]
#[doc(alias = "MB6_16B_WORD0")]
pub type Mb6_16bWord0 = crate::Reg<mb6_16b_word0::Mb6_16bWord0Spec>;
#[doc = "Message Buffer 6 WORD_16B Register"]
pub mod mb6_16b_word0;
#[doc = "MB9_8B_WORD0 (rw) register accessor: Message Buffer 9 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_8b_word0`] module"]
#[doc(alias = "MB9_8B_WORD0")]
pub type Mb9_8bWord0 = crate::Reg<mb9_8b_word0::Mb9_8bWord0Spec>;
#[doc = "Message Buffer 9 WORD_8B Register"]
pub mod mb9_8b_word0;
#[doc = "WORD09 (rw) register accessor: Message Buffer 9 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word09::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word09::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word09`] module"]
#[doc(alias = "WORD09")]
pub type Word09 = crate::Reg<word09::Word09Spec>;
#[doc = "Message Buffer 9 WORD0 Register"]
pub mod word09;
#[doc = "MB2_64B_WORD1 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word1`] module"]
#[doc(alias = "MB2_64B_WORD1")]
pub type Mb2_64bWord1 = crate::Reg<mb2_64b_word1::Mb2_64bWord1Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word1;
#[doc = "MB3_32B_WORD7 (rw) register accessor: Message Buffer 3 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_32b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_32b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_32b_word7`] module"]
#[doc(alias = "MB3_32B_WORD7")]
pub type Mb3_32bWord7 = crate::Reg<mb3_32b_word7::Mb3_32bWord7Spec>;
#[doc = "Message Buffer 3 WORD_32B Register"]
pub mod mb3_32b_word7;
#[doc = "MB6_16B_WORD1 (rw) register accessor: Message Buffer 6 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_16b_word1`] module"]
#[doc(alias = "MB6_16B_WORD1")]
pub type Mb6_16bWord1 = crate::Reg<mb6_16b_word1::Mb6_16bWord1Spec>;
#[doc = "Message Buffer 6 WORD_16B Register"]
pub mod mb6_16b_word1;
#[doc = "MB9_8B_WORD1 (rw) register accessor: Message Buffer 9 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_8b_word1`] module"]
#[doc(alias = "MB9_8B_WORD1")]
pub type Mb9_8bWord1 = crate::Reg<mb9_8b_word1::Mb9_8bWord1Spec>;
#[doc = "Message Buffer 9 WORD_8B Register"]
pub mod mb9_8b_word1;
#[doc = "WORD19 (rw) register accessor: Message Buffer 9 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word19`] module"]
#[doc(alias = "WORD19")]
pub type Word19 = crate::Reg<word19::Word19Spec>;
#[doc = "Message Buffer 9 WORD1 Register"]
pub mod word19;
#[doc = "CS10 (rw) register accessor: Message Buffer 10 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs10`] module"]
#[doc(alias = "CS10")]
pub type Cs10 = crate::Reg<cs10::Cs10Spec>;
#[doc = "Message Buffer 10 CS Register"]
pub mod cs10;
#[doc = "MB10_8B_CS (rw) register accessor: Message Buffer 10 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_8b_cs`] module"]
#[doc(alias = "MB10_8B_CS")]
pub type Mb10_8bCs = crate::Reg<mb10_8b_cs::Mb10_8bCsSpec>;
#[doc = "Message Buffer 10 CS Register"]
pub mod mb10_8b_cs;
#[doc = "MB2_64B_WORD2 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word2`] module"]
#[doc(alias = "MB2_64B_WORD2")]
pub type Mb2_64bWord2 = crate::Reg<mb2_64b_word2::Mb2_64bWord2Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word2;
#[doc = "MB4_32B_CS (rw) register accessor: Message Buffer 4 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_32b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_32b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_32b_cs`] module"]
#[doc(alias = "MB4_32B_CS")]
pub type Mb4_32bCs = crate::Reg<mb4_32b_cs::Mb4_32bCsSpec>;
#[doc = "Message Buffer 4 CS Register"]
pub mod mb4_32b_cs;
#[doc = "MB6_16B_WORD2 (rw) register accessor: Message Buffer 6 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_16b_word2`] module"]
#[doc(alias = "MB6_16B_WORD2")]
pub type Mb6_16bWord2 = crate::Reg<mb6_16b_word2::Mb6_16bWord2Spec>;
#[doc = "Message Buffer 6 WORD_16B Register"]
pub mod mb6_16b_word2;
#[doc = "ID10 (rw) register accessor: Message Buffer 10 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id10`] module"]
#[doc(alias = "ID10")]
pub type Id10 = crate::Reg<id10::Id10Spec>;
#[doc = "Message Buffer 10 ID Register"]
pub mod id10;
#[doc = "MB10_8B_ID (rw) register accessor: Message Buffer 10 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_8b_id`] module"]
#[doc(alias = "MB10_8B_ID")]
pub type Mb10_8bId = crate::Reg<mb10_8b_id::Mb10_8bIdSpec>;
#[doc = "Message Buffer 10 ID Register"]
pub mod mb10_8b_id;
#[doc = "MB2_64B_WORD3 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word3`] module"]
#[doc(alias = "MB2_64B_WORD3")]
pub type Mb2_64bWord3 = crate::Reg<mb2_64b_word3::Mb2_64bWord3Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word3;
#[doc = "MB4_32B_ID (rw) register accessor: Message Buffer 4 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_32b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_32b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_32b_id`] module"]
#[doc(alias = "MB4_32B_ID")]
pub type Mb4_32bId = crate::Reg<mb4_32b_id::Mb4_32bIdSpec>;
#[doc = "Message Buffer 4 ID Register"]
pub mod mb4_32b_id;
#[doc = "MB6_16B_WORD3 (rw) register accessor: Message Buffer 6 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_16b_word3`] module"]
#[doc(alias = "MB6_16B_WORD3")]
pub type Mb6_16bWord3 = crate::Reg<mb6_16b_word3::Mb6_16bWord3Spec>;
#[doc = "Message Buffer 6 WORD_16B Register"]
pub mod mb6_16b_word3;
#[doc = "MB10_8B_WORD0 (rw) register accessor: Message Buffer 10 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_8b_word0`] module"]
#[doc(alias = "MB10_8B_WORD0")]
pub type Mb10_8bWord0 = crate::Reg<mb10_8b_word0::Mb10_8bWord0Spec>;
#[doc = "Message Buffer 10 WORD_8B Register"]
pub mod mb10_8b_word0;
#[doc = "MB2_64B_WORD4 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word4`] module"]
#[doc(alias = "MB2_64B_WORD4")]
pub type Mb2_64bWord4 = crate::Reg<mb2_64b_word4::Mb2_64bWord4Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word4;
#[doc = "MB4_32B_WORD0 (rw) register accessor: Message Buffer 4 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_32b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_32b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_32b_word0`] module"]
#[doc(alias = "MB4_32B_WORD0")]
pub type Mb4_32bWord0 = crate::Reg<mb4_32b_word0::Mb4_32bWord0Spec>;
#[doc = "Message Buffer 4 WORD_32B Register"]
pub mod mb4_32b_word0;
#[doc = "MB7_16B_CS (rw) register accessor: Message Buffer 7 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_16b_cs`] module"]
#[doc(alias = "MB7_16B_CS")]
pub type Mb7_16bCs = crate::Reg<mb7_16b_cs::Mb7_16bCsSpec>;
#[doc = "Message Buffer 7 CS Register"]
pub mod mb7_16b_cs;
#[doc = "WORD010 (rw) register accessor: Message Buffer 10 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word010`] module"]
#[doc(alias = "WORD010")]
pub type Word010 = crate::Reg<word010::Word010Spec>;
#[doc = "Message Buffer 10 WORD0 Register"]
pub mod word010;
#[doc = "MB10_8B_WORD1 (rw) register accessor: Message Buffer 10 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_8b_word1`] module"]
#[doc(alias = "MB10_8B_WORD1")]
pub type Mb10_8bWord1 = crate::Reg<mb10_8b_word1::Mb10_8bWord1Spec>;
#[doc = "Message Buffer 10 WORD_8B Register"]
pub mod mb10_8b_word1;
#[doc = "MB2_64B_WORD5 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word5`] module"]
#[doc(alias = "MB2_64B_WORD5")]
pub type Mb2_64bWord5 = crate::Reg<mb2_64b_word5::Mb2_64bWord5Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word5;
#[doc = "MB4_32B_WORD1 (rw) register accessor: Message Buffer 4 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_32b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_32b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_32b_word1`] module"]
#[doc(alias = "MB4_32B_WORD1")]
pub type Mb4_32bWord1 = crate::Reg<mb4_32b_word1::Mb4_32bWord1Spec>;
#[doc = "Message Buffer 4 WORD_32B Register"]
pub mod mb4_32b_word1;
#[doc = "MB7_16B_ID (rw) register accessor: Message Buffer 7 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_16b_id`] module"]
#[doc(alias = "MB7_16B_ID")]
pub type Mb7_16bId = crate::Reg<mb7_16b_id::Mb7_16bIdSpec>;
#[doc = "Message Buffer 7 ID Register"]
pub mod mb7_16b_id;
#[doc = "WORD110 (rw) register accessor: Message Buffer 10 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word110`] module"]
#[doc(alias = "WORD110")]
pub type Word110 = crate::Reg<word110::Word110Spec>;
#[doc = "Message Buffer 10 WORD1 Register"]
pub mod word110;
#[doc = "CS11 (rw) register accessor: Message Buffer 11 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs11`] module"]
#[doc(alias = "CS11")]
pub type Cs11 = crate::Reg<cs11::Cs11Spec>;
#[doc = "Message Buffer 11 CS Register"]
pub mod cs11;
#[doc = "MB11_8B_CS (rw) register accessor: Message Buffer 11 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_8b_cs`] module"]
#[doc(alias = "MB11_8B_CS")]
pub type Mb11_8bCs = crate::Reg<mb11_8b_cs::Mb11_8bCsSpec>;
#[doc = "Message Buffer 11 CS Register"]
pub mod mb11_8b_cs;
#[doc = "MB2_64B_WORD6 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word6`] module"]
#[doc(alias = "MB2_64B_WORD6")]
pub type Mb2_64bWord6 = crate::Reg<mb2_64b_word6::Mb2_64bWord6Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word6;
#[doc = "MB4_32B_WORD2 (rw) register accessor: Message Buffer 4 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_32b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_32b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_32b_word2`] module"]
#[doc(alias = "MB4_32B_WORD2")]
pub type Mb4_32bWord2 = crate::Reg<mb4_32b_word2::Mb4_32bWord2Spec>;
#[doc = "Message Buffer 4 WORD_32B Register"]
pub mod mb4_32b_word2;
#[doc = "MB7_16B_WORD0 (rw) register accessor: Message Buffer 7 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_16b_word0`] module"]
#[doc(alias = "MB7_16B_WORD0")]
pub type Mb7_16bWord0 = crate::Reg<mb7_16b_word0::Mb7_16bWord0Spec>;
#[doc = "Message Buffer 7 WORD_16B Register"]
pub mod mb7_16b_word0;
#[doc = "ID11 (rw) register accessor: Message Buffer 11 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id11`] module"]
#[doc(alias = "ID11")]
pub type Id11 = crate::Reg<id11::Id11Spec>;
#[doc = "Message Buffer 11 ID Register"]
pub mod id11;
#[doc = "MB11_8B_ID (rw) register accessor: Message Buffer 11 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_8b_id`] module"]
#[doc(alias = "MB11_8B_ID")]
pub type Mb11_8bId = crate::Reg<mb11_8b_id::Mb11_8bIdSpec>;
#[doc = "Message Buffer 11 ID Register"]
pub mod mb11_8b_id;
#[doc = "MB2_64B_WORD7 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word7`] module"]
#[doc(alias = "MB2_64B_WORD7")]
pub type Mb2_64bWord7 = crate::Reg<mb2_64b_word7::Mb2_64bWord7Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word7;
#[doc = "MB4_32B_WORD3 (rw) register accessor: Message Buffer 4 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_32b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_32b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_32b_word3`] module"]
#[doc(alias = "MB4_32B_WORD3")]
pub type Mb4_32bWord3 = crate::Reg<mb4_32b_word3::Mb4_32bWord3Spec>;
#[doc = "Message Buffer 4 WORD_32B Register"]
pub mod mb4_32b_word3;
#[doc = "MB7_16B_WORD1 (rw) register accessor: Message Buffer 7 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_16b_word1`] module"]
#[doc(alias = "MB7_16B_WORD1")]
pub type Mb7_16bWord1 = crate::Reg<mb7_16b_word1::Mb7_16bWord1Spec>;
#[doc = "Message Buffer 7 WORD_16B Register"]
pub mod mb7_16b_word1;
#[doc = "MB11_8B_WORD0 (rw) register accessor: Message Buffer 11 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_8b_word0`] module"]
#[doc(alias = "MB11_8B_WORD0")]
pub type Mb11_8bWord0 = crate::Reg<mb11_8b_word0::Mb11_8bWord0Spec>;
#[doc = "Message Buffer 11 WORD_8B Register"]
pub mod mb11_8b_word0;
#[doc = "MB2_64B_WORD8 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word8`] module"]
#[doc(alias = "MB2_64B_WORD8")]
pub type Mb2_64bWord8 = crate::Reg<mb2_64b_word8::Mb2_64bWord8Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word8;
#[doc = "MB4_32B_WORD4 (rw) register accessor: Message Buffer 4 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_32b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_32b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_32b_word4`] module"]
#[doc(alias = "MB4_32B_WORD4")]
pub type Mb4_32bWord4 = crate::Reg<mb4_32b_word4::Mb4_32bWord4Spec>;
#[doc = "Message Buffer 4 WORD_32B Register"]
pub mod mb4_32b_word4;
#[doc = "MB7_16B_WORD2 (rw) register accessor: Message Buffer 7 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_16b_word2`] module"]
#[doc(alias = "MB7_16B_WORD2")]
pub type Mb7_16bWord2 = crate::Reg<mb7_16b_word2::Mb7_16bWord2Spec>;
#[doc = "Message Buffer 7 WORD_16B Register"]
pub mod mb7_16b_word2;
#[doc = "WORD011 (rw) register accessor: Message Buffer 11 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word011::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word011::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word011`] module"]
#[doc(alias = "WORD011")]
pub type Word011 = crate::Reg<word011::Word011Spec>;
#[doc = "Message Buffer 11 WORD0 Register"]
pub mod word011;
#[doc = "MB11_8B_WORD1 (rw) register accessor: Message Buffer 11 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_8b_word1`] module"]
#[doc(alias = "MB11_8B_WORD1")]
pub type Mb11_8bWord1 = crate::Reg<mb11_8b_word1::Mb11_8bWord1Spec>;
#[doc = "Message Buffer 11 WORD_8B Register"]
pub mod mb11_8b_word1;
#[doc = "MB2_64B_WORD9 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word9`] module"]
#[doc(alias = "MB2_64B_WORD9")]
pub type Mb2_64bWord9 = crate::Reg<mb2_64b_word9::Mb2_64bWord9Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word9;
#[doc = "MB4_32B_WORD5 (rw) register accessor: Message Buffer 4 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_32b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_32b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_32b_word5`] module"]
#[doc(alias = "MB4_32B_WORD5")]
pub type Mb4_32bWord5 = crate::Reg<mb4_32b_word5::Mb4_32bWord5Spec>;
#[doc = "Message Buffer 4 WORD_32B Register"]
pub mod mb4_32b_word5;
#[doc = "MB7_16B_WORD3 (rw) register accessor: Message Buffer 7 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_16b_word3`] module"]
#[doc(alias = "MB7_16B_WORD3")]
pub type Mb7_16bWord3 = crate::Reg<mb7_16b_word3::Mb7_16bWord3Spec>;
#[doc = "Message Buffer 7 WORD_16B Register"]
pub mod mb7_16b_word3;
#[doc = "WORD111 (rw) register accessor: Message Buffer 11 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word111::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word111`] module"]
#[doc(alias = "WORD111")]
pub type Word111 = crate::Reg<word111::Word111Spec>;
#[doc = "Message Buffer 11 WORD1 Register"]
pub mod word111;
#[doc = "CS12 (rw) register accessor: Message Buffer 12 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs12`] module"]
#[doc(alias = "CS12")]
pub type Cs12 = crate::Reg<cs12::Cs12Spec>;
#[doc = "Message Buffer 12 CS Register"]
pub mod cs12;
#[doc = "MB12_8B_CS (rw) register accessor: Message Buffer 12 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb12_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb12_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb12_8b_cs`] module"]
#[doc(alias = "MB12_8B_CS")]
pub type Mb12_8bCs = crate::Reg<mb12_8b_cs::Mb12_8bCsSpec>;
#[doc = "Message Buffer 12 CS Register"]
pub mod mb12_8b_cs;
#[doc = "MB2_64B_WORD10 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word10`] module"]
#[doc(alias = "MB2_64B_WORD10")]
pub type Mb2_64bWord10 = crate::Reg<mb2_64b_word10::Mb2_64bWord10Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word10;
#[doc = "MB4_32B_WORD6 (rw) register accessor: Message Buffer 4 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_32b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_32b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_32b_word6`] module"]
#[doc(alias = "MB4_32B_WORD6")]
pub type Mb4_32bWord6 = crate::Reg<mb4_32b_word6::Mb4_32bWord6Spec>;
#[doc = "Message Buffer 4 WORD_32B Register"]
pub mod mb4_32b_word6;
#[doc = "MB8_16B_CS (rw) register accessor: Message Buffer 8 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_16b_cs`] module"]
#[doc(alias = "MB8_16B_CS")]
pub type Mb8_16bCs = crate::Reg<mb8_16b_cs::Mb8_16bCsSpec>;
#[doc = "Message Buffer 8 CS Register"]
pub mod mb8_16b_cs;
#[doc = "ID12 (rw) register accessor: Message Buffer 12 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id12`] module"]
#[doc(alias = "ID12")]
pub type Id12 = crate::Reg<id12::Id12Spec>;
#[doc = "Message Buffer 12 ID Register"]
pub mod id12;
#[doc = "MB12_8B_ID (rw) register accessor: Message Buffer 12 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb12_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb12_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb12_8b_id`] module"]
#[doc(alias = "MB12_8B_ID")]
pub type Mb12_8bId = crate::Reg<mb12_8b_id::Mb12_8bIdSpec>;
#[doc = "Message Buffer 12 ID Register"]
pub mod mb12_8b_id;
#[doc = "MB2_64B_WORD11 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word11`] module"]
#[doc(alias = "MB2_64B_WORD11")]
pub type Mb2_64bWord11 = crate::Reg<mb2_64b_word11::Mb2_64bWord11Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word11;
#[doc = "MB4_32B_WORD7 (rw) register accessor: Message Buffer 4 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_32b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_32b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_32b_word7`] module"]
#[doc(alias = "MB4_32B_WORD7")]
pub type Mb4_32bWord7 = crate::Reg<mb4_32b_word7::Mb4_32bWord7Spec>;
#[doc = "Message Buffer 4 WORD_32B Register"]
pub mod mb4_32b_word7;
#[doc = "MB8_16B_ID (rw) register accessor: Message Buffer 8 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_16b_id`] module"]
#[doc(alias = "MB8_16B_ID")]
pub type Mb8_16bId = crate::Reg<mb8_16b_id::Mb8_16bIdSpec>;
#[doc = "Message Buffer 8 ID Register"]
pub mod mb8_16b_id;
#[doc = "MB12_8B_WORD0 (rw) register accessor: Message Buffer 12 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb12_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb12_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb12_8b_word0`] module"]
#[doc(alias = "MB12_8B_WORD0")]
pub type Mb12_8bWord0 = crate::Reg<mb12_8b_word0::Mb12_8bWord0Spec>;
#[doc = "Message Buffer 12 WORD_8B Register"]
pub mod mb12_8b_word0;
#[doc = "MB2_64B_WORD12 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word12`] module"]
#[doc(alias = "MB2_64B_WORD12")]
pub type Mb2_64bWord12 = crate::Reg<mb2_64b_word12::Mb2_64bWord12Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word12;
#[doc = "MB5_32B_CS (rw) register accessor: Message Buffer 5 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_32b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_32b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_32b_cs`] module"]
#[doc(alias = "MB5_32B_CS")]
pub type Mb5_32bCs = crate::Reg<mb5_32b_cs::Mb5_32bCsSpec>;
#[doc = "Message Buffer 5 CS Register"]
pub mod mb5_32b_cs;
#[doc = "MB8_16B_WORD0 (rw) register accessor: Message Buffer 8 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_16b_word0`] module"]
#[doc(alias = "MB8_16B_WORD0")]
pub type Mb8_16bWord0 = crate::Reg<mb8_16b_word0::Mb8_16bWord0Spec>;
#[doc = "Message Buffer 8 WORD_16B Register"]
pub mod mb8_16b_word0;
#[doc = "WORD012 (rw) register accessor: Message Buffer 12 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word012::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word012::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word012`] module"]
#[doc(alias = "WORD012")]
pub type Word012 = crate::Reg<word012::Word012Spec>;
#[doc = "Message Buffer 12 WORD0 Register"]
pub mod word012;
#[doc = "MB12_8B_WORD1 (rw) register accessor: Message Buffer 12 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb12_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb12_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb12_8b_word1`] module"]
#[doc(alias = "MB12_8B_WORD1")]
pub type Mb12_8bWord1 = crate::Reg<mb12_8b_word1::Mb12_8bWord1Spec>;
#[doc = "Message Buffer 12 WORD_8B Register"]
pub mod mb12_8b_word1;
#[doc = "MB2_64B_WORD13 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word13`] module"]
#[doc(alias = "MB2_64B_WORD13")]
pub type Mb2_64bWord13 = crate::Reg<mb2_64b_word13::Mb2_64bWord13Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word13;
#[doc = "MB5_32B_ID (rw) register accessor: Message Buffer 5 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_32b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_32b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_32b_id`] module"]
#[doc(alias = "MB5_32B_ID")]
pub type Mb5_32bId = crate::Reg<mb5_32b_id::Mb5_32bIdSpec>;
#[doc = "Message Buffer 5 ID Register"]
pub mod mb5_32b_id;
#[doc = "MB8_16B_WORD1 (rw) register accessor: Message Buffer 8 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_16b_word1`] module"]
#[doc(alias = "MB8_16B_WORD1")]
pub type Mb8_16bWord1 = crate::Reg<mb8_16b_word1::Mb8_16bWord1Spec>;
#[doc = "Message Buffer 8 WORD_16B Register"]
pub mod mb8_16b_word1;
#[doc = "WORD112 (rw) register accessor: Message Buffer 12 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word112::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word112::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word112`] module"]
#[doc(alias = "WORD112")]
pub type Word112 = crate::Reg<word112::Word112Spec>;
#[doc = "Message Buffer 12 WORD1 Register"]
pub mod word112;
#[doc = "CS13 (rw) register accessor: Message Buffer 13 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs13`] module"]
#[doc(alias = "CS13")]
pub type Cs13 = crate::Reg<cs13::Cs13Spec>;
#[doc = "Message Buffer 13 CS Register"]
pub mod cs13;
#[doc = "MB13_8B_CS (rw) register accessor: Message Buffer 13 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb13_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb13_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb13_8b_cs`] module"]
#[doc(alias = "MB13_8B_CS")]
pub type Mb13_8bCs = crate::Reg<mb13_8b_cs::Mb13_8bCsSpec>;
#[doc = "Message Buffer 13 CS Register"]
pub mod mb13_8b_cs;
#[doc = "MB2_64B_WORD14 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word14`] module"]
#[doc(alias = "MB2_64B_WORD14")]
pub type Mb2_64bWord14 = crate::Reg<mb2_64b_word14::Mb2_64bWord14Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word14;
#[doc = "MB5_32B_WORD0 (rw) register accessor: Message Buffer 5 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_32b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_32b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_32b_word0`] module"]
#[doc(alias = "MB5_32B_WORD0")]
pub type Mb5_32bWord0 = crate::Reg<mb5_32b_word0::Mb5_32bWord0Spec>;
#[doc = "Message Buffer 5 WORD_32B Register"]
pub mod mb5_32b_word0;
#[doc = "MB8_16B_WORD2 (rw) register accessor: Message Buffer 8 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_16b_word2`] module"]
#[doc(alias = "MB8_16B_WORD2")]
pub type Mb8_16bWord2 = crate::Reg<mb8_16b_word2::Mb8_16bWord2Spec>;
#[doc = "Message Buffer 8 WORD_16B Register"]
pub mod mb8_16b_word2;
#[doc = "ID13 (rw) register accessor: Message Buffer 13 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id13`] module"]
#[doc(alias = "ID13")]
pub type Id13 = crate::Reg<id13::Id13Spec>;
#[doc = "Message Buffer 13 ID Register"]
pub mod id13;
#[doc = "MB13_8B_ID (rw) register accessor: Message Buffer 13 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb13_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb13_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb13_8b_id`] module"]
#[doc(alias = "MB13_8B_ID")]
pub type Mb13_8bId = crate::Reg<mb13_8b_id::Mb13_8bIdSpec>;
#[doc = "Message Buffer 13 ID Register"]
pub mod mb13_8b_id;
#[doc = "MB2_64B_WORD15 (rw) register accessor: Message Buffer 2 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb2_64b_word15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb2_64b_word15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb2_64b_word15`] module"]
#[doc(alias = "MB2_64B_WORD15")]
pub type Mb2_64bWord15 = crate::Reg<mb2_64b_word15::Mb2_64bWord15Spec>;
#[doc = "Message Buffer 2 WORD_64B Register"]
pub mod mb2_64b_word15;
#[doc = "MB5_32B_WORD1 (rw) register accessor: Message Buffer 5 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_32b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_32b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_32b_word1`] module"]
#[doc(alias = "MB5_32B_WORD1")]
pub type Mb5_32bWord1 = crate::Reg<mb5_32b_word1::Mb5_32bWord1Spec>;
#[doc = "Message Buffer 5 WORD_32B Register"]
pub mod mb5_32b_word1;
#[doc = "MB8_16B_WORD3 (rw) register accessor: Message Buffer 8 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_16b_word3`] module"]
#[doc(alias = "MB8_16B_WORD3")]
pub type Mb8_16bWord3 = crate::Reg<mb8_16b_word3::Mb8_16bWord3Spec>;
#[doc = "Message Buffer 8 WORD_16B Register"]
pub mod mb8_16b_word3;
#[doc = "MB13_8B_WORD0 (rw) register accessor: Message Buffer 13 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb13_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb13_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb13_8b_word0`] module"]
#[doc(alias = "MB13_8B_WORD0")]
pub type Mb13_8bWord0 = crate::Reg<mb13_8b_word0::Mb13_8bWord0Spec>;
#[doc = "Message Buffer 13 WORD_8B Register"]
pub mod mb13_8b_word0;
#[doc = "MB3_64B_CS (rw) register accessor: Message Buffer 3 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_cs`] module"]
#[doc(alias = "MB3_64B_CS")]
pub type Mb3_64bCs = crate::Reg<mb3_64b_cs::Mb3_64bCsSpec>;
#[doc = "Message Buffer 3 CS Register"]
pub mod mb3_64b_cs;
#[doc = "MB5_32B_WORD2 (rw) register accessor: Message Buffer 5 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_32b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_32b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_32b_word2`] module"]
#[doc(alias = "MB5_32B_WORD2")]
pub type Mb5_32bWord2 = crate::Reg<mb5_32b_word2::Mb5_32bWord2Spec>;
#[doc = "Message Buffer 5 WORD_32B Register"]
pub mod mb5_32b_word2;
#[doc = "MB9_16B_CS (rw) register accessor: Message Buffer 9 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_16b_cs`] module"]
#[doc(alias = "MB9_16B_CS")]
pub type Mb9_16bCs = crate::Reg<mb9_16b_cs::Mb9_16bCsSpec>;
#[doc = "Message Buffer 9 CS Register"]
pub mod mb9_16b_cs;
#[doc = "WORD013 (rw) register accessor: Message Buffer 13 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word013::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word013::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word013`] module"]
#[doc(alias = "WORD013")]
pub type Word013 = crate::Reg<word013::Word013Spec>;
#[doc = "Message Buffer 13 WORD0 Register"]
pub mod word013;
#[doc = "MB13_8B_WORD1 (rw) register accessor: Message Buffer 13 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb13_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb13_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb13_8b_word1`] module"]
#[doc(alias = "MB13_8B_WORD1")]
pub type Mb13_8bWord1 = crate::Reg<mb13_8b_word1::Mb13_8bWord1Spec>;
#[doc = "Message Buffer 13 WORD_8B Register"]
pub mod mb13_8b_word1;
#[doc = "MB3_64B_ID (rw) register accessor: Message Buffer 3 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_id`] module"]
#[doc(alias = "MB3_64B_ID")]
pub type Mb3_64bId = crate::Reg<mb3_64b_id::Mb3_64bIdSpec>;
#[doc = "Message Buffer 3 ID Register"]
pub mod mb3_64b_id;
#[doc = "MB5_32B_WORD3 (rw) register accessor: Message Buffer 5 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_32b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_32b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_32b_word3`] module"]
#[doc(alias = "MB5_32B_WORD3")]
pub type Mb5_32bWord3 = crate::Reg<mb5_32b_word3::Mb5_32bWord3Spec>;
#[doc = "Message Buffer 5 WORD_32B Register"]
pub mod mb5_32b_word3;
#[doc = "MB9_16B_ID (rw) register accessor: Message Buffer 9 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_16b_id`] module"]
#[doc(alias = "MB9_16B_ID")]
pub type Mb9_16bId = crate::Reg<mb9_16b_id::Mb9_16bIdSpec>;
#[doc = "Message Buffer 9 ID Register"]
pub mod mb9_16b_id;
#[doc = "WORD113 (rw) register accessor: Message Buffer 13 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word113::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word113::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word113`] module"]
#[doc(alias = "WORD113")]
pub type Word113 = crate::Reg<word113::Word113Spec>;
#[doc = "Message Buffer 13 WORD1 Register"]
pub mod word113;
#[doc = "CS14 (rw) register accessor: Message Buffer 14 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs14`] module"]
#[doc(alias = "CS14")]
pub type Cs14 = crate::Reg<cs14::Cs14Spec>;
#[doc = "Message Buffer 14 CS Register"]
pub mod cs14;
#[doc = "MB14_8B_CS (rw) register accessor: Message Buffer 14 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb14_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb14_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb14_8b_cs`] module"]
#[doc(alias = "MB14_8B_CS")]
pub type Mb14_8bCs = crate::Reg<mb14_8b_cs::Mb14_8bCsSpec>;
#[doc = "Message Buffer 14 CS Register"]
pub mod mb14_8b_cs;
#[doc = "MB3_64B_WORD0 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word0`] module"]
#[doc(alias = "MB3_64B_WORD0")]
pub type Mb3_64bWord0 = crate::Reg<mb3_64b_word0::Mb3_64bWord0Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word0;
#[doc = "MB5_32B_WORD4 (rw) register accessor: Message Buffer 5 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_32b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_32b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_32b_word4`] module"]
#[doc(alias = "MB5_32B_WORD4")]
pub type Mb5_32bWord4 = crate::Reg<mb5_32b_word4::Mb5_32bWord4Spec>;
#[doc = "Message Buffer 5 WORD_32B Register"]
pub mod mb5_32b_word4;
#[doc = "MB9_16B_WORD0 (rw) register accessor: Message Buffer 9 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_16b_word0`] module"]
#[doc(alias = "MB9_16B_WORD0")]
pub type Mb9_16bWord0 = crate::Reg<mb9_16b_word0::Mb9_16bWord0Spec>;
#[doc = "Message Buffer 9 WORD_16B Register"]
pub mod mb9_16b_word0;
#[doc = "ID14 (rw) register accessor: Message Buffer 14 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id14`] module"]
#[doc(alias = "ID14")]
pub type Id14 = crate::Reg<id14::Id14Spec>;
#[doc = "Message Buffer 14 ID Register"]
pub mod id14;
#[doc = "MB14_8B_ID (rw) register accessor: Message Buffer 14 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb14_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb14_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb14_8b_id`] module"]
#[doc(alias = "MB14_8B_ID")]
pub type Mb14_8bId = crate::Reg<mb14_8b_id::Mb14_8bIdSpec>;
#[doc = "Message Buffer 14 ID Register"]
pub mod mb14_8b_id;
#[doc = "MB3_64B_WORD1 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word1`] module"]
#[doc(alias = "MB3_64B_WORD1")]
pub type Mb3_64bWord1 = crate::Reg<mb3_64b_word1::Mb3_64bWord1Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word1;
#[doc = "MB5_32B_WORD5 (rw) register accessor: Message Buffer 5 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_32b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_32b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_32b_word5`] module"]
#[doc(alias = "MB5_32B_WORD5")]
pub type Mb5_32bWord5 = crate::Reg<mb5_32b_word5::Mb5_32bWord5Spec>;
#[doc = "Message Buffer 5 WORD_32B Register"]
pub mod mb5_32b_word5;
#[doc = "MB9_16B_WORD1 (rw) register accessor: Message Buffer 9 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_16b_word1`] module"]
#[doc(alias = "MB9_16B_WORD1")]
pub type Mb9_16bWord1 = crate::Reg<mb9_16b_word1::Mb9_16bWord1Spec>;
#[doc = "Message Buffer 9 WORD_16B Register"]
pub mod mb9_16b_word1;
#[doc = "MB14_8B_WORD0 (rw) register accessor: Message Buffer 14 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb14_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb14_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb14_8b_word0`] module"]
#[doc(alias = "MB14_8B_WORD0")]
pub type Mb14_8bWord0 = crate::Reg<mb14_8b_word0::Mb14_8bWord0Spec>;
#[doc = "Message Buffer 14 WORD_8B Register"]
pub mod mb14_8b_word0;
#[doc = "MB3_64B_WORD2 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word2`] module"]
#[doc(alias = "MB3_64B_WORD2")]
pub type Mb3_64bWord2 = crate::Reg<mb3_64b_word2::Mb3_64bWord2Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word2;
#[doc = "MB5_32B_WORD6 (rw) register accessor: Message Buffer 5 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_32b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_32b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_32b_word6`] module"]
#[doc(alias = "MB5_32B_WORD6")]
pub type Mb5_32bWord6 = crate::Reg<mb5_32b_word6::Mb5_32bWord6Spec>;
#[doc = "Message Buffer 5 WORD_32B Register"]
pub mod mb5_32b_word6;
#[doc = "MB9_16B_WORD2 (rw) register accessor: Message Buffer 9 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_16b_word2`] module"]
#[doc(alias = "MB9_16B_WORD2")]
pub type Mb9_16bWord2 = crate::Reg<mb9_16b_word2::Mb9_16bWord2Spec>;
#[doc = "Message Buffer 9 WORD_16B Register"]
pub mod mb9_16b_word2;
#[doc = "WORD014 (rw) register accessor: Message Buffer 14 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word014`] module"]
#[doc(alias = "WORD014")]
pub type Word014 = crate::Reg<word014::Word014Spec>;
#[doc = "Message Buffer 14 WORD0 Register"]
pub mod word014;
#[doc = "MB14_8B_WORD1 (rw) register accessor: Message Buffer 14 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb14_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb14_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb14_8b_word1`] module"]
#[doc(alias = "MB14_8B_WORD1")]
pub type Mb14_8bWord1 = crate::Reg<mb14_8b_word1::Mb14_8bWord1Spec>;
#[doc = "Message Buffer 14 WORD_8B Register"]
pub mod mb14_8b_word1;
#[doc = "MB3_64B_WORD3 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word3`] module"]
#[doc(alias = "MB3_64B_WORD3")]
pub type Mb3_64bWord3 = crate::Reg<mb3_64b_word3::Mb3_64bWord3Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word3;
#[doc = "MB5_32B_WORD7 (rw) register accessor: Message Buffer 5 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_32b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_32b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_32b_word7`] module"]
#[doc(alias = "MB5_32B_WORD7")]
pub type Mb5_32bWord7 = crate::Reg<mb5_32b_word7::Mb5_32bWord7Spec>;
#[doc = "Message Buffer 5 WORD_32B Register"]
pub mod mb5_32b_word7;
#[doc = "MB9_16B_WORD3 (rw) register accessor: Message Buffer 9 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_16b_word3`] module"]
#[doc(alias = "MB9_16B_WORD3")]
pub type Mb9_16bWord3 = crate::Reg<mb9_16b_word3::Mb9_16bWord3Spec>;
#[doc = "Message Buffer 9 WORD_16B Register"]
pub mod mb9_16b_word3;
#[doc = "WORD114 (rw) register accessor: Message Buffer 14 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word114`] module"]
#[doc(alias = "WORD114")]
pub type Word114 = crate::Reg<word114::Word114Spec>;
#[doc = "Message Buffer 14 WORD1 Register"]
pub mod word114;
#[doc = "CS15 (rw) register accessor: Message Buffer 15 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs15`] module"]
#[doc(alias = "CS15")]
pub type Cs15 = crate::Reg<cs15::Cs15Spec>;
#[doc = "Message Buffer 15 CS Register"]
pub mod cs15;
#[doc = "MB10_16B_CS (rw) register accessor: Message Buffer 10 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_16b_cs`] module"]
#[doc(alias = "MB10_16B_CS")]
pub type Mb10_16bCs = crate::Reg<mb10_16b_cs::Mb10_16bCsSpec>;
#[doc = "Message Buffer 10 CS Register"]
pub mod mb10_16b_cs;
#[doc = "MB15_8B_CS (rw) register accessor: Message Buffer 15 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb15_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb15_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb15_8b_cs`] module"]
#[doc(alias = "MB15_8B_CS")]
pub type Mb15_8bCs = crate::Reg<mb15_8b_cs::Mb15_8bCsSpec>;
#[doc = "Message Buffer 15 CS Register"]
pub mod mb15_8b_cs;
#[doc = "MB3_64B_WORD4 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word4`] module"]
#[doc(alias = "MB3_64B_WORD4")]
pub type Mb3_64bWord4 = crate::Reg<mb3_64b_word4::Mb3_64bWord4Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word4;
#[doc = "MB6_32B_CS (rw) register accessor: Message Buffer 6 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_32b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_32b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_32b_cs`] module"]
#[doc(alias = "MB6_32B_CS")]
pub type Mb6_32bCs = crate::Reg<mb6_32b_cs::Mb6_32bCsSpec>;
#[doc = "Message Buffer 6 CS Register"]
pub mod mb6_32b_cs;
#[doc = "ID15 (rw) register accessor: Message Buffer 15 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id15`] module"]
#[doc(alias = "ID15")]
pub type Id15 = crate::Reg<id15::Id15Spec>;
#[doc = "Message Buffer 15 ID Register"]
pub mod id15;
#[doc = "MB10_16B_ID (rw) register accessor: Message Buffer 10 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_16b_id`] module"]
#[doc(alias = "MB10_16B_ID")]
pub type Mb10_16bId = crate::Reg<mb10_16b_id::Mb10_16bIdSpec>;
#[doc = "Message Buffer 10 ID Register"]
pub mod mb10_16b_id;
#[doc = "MB15_8B_ID (rw) register accessor: Message Buffer 15 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb15_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb15_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb15_8b_id`] module"]
#[doc(alias = "MB15_8B_ID")]
pub type Mb15_8bId = crate::Reg<mb15_8b_id::Mb15_8bIdSpec>;
#[doc = "Message Buffer 15 ID Register"]
pub mod mb15_8b_id;
#[doc = "MB3_64B_WORD5 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word5`] module"]
#[doc(alias = "MB3_64B_WORD5")]
pub type Mb3_64bWord5 = crate::Reg<mb3_64b_word5::Mb3_64bWord5Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word5;
#[doc = "MB6_32B_ID (rw) register accessor: Message Buffer 6 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_32b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_32b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_32b_id`] module"]
#[doc(alias = "MB6_32B_ID")]
pub type Mb6_32bId = crate::Reg<mb6_32b_id::Mb6_32bIdSpec>;
#[doc = "Message Buffer 6 ID Register"]
pub mod mb6_32b_id;
#[doc = "MB10_16B_WORD0 (rw) register accessor: Message Buffer 10 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_16b_word0`] module"]
#[doc(alias = "MB10_16B_WORD0")]
pub type Mb10_16bWord0 = crate::Reg<mb10_16b_word0::Mb10_16bWord0Spec>;
#[doc = "Message Buffer 10 WORD_16B Register"]
pub mod mb10_16b_word0;
#[doc = "MB15_8B_WORD0 (rw) register accessor: Message Buffer 15 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb15_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb15_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb15_8b_word0`] module"]
#[doc(alias = "MB15_8B_WORD0")]
pub type Mb15_8bWord0 = crate::Reg<mb15_8b_word0::Mb15_8bWord0Spec>;
#[doc = "Message Buffer 15 WORD_8B Register"]
pub mod mb15_8b_word0;
#[doc = "MB3_64B_WORD6 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word6`] module"]
#[doc(alias = "MB3_64B_WORD6")]
pub type Mb3_64bWord6 = crate::Reg<mb3_64b_word6::Mb3_64bWord6Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word6;
#[doc = "MB6_32B_WORD0 (rw) register accessor: Message Buffer 6 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_32b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_32b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_32b_word0`] module"]
#[doc(alias = "MB6_32B_WORD0")]
pub type Mb6_32bWord0 = crate::Reg<mb6_32b_word0::Mb6_32bWord0Spec>;
#[doc = "Message Buffer 6 WORD_32B Register"]
pub mod mb6_32b_word0;
#[doc = "WORD015 (rw) register accessor: Message Buffer 15 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word015::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word015::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word015`] module"]
#[doc(alias = "WORD015")]
pub type Word015 = crate::Reg<word015::Word015Spec>;
#[doc = "Message Buffer 15 WORD0 Register"]
pub mod word015;
#[doc = "MB10_16B_WORD1 (rw) register accessor: Message Buffer 10 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_16b_word1`] module"]
#[doc(alias = "MB10_16B_WORD1")]
pub type Mb10_16bWord1 = crate::Reg<mb10_16b_word1::Mb10_16bWord1Spec>;
#[doc = "Message Buffer 10 WORD_16B Register"]
pub mod mb10_16b_word1;
#[doc = "MB15_8B_WORD1 (rw) register accessor: Message Buffer 15 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb15_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb15_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb15_8b_word1`] module"]
#[doc(alias = "MB15_8B_WORD1")]
pub type Mb15_8bWord1 = crate::Reg<mb15_8b_word1::Mb15_8bWord1Spec>;
#[doc = "Message Buffer 15 WORD_8B Register"]
pub mod mb15_8b_word1;
#[doc = "MB3_64B_WORD7 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word7`] module"]
#[doc(alias = "MB3_64B_WORD7")]
pub type Mb3_64bWord7 = crate::Reg<mb3_64b_word7::Mb3_64bWord7Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word7;
#[doc = "MB6_32B_WORD1 (rw) register accessor: Message Buffer 6 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_32b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_32b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_32b_word1`] module"]
#[doc(alias = "MB6_32B_WORD1")]
pub type Mb6_32bWord1 = crate::Reg<mb6_32b_word1::Mb6_32bWord1Spec>;
#[doc = "Message Buffer 6 WORD_32B Register"]
pub mod mb6_32b_word1;
#[doc = "WORD115 (rw) register accessor: Message Buffer 15 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word115::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word115::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word115`] module"]
#[doc(alias = "WORD115")]
pub type Word115 = crate::Reg<word115::Word115Spec>;
#[doc = "Message Buffer 15 WORD1 Register"]
pub mod word115;
#[doc = "CS16 (rw) register accessor: Message Buffer 16 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs16`] module"]
#[doc(alias = "CS16")]
pub type Cs16 = crate::Reg<cs16::Cs16Spec>;
#[doc = "Message Buffer 16 CS Register"]
pub mod cs16;
#[doc = "MB10_16B_WORD2 (rw) register accessor: Message Buffer 10 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_16b_word2`] module"]
#[doc(alias = "MB10_16B_WORD2")]
pub type Mb10_16bWord2 = crate::Reg<mb10_16b_word2::Mb10_16bWord2Spec>;
#[doc = "Message Buffer 10 WORD_16B Register"]
pub mod mb10_16b_word2;
#[doc = "MB16_8B_CS (rw) register accessor: Message Buffer 16 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb16_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb16_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb16_8b_cs`] module"]
#[doc(alias = "MB16_8B_CS")]
pub type Mb16_8bCs = crate::Reg<mb16_8b_cs::Mb16_8bCsSpec>;
#[doc = "Message Buffer 16 CS Register"]
pub mod mb16_8b_cs;
#[doc = "MB3_64B_WORD8 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word8`] module"]
#[doc(alias = "MB3_64B_WORD8")]
pub type Mb3_64bWord8 = crate::Reg<mb3_64b_word8::Mb3_64bWord8Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word8;
#[doc = "MB6_32B_WORD2 (rw) register accessor: Message Buffer 6 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_32b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_32b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_32b_word2`] module"]
#[doc(alias = "MB6_32B_WORD2")]
pub type Mb6_32bWord2 = crate::Reg<mb6_32b_word2::Mb6_32bWord2Spec>;
#[doc = "Message Buffer 6 WORD_32B Register"]
pub mod mb6_32b_word2;
#[doc = "ID16 (rw) register accessor: Message Buffer 16 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id16`] module"]
#[doc(alias = "ID16")]
pub type Id16 = crate::Reg<id16::Id16Spec>;
#[doc = "Message Buffer 16 ID Register"]
pub mod id16;
#[doc = "MB10_16B_WORD3 (rw) register accessor: Message Buffer 10 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_16b_word3`] module"]
#[doc(alias = "MB10_16B_WORD3")]
pub type Mb10_16bWord3 = crate::Reg<mb10_16b_word3::Mb10_16bWord3Spec>;
#[doc = "Message Buffer 10 WORD_16B Register"]
pub mod mb10_16b_word3;
#[doc = "MB16_8B_ID (rw) register accessor: Message Buffer 16 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb16_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb16_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb16_8b_id`] module"]
#[doc(alias = "MB16_8B_ID")]
pub type Mb16_8bId = crate::Reg<mb16_8b_id::Mb16_8bIdSpec>;
#[doc = "Message Buffer 16 ID Register"]
pub mod mb16_8b_id;
#[doc = "MB3_64B_WORD9 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word9`] module"]
#[doc(alias = "MB3_64B_WORD9")]
pub type Mb3_64bWord9 = crate::Reg<mb3_64b_word9::Mb3_64bWord9Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word9;
#[doc = "MB6_32B_WORD3 (rw) register accessor: Message Buffer 6 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_32b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_32b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_32b_word3`] module"]
#[doc(alias = "MB6_32B_WORD3")]
pub type Mb6_32bWord3 = crate::Reg<mb6_32b_word3::Mb6_32bWord3Spec>;
#[doc = "Message Buffer 6 WORD_32B Register"]
pub mod mb6_32b_word3;
#[doc = "MB11_16B_CS (rw) register accessor: Message Buffer 11 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_16b_cs`] module"]
#[doc(alias = "MB11_16B_CS")]
pub type Mb11_16bCs = crate::Reg<mb11_16b_cs::Mb11_16bCsSpec>;
#[doc = "Message Buffer 11 CS Register"]
pub mod mb11_16b_cs;
#[doc = "MB16_8B_WORD0 (rw) register accessor: Message Buffer 16 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb16_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb16_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb16_8b_word0`] module"]
#[doc(alias = "MB16_8B_WORD0")]
pub type Mb16_8bWord0 = crate::Reg<mb16_8b_word0::Mb16_8bWord0Spec>;
#[doc = "Message Buffer 16 WORD_8B Register"]
pub mod mb16_8b_word0;
#[doc = "MB3_64B_WORD10 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word10`] module"]
#[doc(alias = "MB3_64B_WORD10")]
pub type Mb3_64bWord10 = crate::Reg<mb3_64b_word10::Mb3_64bWord10Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word10;
#[doc = "MB6_32B_WORD4 (rw) register accessor: Message Buffer 6 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_32b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_32b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_32b_word4`] module"]
#[doc(alias = "MB6_32B_WORD4")]
pub type Mb6_32bWord4 = crate::Reg<mb6_32b_word4::Mb6_32bWord4Spec>;
#[doc = "Message Buffer 6 WORD_32B Register"]
pub mod mb6_32b_word4;
#[doc = "WORD016 (rw) register accessor: Message Buffer 16 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word016::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word016::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word016`] module"]
#[doc(alias = "WORD016")]
pub type Word016 = crate::Reg<word016::Word016Spec>;
#[doc = "Message Buffer 16 WORD0 Register"]
pub mod word016;
#[doc = "MB11_16B_ID (rw) register accessor: Message Buffer 11 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_16b_id`] module"]
#[doc(alias = "MB11_16B_ID")]
pub type Mb11_16bId = crate::Reg<mb11_16b_id::Mb11_16bIdSpec>;
#[doc = "Message Buffer 11 ID Register"]
pub mod mb11_16b_id;
#[doc = "MB16_8B_WORD1 (rw) register accessor: Message Buffer 16 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb16_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb16_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb16_8b_word1`] module"]
#[doc(alias = "MB16_8B_WORD1")]
pub type Mb16_8bWord1 = crate::Reg<mb16_8b_word1::Mb16_8bWord1Spec>;
#[doc = "Message Buffer 16 WORD_8B Register"]
pub mod mb16_8b_word1;
#[doc = "MB3_64B_WORD11 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word11`] module"]
#[doc(alias = "MB3_64B_WORD11")]
pub type Mb3_64bWord11 = crate::Reg<mb3_64b_word11::Mb3_64bWord11Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word11;
#[doc = "MB6_32B_WORD5 (rw) register accessor: Message Buffer 6 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_32b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_32b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_32b_word5`] module"]
#[doc(alias = "MB6_32B_WORD5")]
pub type Mb6_32bWord5 = crate::Reg<mb6_32b_word5::Mb6_32bWord5Spec>;
#[doc = "Message Buffer 6 WORD_32B Register"]
pub mod mb6_32b_word5;
#[doc = "WORD116 (rw) register accessor: Message Buffer 16 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word116::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word116::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word116`] module"]
#[doc(alias = "WORD116")]
pub type Word116 = crate::Reg<word116::Word116Spec>;
#[doc = "Message Buffer 16 WORD1 Register"]
pub mod word116;
#[doc = "CS17 (rw) register accessor: Message Buffer 17 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs17`] module"]
#[doc(alias = "CS17")]
pub type Cs17 = crate::Reg<cs17::Cs17Spec>;
#[doc = "Message Buffer 17 CS Register"]
pub mod cs17;
#[doc = "MB11_16B_WORD0 (rw) register accessor: Message Buffer 11 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_16b_word0`] module"]
#[doc(alias = "MB11_16B_WORD0")]
pub type Mb11_16bWord0 = crate::Reg<mb11_16b_word0::Mb11_16bWord0Spec>;
#[doc = "Message Buffer 11 WORD_16B Register"]
pub mod mb11_16b_word0;
#[doc = "MB17_8B_CS (rw) register accessor: Message Buffer 17 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb17_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb17_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb17_8b_cs`] module"]
#[doc(alias = "MB17_8B_CS")]
pub type Mb17_8bCs = crate::Reg<mb17_8b_cs::Mb17_8bCsSpec>;
#[doc = "Message Buffer 17 CS Register"]
pub mod mb17_8b_cs;
#[doc = "MB3_64B_WORD12 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word12`] module"]
#[doc(alias = "MB3_64B_WORD12")]
pub type Mb3_64bWord12 = crate::Reg<mb3_64b_word12::Mb3_64bWord12Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word12;
#[doc = "MB6_32B_WORD6 (rw) register accessor: Message Buffer 6 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_32b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_32b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_32b_word6`] module"]
#[doc(alias = "MB6_32B_WORD6")]
pub type Mb6_32bWord6 = crate::Reg<mb6_32b_word6::Mb6_32bWord6Spec>;
#[doc = "Message Buffer 6 WORD_32B Register"]
pub mod mb6_32b_word6;
#[doc = "ID17 (rw) register accessor: Message Buffer 17 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id17`] module"]
#[doc(alias = "ID17")]
pub type Id17 = crate::Reg<id17::Id17Spec>;
#[doc = "Message Buffer 17 ID Register"]
pub mod id17;
#[doc = "MB11_16B_WORD1 (rw) register accessor: Message Buffer 11 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_16b_word1`] module"]
#[doc(alias = "MB11_16B_WORD1")]
pub type Mb11_16bWord1 = crate::Reg<mb11_16b_word1::Mb11_16bWord1Spec>;
#[doc = "Message Buffer 11 WORD_16B Register"]
pub mod mb11_16b_word1;
#[doc = "MB17_8B_ID (rw) register accessor: Message Buffer 17 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb17_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb17_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb17_8b_id`] module"]
#[doc(alias = "MB17_8B_ID")]
pub type Mb17_8bId = crate::Reg<mb17_8b_id::Mb17_8bIdSpec>;
#[doc = "Message Buffer 17 ID Register"]
pub mod mb17_8b_id;
#[doc = "MB3_64B_WORD13 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word13`] module"]
#[doc(alias = "MB3_64B_WORD13")]
pub type Mb3_64bWord13 = crate::Reg<mb3_64b_word13::Mb3_64bWord13Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word13;
#[doc = "MB6_32B_WORD7 (rw) register accessor: Message Buffer 6 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_32b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_32b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_32b_word7`] module"]
#[doc(alias = "MB6_32B_WORD7")]
pub type Mb6_32bWord7 = crate::Reg<mb6_32b_word7::Mb6_32bWord7Spec>;
#[doc = "Message Buffer 6 WORD_32B Register"]
pub mod mb6_32b_word7;
#[doc = "MB11_16B_WORD2 (rw) register accessor: Message Buffer 11 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_16b_word2`] module"]
#[doc(alias = "MB11_16B_WORD2")]
pub type Mb11_16bWord2 = crate::Reg<mb11_16b_word2::Mb11_16bWord2Spec>;
#[doc = "Message Buffer 11 WORD_16B Register"]
pub mod mb11_16b_word2;
#[doc = "MB17_8B_WORD0 (rw) register accessor: Message Buffer 17 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb17_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb17_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb17_8b_word0`] module"]
#[doc(alias = "MB17_8B_WORD0")]
pub type Mb17_8bWord0 = crate::Reg<mb17_8b_word0::Mb17_8bWord0Spec>;
#[doc = "Message Buffer 17 WORD_8B Register"]
pub mod mb17_8b_word0;
#[doc = "MB3_64B_WORD14 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word14`] module"]
#[doc(alias = "MB3_64B_WORD14")]
pub type Mb3_64bWord14 = crate::Reg<mb3_64b_word14::Mb3_64bWord14Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word14;
#[doc = "MB7_32B_CS (rw) register accessor: Message Buffer 7 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_32b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_32b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_32b_cs`] module"]
#[doc(alias = "MB7_32B_CS")]
pub type Mb7_32bCs = crate::Reg<mb7_32b_cs::Mb7_32bCsSpec>;
#[doc = "Message Buffer 7 CS Register"]
pub mod mb7_32b_cs;
#[doc = "WORD017 (rw) register accessor: Message Buffer 17 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word017::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word017::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word017`] module"]
#[doc(alias = "WORD017")]
pub type Word017 = crate::Reg<word017::Word017Spec>;
#[doc = "Message Buffer 17 WORD0 Register"]
pub mod word017;
#[doc = "MB11_16B_WORD3 (rw) register accessor: Message Buffer 11 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_16b_word3`] module"]
#[doc(alias = "MB11_16B_WORD3")]
pub type Mb11_16bWord3 = crate::Reg<mb11_16b_word3::Mb11_16bWord3Spec>;
#[doc = "Message Buffer 11 WORD_16B Register"]
pub mod mb11_16b_word3;
#[doc = "MB17_8B_WORD1 (rw) register accessor: Message Buffer 17 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb17_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb17_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb17_8b_word1`] module"]
#[doc(alias = "MB17_8B_WORD1")]
pub type Mb17_8bWord1 = crate::Reg<mb17_8b_word1::Mb17_8bWord1Spec>;
#[doc = "Message Buffer 17 WORD_8B Register"]
pub mod mb17_8b_word1;
#[doc = "MB3_64B_WORD15 (rw) register accessor: Message Buffer 3 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb3_64b_word15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb3_64b_word15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb3_64b_word15`] module"]
#[doc(alias = "MB3_64B_WORD15")]
pub type Mb3_64bWord15 = crate::Reg<mb3_64b_word15::Mb3_64bWord15Spec>;
#[doc = "Message Buffer 3 WORD_64B Register"]
pub mod mb3_64b_word15;
#[doc = "MB7_32B_ID (rw) register accessor: Message Buffer 7 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_32b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_32b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_32b_id`] module"]
#[doc(alias = "MB7_32B_ID")]
pub type Mb7_32bId = crate::Reg<mb7_32b_id::Mb7_32bIdSpec>;
#[doc = "Message Buffer 7 ID Register"]
pub mod mb7_32b_id;
#[doc = "WORD117 (rw) register accessor: Message Buffer 17 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word117::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word117::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word117`] module"]
#[doc(alias = "WORD117")]
pub type Word117 = crate::Reg<word117::Word117Spec>;
#[doc = "Message Buffer 17 WORD1 Register"]
pub mod word117;
#[doc = "CS18 (rw) register accessor: Message Buffer 18 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs18`] module"]
#[doc(alias = "CS18")]
pub type Cs18 = crate::Reg<cs18::Cs18Spec>;
#[doc = "Message Buffer 18 CS Register"]
pub mod cs18;
#[doc = "MB12_16B_CS (rw) register accessor: Message Buffer 12 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb12_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb12_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb12_16b_cs`] module"]
#[doc(alias = "MB12_16B_CS")]
pub type Mb12_16bCs = crate::Reg<mb12_16b_cs::Mb12_16bCsSpec>;
#[doc = "Message Buffer 12 CS Register"]
pub mod mb12_16b_cs;
#[doc = "MB18_8B_CS (rw) register accessor: Message Buffer 18 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb18_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb18_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb18_8b_cs`] module"]
#[doc(alias = "MB18_8B_CS")]
pub type Mb18_8bCs = crate::Reg<mb18_8b_cs::Mb18_8bCsSpec>;
#[doc = "Message Buffer 18 CS Register"]
pub mod mb18_8b_cs;
#[doc = "MB4_64B_CS (rw) register accessor: Message Buffer 4 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_cs`] module"]
#[doc(alias = "MB4_64B_CS")]
pub type Mb4_64bCs = crate::Reg<mb4_64b_cs::Mb4_64bCsSpec>;
#[doc = "Message Buffer 4 CS Register"]
pub mod mb4_64b_cs;
#[doc = "MB7_32B_WORD0 (rw) register accessor: Message Buffer 7 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_32b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_32b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_32b_word0`] module"]
#[doc(alias = "MB7_32B_WORD0")]
pub type Mb7_32bWord0 = crate::Reg<mb7_32b_word0::Mb7_32bWord0Spec>;
#[doc = "Message Buffer 7 WORD_32B Register"]
pub mod mb7_32b_word0;
#[doc = "ID18 (rw) register accessor: Message Buffer 18 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id18`] module"]
#[doc(alias = "ID18")]
pub type Id18 = crate::Reg<id18::Id18Spec>;
#[doc = "Message Buffer 18 ID Register"]
pub mod id18;
#[doc = "MB12_16B_ID (rw) register accessor: Message Buffer 12 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb12_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb12_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb12_16b_id`] module"]
#[doc(alias = "MB12_16B_ID")]
pub type Mb12_16bId = crate::Reg<mb12_16b_id::Mb12_16bIdSpec>;
#[doc = "Message Buffer 12 ID Register"]
pub mod mb12_16b_id;
#[doc = "MB18_8B_ID (rw) register accessor: Message Buffer 18 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb18_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb18_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb18_8b_id`] module"]
#[doc(alias = "MB18_8B_ID")]
pub type Mb18_8bId = crate::Reg<mb18_8b_id::Mb18_8bIdSpec>;
#[doc = "Message Buffer 18 ID Register"]
pub mod mb18_8b_id;
#[doc = "MB4_64B_ID (rw) register accessor: Message Buffer 4 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_id`] module"]
#[doc(alias = "MB4_64B_ID")]
pub type Mb4_64bId = crate::Reg<mb4_64b_id::Mb4_64bIdSpec>;
#[doc = "Message Buffer 4 ID Register"]
pub mod mb4_64b_id;
#[doc = "MB7_32B_WORD1 (rw) register accessor: Message Buffer 7 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_32b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_32b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_32b_word1`] module"]
#[doc(alias = "MB7_32B_WORD1")]
pub type Mb7_32bWord1 = crate::Reg<mb7_32b_word1::Mb7_32bWord1Spec>;
#[doc = "Message Buffer 7 WORD_32B Register"]
pub mod mb7_32b_word1;
#[doc = "MB12_16B_WORD0 (rw) register accessor: Message Buffer 12 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb12_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb12_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb12_16b_word0`] module"]
#[doc(alias = "MB12_16B_WORD0")]
pub type Mb12_16bWord0 = crate::Reg<mb12_16b_word0::Mb12_16bWord0Spec>;
#[doc = "Message Buffer 12 WORD_16B Register"]
pub mod mb12_16b_word0;
#[doc = "MB18_8B_WORD0 (rw) register accessor: Message Buffer 18 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb18_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb18_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb18_8b_word0`] module"]
#[doc(alias = "MB18_8B_WORD0")]
pub type Mb18_8bWord0 = crate::Reg<mb18_8b_word0::Mb18_8bWord0Spec>;
#[doc = "Message Buffer 18 WORD_8B Register"]
pub mod mb18_8b_word0;
#[doc = "MB4_64B_WORD0 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word0`] module"]
#[doc(alias = "MB4_64B_WORD0")]
pub type Mb4_64bWord0 = crate::Reg<mb4_64b_word0::Mb4_64bWord0Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word0;
#[doc = "MB7_32B_WORD2 (rw) register accessor: Message Buffer 7 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_32b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_32b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_32b_word2`] module"]
#[doc(alias = "MB7_32B_WORD2")]
pub type Mb7_32bWord2 = crate::Reg<mb7_32b_word2::Mb7_32bWord2Spec>;
#[doc = "Message Buffer 7 WORD_32B Register"]
pub mod mb7_32b_word2;
#[doc = "WORD018 (rw) register accessor: Message Buffer 18 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word018`] module"]
#[doc(alias = "WORD018")]
pub type Word018 = crate::Reg<word018::Word018Spec>;
#[doc = "Message Buffer 18 WORD0 Register"]
pub mod word018;
#[doc = "MB12_16B_WORD1 (rw) register accessor: Message Buffer 12 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb12_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb12_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb12_16b_word1`] module"]
#[doc(alias = "MB12_16B_WORD1")]
pub type Mb12_16bWord1 = crate::Reg<mb12_16b_word1::Mb12_16bWord1Spec>;
#[doc = "Message Buffer 12 WORD_16B Register"]
pub mod mb12_16b_word1;
#[doc = "MB18_8B_WORD1 (rw) register accessor: Message Buffer 18 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb18_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb18_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb18_8b_word1`] module"]
#[doc(alias = "MB18_8B_WORD1")]
pub type Mb18_8bWord1 = crate::Reg<mb18_8b_word1::Mb18_8bWord1Spec>;
#[doc = "Message Buffer 18 WORD_8B Register"]
pub mod mb18_8b_word1;
#[doc = "MB4_64B_WORD1 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word1`] module"]
#[doc(alias = "MB4_64B_WORD1")]
pub type Mb4_64bWord1 = crate::Reg<mb4_64b_word1::Mb4_64bWord1Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word1;
#[doc = "MB7_32B_WORD3 (rw) register accessor: Message Buffer 7 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_32b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_32b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_32b_word3`] module"]
#[doc(alias = "MB7_32B_WORD3")]
pub type Mb7_32bWord3 = crate::Reg<mb7_32b_word3::Mb7_32bWord3Spec>;
#[doc = "Message Buffer 7 WORD_32B Register"]
pub mod mb7_32b_word3;
#[doc = "WORD118 (rw) register accessor: Message Buffer 18 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word118`] module"]
#[doc(alias = "WORD118")]
pub type Word118 = crate::Reg<word118::Word118Spec>;
#[doc = "Message Buffer 18 WORD1 Register"]
pub mod word118;
#[doc = "CS19 (rw) register accessor: Message Buffer 19 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs19`] module"]
#[doc(alias = "CS19")]
pub type Cs19 = crate::Reg<cs19::Cs19Spec>;
#[doc = "Message Buffer 19 CS Register"]
pub mod cs19;
#[doc = "MB12_16B_WORD2 (rw) register accessor: Message Buffer 12 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb12_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb12_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb12_16b_word2`] module"]
#[doc(alias = "MB12_16B_WORD2")]
pub type Mb12_16bWord2 = crate::Reg<mb12_16b_word2::Mb12_16bWord2Spec>;
#[doc = "Message Buffer 12 WORD_16B Register"]
pub mod mb12_16b_word2;
#[doc = "MB19_8B_CS (rw) register accessor: Message Buffer 19 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb19_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb19_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb19_8b_cs`] module"]
#[doc(alias = "MB19_8B_CS")]
pub type Mb19_8bCs = crate::Reg<mb19_8b_cs::Mb19_8bCsSpec>;
#[doc = "Message Buffer 19 CS Register"]
pub mod mb19_8b_cs;
#[doc = "MB4_64B_WORD2 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word2`] module"]
#[doc(alias = "MB4_64B_WORD2")]
pub type Mb4_64bWord2 = crate::Reg<mb4_64b_word2::Mb4_64bWord2Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word2;
#[doc = "MB7_32B_WORD4 (rw) register accessor: Message Buffer 7 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_32b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_32b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_32b_word4`] module"]
#[doc(alias = "MB7_32B_WORD4")]
pub type Mb7_32bWord4 = crate::Reg<mb7_32b_word4::Mb7_32bWord4Spec>;
#[doc = "Message Buffer 7 WORD_32B Register"]
pub mod mb7_32b_word4;
#[doc = "ID19 (rw) register accessor: Message Buffer 19 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id19`] module"]
#[doc(alias = "ID19")]
pub type Id19 = crate::Reg<id19::Id19Spec>;
#[doc = "Message Buffer 19 ID Register"]
pub mod id19;
#[doc = "MB12_16B_WORD3 (rw) register accessor: Message Buffer 12 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb12_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb12_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb12_16b_word3`] module"]
#[doc(alias = "MB12_16B_WORD3")]
pub type Mb12_16bWord3 = crate::Reg<mb12_16b_word3::Mb12_16bWord3Spec>;
#[doc = "Message Buffer 12 WORD_16B Register"]
pub mod mb12_16b_word3;
#[doc = "MB19_8B_ID (rw) register accessor: Message Buffer 19 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb19_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb19_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb19_8b_id`] module"]
#[doc(alias = "MB19_8B_ID")]
pub type Mb19_8bId = crate::Reg<mb19_8b_id::Mb19_8bIdSpec>;
#[doc = "Message Buffer 19 ID Register"]
pub mod mb19_8b_id;
#[doc = "MB4_64B_WORD3 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word3`] module"]
#[doc(alias = "MB4_64B_WORD3")]
pub type Mb4_64bWord3 = crate::Reg<mb4_64b_word3::Mb4_64bWord3Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word3;
#[doc = "MB7_32B_WORD5 (rw) register accessor: Message Buffer 7 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_32b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_32b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_32b_word5`] module"]
#[doc(alias = "MB7_32B_WORD5")]
pub type Mb7_32bWord5 = crate::Reg<mb7_32b_word5::Mb7_32bWord5Spec>;
#[doc = "Message Buffer 7 WORD_32B Register"]
pub mod mb7_32b_word5;
#[doc = "MB13_16B_CS (rw) register accessor: Message Buffer 13 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb13_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb13_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb13_16b_cs`] module"]
#[doc(alias = "MB13_16B_CS")]
pub type Mb13_16bCs = crate::Reg<mb13_16b_cs::Mb13_16bCsSpec>;
#[doc = "Message Buffer 13 CS Register"]
pub mod mb13_16b_cs;
#[doc = "MB19_8B_WORD0 (rw) register accessor: Message Buffer 19 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb19_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb19_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb19_8b_word0`] module"]
#[doc(alias = "MB19_8B_WORD0")]
pub type Mb19_8bWord0 = crate::Reg<mb19_8b_word0::Mb19_8bWord0Spec>;
#[doc = "Message Buffer 19 WORD_8B Register"]
pub mod mb19_8b_word0;
#[doc = "MB4_64B_WORD4 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word4`] module"]
#[doc(alias = "MB4_64B_WORD4")]
pub type Mb4_64bWord4 = crate::Reg<mb4_64b_word4::Mb4_64bWord4Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word4;
#[doc = "MB7_32B_WORD6 (rw) register accessor: Message Buffer 7 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_32b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_32b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_32b_word6`] module"]
#[doc(alias = "MB7_32B_WORD6")]
pub type Mb7_32bWord6 = crate::Reg<mb7_32b_word6::Mb7_32bWord6Spec>;
#[doc = "Message Buffer 7 WORD_32B Register"]
pub mod mb7_32b_word6;
#[doc = "WORD019 (rw) register accessor: Message Buffer 19 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word019::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word019::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word019`] module"]
#[doc(alias = "WORD019")]
pub type Word019 = crate::Reg<word019::Word019Spec>;
#[doc = "Message Buffer 19 WORD0 Register"]
pub mod word019;
#[doc = "MB13_16B_ID (rw) register accessor: Message Buffer 13 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb13_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb13_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb13_16b_id`] module"]
#[doc(alias = "MB13_16B_ID")]
pub type Mb13_16bId = crate::Reg<mb13_16b_id::Mb13_16bIdSpec>;
#[doc = "Message Buffer 13 ID Register"]
pub mod mb13_16b_id;
#[doc = "MB19_8B_WORD1 (rw) register accessor: Message Buffer 19 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb19_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb19_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb19_8b_word1`] module"]
#[doc(alias = "MB19_8B_WORD1")]
pub type Mb19_8bWord1 = crate::Reg<mb19_8b_word1::Mb19_8bWord1Spec>;
#[doc = "Message Buffer 19 WORD_8B Register"]
pub mod mb19_8b_word1;
#[doc = "MB4_64B_WORD5 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word5`] module"]
#[doc(alias = "MB4_64B_WORD5")]
pub type Mb4_64bWord5 = crate::Reg<mb4_64b_word5::Mb4_64bWord5Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word5;
#[doc = "MB7_32B_WORD7 (rw) register accessor: Message Buffer 7 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb7_32b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb7_32b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb7_32b_word7`] module"]
#[doc(alias = "MB7_32B_WORD7")]
pub type Mb7_32bWord7 = crate::Reg<mb7_32b_word7::Mb7_32bWord7Spec>;
#[doc = "Message Buffer 7 WORD_32B Register"]
pub mod mb7_32b_word7;
#[doc = "WORD119 (rw) register accessor: Message Buffer 19 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word119::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word119::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word119`] module"]
#[doc(alias = "WORD119")]
pub type Word119 = crate::Reg<word119::Word119Spec>;
#[doc = "Message Buffer 19 WORD1 Register"]
pub mod word119;
#[doc = "CS20 (rw) register accessor: Message Buffer 20 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs20`] module"]
#[doc(alias = "CS20")]
pub type Cs20 = crate::Reg<cs20::Cs20Spec>;
#[doc = "Message Buffer 20 CS Register"]
pub mod cs20;
#[doc = "MB13_16B_WORD0 (rw) register accessor: Message Buffer 13 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb13_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb13_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb13_16b_word0`] module"]
#[doc(alias = "MB13_16B_WORD0")]
pub type Mb13_16bWord0 = crate::Reg<mb13_16b_word0::Mb13_16bWord0Spec>;
#[doc = "Message Buffer 13 WORD_16B Register"]
pub mod mb13_16b_word0;
#[doc = "MB20_8B_CS (rw) register accessor: Message Buffer 20 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb20_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb20_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb20_8b_cs`] module"]
#[doc(alias = "MB20_8B_CS")]
pub type Mb20_8bCs = crate::Reg<mb20_8b_cs::Mb20_8bCsSpec>;
#[doc = "Message Buffer 20 CS Register"]
pub mod mb20_8b_cs;
#[doc = "MB4_64B_WORD6 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word6`] module"]
#[doc(alias = "MB4_64B_WORD6")]
pub type Mb4_64bWord6 = crate::Reg<mb4_64b_word6::Mb4_64bWord6Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word6;
#[doc = "MB8_32B_CS (rw) register accessor: Message Buffer 8 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_32b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_32b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_32b_cs`] module"]
#[doc(alias = "MB8_32B_CS")]
pub type Mb8_32bCs = crate::Reg<mb8_32b_cs::Mb8_32bCsSpec>;
#[doc = "Message Buffer 8 CS Register"]
pub mod mb8_32b_cs;
#[doc = "ID20 (rw) register accessor: Message Buffer 20 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id20`] module"]
#[doc(alias = "ID20")]
pub type Id20 = crate::Reg<id20::Id20Spec>;
#[doc = "Message Buffer 20 ID Register"]
pub mod id20;
#[doc = "MB13_16B_WORD1 (rw) register accessor: Message Buffer 13 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb13_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb13_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb13_16b_word1`] module"]
#[doc(alias = "MB13_16B_WORD1")]
pub type Mb13_16bWord1 = crate::Reg<mb13_16b_word1::Mb13_16bWord1Spec>;
#[doc = "Message Buffer 13 WORD_16B Register"]
pub mod mb13_16b_word1;
#[doc = "MB20_8B_ID (rw) register accessor: Message Buffer 20 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb20_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb20_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb20_8b_id`] module"]
#[doc(alias = "MB20_8B_ID")]
pub type Mb20_8bId = crate::Reg<mb20_8b_id::Mb20_8bIdSpec>;
#[doc = "Message Buffer 20 ID Register"]
pub mod mb20_8b_id;
#[doc = "MB4_64B_WORD7 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word7`] module"]
#[doc(alias = "MB4_64B_WORD7")]
pub type Mb4_64bWord7 = crate::Reg<mb4_64b_word7::Mb4_64bWord7Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word7;
#[doc = "MB8_32B_ID (rw) register accessor: Message Buffer 8 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_32b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_32b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_32b_id`] module"]
#[doc(alias = "MB8_32B_ID")]
pub type Mb8_32bId = crate::Reg<mb8_32b_id::Mb8_32bIdSpec>;
#[doc = "Message Buffer 8 ID Register"]
pub mod mb8_32b_id;
#[doc = "MB13_16B_WORD2 (rw) register accessor: Message Buffer 13 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb13_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb13_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb13_16b_word2`] module"]
#[doc(alias = "MB13_16B_WORD2")]
pub type Mb13_16bWord2 = crate::Reg<mb13_16b_word2::Mb13_16bWord2Spec>;
#[doc = "Message Buffer 13 WORD_16B Register"]
pub mod mb13_16b_word2;
#[doc = "MB20_8B_WORD0 (rw) register accessor: Message Buffer 20 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb20_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb20_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb20_8b_word0`] module"]
#[doc(alias = "MB20_8B_WORD0")]
pub type Mb20_8bWord0 = crate::Reg<mb20_8b_word0::Mb20_8bWord0Spec>;
#[doc = "Message Buffer 20 WORD_8B Register"]
pub mod mb20_8b_word0;
#[doc = "MB4_64B_WORD8 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word8`] module"]
#[doc(alias = "MB4_64B_WORD8")]
pub type Mb4_64bWord8 = crate::Reg<mb4_64b_word8::Mb4_64bWord8Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word8;
#[doc = "MB8_32B_WORD0 (rw) register accessor: Message Buffer 8 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_32b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_32b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_32b_word0`] module"]
#[doc(alias = "MB8_32B_WORD0")]
pub type Mb8_32bWord0 = crate::Reg<mb8_32b_word0::Mb8_32bWord0Spec>;
#[doc = "Message Buffer 8 WORD_32B Register"]
pub mod mb8_32b_word0;
#[doc = "WORD020 (rw) register accessor: Message Buffer 20 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word020`] module"]
#[doc(alias = "WORD020")]
pub type Word020 = crate::Reg<word020::Word020Spec>;
#[doc = "Message Buffer 20 WORD0 Register"]
pub mod word020;
#[doc = "MB13_16B_WORD3 (rw) register accessor: Message Buffer 13 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb13_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb13_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb13_16b_word3`] module"]
#[doc(alias = "MB13_16B_WORD3")]
pub type Mb13_16bWord3 = crate::Reg<mb13_16b_word3::Mb13_16bWord3Spec>;
#[doc = "Message Buffer 13 WORD_16B Register"]
pub mod mb13_16b_word3;
#[doc = "MB20_8B_WORD1 (rw) register accessor: Message Buffer 20 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb20_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb20_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb20_8b_word1`] module"]
#[doc(alias = "MB20_8B_WORD1")]
pub type Mb20_8bWord1 = crate::Reg<mb20_8b_word1::Mb20_8bWord1Spec>;
#[doc = "Message Buffer 20 WORD_8B Register"]
pub mod mb20_8b_word1;
#[doc = "MB4_64B_WORD9 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word9`] module"]
#[doc(alias = "MB4_64B_WORD9")]
pub type Mb4_64bWord9 = crate::Reg<mb4_64b_word9::Mb4_64bWord9Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word9;
#[doc = "MB8_32B_WORD1 (rw) register accessor: Message Buffer 8 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_32b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_32b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_32b_word1`] module"]
#[doc(alias = "MB8_32B_WORD1")]
pub type Mb8_32bWord1 = crate::Reg<mb8_32b_word1::Mb8_32bWord1Spec>;
#[doc = "Message Buffer 8 WORD_32B Register"]
pub mod mb8_32b_word1;
#[doc = "WORD120 (rw) register accessor: Message Buffer 20 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word120`] module"]
#[doc(alias = "WORD120")]
pub type Word120 = crate::Reg<word120::Word120Spec>;
#[doc = "Message Buffer 20 WORD1 Register"]
pub mod word120;
#[doc = "CS21 (rw) register accessor: Message Buffer 21 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs21`] module"]
#[doc(alias = "CS21")]
pub type Cs21 = crate::Reg<cs21::Cs21Spec>;
#[doc = "Message Buffer 21 CS Register"]
pub mod cs21;
#[doc = "MB14_16B_CS (rw) register accessor: Message Buffer 14 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb14_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb14_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb14_16b_cs`] module"]
#[doc(alias = "MB14_16B_CS")]
pub type Mb14_16bCs = crate::Reg<mb14_16b_cs::Mb14_16bCsSpec>;
#[doc = "Message Buffer 14 CS Register"]
pub mod mb14_16b_cs;
#[doc = "MB21_8B_CS (rw) register accessor: Message Buffer 21 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb21_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb21_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb21_8b_cs`] module"]
#[doc(alias = "MB21_8B_CS")]
pub type Mb21_8bCs = crate::Reg<mb21_8b_cs::Mb21_8bCsSpec>;
#[doc = "Message Buffer 21 CS Register"]
pub mod mb21_8b_cs;
#[doc = "MB4_64B_WORD10 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word10`] module"]
#[doc(alias = "MB4_64B_WORD10")]
pub type Mb4_64bWord10 = crate::Reg<mb4_64b_word10::Mb4_64bWord10Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word10;
#[doc = "MB8_32B_WORD2 (rw) register accessor: Message Buffer 8 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_32b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_32b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_32b_word2`] module"]
#[doc(alias = "MB8_32B_WORD2")]
pub type Mb8_32bWord2 = crate::Reg<mb8_32b_word2::Mb8_32bWord2Spec>;
#[doc = "Message Buffer 8 WORD_32B Register"]
pub mod mb8_32b_word2;
#[doc = "ID21 (rw) register accessor: Message Buffer 21 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id21`] module"]
#[doc(alias = "ID21")]
pub type Id21 = crate::Reg<id21::Id21Spec>;
#[doc = "Message Buffer 21 ID Register"]
pub mod id21;
#[doc = "MB14_16B_ID (rw) register accessor: Message Buffer 14 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb14_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb14_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb14_16b_id`] module"]
#[doc(alias = "MB14_16B_ID")]
pub type Mb14_16bId = crate::Reg<mb14_16b_id::Mb14_16bIdSpec>;
#[doc = "Message Buffer 14 ID Register"]
pub mod mb14_16b_id;
#[doc = "MB21_8B_ID (rw) register accessor: Message Buffer 21 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb21_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb21_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb21_8b_id`] module"]
#[doc(alias = "MB21_8B_ID")]
pub type Mb21_8bId = crate::Reg<mb21_8b_id::Mb21_8bIdSpec>;
#[doc = "Message Buffer 21 ID Register"]
pub mod mb21_8b_id;
#[doc = "MB4_64B_WORD11 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word11`] module"]
#[doc(alias = "MB4_64B_WORD11")]
pub type Mb4_64bWord11 = crate::Reg<mb4_64b_word11::Mb4_64bWord11Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word11;
#[doc = "MB8_32B_WORD3 (rw) register accessor: Message Buffer 8 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_32b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_32b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_32b_word3`] module"]
#[doc(alias = "MB8_32B_WORD3")]
pub type Mb8_32bWord3 = crate::Reg<mb8_32b_word3::Mb8_32bWord3Spec>;
#[doc = "Message Buffer 8 WORD_32B Register"]
pub mod mb8_32b_word3;
#[doc = "MB14_16B_WORD0 (rw) register accessor: Message Buffer 14 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb14_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb14_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb14_16b_word0`] module"]
#[doc(alias = "MB14_16B_WORD0")]
pub type Mb14_16bWord0 = crate::Reg<mb14_16b_word0::Mb14_16bWord0Spec>;
#[doc = "Message Buffer 14 WORD_16B Register"]
pub mod mb14_16b_word0;
#[doc = "MB21_8B_WORD0 (rw) register accessor: Message Buffer 21 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb21_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb21_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb21_8b_word0`] module"]
#[doc(alias = "MB21_8B_WORD0")]
pub type Mb21_8bWord0 = crate::Reg<mb21_8b_word0::Mb21_8bWord0Spec>;
#[doc = "Message Buffer 21 WORD_8B Register"]
pub mod mb21_8b_word0;
#[doc = "MB4_64B_WORD12 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word12`] module"]
#[doc(alias = "MB4_64B_WORD12")]
pub type Mb4_64bWord12 = crate::Reg<mb4_64b_word12::Mb4_64bWord12Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word12;
#[doc = "MB8_32B_WORD4 (rw) register accessor: Message Buffer 8 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_32b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_32b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_32b_word4`] module"]
#[doc(alias = "MB8_32B_WORD4")]
pub type Mb8_32bWord4 = crate::Reg<mb8_32b_word4::Mb8_32bWord4Spec>;
#[doc = "Message Buffer 8 WORD_32B Register"]
pub mod mb8_32b_word4;
#[doc = "WORD021 (rw) register accessor: Message Buffer 21 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word021::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word021::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word021`] module"]
#[doc(alias = "WORD021")]
pub type Word021 = crate::Reg<word021::Word021Spec>;
#[doc = "Message Buffer 21 WORD0 Register"]
pub mod word021;
#[doc = "MB14_16B_WORD1 (rw) register accessor: Message Buffer 14 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb14_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb14_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb14_16b_word1`] module"]
#[doc(alias = "MB14_16B_WORD1")]
pub type Mb14_16bWord1 = crate::Reg<mb14_16b_word1::Mb14_16bWord1Spec>;
#[doc = "Message Buffer 14 WORD_16B Register"]
pub mod mb14_16b_word1;
#[doc = "MB21_8B_WORD1 (rw) register accessor: Message Buffer 21 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb21_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb21_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb21_8b_word1`] module"]
#[doc(alias = "MB21_8B_WORD1")]
pub type Mb21_8bWord1 = crate::Reg<mb21_8b_word1::Mb21_8bWord1Spec>;
#[doc = "Message Buffer 21 WORD_8B Register"]
pub mod mb21_8b_word1;
#[doc = "MB4_64B_WORD13 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word13`] module"]
#[doc(alias = "MB4_64B_WORD13")]
pub type Mb4_64bWord13 = crate::Reg<mb4_64b_word13::Mb4_64bWord13Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word13;
#[doc = "MB8_32B_WORD5 (rw) register accessor: Message Buffer 8 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_32b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_32b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_32b_word5`] module"]
#[doc(alias = "MB8_32B_WORD5")]
pub type Mb8_32bWord5 = crate::Reg<mb8_32b_word5::Mb8_32bWord5Spec>;
#[doc = "Message Buffer 8 WORD_32B Register"]
pub mod mb8_32b_word5;
#[doc = "WORD121 (rw) register accessor: Message Buffer 21 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word121::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word121::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word121`] module"]
#[doc(alias = "WORD121")]
pub type Word121 = crate::Reg<word121::Word121Spec>;
#[doc = "Message Buffer 21 WORD1 Register"]
pub mod word121;
#[doc = "CS22 (rw) register accessor: Message Buffer 22 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs22`] module"]
#[doc(alias = "CS22")]
pub type Cs22 = crate::Reg<cs22::Cs22Spec>;
#[doc = "Message Buffer 22 CS Register"]
pub mod cs22;
#[doc = "MB14_16B_WORD2 (rw) register accessor: Message Buffer 14 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb14_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb14_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb14_16b_word2`] module"]
#[doc(alias = "MB14_16B_WORD2")]
pub type Mb14_16bWord2 = crate::Reg<mb14_16b_word2::Mb14_16bWord2Spec>;
#[doc = "Message Buffer 14 WORD_16B Register"]
pub mod mb14_16b_word2;
#[doc = "MB22_8B_CS (rw) register accessor: Message Buffer 22 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb22_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb22_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb22_8b_cs`] module"]
#[doc(alias = "MB22_8B_CS")]
pub type Mb22_8bCs = crate::Reg<mb22_8b_cs::Mb22_8bCsSpec>;
#[doc = "Message Buffer 22 CS Register"]
pub mod mb22_8b_cs;
#[doc = "MB4_64B_WORD14 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word14`] module"]
#[doc(alias = "MB4_64B_WORD14")]
pub type Mb4_64bWord14 = crate::Reg<mb4_64b_word14::Mb4_64bWord14Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word14;
#[doc = "MB8_32B_WORD6 (rw) register accessor: Message Buffer 8 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_32b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_32b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_32b_word6`] module"]
#[doc(alias = "MB8_32B_WORD6")]
pub type Mb8_32bWord6 = crate::Reg<mb8_32b_word6::Mb8_32bWord6Spec>;
#[doc = "Message Buffer 8 WORD_32B Register"]
pub mod mb8_32b_word6;
#[doc = "ID22 (rw) register accessor: Message Buffer 22 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id22`] module"]
#[doc(alias = "ID22")]
pub type Id22 = crate::Reg<id22::Id22Spec>;
#[doc = "Message Buffer 22 ID Register"]
pub mod id22;
#[doc = "MB14_16B_WORD3 (rw) register accessor: Message Buffer 14 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb14_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb14_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb14_16b_word3`] module"]
#[doc(alias = "MB14_16B_WORD3")]
pub type Mb14_16bWord3 = crate::Reg<mb14_16b_word3::Mb14_16bWord3Spec>;
#[doc = "Message Buffer 14 WORD_16B Register"]
pub mod mb14_16b_word3;
#[doc = "MB22_8B_ID (rw) register accessor: Message Buffer 22 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb22_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb22_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb22_8b_id`] module"]
#[doc(alias = "MB22_8B_ID")]
pub type Mb22_8bId = crate::Reg<mb22_8b_id::Mb22_8bIdSpec>;
#[doc = "Message Buffer 22 ID Register"]
pub mod mb22_8b_id;
#[doc = "MB4_64B_WORD15 (rw) register accessor: Message Buffer 4 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb4_64b_word15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb4_64b_word15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb4_64b_word15`] module"]
#[doc(alias = "MB4_64B_WORD15")]
pub type Mb4_64bWord15 = crate::Reg<mb4_64b_word15::Mb4_64bWord15Spec>;
#[doc = "Message Buffer 4 WORD_64B Register"]
pub mod mb4_64b_word15;
#[doc = "MB8_32B_WORD7 (rw) register accessor: Message Buffer 8 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb8_32b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb8_32b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb8_32b_word7`] module"]
#[doc(alias = "MB8_32B_WORD7")]
pub type Mb8_32bWord7 = crate::Reg<mb8_32b_word7::Mb8_32bWord7Spec>;
#[doc = "Message Buffer 8 WORD_32B Register"]
pub mod mb8_32b_word7;
#[doc = "MB15_16B_CS (rw) register accessor: Message Buffer 15 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb15_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb15_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb15_16b_cs`] module"]
#[doc(alias = "MB15_16B_CS")]
pub type Mb15_16bCs = crate::Reg<mb15_16b_cs::Mb15_16bCsSpec>;
#[doc = "Message Buffer 15 CS Register"]
pub mod mb15_16b_cs;
#[doc = "MB22_8B_WORD0 (rw) register accessor: Message Buffer 22 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb22_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb22_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb22_8b_word0`] module"]
#[doc(alias = "MB22_8B_WORD0")]
pub type Mb22_8bWord0 = crate::Reg<mb22_8b_word0::Mb22_8bWord0Spec>;
#[doc = "Message Buffer 22 WORD_8B Register"]
pub mod mb22_8b_word0;
#[doc = "MB5_64B_CS (rw) register accessor: Message Buffer 5 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_cs`] module"]
#[doc(alias = "MB5_64B_CS")]
pub type Mb5_64bCs = crate::Reg<mb5_64b_cs::Mb5_64bCsSpec>;
#[doc = "Message Buffer 5 CS Register"]
pub mod mb5_64b_cs;
#[doc = "MB9_32B_CS (rw) register accessor: Message Buffer 9 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_32b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_32b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_32b_cs`] module"]
#[doc(alias = "MB9_32B_CS")]
pub type Mb9_32bCs = crate::Reg<mb9_32b_cs::Mb9_32bCsSpec>;
#[doc = "Message Buffer 9 CS Register"]
pub mod mb9_32b_cs;
#[doc = "WORD022 (rw) register accessor: Message Buffer 22 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word022::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word022::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word022`] module"]
#[doc(alias = "WORD022")]
pub type Word022 = crate::Reg<word022::Word022Spec>;
#[doc = "Message Buffer 22 WORD0 Register"]
pub mod word022;
#[doc = "MB15_16B_ID (rw) register accessor: Message Buffer 15 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb15_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb15_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb15_16b_id`] module"]
#[doc(alias = "MB15_16B_ID")]
pub type Mb15_16bId = crate::Reg<mb15_16b_id::Mb15_16bIdSpec>;
#[doc = "Message Buffer 15 ID Register"]
pub mod mb15_16b_id;
#[doc = "MB22_8B_WORD1 (rw) register accessor: Message Buffer 22 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb22_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb22_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb22_8b_word1`] module"]
#[doc(alias = "MB22_8B_WORD1")]
pub type Mb22_8bWord1 = crate::Reg<mb22_8b_word1::Mb22_8bWord1Spec>;
#[doc = "Message Buffer 22 WORD_8B Register"]
pub mod mb22_8b_word1;
#[doc = "MB5_64B_ID (rw) register accessor: Message Buffer 5 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_id`] module"]
#[doc(alias = "MB5_64B_ID")]
pub type Mb5_64bId = crate::Reg<mb5_64b_id::Mb5_64bIdSpec>;
#[doc = "Message Buffer 5 ID Register"]
pub mod mb5_64b_id;
#[doc = "MB9_32B_ID (rw) register accessor: Message Buffer 9 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_32b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_32b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_32b_id`] module"]
#[doc(alias = "MB9_32B_ID")]
pub type Mb9_32bId = crate::Reg<mb9_32b_id::Mb9_32bIdSpec>;
#[doc = "Message Buffer 9 ID Register"]
pub mod mb9_32b_id;
#[doc = "WORD122 (rw) register accessor: Message Buffer 22 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word122::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word122::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word122`] module"]
#[doc(alias = "WORD122")]
pub type Word122 = crate::Reg<word122::Word122Spec>;
#[doc = "Message Buffer 22 WORD1 Register"]
pub mod word122;
#[doc = "CS23 (rw) register accessor: Message Buffer 23 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs23`] module"]
#[doc(alias = "CS23")]
pub type Cs23 = crate::Reg<cs23::Cs23Spec>;
#[doc = "Message Buffer 23 CS Register"]
pub mod cs23;
#[doc = "MB15_16B_WORD0 (rw) register accessor: Message Buffer 15 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb15_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb15_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb15_16b_word0`] module"]
#[doc(alias = "MB15_16B_WORD0")]
pub type Mb15_16bWord0 = crate::Reg<mb15_16b_word0::Mb15_16bWord0Spec>;
#[doc = "Message Buffer 15 WORD_16B Register"]
pub mod mb15_16b_word0;
#[doc = "MB23_8B_CS (rw) register accessor: Message Buffer 23 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb23_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb23_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb23_8b_cs`] module"]
#[doc(alias = "MB23_8B_CS")]
pub type Mb23_8bCs = crate::Reg<mb23_8b_cs::Mb23_8bCsSpec>;
#[doc = "Message Buffer 23 CS Register"]
pub mod mb23_8b_cs;
#[doc = "MB5_64B_WORD0 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word0`] module"]
#[doc(alias = "MB5_64B_WORD0")]
pub type Mb5_64bWord0 = crate::Reg<mb5_64b_word0::Mb5_64bWord0Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word0;
#[doc = "MB9_32B_WORD0 (rw) register accessor: Message Buffer 9 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_32b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_32b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_32b_word0`] module"]
#[doc(alias = "MB9_32B_WORD0")]
pub type Mb9_32bWord0 = crate::Reg<mb9_32b_word0::Mb9_32bWord0Spec>;
#[doc = "Message Buffer 9 WORD_32B Register"]
pub mod mb9_32b_word0;
#[doc = "ID23 (rw) register accessor: Message Buffer 23 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id23`] module"]
#[doc(alias = "ID23")]
pub type Id23 = crate::Reg<id23::Id23Spec>;
#[doc = "Message Buffer 23 ID Register"]
pub mod id23;
#[doc = "MB15_16B_WORD1 (rw) register accessor: Message Buffer 15 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb15_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb15_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb15_16b_word1`] module"]
#[doc(alias = "MB15_16B_WORD1")]
pub type Mb15_16bWord1 = crate::Reg<mb15_16b_word1::Mb15_16bWord1Spec>;
#[doc = "Message Buffer 15 WORD_16B Register"]
pub mod mb15_16b_word1;
#[doc = "MB23_8B_ID (rw) register accessor: Message Buffer 23 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb23_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb23_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb23_8b_id`] module"]
#[doc(alias = "MB23_8B_ID")]
pub type Mb23_8bId = crate::Reg<mb23_8b_id::Mb23_8bIdSpec>;
#[doc = "Message Buffer 23 ID Register"]
pub mod mb23_8b_id;
#[doc = "MB5_64B_WORD1 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word1`] module"]
#[doc(alias = "MB5_64B_WORD1")]
pub type Mb5_64bWord1 = crate::Reg<mb5_64b_word1::Mb5_64bWord1Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word1;
#[doc = "MB9_32B_WORD1 (rw) register accessor: Message Buffer 9 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_32b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_32b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_32b_word1`] module"]
#[doc(alias = "MB9_32B_WORD1")]
pub type Mb9_32bWord1 = crate::Reg<mb9_32b_word1::Mb9_32bWord1Spec>;
#[doc = "Message Buffer 9 WORD_32B Register"]
pub mod mb9_32b_word1;
#[doc = "MB15_16B_WORD2 (rw) register accessor: Message Buffer 15 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb15_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb15_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb15_16b_word2`] module"]
#[doc(alias = "MB15_16B_WORD2")]
pub type Mb15_16bWord2 = crate::Reg<mb15_16b_word2::Mb15_16bWord2Spec>;
#[doc = "Message Buffer 15 WORD_16B Register"]
pub mod mb15_16b_word2;
#[doc = "MB23_8B_WORD0 (rw) register accessor: Message Buffer 23 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb23_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb23_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb23_8b_word0`] module"]
#[doc(alias = "MB23_8B_WORD0")]
pub type Mb23_8bWord0 = crate::Reg<mb23_8b_word0::Mb23_8bWord0Spec>;
#[doc = "Message Buffer 23 WORD_8B Register"]
pub mod mb23_8b_word0;
#[doc = "MB5_64B_WORD2 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word2`] module"]
#[doc(alias = "MB5_64B_WORD2")]
pub type Mb5_64bWord2 = crate::Reg<mb5_64b_word2::Mb5_64bWord2Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word2;
#[doc = "MB9_32B_WORD2 (rw) register accessor: Message Buffer 9 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_32b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_32b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_32b_word2`] module"]
#[doc(alias = "MB9_32B_WORD2")]
pub type Mb9_32bWord2 = crate::Reg<mb9_32b_word2::Mb9_32bWord2Spec>;
#[doc = "Message Buffer 9 WORD_32B Register"]
pub mod mb9_32b_word2;
#[doc = "WORD023 (rw) register accessor: Message Buffer 23 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word023::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word023::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word023`] module"]
#[doc(alias = "WORD023")]
pub type Word023 = crate::Reg<word023::Word023Spec>;
#[doc = "Message Buffer 23 WORD0 Register"]
pub mod word023;
#[doc = "MB15_16B_WORD3 (rw) register accessor: Message Buffer 15 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb15_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb15_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb15_16b_word3`] module"]
#[doc(alias = "MB15_16B_WORD3")]
pub type Mb15_16bWord3 = crate::Reg<mb15_16b_word3::Mb15_16bWord3Spec>;
#[doc = "Message Buffer 15 WORD_16B Register"]
pub mod mb15_16b_word3;
#[doc = "MB23_8B_WORD1 (rw) register accessor: Message Buffer 23 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb23_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb23_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb23_8b_word1`] module"]
#[doc(alias = "MB23_8B_WORD1")]
pub type Mb23_8bWord1 = crate::Reg<mb23_8b_word1::Mb23_8bWord1Spec>;
#[doc = "Message Buffer 23 WORD_8B Register"]
pub mod mb23_8b_word1;
#[doc = "MB5_64B_WORD3 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word3`] module"]
#[doc(alias = "MB5_64B_WORD3")]
pub type Mb5_64bWord3 = crate::Reg<mb5_64b_word3::Mb5_64bWord3Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word3;
#[doc = "MB9_32B_WORD3 (rw) register accessor: Message Buffer 9 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_32b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_32b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_32b_word3`] module"]
#[doc(alias = "MB9_32B_WORD3")]
pub type Mb9_32bWord3 = crate::Reg<mb9_32b_word3::Mb9_32bWord3Spec>;
#[doc = "Message Buffer 9 WORD_32B Register"]
pub mod mb9_32b_word3;
#[doc = "WORD123 (rw) register accessor: Message Buffer 23 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word123::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word123::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word123`] module"]
#[doc(alias = "WORD123")]
pub type Word123 = crate::Reg<word123::Word123Spec>;
#[doc = "Message Buffer 23 WORD1 Register"]
pub mod word123;
#[doc = "CS24 (rw) register accessor: Message Buffer 24 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs24`] module"]
#[doc(alias = "CS24")]
pub type Cs24 = crate::Reg<cs24::Cs24Spec>;
#[doc = "Message Buffer 24 CS Register"]
pub mod cs24;
#[doc = "MB16_16B_CS (rw) register accessor: Message Buffer 16 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb16_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb16_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb16_16b_cs`] module"]
#[doc(alias = "MB16_16B_CS")]
pub type Mb16_16bCs = crate::Reg<mb16_16b_cs::Mb16_16bCsSpec>;
#[doc = "Message Buffer 16 CS Register"]
pub mod mb16_16b_cs;
#[doc = "MB24_8B_CS (rw) register accessor: Message Buffer 24 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb24_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb24_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb24_8b_cs`] module"]
#[doc(alias = "MB24_8B_CS")]
pub type Mb24_8bCs = crate::Reg<mb24_8b_cs::Mb24_8bCsSpec>;
#[doc = "Message Buffer 24 CS Register"]
pub mod mb24_8b_cs;
#[doc = "MB5_64B_WORD4 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word4`] module"]
#[doc(alias = "MB5_64B_WORD4")]
pub type Mb5_64bWord4 = crate::Reg<mb5_64b_word4::Mb5_64bWord4Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word4;
#[doc = "MB9_32B_WORD4 (rw) register accessor: Message Buffer 9 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_32b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_32b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_32b_word4`] module"]
#[doc(alias = "MB9_32B_WORD4")]
pub type Mb9_32bWord4 = crate::Reg<mb9_32b_word4::Mb9_32bWord4Spec>;
#[doc = "Message Buffer 9 WORD_32B Register"]
pub mod mb9_32b_word4;
#[doc = "ID24 (rw) register accessor: Message Buffer 24 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id24`] module"]
#[doc(alias = "ID24")]
pub type Id24 = crate::Reg<id24::Id24Spec>;
#[doc = "Message Buffer 24 ID Register"]
pub mod id24;
#[doc = "MB16_16B_ID (rw) register accessor: Message Buffer 16 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb16_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb16_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb16_16b_id`] module"]
#[doc(alias = "MB16_16B_ID")]
pub type Mb16_16bId = crate::Reg<mb16_16b_id::Mb16_16bIdSpec>;
#[doc = "Message Buffer 16 ID Register"]
pub mod mb16_16b_id;
#[doc = "MB24_8B_ID (rw) register accessor: Message Buffer 24 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb24_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb24_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb24_8b_id`] module"]
#[doc(alias = "MB24_8B_ID")]
pub type Mb24_8bId = crate::Reg<mb24_8b_id::Mb24_8bIdSpec>;
#[doc = "Message Buffer 24 ID Register"]
pub mod mb24_8b_id;
#[doc = "MB5_64B_WORD5 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word5`] module"]
#[doc(alias = "MB5_64B_WORD5")]
pub type Mb5_64bWord5 = crate::Reg<mb5_64b_word5::Mb5_64bWord5Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word5;
#[doc = "MB9_32B_WORD5 (rw) register accessor: Message Buffer 9 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_32b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_32b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_32b_word5`] module"]
#[doc(alias = "MB9_32B_WORD5")]
pub type Mb9_32bWord5 = crate::Reg<mb9_32b_word5::Mb9_32bWord5Spec>;
#[doc = "Message Buffer 9 WORD_32B Register"]
pub mod mb9_32b_word5;
#[doc = "MB16_16B_WORD0 (rw) register accessor: Message Buffer 16 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb16_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb16_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb16_16b_word0`] module"]
#[doc(alias = "MB16_16B_WORD0")]
pub type Mb16_16bWord0 = crate::Reg<mb16_16b_word0::Mb16_16bWord0Spec>;
#[doc = "Message Buffer 16 WORD_16B Register"]
pub mod mb16_16b_word0;
#[doc = "MB24_8B_WORD0 (rw) register accessor: Message Buffer 24 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb24_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb24_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb24_8b_word0`] module"]
#[doc(alias = "MB24_8B_WORD0")]
pub type Mb24_8bWord0 = crate::Reg<mb24_8b_word0::Mb24_8bWord0Spec>;
#[doc = "Message Buffer 24 WORD_8B Register"]
pub mod mb24_8b_word0;
#[doc = "MB5_64B_WORD6 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word6`] module"]
#[doc(alias = "MB5_64B_WORD6")]
pub type Mb5_64bWord6 = crate::Reg<mb5_64b_word6::Mb5_64bWord6Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word6;
#[doc = "MB9_32B_WORD6 (rw) register accessor: Message Buffer 9 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_32b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_32b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_32b_word6`] module"]
#[doc(alias = "MB9_32B_WORD6")]
pub type Mb9_32bWord6 = crate::Reg<mb9_32b_word6::Mb9_32bWord6Spec>;
#[doc = "Message Buffer 9 WORD_32B Register"]
pub mod mb9_32b_word6;
#[doc = "WORD024 (rw) register accessor: Message Buffer 24 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word024`] module"]
#[doc(alias = "WORD024")]
pub type Word024 = crate::Reg<word024::Word024Spec>;
#[doc = "Message Buffer 24 WORD0 Register"]
pub mod word024;
#[doc = "MB16_16B_WORD1 (rw) register accessor: Message Buffer 16 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb16_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb16_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb16_16b_word1`] module"]
#[doc(alias = "MB16_16B_WORD1")]
pub type Mb16_16bWord1 = crate::Reg<mb16_16b_word1::Mb16_16bWord1Spec>;
#[doc = "Message Buffer 16 WORD_16B Register"]
pub mod mb16_16b_word1;
#[doc = "MB24_8B_WORD1 (rw) register accessor: Message Buffer 24 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb24_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb24_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb24_8b_word1`] module"]
#[doc(alias = "MB24_8B_WORD1")]
pub type Mb24_8bWord1 = crate::Reg<mb24_8b_word1::Mb24_8bWord1Spec>;
#[doc = "Message Buffer 24 WORD_8B Register"]
pub mod mb24_8b_word1;
#[doc = "MB5_64B_WORD7 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word7`] module"]
#[doc(alias = "MB5_64B_WORD7")]
pub type Mb5_64bWord7 = crate::Reg<mb5_64b_word7::Mb5_64bWord7Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word7;
#[doc = "MB9_32B_WORD7 (rw) register accessor: Message Buffer 9 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb9_32b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb9_32b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb9_32b_word7`] module"]
#[doc(alias = "MB9_32B_WORD7")]
pub type Mb9_32bWord7 = crate::Reg<mb9_32b_word7::Mb9_32bWord7Spec>;
#[doc = "Message Buffer 9 WORD_32B Register"]
pub mod mb9_32b_word7;
#[doc = "WORD124 (rw) register accessor: Message Buffer 24 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word124`] module"]
#[doc(alias = "WORD124")]
pub type Word124 = crate::Reg<word124::Word124Spec>;
#[doc = "Message Buffer 24 WORD1 Register"]
pub mod word124;
#[doc = "CS25 (rw) register accessor: Message Buffer 25 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs25`] module"]
#[doc(alias = "CS25")]
pub type Cs25 = crate::Reg<cs25::Cs25Spec>;
#[doc = "Message Buffer 25 CS Register"]
pub mod cs25;
#[doc = "MB10_32B_CS (rw) register accessor: Message Buffer 10 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_32b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_32b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_32b_cs`] module"]
#[doc(alias = "MB10_32B_CS")]
pub type Mb10_32bCs = crate::Reg<mb10_32b_cs::Mb10_32bCsSpec>;
#[doc = "Message Buffer 10 CS Register"]
pub mod mb10_32b_cs;
#[doc = "MB16_16B_WORD2 (rw) register accessor: Message Buffer 16 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb16_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb16_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb16_16b_word2`] module"]
#[doc(alias = "MB16_16B_WORD2")]
pub type Mb16_16bWord2 = crate::Reg<mb16_16b_word2::Mb16_16bWord2Spec>;
#[doc = "Message Buffer 16 WORD_16B Register"]
pub mod mb16_16b_word2;
#[doc = "MB25_8B_CS (rw) register accessor: Message Buffer 25 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb25_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb25_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb25_8b_cs`] module"]
#[doc(alias = "MB25_8B_CS")]
pub type Mb25_8bCs = crate::Reg<mb25_8b_cs::Mb25_8bCsSpec>;
#[doc = "Message Buffer 25 CS Register"]
pub mod mb25_8b_cs;
#[doc = "MB5_64B_WORD8 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word8`] module"]
#[doc(alias = "MB5_64B_WORD8")]
pub type Mb5_64bWord8 = crate::Reg<mb5_64b_word8::Mb5_64bWord8Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word8;
#[doc = "ID25 (rw) register accessor: Message Buffer 25 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id25`] module"]
#[doc(alias = "ID25")]
pub type Id25 = crate::Reg<id25::Id25Spec>;
#[doc = "Message Buffer 25 ID Register"]
pub mod id25;
#[doc = "MB10_32B_ID (rw) register accessor: Message Buffer 10 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_32b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_32b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_32b_id`] module"]
#[doc(alias = "MB10_32B_ID")]
pub type Mb10_32bId = crate::Reg<mb10_32b_id::Mb10_32bIdSpec>;
#[doc = "Message Buffer 10 ID Register"]
pub mod mb10_32b_id;
#[doc = "MB16_16B_WORD3 (rw) register accessor: Message Buffer 16 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb16_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb16_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb16_16b_word3`] module"]
#[doc(alias = "MB16_16B_WORD3")]
pub type Mb16_16bWord3 = crate::Reg<mb16_16b_word3::Mb16_16bWord3Spec>;
#[doc = "Message Buffer 16 WORD_16B Register"]
pub mod mb16_16b_word3;
#[doc = "MB25_8B_ID (rw) register accessor: Message Buffer 25 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb25_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb25_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb25_8b_id`] module"]
#[doc(alias = "MB25_8B_ID")]
pub type Mb25_8bId = crate::Reg<mb25_8b_id::Mb25_8bIdSpec>;
#[doc = "Message Buffer 25 ID Register"]
pub mod mb25_8b_id;
#[doc = "MB5_64B_WORD9 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word9`] module"]
#[doc(alias = "MB5_64B_WORD9")]
pub type Mb5_64bWord9 = crate::Reg<mb5_64b_word9::Mb5_64bWord9Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word9;
#[doc = "MB10_32B_WORD0 (rw) register accessor: Message Buffer 10 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_32b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_32b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_32b_word0`] module"]
#[doc(alias = "MB10_32B_WORD0")]
pub type Mb10_32bWord0 = crate::Reg<mb10_32b_word0::Mb10_32bWord0Spec>;
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod mb10_32b_word0;
#[doc = "MB17_16B_CS (rw) register accessor: Message Buffer 17 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb17_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb17_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb17_16b_cs`] module"]
#[doc(alias = "MB17_16B_CS")]
pub type Mb17_16bCs = crate::Reg<mb17_16b_cs::Mb17_16bCsSpec>;
#[doc = "Message Buffer 17 CS Register"]
pub mod mb17_16b_cs;
#[doc = "MB25_8B_WORD0 (rw) register accessor: Message Buffer 25 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb25_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb25_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb25_8b_word0`] module"]
#[doc(alias = "MB25_8B_WORD0")]
pub type Mb25_8bWord0 = crate::Reg<mb25_8b_word0::Mb25_8bWord0Spec>;
#[doc = "Message Buffer 25 WORD_8B Register"]
pub mod mb25_8b_word0;
#[doc = "MB5_64B_WORD10 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word10`] module"]
#[doc(alias = "MB5_64B_WORD10")]
pub type Mb5_64bWord10 = crate::Reg<mb5_64b_word10::Mb5_64bWord10Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word10;
#[doc = "WORD025 (rw) register accessor: Message Buffer 25 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word025::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word025::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word025`] module"]
#[doc(alias = "WORD025")]
pub type Word025 = crate::Reg<word025::Word025Spec>;
#[doc = "Message Buffer 25 WORD0 Register"]
pub mod word025;
#[doc = "MB10_32B_WORD1 (rw) register accessor: Message Buffer 10 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_32b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_32b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_32b_word1`] module"]
#[doc(alias = "MB10_32B_WORD1")]
pub type Mb10_32bWord1 = crate::Reg<mb10_32b_word1::Mb10_32bWord1Spec>;
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod mb10_32b_word1;
#[doc = "MB17_16B_ID (rw) register accessor: Message Buffer 17 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb17_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb17_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb17_16b_id`] module"]
#[doc(alias = "MB17_16B_ID")]
pub type Mb17_16bId = crate::Reg<mb17_16b_id::Mb17_16bIdSpec>;
#[doc = "Message Buffer 17 ID Register"]
pub mod mb17_16b_id;
#[doc = "MB25_8B_WORD1 (rw) register accessor: Message Buffer 25 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb25_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb25_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb25_8b_word1`] module"]
#[doc(alias = "MB25_8B_WORD1")]
pub type Mb25_8bWord1 = crate::Reg<mb25_8b_word1::Mb25_8bWord1Spec>;
#[doc = "Message Buffer 25 WORD_8B Register"]
pub mod mb25_8b_word1;
#[doc = "MB5_64B_WORD11 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word11`] module"]
#[doc(alias = "MB5_64B_WORD11")]
pub type Mb5_64bWord11 = crate::Reg<mb5_64b_word11::Mb5_64bWord11Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word11;
#[doc = "WORD125 (rw) register accessor: Message Buffer 25 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word125::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word125::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word125`] module"]
#[doc(alias = "WORD125")]
pub type Word125 = crate::Reg<word125::Word125Spec>;
#[doc = "Message Buffer 25 WORD1 Register"]
pub mod word125;
#[doc = "CS26 (rw) register accessor: Message Buffer 26 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs26`] module"]
#[doc(alias = "CS26")]
pub type Cs26 = crate::Reg<cs26::Cs26Spec>;
#[doc = "Message Buffer 26 CS Register"]
pub mod cs26;
#[doc = "MB10_32B_WORD2 (rw) register accessor: Message Buffer 10 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_32b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_32b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_32b_word2`] module"]
#[doc(alias = "MB10_32B_WORD2")]
pub type Mb10_32bWord2 = crate::Reg<mb10_32b_word2::Mb10_32bWord2Spec>;
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod mb10_32b_word2;
#[doc = "MB17_16B_WORD0 (rw) register accessor: Message Buffer 17 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb17_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb17_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb17_16b_word0`] module"]
#[doc(alias = "MB17_16B_WORD0")]
pub type Mb17_16bWord0 = crate::Reg<mb17_16b_word0::Mb17_16bWord0Spec>;
#[doc = "Message Buffer 17 WORD_16B Register"]
pub mod mb17_16b_word0;
#[doc = "MB26_8B_CS (rw) register accessor: Message Buffer 26 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb26_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb26_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb26_8b_cs`] module"]
#[doc(alias = "MB26_8B_CS")]
pub type Mb26_8bCs = crate::Reg<mb26_8b_cs::Mb26_8bCsSpec>;
#[doc = "Message Buffer 26 CS Register"]
pub mod mb26_8b_cs;
#[doc = "MB5_64B_WORD12 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word12`] module"]
#[doc(alias = "MB5_64B_WORD12")]
pub type Mb5_64bWord12 = crate::Reg<mb5_64b_word12::Mb5_64bWord12Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word12;
#[doc = "ID26 (rw) register accessor: Message Buffer 26 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id26`] module"]
#[doc(alias = "ID26")]
pub type Id26 = crate::Reg<id26::Id26Spec>;
#[doc = "Message Buffer 26 ID Register"]
pub mod id26;
#[doc = "MB10_32B_WORD3 (rw) register accessor: Message Buffer 10 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_32b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_32b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_32b_word3`] module"]
#[doc(alias = "MB10_32B_WORD3")]
pub type Mb10_32bWord3 = crate::Reg<mb10_32b_word3::Mb10_32bWord3Spec>;
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod mb10_32b_word3;
#[doc = "MB17_16B_WORD1 (rw) register accessor: Message Buffer 17 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb17_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb17_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb17_16b_word1`] module"]
#[doc(alias = "MB17_16B_WORD1")]
pub type Mb17_16bWord1 = crate::Reg<mb17_16b_word1::Mb17_16bWord1Spec>;
#[doc = "Message Buffer 17 WORD_16B Register"]
pub mod mb17_16b_word1;
#[doc = "MB26_8B_ID (rw) register accessor: Message Buffer 26 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb26_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb26_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb26_8b_id`] module"]
#[doc(alias = "MB26_8B_ID")]
pub type Mb26_8bId = crate::Reg<mb26_8b_id::Mb26_8bIdSpec>;
#[doc = "Message Buffer 26 ID Register"]
pub mod mb26_8b_id;
#[doc = "MB5_64B_WORD13 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word13`] module"]
#[doc(alias = "MB5_64B_WORD13")]
pub type Mb5_64bWord13 = crate::Reg<mb5_64b_word13::Mb5_64bWord13Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word13;
#[doc = "MB10_32B_WORD4 (rw) register accessor: Message Buffer 10 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_32b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_32b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_32b_word4`] module"]
#[doc(alias = "MB10_32B_WORD4")]
pub type Mb10_32bWord4 = crate::Reg<mb10_32b_word4::Mb10_32bWord4Spec>;
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod mb10_32b_word4;
#[doc = "MB17_16B_WORD2 (rw) register accessor: Message Buffer 17 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb17_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb17_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb17_16b_word2`] module"]
#[doc(alias = "MB17_16B_WORD2")]
pub type Mb17_16bWord2 = crate::Reg<mb17_16b_word2::Mb17_16bWord2Spec>;
#[doc = "Message Buffer 17 WORD_16B Register"]
pub mod mb17_16b_word2;
#[doc = "MB26_8B_WORD0 (rw) register accessor: Message Buffer 26 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb26_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb26_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb26_8b_word0`] module"]
#[doc(alias = "MB26_8B_WORD0")]
pub type Mb26_8bWord0 = crate::Reg<mb26_8b_word0::Mb26_8bWord0Spec>;
#[doc = "Message Buffer 26 WORD_8B Register"]
pub mod mb26_8b_word0;
#[doc = "MB5_64B_WORD14 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word14`] module"]
#[doc(alias = "MB5_64B_WORD14")]
pub type Mb5_64bWord14 = crate::Reg<mb5_64b_word14::Mb5_64bWord14Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word14;
#[doc = "WORD026 (rw) register accessor: Message Buffer 26 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word026::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word026::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word026`] module"]
#[doc(alias = "WORD026")]
pub type Word026 = crate::Reg<word026::Word026Spec>;
#[doc = "Message Buffer 26 WORD0 Register"]
pub mod word026;
#[doc = "MB10_32B_WORD5 (rw) register accessor: Message Buffer 10 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_32b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_32b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_32b_word5`] module"]
#[doc(alias = "MB10_32B_WORD5")]
pub type Mb10_32bWord5 = crate::Reg<mb10_32b_word5::Mb10_32bWord5Spec>;
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod mb10_32b_word5;
#[doc = "MB17_16B_WORD3 (rw) register accessor: Message Buffer 17 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb17_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb17_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb17_16b_word3`] module"]
#[doc(alias = "MB17_16B_WORD3")]
pub type Mb17_16bWord3 = crate::Reg<mb17_16b_word3::Mb17_16bWord3Spec>;
#[doc = "Message Buffer 17 WORD_16B Register"]
pub mod mb17_16b_word3;
#[doc = "MB26_8B_WORD1 (rw) register accessor: Message Buffer 26 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb26_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb26_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb26_8b_word1`] module"]
#[doc(alias = "MB26_8B_WORD1")]
pub type Mb26_8bWord1 = crate::Reg<mb26_8b_word1::Mb26_8bWord1Spec>;
#[doc = "Message Buffer 26 WORD_8B Register"]
pub mod mb26_8b_word1;
#[doc = "MB5_64B_WORD15 (rw) register accessor: Message Buffer 5 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb5_64b_word15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb5_64b_word15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb5_64b_word15`] module"]
#[doc(alias = "MB5_64B_WORD15")]
pub type Mb5_64bWord15 = crate::Reg<mb5_64b_word15::Mb5_64bWord15Spec>;
#[doc = "Message Buffer 5 WORD_64B Register"]
pub mod mb5_64b_word15;
#[doc = "WORD126 (rw) register accessor: Message Buffer 26 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word126::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word126::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word126`] module"]
#[doc(alias = "WORD126")]
pub type Word126 = crate::Reg<word126::Word126Spec>;
#[doc = "Message Buffer 26 WORD1 Register"]
pub mod word126;
#[doc = "CS27 (rw) register accessor: Message Buffer 27 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs27`] module"]
#[doc(alias = "CS27")]
pub type Cs27 = crate::Reg<cs27::Cs27Spec>;
#[doc = "Message Buffer 27 CS Register"]
pub mod cs27;
#[doc = "MB10_32B_WORD6 (rw) register accessor: Message Buffer 10 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_32b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_32b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_32b_word6`] module"]
#[doc(alias = "MB10_32B_WORD6")]
pub type Mb10_32bWord6 = crate::Reg<mb10_32b_word6::Mb10_32bWord6Spec>;
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod mb10_32b_word6;
#[doc = "MB18_16B_CS (rw) register accessor: Message Buffer 18 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb18_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb18_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb18_16b_cs`] module"]
#[doc(alias = "MB18_16B_CS")]
pub type Mb18_16bCs = crate::Reg<mb18_16b_cs::Mb18_16bCsSpec>;
#[doc = "Message Buffer 18 CS Register"]
pub mod mb18_16b_cs;
#[doc = "MB27_8B_CS (rw) register accessor: Message Buffer 27 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb27_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb27_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb27_8b_cs`] module"]
#[doc(alias = "MB27_8B_CS")]
pub type Mb27_8bCs = crate::Reg<mb27_8b_cs::Mb27_8bCsSpec>;
#[doc = "Message Buffer 27 CS Register"]
pub mod mb27_8b_cs;
#[doc = "MB6_64B_CS (rw) register accessor: Message Buffer 6 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_cs`] module"]
#[doc(alias = "MB6_64B_CS")]
pub type Mb6_64bCs = crate::Reg<mb6_64b_cs::Mb6_64bCsSpec>;
#[doc = "Message Buffer 6 CS Register"]
pub mod mb6_64b_cs;
#[doc = "ID27 (rw) register accessor: Message Buffer 27 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id27`] module"]
#[doc(alias = "ID27")]
pub type Id27 = crate::Reg<id27::Id27Spec>;
#[doc = "Message Buffer 27 ID Register"]
pub mod id27;
#[doc = "MB10_32B_WORD7 (rw) register accessor: Message Buffer 10 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb10_32b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb10_32b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb10_32b_word7`] module"]
#[doc(alias = "MB10_32B_WORD7")]
pub type Mb10_32bWord7 = crate::Reg<mb10_32b_word7::Mb10_32bWord7Spec>;
#[doc = "Message Buffer 10 WORD_32B Register"]
pub mod mb10_32b_word7;
#[doc = "MB18_16B_ID (rw) register accessor: Message Buffer 18 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb18_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb18_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb18_16b_id`] module"]
#[doc(alias = "MB18_16B_ID")]
pub type Mb18_16bId = crate::Reg<mb18_16b_id::Mb18_16bIdSpec>;
#[doc = "Message Buffer 18 ID Register"]
pub mod mb18_16b_id;
#[doc = "MB27_8B_ID (rw) register accessor: Message Buffer 27 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb27_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb27_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb27_8b_id`] module"]
#[doc(alias = "MB27_8B_ID")]
pub type Mb27_8bId = crate::Reg<mb27_8b_id::Mb27_8bIdSpec>;
#[doc = "Message Buffer 27 ID Register"]
pub mod mb27_8b_id;
#[doc = "MB6_64B_ID (rw) register accessor: Message Buffer 6 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_id`] module"]
#[doc(alias = "MB6_64B_ID")]
pub type Mb6_64bId = crate::Reg<mb6_64b_id::Mb6_64bIdSpec>;
#[doc = "Message Buffer 6 ID Register"]
pub mod mb6_64b_id;
#[doc = "MB11_32B_CS (rw) register accessor: Message Buffer 11 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_32b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_32b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_32b_cs`] module"]
#[doc(alias = "MB11_32B_CS")]
pub type Mb11_32bCs = crate::Reg<mb11_32b_cs::Mb11_32bCsSpec>;
#[doc = "Message Buffer 11 CS Register"]
pub mod mb11_32b_cs;
#[doc = "MB18_16B_WORD0 (rw) register accessor: Message Buffer 18 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb18_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb18_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb18_16b_word0`] module"]
#[doc(alias = "MB18_16B_WORD0")]
pub type Mb18_16bWord0 = crate::Reg<mb18_16b_word0::Mb18_16bWord0Spec>;
#[doc = "Message Buffer 18 WORD_16B Register"]
pub mod mb18_16b_word0;
#[doc = "MB27_8B_WORD0 (rw) register accessor: Message Buffer 27 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb27_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb27_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb27_8b_word0`] module"]
#[doc(alias = "MB27_8B_WORD0")]
pub type Mb27_8bWord0 = crate::Reg<mb27_8b_word0::Mb27_8bWord0Spec>;
#[doc = "Message Buffer 27 WORD_8B Register"]
pub mod mb27_8b_word0;
#[doc = "MB6_64B_WORD0 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word0`] module"]
#[doc(alias = "MB6_64B_WORD0")]
pub type Mb6_64bWord0 = crate::Reg<mb6_64b_word0::Mb6_64bWord0Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word0;
#[doc = "WORD027 (rw) register accessor: Message Buffer 27 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word027::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word027::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word027`] module"]
#[doc(alias = "WORD027")]
pub type Word027 = crate::Reg<word027::Word027Spec>;
#[doc = "Message Buffer 27 WORD0 Register"]
pub mod word027;
#[doc = "MB11_32B_ID (rw) register accessor: Message Buffer 11 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_32b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_32b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_32b_id`] module"]
#[doc(alias = "MB11_32B_ID")]
pub type Mb11_32bId = crate::Reg<mb11_32b_id::Mb11_32bIdSpec>;
#[doc = "Message Buffer 11 ID Register"]
pub mod mb11_32b_id;
#[doc = "MB18_16B_WORD1 (rw) register accessor: Message Buffer 18 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb18_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb18_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb18_16b_word1`] module"]
#[doc(alias = "MB18_16B_WORD1")]
pub type Mb18_16bWord1 = crate::Reg<mb18_16b_word1::Mb18_16bWord1Spec>;
#[doc = "Message Buffer 18 WORD_16B Register"]
pub mod mb18_16b_word1;
#[doc = "MB27_8B_WORD1 (rw) register accessor: Message Buffer 27 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb27_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb27_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb27_8b_word1`] module"]
#[doc(alias = "MB27_8B_WORD1")]
pub type Mb27_8bWord1 = crate::Reg<mb27_8b_word1::Mb27_8bWord1Spec>;
#[doc = "Message Buffer 27 WORD_8B Register"]
pub mod mb27_8b_word1;
#[doc = "MB6_64B_WORD1 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word1`] module"]
#[doc(alias = "MB6_64B_WORD1")]
pub type Mb6_64bWord1 = crate::Reg<mb6_64b_word1::Mb6_64bWord1Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word1;
#[doc = "WORD127 (rw) register accessor: Message Buffer 27 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word127::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word127::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word127`] module"]
#[doc(alias = "WORD127")]
pub type Word127 = crate::Reg<word127::Word127Spec>;
#[doc = "Message Buffer 27 WORD1 Register"]
pub mod word127;
#[doc = "CS28 (rw) register accessor: Message Buffer 28 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs28`] module"]
#[doc(alias = "CS28")]
pub type Cs28 = crate::Reg<cs28::Cs28Spec>;
#[doc = "Message Buffer 28 CS Register"]
pub mod cs28;
#[doc = "MB11_32B_WORD0 (rw) register accessor: Message Buffer 11 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_32b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_32b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_32b_word0`] module"]
#[doc(alias = "MB11_32B_WORD0")]
pub type Mb11_32bWord0 = crate::Reg<mb11_32b_word0::Mb11_32bWord0Spec>;
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod mb11_32b_word0;
#[doc = "MB18_16B_WORD2 (rw) register accessor: Message Buffer 18 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb18_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb18_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb18_16b_word2`] module"]
#[doc(alias = "MB18_16B_WORD2")]
pub type Mb18_16bWord2 = crate::Reg<mb18_16b_word2::Mb18_16bWord2Spec>;
#[doc = "Message Buffer 18 WORD_16B Register"]
pub mod mb18_16b_word2;
#[doc = "MB28_8B_CS (rw) register accessor: Message Buffer 28 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb28_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb28_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb28_8b_cs`] module"]
#[doc(alias = "MB28_8B_CS")]
pub type Mb28_8bCs = crate::Reg<mb28_8b_cs::Mb28_8bCsSpec>;
#[doc = "Message Buffer 28 CS Register"]
pub mod mb28_8b_cs;
#[doc = "MB6_64B_WORD2 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word2`] module"]
#[doc(alias = "MB6_64B_WORD2")]
pub type Mb6_64bWord2 = crate::Reg<mb6_64b_word2::Mb6_64bWord2Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word2;
#[doc = "ID28 (rw) register accessor: Message Buffer 28 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id28`] module"]
#[doc(alias = "ID28")]
pub type Id28 = crate::Reg<id28::Id28Spec>;
#[doc = "Message Buffer 28 ID Register"]
pub mod id28;
#[doc = "MB11_32B_WORD1 (rw) register accessor: Message Buffer 11 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_32b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_32b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_32b_word1`] module"]
#[doc(alias = "MB11_32B_WORD1")]
pub type Mb11_32bWord1 = crate::Reg<mb11_32b_word1::Mb11_32bWord1Spec>;
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod mb11_32b_word1;
#[doc = "MB18_16B_WORD3 (rw) register accessor: Message Buffer 18 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb18_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb18_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb18_16b_word3`] module"]
#[doc(alias = "MB18_16B_WORD3")]
pub type Mb18_16bWord3 = crate::Reg<mb18_16b_word3::Mb18_16bWord3Spec>;
#[doc = "Message Buffer 18 WORD_16B Register"]
pub mod mb18_16b_word3;
#[doc = "MB28_8B_ID (rw) register accessor: Message Buffer 28 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb28_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb28_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb28_8b_id`] module"]
#[doc(alias = "MB28_8B_ID")]
pub type Mb28_8bId = crate::Reg<mb28_8b_id::Mb28_8bIdSpec>;
#[doc = "Message Buffer 28 ID Register"]
pub mod mb28_8b_id;
#[doc = "MB6_64B_WORD3 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word3`] module"]
#[doc(alias = "MB6_64B_WORD3")]
pub type Mb6_64bWord3 = crate::Reg<mb6_64b_word3::Mb6_64bWord3Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word3;
#[doc = "MB11_32B_WORD2 (rw) register accessor: Message Buffer 11 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_32b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_32b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_32b_word2`] module"]
#[doc(alias = "MB11_32B_WORD2")]
pub type Mb11_32bWord2 = crate::Reg<mb11_32b_word2::Mb11_32bWord2Spec>;
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod mb11_32b_word2;
#[doc = "MB19_16B_CS (rw) register accessor: Message Buffer 19 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb19_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb19_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb19_16b_cs`] module"]
#[doc(alias = "MB19_16B_CS")]
pub type Mb19_16bCs = crate::Reg<mb19_16b_cs::Mb19_16bCsSpec>;
#[doc = "Message Buffer 19 CS Register"]
pub mod mb19_16b_cs;
#[doc = "MB28_8B_WORD0 (rw) register accessor: Message Buffer 28 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb28_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb28_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb28_8b_word0`] module"]
#[doc(alias = "MB28_8B_WORD0")]
pub type Mb28_8bWord0 = crate::Reg<mb28_8b_word0::Mb28_8bWord0Spec>;
#[doc = "Message Buffer 28 WORD_8B Register"]
pub mod mb28_8b_word0;
#[doc = "MB6_64B_WORD4 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word4`] module"]
#[doc(alias = "MB6_64B_WORD4")]
pub type Mb6_64bWord4 = crate::Reg<mb6_64b_word4::Mb6_64bWord4Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word4;
#[doc = "WORD028 (rw) register accessor: Message Buffer 28 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word028`] module"]
#[doc(alias = "WORD028")]
pub type Word028 = crate::Reg<word028::Word028Spec>;
#[doc = "Message Buffer 28 WORD0 Register"]
pub mod word028;
#[doc = "MB11_32B_WORD3 (rw) register accessor: Message Buffer 11 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_32b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_32b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_32b_word3`] module"]
#[doc(alias = "MB11_32B_WORD3")]
pub type Mb11_32bWord3 = crate::Reg<mb11_32b_word3::Mb11_32bWord3Spec>;
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod mb11_32b_word3;
#[doc = "MB19_16B_ID (rw) register accessor: Message Buffer 19 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb19_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb19_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb19_16b_id`] module"]
#[doc(alias = "MB19_16B_ID")]
pub type Mb19_16bId = crate::Reg<mb19_16b_id::Mb19_16bIdSpec>;
#[doc = "Message Buffer 19 ID Register"]
pub mod mb19_16b_id;
#[doc = "MB28_8B_WORD1 (rw) register accessor: Message Buffer 28 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb28_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb28_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb28_8b_word1`] module"]
#[doc(alias = "MB28_8B_WORD1")]
pub type Mb28_8bWord1 = crate::Reg<mb28_8b_word1::Mb28_8bWord1Spec>;
#[doc = "Message Buffer 28 WORD_8B Register"]
pub mod mb28_8b_word1;
#[doc = "MB6_64B_WORD5 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word5`] module"]
#[doc(alias = "MB6_64B_WORD5")]
pub type Mb6_64bWord5 = crate::Reg<mb6_64b_word5::Mb6_64bWord5Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word5;
#[doc = "WORD128 (rw) register accessor: Message Buffer 28 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word128`] module"]
#[doc(alias = "WORD128")]
pub type Word128 = crate::Reg<word128::Word128Spec>;
#[doc = "Message Buffer 28 WORD1 Register"]
pub mod word128;
#[doc = "CS29 (rw) register accessor: Message Buffer 29 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs29`] module"]
#[doc(alias = "CS29")]
pub type Cs29 = crate::Reg<cs29::Cs29Spec>;
#[doc = "Message Buffer 29 CS Register"]
pub mod cs29;
#[doc = "MB11_32B_WORD4 (rw) register accessor: Message Buffer 11 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_32b_word4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_32b_word4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_32b_word4`] module"]
#[doc(alias = "MB11_32B_WORD4")]
pub type Mb11_32bWord4 = crate::Reg<mb11_32b_word4::Mb11_32bWord4Spec>;
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod mb11_32b_word4;
#[doc = "MB19_16B_WORD0 (rw) register accessor: Message Buffer 19 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb19_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb19_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb19_16b_word0`] module"]
#[doc(alias = "MB19_16B_WORD0")]
pub type Mb19_16bWord0 = crate::Reg<mb19_16b_word0::Mb19_16bWord0Spec>;
#[doc = "Message Buffer 19 WORD_16B Register"]
pub mod mb19_16b_word0;
#[doc = "MB29_8B_CS (rw) register accessor: Message Buffer 29 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb29_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb29_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb29_8b_cs`] module"]
#[doc(alias = "MB29_8B_CS")]
pub type Mb29_8bCs = crate::Reg<mb29_8b_cs::Mb29_8bCsSpec>;
#[doc = "Message Buffer 29 CS Register"]
pub mod mb29_8b_cs;
#[doc = "MB6_64B_WORD6 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word6`] module"]
#[doc(alias = "MB6_64B_WORD6")]
pub type Mb6_64bWord6 = crate::Reg<mb6_64b_word6::Mb6_64bWord6Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word6;
#[doc = "ID29 (rw) register accessor: Message Buffer 29 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id29`] module"]
#[doc(alias = "ID29")]
pub type Id29 = crate::Reg<id29::Id29Spec>;
#[doc = "Message Buffer 29 ID Register"]
pub mod id29;
#[doc = "MB11_32B_WORD5 (rw) register accessor: Message Buffer 11 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_32b_word5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_32b_word5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_32b_word5`] module"]
#[doc(alias = "MB11_32B_WORD5")]
pub type Mb11_32bWord5 = crate::Reg<mb11_32b_word5::Mb11_32bWord5Spec>;
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod mb11_32b_word5;
#[doc = "MB19_16B_WORD1 (rw) register accessor: Message Buffer 19 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb19_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb19_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb19_16b_word1`] module"]
#[doc(alias = "MB19_16B_WORD1")]
pub type Mb19_16bWord1 = crate::Reg<mb19_16b_word1::Mb19_16bWord1Spec>;
#[doc = "Message Buffer 19 WORD_16B Register"]
pub mod mb19_16b_word1;
#[doc = "MB29_8B_ID (rw) register accessor: Message Buffer 29 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb29_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb29_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb29_8b_id`] module"]
#[doc(alias = "MB29_8B_ID")]
pub type Mb29_8bId = crate::Reg<mb29_8b_id::Mb29_8bIdSpec>;
#[doc = "Message Buffer 29 ID Register"]
pub mod mb29_8b_id;
#[doc = "MB6_64B_WORD7 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word7`] module"]
#[doc(alias = "MB6_64B_WORD7")]
pub type Mb6_64bWord7 = crate::Reg<mb6_64b_word7::Mb6_64bWord7Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word7;
#[doc = "MB11_32B_WORD6 (rw) register accessor: Message Buffer 11 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_32b_word6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_32b_word6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_32b_word6`] module"]
#[doc(alias = "MB11_32B_WORD6")]
pub type Mb11_32bWord6 = crate::Reg<mb11_32b_word6::Mb11_32bWord6Spec>;
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod mb11_32b_word6;
#[doc = "MB19_16B_WORD2 (rw) register accessor: Message Buffer 19 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb19_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb19_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb19_16b_word2`] module"]
#[doc(alias = "MB19_16B_WORD2")]
pub type Mb19_16bWord2 = crate::Reg<mb19_16b_word2::Mb19_16bWord2Spec>;
#[doc = "Message Buffer 19 WORD_16B Register"]
pub mod mb19_16b_word2;
#[doc = "MB29_8B_WORD0 (rw) register accessor: Message Buffer 29 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb29_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb29_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb29_8b_word0`] module"]
#[doc(alias = "MB29_8B_WORD0")]
pub type Mb29_8bWord0 = crate::Reg<mb29_8b_word0::Mb29_8bWord0Spec>;
#[doc = "Message Buffer 29 WORD_8B Register"]
pub mod mb29_8b_word0;
#[doc = "MB6_64B_WORD8 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word8`] module"]
#[doc(alias = "MB6_64B_WORD8")]
pub type Mb6_64bWord8 = crate::Reg<mb6_64b_word8::Mb6_64bWord8Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word8;
#[doc = "WORD029 (rw) register accessor: Message Buffer 29 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word029::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word029::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word029`] module"]
#[doc(alias = "WORD029")]
pub type Word029 = crate::Reg<word029::Word029Spec>;
#[doc = "Message Buffer 29 WORD0 Register"]
pub mod word029;
#[doc = "MB11_32B_WORD7 (rw) register accessor: Message Buffer 11 WORD_32B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb11_32b_word7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb11_32b_word7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb11_32b_word7`] module"]
#[doc(alias = "MB11_32B_WORD7")]
pub type Mb11_32bWord7 = crate::Reg<mb11_32b_word7::Mb11_32bWord7Spec>;
#[doc = "Message Buffer 11 WORD_32B Register"]
pub mod mb11_32b_word7;
#[doc = "MB19_16B_WORD3 (rw) register accessor: Message Buffer 19 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb19_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb19_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb19_16b_word3`] module"]
#[doc(alias = "MB19_16B_WORD3")]
pub type Mb19_16bWord3 = crate::Reg<mb19_16b_word3::Mb19_16bWord3Spec>;
#[doc = "Message Buffer 19 WORD_16B Register"]
pub mod mb19_16b_word3;
#[doc = "MB29_8B_WORD1 (rw) register accessor: Message Buffer 29 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb29_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb29_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb29_8b_word1`] module"]
#[doc(alias = "MB29_8B_WORD1")]
pub type Mb29_8bWord1 = crate::Reg<mb29_8b_word1::Mb29_8bWord1Spec>;
#[doc = "Message Buffer 29 WORD_8B Register"]
pub mod mb29_8b_word1;
#[doc = "MB6_64B_WORD9 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word9`] module"]
#[doc(alias = "MB6_64B_WORD9")]
pub type Mb6_64bWord9 = crate::Reg<mb6_64b_word9::Mb6_64bWord9Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word9;
#[doc = "WORD129 (rw) register accessor: Message Buffer 29 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word129::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word129::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word129`] module"]
#[doc(alias = "WORD129")]
pub type Word129 = crate::Reg<word129::Word129Spec>;
#[doc = "Message Buffer 29 WORD1 Register"]
pub mod word129;
#[doc = "CS30 (rw) register accessor: Message Buffer 30 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs30`] module"]
#[doc(alias = "CS30")]
pub type Cs30 = crate::Reg<cs30::Cs30Spec>;
#[doc = "Message Buffer 30 CS Register"]
pub mod cs30;
#[doc = "MB20_16B_CS (rw) register accessor: Message Buffer 20 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb20_16b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb20_16b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb20_16b_cs`] module"]
#[doc(alias = "MB20_16B_CS")]
pub type Mb20_16bCs = crate::Reg<mb20_16b_cs::Mb20_16bCsSpec>;
#[doc = "Message Buffer 20 CS Register"]
pub mod mb20_16b_cs;
#[doc = "MB30_8B_CS (rw) register accessor: Message Buffer 30 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb30_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb30_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb30_8b_cs`] module"]
#[doc(alias = "MB30_8B_CS")]
pub type Mb30_8bCs = crate::Reg<mb30_8b_cs::Mb30_8bCsSpec>;
#[doc = "Message Buffer 30 CS Register"]
pub mod mb30_8b_cs;
#[doc = "MB6_64B_WORD10 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word10`] module"]
#[doc(alias = "MB6_64B_WORD10")]
pub type Mb6_64bWord10 = crate::Reg<mb6_64b_word10::Mb6_64bWord10Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word10;
#[doc = "ID30 (rw) register accessor: Message Buffer 30 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id30`] module"]
#[doc(alias = "ID30")]
pub type Id30 = crate::Reg<id30::Id30Spec>;
#[doc = "Message Buffer 30 ID Register"]
pub mod id30;
#[doc = "MB20_16B_ID (rw) register accessor: Message Buffer 20 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb20_16b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb20_16b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb20_16b_id`] module"]
#[doc(alias = "MB20_16B_ID")]
pub type Mb20_16bId = crate::Reg<mb20_16b_id::Mb20_16bIdSpec>;
#[doc = "Message Buffer 20 ID Register"]
pub mod mb20_16b_id;
#[doc = "MB30_8B_ID (rw) register accessor: Message Buffer 30 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb30_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb30_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb30_8b_id`] module"]
#[doc(alias = "MB30_8B_ID")]
pub type Mb30_8bId = crate::Reg<mb30_8b_id::Mb30_8bIdSpec>;
#[doc = "Message Buffer 30 ID Register"]
pub mod mb30_8b_id;
#[doc = "MB6_64B_WORD11 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word11`] module"]
#[doc(alias = "MB6_64B_WORD11")]
pub type Mb6_64bWord11 = crate::Reg<mb6_64b_word11::Mb6_64bWord11Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word11;
#[doc = "MB20_16B_WORD0 (rw) register accessor: Message Buffer 20 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb20_16b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb20_16b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb20_16b_word0`] module"]
#[doc(alias = "MB20_16B_WORD0")]
pub type Mb20_16bWord0 = crate::Reg<mb20_16b_word0::Mb20_16bWord0Spec>;
#[doc = "Message Buffer 20 WORD_16B Register"]
pub mod mb20_16b_word0;
#[doc = "MB30_8B_WORD0 (rw) register accessor: Message Buffer 30 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb30_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb30_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb30_8b_word0`] module"]
#[doc(alias = "MB30_8B_WORD0")]
pub type Mb30_8bWord0 = crate::Reg<mb30_8b_word0::Mb30_8bWord0Spec>;
#[doc = "Message Buffer 30 WORD_8B Register"]
pub mod mb30_8b_word0;
#[doc = "MB6_64B_WORD12 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word12`] module"]
#[doc(alias = "MB6_64B_WORD12")]
pub type Mb6_64bWord12 = crate::Reg<mb6_64b_word12::Mb6_64bWord12Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word12;
#[doc = "WORD030 (rw) register accessor: Message Buffer 30 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word030`] module"]
#[doc(alias = "WORD030")]
pub type Word030 = crate::Reg<word030::Word030Spec>;
#[doc = "Message Buffer 30 WORD0 Register"]
pub mod word030;
#[doc = "MB20_16B_WORD1 (rw) register accessor: Message Buffer 20 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb20_16b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb20_16b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb20_16b_word1`] module"]
#[doc(alias = "MB20_16B_WORD1")]
pub type Mb20_16bWord1 = crate::Reg<mb20_16b_word1::Mb20_16bWord1Spec>;
#[doc = "Message Buffer 20 WORD_16B Register"]
pub mod mb20_16b_word1;
#[doc = "MB30_8B_WORD1 (rw) register accessor: Message Buffer 30 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb30_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb30_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb30_8b_word1`] module"]
#[doc(alias = "MB30_8B_WORD1")]
pub type Mb30_8bWord1 = crate::Reg<mb30_8b_word1::Mb30_8bWord1Spec>;
#[doc = "Message Buffer 30 WORD_8B Register"]
pub mod mb30_8b_word1;
#[doc = "MB6_64B_WORD13 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word13`] module"]
#[doc(alias = "MB6_64B_WORD13")]
pub type Mb6_64bWord13 = crate::Reg<mb6_64b_word13::Mb6_64bWord13Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word13;
#[doc = "WORD130 (rw) register accessor: Message Buffer 30 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word130`] module"]
#[doc(alias = "WORD130")]
pub type Word130 = crate::Reg<word130::Word130Spec>;
#[doc = "Message Buffer 30 WORD1 Register"]
pub mod word130;
#[doc = "CS31 (rw) register accessor: Message Buffer 31 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cs31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cs31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cs31`] module"]
#[doc(alias = "CS31")]
pub type Cs31 = crate::Reg<cs31::Cs31Spec>;
#[doc = "Message Buffer 31 CS Register"]
pub mod cs31;
#[doc = "MB20_16B_WORD2 (rw) register accessor: Message Buffer 20 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb20_16b_word2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb20_16b_word2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb20_16b_word2`] module"]
#[doc(alias = "MB20_16B_WORD2")]
pub type Mb20_16bWord2 = crate::Reg<mb20_16b_word2::Mb20_16bWord2Spec>;
#[doc = "Message Buffer 20 WORD_16B Register"]
pub mod mb20_16b_word2;
#[doc = "MB31_8B_CS (rw) register accessor: Message Buffer 31 CS Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb31_8b_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb31_8b_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb31_8b_cs`] module"]
#[doc(alias = "MB31_8B_CS")]
pub type Mb31_8bCs = crate::Reg<mb31_8b_cs::Mb31_8bCsSpec>;
#[doc = "Message Buffer 31 CS Register"]
pub mod mb31_8b_cs;
#[doc = "MB6_64B_WORD14 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word14`] module"]
#[doc(alias = "MB6_64B_WORD14")]
pub type Mb6_64bWord14 = crate::Reg<mb6_64b_word14::Mb6_64bWord14Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word14;
#[doc = "ID31 (rw) register accessor: Message Buffer 31 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id31`] module"]
#[doc(alias = "ID31")]
pub type Id31 = crate::Reg<id31::Id31Spec>;
#[doc = "Message Buffer 31 ID Register"]
pub mod id31;
#[doc = "MB20_16B_WORD3 (rw) register accessor: Message Buffer 20 WORD_16B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb20_16b_word3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb20_16b_word3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb20_16b_word3`] module"]
#[doc(alias = "MB20_16B_WORD3")]
pub type Mb20_16bWord3 = crate::Reg<mb20_16b_word3::Mb20_16bWord3Spec>;
#[doc = "Message Buffer 20 WORD_16B Register"]
pub mod mb20_16b_word3;
#[doc = "MB31_8B_ID (rw) register accessor: Message Buffer 31 ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb31_8b_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb31_8b_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb31_8b_id`] module"]
#[doc(alias = "MB31_8B_ID")]
pub type Mb31_8bId = crate::Reg<mb31_8b_id::Mb31_8bIdSpec>;
#[doc = "Message Buffer 31 ID Register"]
pub mod mb31_8b_id;
#[doc = "MB6_64B_WORD15 (rw) register accessor: Message Buffer 6 WORD_64B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb6_64b_word15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb6_64b_word15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb6_64b_word15`] module"]
#[doc(alias = "MB6_64B_WORD15")]
pub type Mb6_64bWord15 = crate::Reg<mb6_64b_word15::Mb6_64bWord15Spec>;
#[doc = "Message Buffer 6 WORD_64B Register"]
pub mod mb6_64b_word15;
#[doc = "MB31_8B_WORD0 (rw) register accessor: Message Buffer 31 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb31_8b_word0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb31_8b_word0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb31_8b_word0`] module"]
#[doc(alias = "MB31_8B_WORD0")]
pub type Mb31_8bWord0 = crate::Reg<mb31_8b_word0::Mb31_8bWord0Spec>;
#[doc = "Message Buffer 31 WORD_8B Register"]
pub mod mb31_8b_word0;
#[doc = "WORD031 (rw) register accessor: Message Buffer 31 WORD0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word031::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word031::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word031`] module"]
#[doc(alias = "WORD031")]
pub type Word031 = crate::Reg<word031::Word031Spec>;
#[doc = "Message Buffer 31 WORD0 Register"]
pub mod word031;
#[doc = "MB31_8B_WORD1 (rw) register accessor: Message Buffer 31 WORD_8B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mb31_8b_word1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mb31_8b_word1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mb31_8b_word1`] module"]
#[doc(alias = "MB31_8B_WORD1")]
pub type Mb31_8bWord1 = crate::Reg<mb31_8b_word1::Mb31_8bWord1Spec>;
#[doc = "Message Buffer 31 WORD_8B Register"]
pub mod mb31_8b_word1;
#[doc = "WORD131 (rw) register accessor: Message Buffer 31 WORD1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`word131::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`word131::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word131`] module"]
#[doc(alias = "WORD131")]
pub type Word131 = crate::Reg<word131::Word131Spec>;
#[doc = "Message Buffer 31 WORD1 Register"]
pub mod word131;
#[doc = "RXIMR (rw) register accessor: Receive Individual Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`rximr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rximr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rximr`] module"]
#[doc(alias = "RXIMR")]
pub type Rximr = crate::Reg<rximr::RximrSpec>;
#[doc = "Receive Individual Mask"]
pub mod rximr;
#[doc = "CTRL1_PN (rw) register accessor: Pretended Networking Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1_pn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1_pn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1_pn`] module"]
#[doc(alias = "CTRL1_PN")]
pub type Ctrl1Pn = crate::Reg<ctrl1_pn::Ctrl1PnSpec>;
#[doc = "Pretended Networking Control 1"]
pub mod ctrl1_pn;
#[doc = "CTRL2_PN (rw) register accessor: Pretended Networking Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2_pn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2_pn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2_pn`] module"]
#[doc(alias = "CTRL2_PN")]
pub type Ctrl2Pn = crate::Reg<ctrl2_pn::Ctrl2PnSpec>;
#[doc = "Pretended Networking Control 2"]
pub mod ctrl2_pn;
#[doc = "WU_MTC (rw) register accessor: Pretended Networking Wake-Up Match\n\nYou can [`read`](crate::Reg::read) this register and get [`wu_mtc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wu_mtc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wu_mtc`] module"]
#[doc(alias = "WU_MTC")]
pub type WuMtc = crate::Reg<wu_mtc::WuMtcSpec>;
#[doc = "Pretended Networking Wake-Up Match"]
pub mod wu_mtc;
#[doc = "FLT_ID1 (rw) register accessor: Pretended Networking ID Filter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`flt_id1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt_id1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flt_id1`] module"]
#[doc(alias = "FLT_ID1")]
pub type FltId1 = crate::Reg<flt_id1::FltId1Spec>;
#[doc = "Pretended Networking ID Filter 1"]
pub mod flt_id1;
#[doc = "FLT_DLC (rw) register accessor: Pretended Networking Data Length Code (DLC) Filter\n\nYou can [`read`](crate::Reg::read) this register and get [`flt_dlc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt_dlc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flt_dlc`] module"]
#[doc(alias = "FLT_DLC")]
pub type FltDlc = crate::Reg<flt_dlc::FltDlcSpec>;
#[doc = "Pretended Networking Data Length Code (DLC) Filter"]
pub mod flt_dlc;
#[doc = "PL1_LO (rw) register accessor: Pretended Networking Payload Low Filter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pl1_lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl1_lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl1_lo`] module"]
#[doc(alias = "PL1_LO")]
pub type Pl1Lo = crate::Reg<pl1_lo::Pl1LoSpec>;
#[doc = "Pretended Networking Payload Low Filter 1"]
pub mod pl1_lo;
#[doc = "PL1_HI (rw) register accessor: Pretended Networking Payload High Filter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pl1_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl1_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl1_hi`] module"]
#[doc(alias = "PL1_HI")]
pub type Pl1Hi = crate::Reg<pl1_hi::Pl1HiSpec>;
#[doc = "Pretended Networking Payload High Filter 1"]
pub mod pl1_hi;
#[doc = "FLT_ID2_IDMASK (rw) register accessor: Pretended Networking ID Filter 2 or ID Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`flt_id2_idmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt_id2_idmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flt_id2_idmask`] module"]
#[doc(alias = "FLT_ID2_IDMASK")]
pub type FltId2Idmask = crate::Reg<flt_id2_idmask::FltId2IdmaskSpec>;
#[doc = "Pretended Networking ID Filter 2 or ID Mask"]
pub mod flt_id2_idmask;
#[doc = "PL2_PLMASK_LO (rw) register accessor: Pretended Networking Payload Low Filter 2 and Payload Low Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`pl2_plmask_lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl2_plmask_lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl2_plmask_lo`] module"]
#[doc(alias = "PL2_PLMASK_LO")]
pub type Pl2PlmaskLo = crate::Reg<pl2_plmask_lo::Pl2PlmaskLoSpec>;
#[doc = "Pretended Networking Payload Low Filter 2 and Payload Low Mask"]
pub mod pl2_plmask_lo;
#[doc = "PL2_PLMASK_HI (rw) register accessor: Pretended Networking Payload High Filter 2 and Payload High Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`pl2_plmask_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl2_plmask_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pl2_plmask_hi`] module"]
#[doc(alias = "PL2_PLMASK_HI")]
pub type Pl2PlmaskHi = crate::Reg<pl2_plmask_hi::Pl2PlmaskHiSpec>;
#[doc = "Pretended Networking Payload High Filter 2 and Payload High Mask"]
pub mod pl2_plmask_hi;
#[doc = "Array of registers: WMB_CS, WMB_D03, WMB_D47, WMB_ID"]
pub use self::wmb::Wmb;
#[doc = r"Cluster"]
#[doc = "Array of registers: WMB_CS, WMB_D03, WMB_D47, WMB_ID"]
pub mod wmb;
#[doc = "EPRS (rw) register accessor: Enhanced CAN Bit Timing Prescalers\n\nYou can [`read`](crate::Reg::read) this register and get [`eprs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eprs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eprs`] module"]
#[doc(alias = "EPRS")]
pub type Eprs = crate::Reg<eprs::EprsSpec>;
#[doc = "Enhanced CAN Bit Timing Prescalers"]
pub mod eprs;
#[doc = "ENCBT (rw) register accessor: Enhanced Nominal CAN Bit Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`encbt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`encbt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@encbt`] module"]
#[doc(alias = "ENCBT")]
pub type Encbt = crate::Reg<encbt::EncbtSpec>;
#[doc = "Enhanced Nominal CAN Bit Timing"]
pub mod encbt;
#[doc = "EDCBT (rw) register accessor: Enhanced Data Phase CAN Bit Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`edcbt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edcbt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edcbt`] module"]
#[doc(alias = "EDCBT")]
pub type Edcbt = crate::Reg<edcbt::EdcbtSpec>;
#[doc = "Enhanced Data Phase CAN Bit Timing"]
pub mod edcbt;
#[doc = "ETDC (rw) register accessor: Enhanced Transceiver Delay Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`etdc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etdc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etdc`] module"]
#[doc(alias = "ETDC")]
pub type Etdc = crate::Reg<etdc::EtdcSpec>;
#[doc = "Enhanced Transceiver Delay Compensation"]
pub mod etdc;
#[doc = "FDCTRL (rw) register accessor: CAN FD Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fdctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdctrl`] module"]
#[doc(alias = "FDCTRL")]
pub type Fdctrl = crate::Reg<fdctrl::FdctrlSpec>;
#[doc = "CAN FD Control"]
pub mod fdctrl;
#[doc = "FDCBT (rw) register accessor: CAN FD Bit Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcbt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcbt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcbt`] module"]
#[doc(alias = "FDCBT")]
pub type Fdcbt = crate::Reg<fdcbt::FdcbtSpec>;
#[doc = "CAN FD Bit Timing"]
pub mod fdcbt;
#[doc = "FDCRC (r) register accessor: CAN FD CRC\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcrc`] module"]
#[doc(alias = "FDCRC")]
pub type Fdcrc = crate::Reg<fdcrc::FdcrcSpec>;
#[doc = "CAN FD CRC"]
pub mod fdcrc;
#[doc = "ERFCR (rw) register accessor: Enhanced RX FIFO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`erfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erfcr`] module"]
#[doc(alias = "ERFCR")]
pub type Erfcr = crate::Reg<erfcr::ErfcrSpec>;
#[doc = "Enhanced RX FIFO Control"]
pub mod erfcr;
#[doc = "ERFIER (rw) register accessor: Enhanced RX FIFO Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`erfier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erfier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erfier`] module"]
#[doc(alias = "ERFIER")]
pub type Erfier = crate::Reg<erfier::ErfierSpec>;
#[doc = "Enhanced RX FIFO Interrupt Enable"]
pub mod erfier;
#[doc = "ERFSR (rw) register accessor: Enhanced RX FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`erfsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erfsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erfsr`] module"]
#[doc(alias = "ERFSR")]
pub type Erfsr = crate::Reg<erfsr::ErfsrSpec>;
#[doc = "Enhanced RX FIFO Status"]
pub mod erfsr;
#[doc = "ERFFEL (rw) register accessor: Enhanced RX FIFO Filter Element\n\nYou can [`read`](crate::Reg::read) this register and get [`erffel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erffel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erffel`] module"]
#[doc(alias = "ERFFEL")]
pub type Erffel = crate::Reg<erffel::ErffelSpec>;
#[doc = "Enhanced RX FIFO Filter Element"]
pub mod erffel;
