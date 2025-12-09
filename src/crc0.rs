#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_data: [u8; 0x04],
    _reserved_1_gpoly: [u8; 0x04],
    ctrl: Ctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC data register, 8-bit access"]
    #[inline(always)]
    pub const fn data8(&self) -> &Data8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - CRC data register, 16-bit access"]
    #[inline(always)]
    pub const fn data16(&self) -> &Data16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - CRC data register, 32-bit access"]
    #[inline(always)]
    pub const fn data32(&self) -> &Data32 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Data"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - Polynomial, 16-bit access"]
    #[inline(always)]
    pub const fn gpoly16(&self) -> &Gpoly16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Polynomial, 32-bit access"]
    #[inline(always)]
    pub const fn gpoly32(&self) -> &Gpoly32 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Polynomial"]
    #[inline(always)]
    pub const fn gpoly(&self) -> &Gpoly {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
}
#[doc = "DATA (rw) register accessor: Data\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Data"]
pub mod data;
#[doc = "DATA32 (rw) register accessor: CRC data register, 32-bit access\n\nYou can [`read`](crate::Reg::read) this register and get [`data32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data32`] module"]
#[doc(alias = "DATA32")]
pub type Data32 = crate::Reg<data32::Data32Spec>;
#[doc = "CRC data register, 32-bit access"]
pub mod data32;
#[doc = "DATA16 (rw) register accessor: CRC data register, 16-bit access\n\nYou can [`read`](crate::Reg::read) this register and get [`data16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data16`] module"]
#[doc(alias = "DATA16")]
pub type Data16 = crate::Reg<data16::Data16Spec>;
#[doc = "CRC data register, 16-bit access"]
pub mod data16;
#[doc = "DATA8 (rw) register accessor: CRC data register, 8-bit access\n\nYou can [`read`](crate::Reg::read) this register and get [`data8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data8`] module"]
#[doc(alias = "DATA8")]
pub type Data8 = crate::Reg<data8::Data8Spec>;
#[doc = "CRC data register, 8-bit access"]
pub mod data8;
#[doc = "GPOLY (rw) register accessor: Polynomial\n\nYou can [`read`](crate::Reg::read) this register and get [`gpoly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpoly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpoly`] module"]
#[doc(alias = "GPOLY")]
pub type Gpoly = crate::Reg<gpoly::GpolySpec>;
#[doc = "Polynomial"]
pub mod gpoly;
#[doc = "GPOLY32 (rw) register accessor: Polynomial, 32-bit access\n\nYou can [`read`](crate::Reg::read) this register and get [`gpoly32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpoly32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpoly32`] module"]
#[doc(alias = "GPOLY32")]
pub type Gpoly32 = crate::Reg<gpoly32::Gpoly32Spec>;
#[doc = "Polynomial, 32-bit access"]
pub mod gpoly32;
#[doc = "GPOLY16 (rw) register accessor: Polynomial, 16-bit access\n\nYou can [`read`](crate::Reg::read) this register and get [`gpoly16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpoly16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpoly16`] module"]
#[doc(alias = "GPOLY16")]
pub type Gpoly16 = crate::Reg<gpoly16::Gpoly16Spec>;
#[doc = "Polynomial, 16-bit access"]
pub mod gpoly16;
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control"]
pub mod ctrl;
