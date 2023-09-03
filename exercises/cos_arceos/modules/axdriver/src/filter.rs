use cfg_if::cfg_if;
pub struct NetFilter<T> {
    pub inner: T,
}

cfg_if! {
    if #[cfg(net_dev = "virtio-net")] {
        use alloc::{sync::Arc, vec::Vec};
        use driver_net::{EthernetAddress, NetBuf, NetBufBox, NetBufPool, NetBufPtr, NetDriverOps};
        use driver_common::{BaseDriverOps,DevResult,DevError,DeviceType};

        extern crate alloc;


        pub const QS:usize=64;
        unsafe impl<T> Send for NetFilter<T> {}
        unsafe impl<T> Sync for NetFilter<T> {}

        impl<T> BaseDriverOps for NetFilter<T>{
            fn device_name(&self) -> &str {
                "netfilter-net"
            }

            fn device_type(&self) -> DeviceType {
                DeviceType::Net
            }
        }

        impl<T: driver_net::NetDriverOps> NetDriverOps for NetFilter<T>{
            #[inline]
            fn mac_address(&self) -> EthernetAddress {
                self.inner.mac_address()
            }
            #[inline]
            fn can_transmit(&self) -> bool {
                self.inner.can_transmit()
            }
            #[inline]
            fn can_receive(&self) -> bool {
                self.inner.can_receive()
            }
            #[inline]
            fn rx_queue_size(&self) -> usize {
                self.inner.rx_queue_size()
            }
            #[inline]
            fn tx_queue_size(&self) -> usize {
                self.inner.tx_queue_size()
            }
            fn recycle_rx_buffer(&mut self, rx_buf: NetBufPtr) -> DevResult {
                self.inner.recycle_rx_buffer(rx_buf)
            }
            fn recycle_tx_buffers(&mut self) -> DevResult {
                self.inner.recycle_tx_buffers()
            }
            fn transmit(&mut self, tx_buf: NetBufPtr) -> DevResult {
                log::warn!("Filter: transmit len[]");
                self.inner.transmit(tx_buf)
            }
            fn receive(&mut self) -> DevResult<NetBufPtr> {
                log::warn!("Filter: receive len[]");
                self.inner.receive()
            }
            fn alloc_tx_buffer(&mut self, size: usize) -> DevResult<NetBufPtr> {
                self.inner.alloc_tx_buffer(size)
            }
        }
    }
}