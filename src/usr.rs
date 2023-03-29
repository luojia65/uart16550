use crate::{Register, USR};

/// 串口控制设置。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UartStatus(u8);

impl<R: Register> USR<R> {
    /// 写入线控制设置。
    #[inline]
    pub fn write(&self, val: UartStatus) {
        unsafe { self.0.get().write_volatile(R::from(val.0)) }
    }

    /// 读取线控制设置。
    #[inline]
    pub fn read(&self) -> UartStatus {
        UartStatus(unsafe { self.0.get().read_volatile() }.val())
    }
}

impl UartStatus {
    const RFF: u8 = 1 << 4;
    const RFNE: u8 = 1 << 3;
    const TFE: u8 = 1 << 2;
    const TFNF: u8 = 1 << 1;
    const BUSY: u8 = 1 << 0;

    /// 接收队列是否为满。
    #[inline]
    pub const fn receive_fifo_full(self) -> bool {
        self.0 & Self::RFF != 0
    }

    /// 接收队列是否非空。
    #[inline]
    pub const fn receive_fifo_not_empty(self) -> bool {
        self.0 & Self::RFNE != 0
    }

    /// 发送队列是否为空。
    #[inline]
    pub const fn transmit_fifo_empty(self) -> bool {
        self.0 & Self::TFE != 0
    }

    /// 发送队列是否未满。
    #[inline]
    pub const fn transmit_fifo_not_full(self) -> bool {
        self.0 & Self::TFNF != 0
    }

    /// 线路是否忙碌。
    #[inline]
    pub const fn busy(self) -> bool {
        self.0 & Self::BUSY != 0
    }
}
