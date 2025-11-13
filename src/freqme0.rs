#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_ctrl: [u8; 0x04],
    ctrlstat: Ctrlstat,
    min: Min,
    max: Max,
}
impl RegisterBlock {
    #[doc = "0x00 - Control (in Write mode)"]
    #[inline(always)]
    pub const fn ctrl_w(&self) -> &CtrlW {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Control (in Read mode)"]
    #[inline(always)]
    pub const fn ctrl_r(&self) -> &CtrlR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - Control Status"]
    #[inline(always)]
    pub const fn ctrlstat(&self) -> &Ctrlstat {
        &self.ctrlstat
    }
    #[doc = "0x08 - Minimum"]
    #[inline(always)]
    pub const fn min(&self) -> &Min {
        &self.min
    }
    #[doc = "0x0c - Maximum"]
    #[inline(always)]
    pub const fn max(&self) -> &Max {
        &self.max
    }
}
#[doc = "CTRL_R (r) register accessor: Control (in Read mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_r`] module"]
#[doc(alias = "CTRL_R")]
pub type CtrlR = crate::Reg<ctrl_r::CtrlRSpec>;
#[doc = "Control (in Read mode)"]
pub mod ctrl_r;
#[doc = "CTRL_W (w) register accessor: Control (in Write mode)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_w::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_w`] module"]
#[doc(alias = "CTRL_W")]
pub type CtrlW = crate::Reg<ctrl_w::CtrlWSpec>;
#[doc = "Control (in Write mode)"]
pub mod ctrl_w;
#[doc = "CTRLSTAT (rw) register accessor: Control Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlstat`] module"]
#[doc(alias = "CTRLSTAT")]
pub type Ctrlstat = crate::Reg<ctrlstat::CtrlstatSpec>;
#[doc = "Control Status"]
pub mod ctrlstat;
#[doc = "MIN (rw) register accessor: Minimum\n\nYou can [`read`](crate::Reg::read) this register and get [`min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@min`] module"]
#[doc(alias = "MIN")]
pub type Min = crate::Reg<min::MinSpec>;
#[doc = "Minimum"]
pub mod min;
#[doc = "MAX (rw) register accessor: Maximum\n\nYou can [`read`](crate::Reg::read) this register and get [`max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max`] module"]
#[doc(alias = "MAX")]
pub type Max = crate::Reg<max::MaxSpec>;
#[doc = "Maximum"]
pub mod max;
