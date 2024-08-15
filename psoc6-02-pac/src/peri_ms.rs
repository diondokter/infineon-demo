#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ppu_pr: [PpuPr; 8],
    _reserved1: [u8; 0x0600],
    ppu_fx: [PpuFx; 230],
}
impl RegisterBlock {
    #[doc = "0x00..0x200 - Programmable protection structure pair"]
    #[inline(always)]
    pub const fn ppu_pr(&self, n: usize) -> &PpuPr {
        &self.ppu_pr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x200 - Programmable protection structure pair"]
    #[inline(always)]
    pub fn ppu_pr_iter(&self) -> impl Iterator<Item = &PpuPr> {
        self.ppu_pr.iter()
    }
    #[doc = "0x800..0x4180 - Fixed protection structure pair"]
    #[inline(always)]
    pub const fn ppu_fx(&self, n: usize) -> &PpuFx {
        &self.ppu_fx[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x4180 - Fixed protection structure pair"]
    #[inline(always)]
    pub fn ppu_fx_iter(&self) -> impl Iterator<Item = &PpuFx> {
        self.ppu_fx.iter()
    }
}
#[doc = "Programmable protection structure pair"]
pub use self::ppu_pr::PpuPr;
#[doc = r"Cluster"]
#[doc = "Programmable protection structure pair"]
pub mod ppu_pr;
#[doc = "Fixed protection structure pair"]
pub use self::ppu_fx::PpuFx;
#[doc = r"Cluster"]
#[doc = "Fixed protection structure pair"]
pub mod ppu_fx;
