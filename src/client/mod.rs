//! Process HTTP connections on the client.

use futures::io::{self, AsyncRead as Read, AsyncWrite as Write};
use http_types::{Request, Response};

mod decode;
mod encode;

pub use decode::decode;
pub use encode::Encoder;

use crate::client::decode::decode_inner;

/// Opens an HTTP/1.1 connection to a remote host.
pub async fn connect<RW>(mut stream: RW, req: Request) -> http_types::Result<Response>
where
    RW: Read + Write + Send + Sync + Unpin + 'static,
{
    let mut req = Encoder::new(req);
    log::trace!("> {:?}", &req);

    io::copy(&mut req, &mut stream).await?;

    let res = decode(stream).await?;
    log::trace!("< {:?}", &res);

    Ok(res)
}

/// Opens an HTTP/1.1 connection to a remote host.
pub async fn connect2<'a, RW>(
    mut stream: RW,
    req: Request,
) -> http_types::Result<(Response, impl Read + 'a)>
where
    RW: Read + Write + Send + Sync + Unpin + 'a,
{
    let mut req = Encoder::new(req);
    log::trace!("> {:?}", &req);

    io::copy(&mut req, &mut stream).await?;

    let res = decode_inner(stream).await?;
    log::trace!("< {:?}", &res.0);

    Ok(res)
}
