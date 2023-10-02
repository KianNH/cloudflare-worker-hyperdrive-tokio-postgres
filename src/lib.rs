use std::str::FromStr;
use tokio_postgres::config::Config as PgConfig;
use tokio_postgres::tls as PgTls;
use worker::Error::RustError;
use worker::*;

mod hyperdrive;

#[event(start)]
fn start() {
    console_error_panic_hook::set_once();
}

#[event(fetch)]
async fn main(_req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let hyperdrive = env.get_binding::<hyperdrive::Hyperdrive>("HYPERDRIVE")?;
    let conn_string = hyperdrive.connection_string();

    let url = Url::parse(&conn_string)?;
    let hostname = url
        .host_str()
        .ok_or_else(|| RustError("unable to parse host from url".to_string()))?;

    let socket = Socket::builder().connect(hostname, 5432)?;

    let config = PgConfig::from_str(&conn_string)
        .map_err(|e| RustError(format!("tokio-postgres: {e:?}")))?;

    let (client, connection) = config
        .connect_raw(socket, PgTls::NoTls)
        .await
        .map_err(|e| RustError(format!("tokio-postgres: {e:?}")))?;

    wasm_bindgen_futures::spawn_local(async move {
        if let Err(error) = connection.await {
            console_log!("connection error: {:?}", error);
        }
    });

    let rows = client
        .query("SELECT * FROM playing_with_neon", &[])
        .await
        .map_err(|e| RustError(format!("tokio-postgres: {e:?}")))?;

    let row: &str = rows[0].get(1);

    Response::ok(format!("{row}"))
}
