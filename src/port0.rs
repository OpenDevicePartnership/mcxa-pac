#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    verid: Verid,
    _reserved1: [u8; 0x0c],
    gpclr: Gpclr,
    gpchr: Gpchr,
    _reserved3: [u8; 0x08],
    config: Config,
    _reserved4: [u8; 0x3c],
    calib0: Calib0,
    calib1: Calib1,
    _reserved6: [u8; 0x18],
    pcr0: Pcr0,
    pcr1: Pcr1,
    pcr2: Pcr2,
    pcr3: Pcr3,
    pcr4: Pcr4,
    pcr5: Pcr5,
    pcr6: Pcr6,
    pcr7: Pcr7,
    pcr8: Pcr8,
    pcr9: Pcr9,
    pcr10: Pcr10,
    pcr11: Pcr11,
    pcr12: Pcr12,
    pcr13: Pcr13,
    pcr14: Pcr14,
    pcr15: Pcr15,
    pcr16: Pcr16,
    pcr17: Pcr17,
    pcr18: Pcr18,
    pcr19: Pcr19,
    pcr20: Pcr20,
    pcr21: Pcr21,
    pcr22: Pcr22,
    pcr23: Pcr23,
    pcr24: Pcr24,
    pcr25: Pcr25,
    pcr26: Pcr26,
    pcr27: Pcr27,
    pcr28: Pcr28,
    pcr29: Pcr29,
    pcr30: Pcr30,
    pcr31: Pcr31,
}
impl RegisterBlock {
    #[doc = "0x00 - Version ID"]
    #[inline(always)]
    pub const fn verid(&self) -> &Verid {
        &self.verid
    }
    #[doc = "0x10 - Global Pin Control Low"]
    #[inline(always)]
    pub const fn gpclr(&self) -> &Gpclr {
        &self.gpclr
    }
    #[doc = "0x14 - Global Pin Control High"]
    #[inline(always)]
    pub const fn gpchr(&self) -> &Gpchr {
        &self.gpchr
    }
    #[doc = "0x20 - Configuration"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x60 - Calibration 0"]
    #[inline(always)]
    pub const fn calib0(&self) -> &Calib0 {
        &self.calib0
    }
    #[doc = "0x64 - Calibration 1"]
    #[inline(always)]
    pub const fn calib1(&self) -> &Calib1 {
        &self.calib1
    }
    #[doc = "0x80 - Pin Control 0"]
    #[inline(always)]
    pub const fn pcr0(&self) -> &Pcr0 {
        &self.pcr0
    }
    #[doc = "0x84 - Pin Control 1"]
    #[inline(always)]
    pub const fn pcr1(&self) -> &Pcr1 {
        &self.pcr1
    }
    #[doc = "0x88 - Pin Control 2"]
    #[inline(always)]
    pub const fn pcr2(&self) -> &Pcr2 {
        &self.pcr2
    }
    #[doc = "0x8c - Pin Control 3"]
    #[inline(always)]
    pub const fn pcr3(&self) -> &Pcr3 {
        &self.pcr3
    }
    #[doc = "0x90 - Pin Control 4"]
    #[inline(always)]
    pub const fn pcr4(&self) -> &Pcr4 {
        &self.pcr4
    }
    #[doc = "0x94 - Pin Control 5"]
    #[inline(always)]
    pub const fn pcr5(&self) -> &Pcr5 {
        &self.pcr5
    }
    #[doc = "0x98 - Pin Control 6"]
    #[inline(always)]
    pub const fn pcr6(&self) -> &Pcr6 {
        &self.pcr6
    }
    #[doc = "0x9c - Pin Control 7"]
    #[inline(always)]
    pub const fn pcr7(&self) -> &Pcr7 {
        &self.pcr7
    }
    #[doc = "0xa0 - Pin Control 8"]
    #[inline(always)]
    pub const fn pcr8(&self) -> &Pcr8 {
        &self.pcr8
    }
    #[doc = "0xa4 - Pin Control 9"]
    #[inline(always)]
    pub const fn pcr9(&self) -> &Pcr9 {
        &self.pcr9
    }
    #[doc = "0xa8 - Pin Control 10"]
    #[inline(always)]
    pub const fn pcr10(&self) -> &Pcr10 {
        &self.pcr10
    }
    #[doc = "0xac - Pin Control 11"]
    #[inline(always)]
    pub const fn pcr11(&self) -> &Pcr11 {
        &self.pcr11
    }
    #[doc = "0xb0 - Pin Control 12"]
    #[inline(always)]
    pub const fn pcr12(&self) -> &Pcr12 {
        &self.pcr12
    }
    #[doc = "0xb4 - Pin Control 13"]
    #[inline(always)]
    pub const fn pcr13(&self) -> &Pcr13 {
        &self.pcr13
    }
    #[doc = "0xb8 - Pin Control 14"]
    #[inline(always)]
    pub const fn pcr14(&self) -> &Pcr14 {
        &self.pcr14
    }
    #[doc = "0xbc - Pin Control 15"]
    #[inline(always)]
    pub const fn pcr15(&self) -> &Pcr15 {
        &self.pcr15
    }
    #[doc = "0xc0 - Pin Control 16"]
    #[inline(always)]
    pub const fn pcr16(&self) -> &Pcr16 {
        &self.pcr16
    }
    #[doc = "0xc4 - Pin Control 17"]
    #[inline(always)]
    pub const fn pcr17(&self) -> &Pcr17 {
        &self.pcr17
    }
    #[doc = "0xc8 - Pin Control 18"]
    #[inline(always)]
    pub const fn pcr18(&self) -> &Pcr18 {
        &self.pcr18
    }
    #[doc = "0xcc - Pin Control 19"]
    #[inline(always)]
    pub const fn pcr19(&self) -> &Pcr19 {
        &self.pcr19
    }
    #[doc = "0xd0 - Pin Control 20"]
    #[inline(always)]
    pub const fn pcr20(&self) -> &Pcr20 {
        &self.pcr20
    }
    #[doc = "0xd4 - Pin Control 21"]
    #[inline(always)]
    pub const fn pcr21(&self) -> &Pcr21 {
        &self.pcr21
    }
    #[doc = "0xd8 - Pin Control 22"]
    #[inline(always)]
    pub const fn pcr22(&self) -> &Pcr22 {
        &self.pcr22
    }
    #[doc = "0xdc - Pin Control 23"]
    #[inline(always)]
    pub const fn pcr23(&self) -> &Pcr23 {
        &self.pcr23
    }
    #[doc = "0xe0 - Pin Control 24"]
    #[inline(always)]
    pub const fn pcr24(&self) -> &Pcr24 {
        &self.pcr24
    }
    #[doc = "0xe4 - Pin Control 25"]
    #[inline(always)]
    pub const fn pcr25(&self) -> &Pcr25 {
        &self.pcr25
    }
    #[doc = "0xe8 - Pin Control 26"]
    #[inline(always)]
    pub const fn pcr26(&self) -> &Pcr26 {
        &self.pcr26
    }
    #[doc = "0xec - Pin Control 27"]
    #[inline(always)]
    pub const fn pcr27(&self) -> &Pcr27 {
        &self.pcr27
    }
    #[doc = "0xf0 - Pin Control 28"]
    #[inline(always)]
    pub const fn pcr28(&self) -> &Pcr28 {
        &self.pcr28
    }
    #[doc = "0xf4 - Pin Control 29"]
    #[inline(always)]
    pub const fn pcr29(&self) -> &Pcr29 {
        &self.pcr29
    }
    #[doc = "0xf8 - Pin Control 30"]
    #[inline(always)]
    pub const fn pcr30(&self) -> &Pcr30 {
        &self.pcr30
    }
    #[doc = "0xfc - Pin Control 31"]
    #[inline(always)]
    pub const fn pcr31(&self) -> &Pcr31 {
        &self.pcr31
    }
}
#[doc = "VERID (r) register accessor: Version ID\n\nYou can [`read`](crate::Reg::read) this register and get [`verid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verid`] module"]
#[doc(alias = "VERID")]
pub type Verid = crate::Reg<verid::VeridSpec>;
#[doc = "Version ID"]
pub mod verid;
#[doc = "GPCLR (rw) register accessor: Global Pin Control Low\n\nYou can [`read`](crate::Reg::read) this register and get [`gpclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpclr`] module"]
#[doc(alias = "GPCLR")]
pub type Gpclr = crate::Reg<gpclr::GpclrSpec>;
#[doc = "Global Pin Control Low"]
pub mod gpclr;
#[doc = "GPCHR (rw) register accessor: Global Pin Control High\n\nYou can [`read`](crate::Reg::read) this register and get [`gpchr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpchr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpchr`] module"]
#[doc(alias = "GPCHR")]
pub type Gpchr = crate::Reg<gpchr::GpchrSpec>;
#[doc = "Global Pin Control High"]
pub mod gpchr;
#[doc = "CONFIG (rw) register accessor: Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration"]
pub mod config;
#[doc = "CALIB0 (rw) register accessor: Calibration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`calib0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calib0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calib0`] module"]
#[doc(alias = "CALIB0")]
pub type Calib0 = crate::Reg<calib0::Calib0Spec>;
#[doc = "Calibration 0"]
pub mod calib0;
#[doc = "CALIB1 (rw) register accessor: Calibration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`calib1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calib1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calib1`] module"]
#[doc(alias = "CALIB1")]
pub type Calib1 = crate::Reg<calib1::Calib1Spec>;
#[doc = "Calibration 1"]
pub mod calib1;
#[doc = "PCR0 (rw) register accessor: Pin Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr0`] module"]
#[doc(alias = "PCR0")]
pub type Pcr0 = crate::Reg<pcr0::Pcr0Spec>;
#[doc = "Pin Control 0"]
pub mod pcr0;
pub use pcr0 as pcr1;
pub use pcr0 as pcr2;
pub use pcr0 as pcr3;
pub use pcr0 as pcr4;
pub use pcr0 as pcr5;
pub use pcr0 as pcr6;
pub use pcr0 as pcr7;
pub use pcr0 as pcr8;
pub use pcr0 as pcr9;
pub use pcr0 as pcr10;
pub use pcr0 as pcr11;
pub use pcr0 as pcr12;
pub use pcr0 as pcr13;
pub use pcr0 as pcr14;
pub use pcr0 as pcr15;
pub use pcr0 as pcr16;
pub use pcr0 as pcr17;
pub use pcr0 as pcr18;
pub use pcr0 as pcr19;
pub use pcr0 as pcr20;
pub use pcr0 as pcr21;
pub use pcr0 as pcr22;
pub use pcr0 as pcr23;
pub use pcr0 as pcr24;
pub use pcr0 as pcr25;
pub use pcr0 as pcr26;
pub use pcr0 as pcr27;
pub use pcr0 as pcr28;
pub use pcr0 as pcr29;
pub use pcr0 as pcr30;
pub use pcr0 as pcr31;
pub use Pcr0 as Pcr1;
pub use Pcr0 as Pcr2;
pub use Pcr0 as Pcr3;
pub use Pcr0 as Pcr4;
pub use Pcr0 as Pcr5;
pub use Pcr0 as Pcr6;
pub use Pcr0 as Pcr7;
pub use Pcr0 as Pcr8;
pub use Pcr0 as Pcr9;
pub use Pcr0 as Pcr10;
pub use Pcr0 as Pcr11;
pub use Pcr0 as Pcr12;
pub use Pcr0 as Pcr13;
pub use Pcr0 as Pcr14;
pub use Pcr0 as Pcr15;
pub use Pcr0 as Pcr16;
pub use Pcr0 as Pcr17;
pub use Pcr0 as Pcr18;
pub use Pcr0 as Pcr19;
pub use Pcr0 as Pcr20;
pub use Pcr0 as Pcr21;
pub use Pcr0 as Pcr22;
pub use Pcr0 as Pcr23;
pub use Pcr0 as Pcr24;
pub use Pcr0 as Pcr25;
pub use Pcr0 as Pcr26;
pub use Pcr0 as Pcr27;
pub use Pcr0 as Pcr28;
pub use Pcr0 as Pcr29;
pub use Pcr0 as Pcr30;
pub use Pcr0 as Pcr31;
