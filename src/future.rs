use crate::ThinBox;
use docfg::docfg;
use std::{alloc::Allocator, future::Future, pin::Pin, ops::DerefMut};

impl<T: ?Sized + Future + Unpin, A: 'static + Allocator> Future for ThinBox<T, A> {
    type Output = T::Output;

    #[inline]
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        T::poll(Pin::new(&mut *self), cx)
    }
}

#[docfg(feature = "futures")]
impl<T: ?Sized + futures::Stream + Unpin, A: 'static + Allocator> futures::Stream
    for ThinBox<T, A>
{
    type Item = T::Item;

    #[inline]
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        T::poll_next(Pin::new(&mut *self), cx)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        T::size_hint(self)
    }
}

#[docfg(feature = "futures")]
impl<Item, T: ?Sized + futures::Sink<Item> + Unpin, A: 'static + Allocator> futures::Sink<Item>
    for ThinBox<T, A>
{
    type Error = T::Error;

    #[inline]
    fn poll_ready(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        T::poll_ready(Pin::new(&mut *self), cx)
    }

    #[inline]
    fn start_send(mut self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        T::start_send(Pin::new(&mut *self), item)
    }

    #[inline]
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        T::poll_flush(Pin::new(&mut *self), cx)
    }
    
    #[inline]
    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        T::poll_close(Pin::new(&mut *self), cx)
    }
}

#[docfg(feature = "futures")]
impl<T: ?Sized + futures::AsyncRead + Unpin, A: 'static + Allocator> futures::AsyncRead
    for ThinBox<T, A>
{
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        T::poll_read(Pin::new(&mut *self), cx, buf)
    }
}

#[docfg(feature = "futures")]
impl<T: ?Sized + futures::AsyncWrite + Unpin, A: 'static + Allocator> futures::AsyncWrite
    for ThinBox<T, A>
{
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        T::poll_write(Pin::new(&mut *self), cx, buf)
    }

    #[inline]
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        T::poll_flush(Pin::new(&mut *self), cx)
    }

    #[inline]
    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        T::poll_close(Pin::new(&mut *self), cx)
    }
}

#[docfg(feature = "futures")]
impl<T: ?Sized + futures::AsyncBufRead + Unpin, A: 'static + Allocator> futures::AsyncBufRead
    for ThinBox<T, A>
{
    #[inline]
    fn poll_fill_buf<'a> (self: Pin<&'a mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<std::io::Result<&'a [u8]>> {
        let pin: Pin<&'a mut T> = Pin::new(Pin::get_mut(self).deref_mut());
        T::poll_fill_buf(pin, cx)
    }

    #[inline]
    fn consume(mut self: Pin<&mut Self>, amt: usize) {
        T::consume(Pin::new(&mut *self), amt)
    }
}