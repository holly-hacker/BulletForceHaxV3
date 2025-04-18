use std::borrow::Cow;
use std::fmt::Debug;
use std::time::{Duration, Instant};

use photon_lib::pun::lifting::{PunOperationRequest, RaiseEventParsed};
use photon_lib::{
    WriteError,
    photon::message::{OperationResponse, PhotonMessage},
    pun::{constants::internal_operation_code, lifting::PingRequest},
};
use strum::IntoDiscriminant;
use tracing::{debug, trace};

use crate::{errors::HandlerError, utils::to_internal_operation_request};

pub mod game;
pub mod lobby;

const PING_INTERVAL: Duration = Duration::from_secs(1);

pub trait ClientImpl: Default {
    type Settings;
    type State: Default + IntoDiscriminant;

    fn get_url(&self, ctx: &ClientContext<Self>) -> Cow<'static, str>;

    /// Handle incoming websocket messages
    fn handle_incoming_packet(
        &mut self,
        ctx: &mut ClientContext<Self>,
        packet: PhotonMessage,
    ) -> Result<(), HandlerError>;
}

pub struct ClientContext<T: ClientImpl> {
    pub state: T::State,
    pub settings: T::Settings,

    queued_messages: Vec<Vec<u8>>,
}

impl<T: ClientImpl> ClientContext<T> {
    pub fn enqueue_sent_message(&mut self, message: PhotonMessage) -> Result<(), WriteError> {
        let mut buf = vec![];
        message.to_websocket_bytes(&mut buf)?;
        self.queued_messages.push(buf);
        Ok(())
    }

    pub fn set_new_state(&mut self, new_state: T::State)
    where
        <<T as ClientImpl>::State as IntoDiscriminant>::Discriminant: Debug,
    {
        debug!(
            old_state = format!("{:?}", self.state.discriminant()),
            new_state = format!("{:?}", new_state.discriminant()),
            "State changed",
        );
        self.state = new_state;
    }
}

/// Represents a client that can handle connections to/from the server.
pub struct Client<T: ClientImpl> {
    /// The underlying client implementation
    client: T,

    /// Data that the underlying client implementation may need access to
    context: ClientContext<T>,

    /// When we last received a ping response. If this is `None`, we sent a ping request and are
    /// waiting for a response.
    last_ping_received: Option<Instant>,
}

impl<T: ClientImpl> Client<T> {
    pub fn create(settings: T::Settings) -> Self {
        Self {
            client: T::default(),
            context: ClientContext {
                state: T::State::default(),
                settings,
                queued_messages: vec![],
            },
            last_ping_received: Some(Instant::now()),
        }
    }

    /// Get the URL the socket should connect to.
    pub fn get_url(&self) -> Cow<'static, str> {
        self.client.get_url(&self.context)
    }

    /// Ingest a network packet
    pub fn handle_input(&mut self, mut data: &[u8]) -> Result<(), HandlerError> {
        let packet = PhotonMessage::from_websocket_bytes(&mut data)?;
        trace!("Received message: {packet:?}");

        // early exit for pongs
        if let PhotonMessage::InternalOperationResponse(OperationResponse {
            operation_code: internal_operation_code::PING,
            ..
        }) = &packet
        {
            // don't parse for now, but in the future we could
            self.last_ping_received = Some(Instant::now());
            return Ok(());
        }

        self.queue_ping_if_needed()?;

        self.client
            .handle_incoming_packet(&mut self.context, packet)
    }

    /// Get a message to send out through the websocket connection
    pub fn take_messages_to_send(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.context.queued_messages)
    }

    pub fn get_state(&self) -> &T::State {
        &self.context.state
    }

    pub fn raise_event(&mut self, event: RaiseEventParsed) -> Result<(), WriteError> {
        let request = event.into();
        let pun_op_req: PunOperationRequest = PunOperationRequest::RaiseEvent(Box::new(request));
        let op_req = pun_op_req.unparse();
        let message = PhotonMessage::OperationRequest(op_req);
        self.context.enqueue_sent_message(message)
    }

    pub fn queue_ping_if_needed(&mut self) -> Result<(), HandlerError> {
        let Some(last_received) = self.last_ping_received else {
            // a ping is already in-flight, waiting for a response
            return Ok(());
        };

        if last_received.elapsed() > PING_INTERVAL {
            trace!("Sending a new ping request");
            self.context
                .enqueue_sent_message(to_internal_operation_request(
                    internal_operation_code::PING,
                    PingRequest { client_time: 0 },
                ))?;

            self.last_ping_received = None;
        }

        Ok(())
    }
}
