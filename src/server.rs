use std::io::{self, ErrorKind};
use std::rc::Rc;

use mio::*;
use mio::tcp::*;
use slab;

use connection::Connection;

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use bincode::SizeLimit::Infinite;
use bincode::serde::{DeserializeError, DeserializeResult};

use sensor::{SensorValueArray, SensorValue, TestStruct};


type Slab<T> = slab::Slab<T, Token>;

pub struct TcpServer {
    // main socket for our Tcpserver
    sock: TcpListener,

    // token of our Tcpserver. we keep track of it here instead of doing `const TcpSERVER = Token(0)`.
    token: Token,

    // a list of connections _accepted_ by our Tcpserver
    conns: Slab<Connection>,

    // a list of events to process
    events: Events,
}

impl TcpServer {
    pub fn new(sock: TcpListener) -> TcpServer {
        TcpServer {
            sock: sock,

            // Give our Tcpserver token a number much larger than our slab capacity. The slab used to
            // track an internal offset, but does not anymore.
            token: Token(10_000_000),

            // TcpSERVER is Token(1), so start after that
            // we can deal with a max of 126 connections
            conns: Slab::with_capacity(512),

            // list of events from the poller that the Tcpserver needs to process
            events: Events::with_capacity(1024),
        }
    }

    pub fn run(&mut self, poll: &mut Poll) -> io::Result<()> {

        try!(self.register(poll));

        info!("TcpServer run loop starting...");
        loop {
            let cnt = try!(poll.poll(&mut self.events, None));

            let mut i = 0;

            trace!("processing events... cnt={}; len={}",
                   cnt,
                   self.events.len());

            // Iterate over the notifications. Each event provides the token
            // it was registered with (which usually represents, at least, the
            // handle that the event is about) as well as information about
            // what kind of event occurred (readable, writable, signal, etc.)
            while i < cnt {
                // TODO this would be nice if it would turn a Result type. trying to convert this
                // into a io::Result runs into a problem because .ok_or() expects std::Result and
                // not io::Result
                let event = self.events.get(i).expect("Failed to get event");

                println!("event={:?}; idx={:?}", event, i);
                self.ready(poll, event.token(), event.kind());

                i += 1;
            }

            self.tick(poll);
        }
    }

    /// Register TcpServer with the poller.
    ///
    /// This keeps the registration details neatly tucked away inside of our implementation.
    pub fn register(&mut self, poll: &mut Poll) -> io::Result<()> {
        poll.register(&self.sock, self.token, Ready::readable(), PollOpt::edge())
            .or_else(|e| {
                error!("Failed to register Tcpserver {:?}, {:?}", self.token, e);
                Err(e)
            })
    }

    fn tick(&mut self, poll: &mut Poll) {
        trace!("Handling end of tick");

        let mut reset_tokens = Vec::new();

        for c in self.conns.iter_mut() {
            if c.is_reset() {
                reset_tokens.push(c.token);
            } else if c.is_idle() {
                c.reregister(poll)
                    .unwrap_or_else(|e| {
                        warn!("Reregister failed {:?}", e);
                        c.mark_reset();
                        reset_tokens.push(c.token);
                    });
            }
        }

        for token in reset_tokens {
            match self.conns.remove(token) {
                Some(_c) => {
                    debug!("reset connection; token={:?}", token);
                }
                None => {
                    warn!("Unable to remove connection for {:?}", token);
                }
            }
        }
    }

    fn ready(&mut self, poll: &mut Poll, token: Token, event: Ready) {
        debug!("{:?} event = {:?}", token, event);

        if event.is_error() {
            println!("Error event for {:?}", token);
            self.find_connection_by_token(token).mark_reset();
            return;
        }

        if event.is_hup() {
            trace!("Hup event for {:?}", token);
            self.find_connection_by_token(token).mark_reset();
            return;
        }

        // We never expect a write event for our `TcpServer` token . A write event for any other token
        // should be handed off to that connection.
        // if event.is_writable() {
        //     trace!("Write event for {:?}", token);
        //     assert!(self.token != token,
        //             "Received writable event for TcpServer");

        //     let conn = self.find_connection_by_token(token);

        //     if conn.is_reset() {
        //         info!("{:?} has already been reset", token);
        //         return;
        //     }

        //     conn.writable()
        //         .unwrap_or_else(|e| {
        //             warn!("Write event failed for {:?}, {:?}", token, e);
        //             conn.mark_reset();
        //         });
        // }

        // A read event for our `TcpServer` token means we are establishing a new connection. A read
        // event for any other token should be handed off to that connection.
        if event.is_readable() {
            trace!("Read event for {:?}", token);
            if self.token == token {
                self.accept(poll);
            } else {

                if self.find_connection_by_token(token).is_reset() {
                    info!("{:?} has already been reset", token);
                    return;
                }

                self.readable(token);

                // self.readable(token)
                //     .unwrap_or_else(|e| {
                //         warn!("Read event failed for {:?}: {:?}", token, e);
                //         println!("Read event failed for {:?}: {:?}", token, e);
                //         self.find_connection_by_token(token).mark_reset();
                //     });
            }
        }

        if self.token != token {
            self.find_connection_by_token(token).mark_idle();
        }
    }

    /// Accept a _new_ client connection.
    ///
    /// The Tcpserver will keep track of the new connection and forward any events from the poller
    /// to this connection.
    fn accept(&mut self, poll: &mut Poll) {
        debug!("Tcpserver accepting new socket");

        loop {
            // Log an error if there is no socket, but otherwise move on so we do not tear down the
            // entire Tcpserver.
            let sock = match self.sock.accept() {
                Ok((sock, _)) => sock,
                Err(e) => {
                    if e.kind() == ErrorKind::WouldBlock {
                        println!("accept encountered WouldBlock");
                    } else {
                        println!("Failed to accept new socket, {:?}", e);
                    }
                    return;
                }
            };

            let token = match self.conns.vacant_entry() {
                Some(entry) => {
                    debug!("registering {:?} with poller", entry.index());
                    let c = Connection::new(sock, entry.index());
                    entry.insert(c).index()
                }
                None => {
                    error!("Failed to insert connection into slab");
                    return;
                }
            };

            match self.find_connection_by_token(token).register(poll) {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to register {:?} connection with poller, {:?}",
                           token,
                           e);
                    self.conns.remove(token);
                }
            }
        }
    }

    /// Forward a readable event to an established connection.
    ///
    /// Connections are identified by the token provided to us from the poller. Once a read has
    /// finished, push the receive buffer into the all the existing connections so we can
    /// broadcast.
    fn readable(&mut self, token: Token) {
        debug!("Tcpserver conn readable; token={:?}", token);

        let c = self.find_connection_by_token(token);
        //try!(c.readable());
        
        match c.readable() {
            Some(message) => {
                println!("HERE {:?}", message);
                //Some(message)
            }
            None => println!("No gift? Oh well."),
        }

        // while let Some(message) = try!(c.readable()) {

        // //     println!("HERE {:?}", message);
        // }

        // println!("HARLEY ");
        //None
    }

    /// Find a connection in the slab using the given token.
    fn find_connection_by_token(&mut self, token: Token) -> &mut Connection {
        &mut self.conns[token]
    }
}
