use crate::{contexts::WriteContext, space::CryptoStream, transmission};
use core::ops::RangeInclusive;
use s2n_quic_core::{frame::crypto::CryptoRef, packet::number::PacketNumberSpace};

pub struct Payload<'a> {
    pub crypto_stream: &'a mut CryptoStream,
    pub packet_number_space: PacketNumberSpace,
}

/// Rather than creating a packet with a very small CRYPTO frame (under 16 bytes), it would be
/// better to wait for another transmission and send something larger. This should be better for
/// performance, anyway, since you end up paying for encryption/decryption.
const MIN_SIZE: usize = CryptoRef::get_max_frame_size(16);

impl<'a> super::Payload for Payload<'a> {
    fn size_hint(&self, range: RangeInclusive<usize>) -> usize {
        (*range.start()).max(MIN_SIZE)
    }

    fn on_transmit<W: WriteContext>(&mut self, context: &mut W) {
        let _ = self.crypto_stream.tx.on_transmit((), context);

        // TODO add required padding
        // https://github.com/awslabs/s2n-quic/issues/179

        //= https://tools.ietf.org/id/draft-ietf-quic-tls-32.txt#4.9
        //# These packets MAY also include PADDING frames.
    }

    fn packet_number_space(&self) -> PacketNumberSpace {
        self.packet_number_space
    }
}

impl<'a> transmission::interest::Provider for Payload<'a> {
    fn transmission_interest(&self) -> transmission::Interest {
        transmission::Interest::default() + self.crypto_stream.transmission_interest()
    }
}