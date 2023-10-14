use enum_dispatch::enum_dispatch;

use crate::packet::message::MessageContainer;
use crate::packet::packet_manager::PacketManager;
use crate::packet::wrapping_id::MessageId;
use crate::protocol::BitSerializable;

pub(crate) mod reliable;
pub(crate) mod unreliable;

/// A trait for sending messages to a channel.
/// A channel is a buffer over packets to be able to add ordering/reliability
#[enum_dispatch]
pub trait ChannelSend<P: BitSerializable> {
    /// Queues a message to be transmitted
    fn buffer_send(&mut self, message: MessageContainer<P>);

    /// Reads from the buffer of messages to send to prepare a list of Packets
    /// that can be sent over the network for this channel
    fn send_packet(&mut self, packet_manager: &mut PacketManager<P>);

    /// Collect the list of messages that need to be sent
    /// Either because they have never been sent, or because they need to be resent (for reliability)
    /// Needs to be called before [`ReliableSender::send_packet`]
    fn collect_messages_to_send(&mut self);

    /// Called when we receive acknowledgement that a Message has been received
    fn notify_message_delivered(&mut self, message_id: &MessageId);

    /// Returns true if there are messages in the buffer that are ready to be sent
    fn has_messages_to_send(&self) -> bool;
}

/// Enum dispatch lets us derive ChannelSend on each enum variant
#[enum_dispatch(ChannelSend<P>)]
pub enum ChannelSender<P: BitSerializable> {
    UnorderedUnreliable(unreliable::UnorderedUnreliableSender<P>),
    SequencedUnreliable(unreliable::SequencedUnreliableSender<P>),
    Reliable(reliable::ReliableSender<P>),
}