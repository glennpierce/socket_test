use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::rc::Rc;

use byteorder::{ByteOrder, BigEndian};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use bincode::SizeLimit::Infinite;
use bincode::serde::{DeserializeError, DeserializeResult};
use bincode;
use sensor::{SensorValueArray, SensorValue};

use mio::*;
use mio::tcp::*;

/// A stateful wrapper around a non-blocking stream. This connection is not
/// the SERVER connection. This connection represents the client connections
/// _accepted_ by the SERVER connection.
pub struct Connection {
    // handle to the accepted socket
    sock: TcpStream,

    // token used to register with the poller
    pub token: Token,

    // set of events we are interested in
    interest: Ready,

    // track whether a connection needs to be (re)registered
    is_idle: bool,

    // track whether a connection is reset
    is_reset: bool,
}

impl Connection {
    pub fn new(sock: TcpStream, token: Token) -> Connection {
        Connection {
            sock: sock,
            token: token,
            interest: Ready::hup(),
            is_idle: true,
            is_reset: false,
        }
    }

    /// Handle read event from poller.
    ///
    /// The Handler must continue calling until None is returned.
    ///
    /// The recieve buffer is sent back to `Server` so the message can be broadcast to all
    /// listening connections.
    pub fn readable(&mut self) -> DeserializeResult<Option<Vec<u8>>> {

        // UFCS: resolve "multiple applicable items in scope [E0034]" error
       let sock_ref = <TcpStream as Read>::by_ref(&mut self.sock);

        match bincode::serde::deserialize_from(sock_ref, Infinite) {
        
            Ok(n) => {
                println!("CONN : we read {:#?} bytes", n);

                Ok(Some(n))
            }
            Err(DeserializeError::IoError(e)) => {

                if e.kind() == ErrorKind::WouldBlock {
                    error!("CONN : read encountered WouldBlock");
                    Ok(None)
                } else {
                    error!("Failed to read buffer for token {:?}, error: {}", self.token, e);
                    Err(DeserializeError::IoError(e))
                }
            }
            Err(err) => {
                error!("unhandled error: {}", err);
                Err(err)
            } 
        }

    }

    /// Register interest in read events with poll.
    ///
    /// This will let our connection accept reads starting next poller tick.
    pub fn register(&mut self, poll: &mut Poll) -> io::Result<()> {
        trace!("connection register; token={:?}", self.token);

        self.interest.insert(Ready::readable());

        poll.register(
            &self.sock,
            self.token,
            self.interest,
            PollOpt::edge() | PollOpt::oneshot()
        ).and_then(|(),| {
            self.is_idle = false;
            Ok(())
        }).or_else(|e| {
            error!("Failed to reregister {:?}, {:?}", self.token, e);
            Err(e)
        })
    }

    /// Re-register interest in read events with poll.
    pub fn reregister(&mut self, poll: &mut Poll) -> io::Result<()> {
        trace!("connection reregister; token={:?}", self.token);

        poll.reregister(
            &self.sock,
            self.token,
            self.interest,
            PollOpt::edge() | PollOpt::oneshot()
        ).and_then(|(),| {
            self.is_idle = false;
            Ok(())
        }).or_else(|e| {
            error!("Failed to reregister {:?}, {:?}", self.token, e);
            Err(e)
        })
    }

    pub fn mark_reset(&mut self) {
        trace!("connection mark_reset; token={:?}", self.token);

        self.is_reset = true;
    }

    #[inline]
    pub fn is_reset(&self) -> bool {
        self.is_reset
    }

    pub fn mark_idle(&mut self) {
        trace!("connection mark_idle; token={:?}", self.token);

        self.is_idle = true;
    }

    #[inline]
    pub fn is_idle(&self) -> bool {
        self.is_idle
    }
}
