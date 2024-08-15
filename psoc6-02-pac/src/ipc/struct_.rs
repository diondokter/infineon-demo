#[repr(C)]
#[doc = "IPC structure"]
#[doc(alias = "STRUCT")]
pub struct Struct {
    acquire: Acquire,
    release: Release,
    notify: Notify,
    data0: Data0,
    data1: Data1,
    _reserved5: [u8; 0x08],
    lock_status: LockStatus,
}
impl Struct {
    #[doc = "0x00 - IPC acquire"]
    #[inline(always)]
    pub const fn acquire(&self) -> &Acquire {
        &self.acquire
    }
    #[doc = "0x04 - IPC release"]
    #[inline(always)]
    pub const fn release(&self) -> &Release {
        &self.release
    }
    #[doc = "0x08 - IPC notification"]
    #[inline(always)]
    pub const fn notify(&self) -> &Notify {
        &self.notify
    }
    #[doc = "0x0c - IPC data 0"]
    #[inline(always)]
    pub const fn data0(&self) -> &Data0 {
        &self.data0
    }
    #[doc = "0x10 - IPC data 1"]
    #[inline(always)]
    pub const fn data1(&self) -> &Data1 {
        &self.data1
    }
    #[doc = "0x1c - IPC lock status"]
    #[inline(always)]
    pub const fn lock_status(&self) -> &LockStatus {
        &self.lock_status
    }
}
#[doc = "ACQUIRE (r) register accessor: IPC acquire\n\nYou can [`read`](crate::Reg::read) this register and get [`acquire::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acquire`]
module"]
#[doc(alias = "ACQUIRE")]
pub type Acquire = crate::Reg<acquire::AcquireSpec>;
#[doc = "IPC acquire"]
pub mod acquire;
#[doc = "RELEASE (w) register accessor: IPC release\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`release::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@release`]
module"]
#[doc(alias = "RELEASE")]
pub type Release = crate::Reg<release::ReleaseSpec>;
#[doc = "IPC release"]
pub mod release;
#[doc = "NOTIFY (w) register accessor: IPC notification\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`notify::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@notify`]
module"]
#[doc(alias = "NOTIFY")]
pub type Notify = crate::Reg<notify::NotifySpec>;
#[doc = "IPC notification"]
pub mod notify;
#[doc = "DATA0 (rw) register accessor: IPC data 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0`]
module"]
#[doc(alias = "DATA0")]
pub type Data0 = crate::Reg<data0::Data0Spec>;
#[doc = "IPC data 0"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: IPC data 1\n\nYou can [`read`](crate::Reg::read) this register and get [`data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1`]
module"]
#[doc(alias = "DATA1")]
pub type Data1 = crate::Reg<data1::Data1Spec>;
#[doc = "IPC data 1"]
pub mod data1;
#[doc = "LOCK_STATUS (r) register accessor: IPC lock status\n\nYou can [`read`](crate::Reg::read) this register and get [`lock_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock_status`]
module"]
#[doc(alias = "LOCK_STATUS")]
pub type LockStatus = crate::Reg<lock_status::LockStatusSpec>;
#[doc = "IPC lock status"]
pub mod lock_status;
