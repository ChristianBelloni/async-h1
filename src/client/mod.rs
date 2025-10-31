//! Process HTTP connections on the client.

use futures::{
    io::{self, AsyncRead as Read, AsyncWrite as Write},
    AsyncBufRead,
};
use http_types::{Request, Response};

mod decode;
mod encode;

pub use decode::decode;
pub use encode::Encoder;

use crate::{
    chunked::ChunkedDecoder,
    client::decode::{decode_custom_buf_read, decode_inner},
};

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
pub async fn connect_custom_buf_reader<RW, B, K, F>(
    mut stream: RW,
    req: Request,
    buf_read_fn: F,
) -> http_types::Result<Response>
where
    RW: Read + Write + Send + Sync + Unpin + 'static,
    B: AsyncBufRead + Send + Sync + 'static + Unpin,
    F: Fn(RW) -> B,
    F: Fn(ChunkedDecoder<B>) -> K,
    K: Send + Sync + 'static + Unpin + AsyncBufRead,
{
    let mut req = Encoder::new(req);
    log::trace!("> {:?}", &req);

    io::copy(&mut req, &mut stream).await?;

    let res = decode_custom_buf_read(stream, buf_read_fn).await?;
    log::trace!("< {:?}", &res);

    Ok(res)
}

/// Opens an HTTP/1.1 connection to a remote host.
pub async fn connect2<'a, RW>(
    mut stream: RW,
    req: Request,
) -> http_types::Result<(
    Response,
    futures::future::Either<
        futures::future::Either<
            io::BufReader<crate::chunked::ChunkedDecoder<io::BufReader<RW>>>,
            io::Take<io::BufReader<RW>>,
        >,
        io::Empty,
    >,
)>
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
