use std::{
    str
};

use mail_test_account::test_account_info;

use failure::Error;
use futures::{
    Future,
    sync::oneshot
};
use mail::{
    MailType,
    Mail,
    IRI,
    Context,
    smtp::{
        self,
        ConnectionConfig,
        auth::Plain,
        misc::{SetupTls, Cmd}
    }
};


use self::templates::{
    HelloWorldData
};

mod context;
mod templates;


fn run() -> Result<(), Error> {
    let ctx = &context::partial_random_context();

    println!("get test account");
    let info = test_account_info()?;

    println!("used test account: {:?}", &info.account);
    let auth = Plain::from_username(info.account.username, info.account.password)?;

    let smtp = info.smtp.unwrap();
    println!("used smtp setup: {:?}", &smtp);
    let host = smtp.host.parse().unwrap();
    let port = smtp.port;

    let con_config = ConnectionConfig
        ::builder_with_port(host, port)
        .map(|builder| if smtp.use_tls_directly {
            builder.use_direct_tls()
        } else {
            builder.use_start_tls()
        })
        .map(|builder| builder.auth(auth))
        .map(|builder| builder.build())?;

    println!("============================================");
    println!("now running hello-world example");
    println!("--------------------------------------------");
    run_hello_world_example(con_config.clone(), ctx)?;

    println!("============================================");
    println!("now running avatar example");
    println!("--------------------------------------------");
    run_avatar_example(con_config, ctx)?;

    Ok(())
}


fn run_hello_world_example<A, S>(
    con_config: ConnectionConfig<A, S>,
    ctx: &impl Context,
) -> Result<(), Error>
    where A: Cmd, S: SetupTls
{
    let template = templates::HelloWorldTemplate::load(ctx)?;
    let mail = template.create_mail(
        "from@example.com",
        "to@example.com",
        HelloWorldData {
            name: "Lucy".to_owned(),
            target: "Tom".to_owned()
        },
        ctx
    )?;

    let encodable = mail.into_encodable_mail(ctx.clone()).wait()?;
    let encoded = encodable.encode_into_bytes(MailType::Ascii)?;

    let mail_str = str::from_utf8(&encoded).unwrap();
    println!("--------------------------------------------");
    println!("{}", mail_str);
    println!("--------------------------------------------");

    // Turn mail back into a normal way, the
    // into_encodable is normally done by functions
    // like `smtp::send`.
    let mail: Mail = encodable.into();

    let fut = smtp::send(mail.into(), con_config, ctx.clone());
    run_with_tokio(fut)?;

    Ok(())
}


fn run_avatar_example<A, S>(
    con_config: ConnectionConfig<A, S>,
    ctx: &impl Context,
) -> Result<(), Error>
    where A: Cmd, S: SetupTls
{
    let template = templates::AvatarTemplate::load(ctx)?;
    let avatar = IRI::from_parts("path", "./templates/hello_world/logo.png").unwrap();

    let mail = template.create_mail(
        "from@example.com",
        "to@example.com",
        avatar,
        ctx
    )?;

    let fut = smtp::send(mail.into(), con_config, ctx.clone());
    run_with_tokio(fut)?;

    Ok(())
}

fn run_with_tokio<I, E>(f: impl Future<Item=I, Error=E> + Send + 'static) -> Result<I, E>
    where I: Send + 'static, E: Send + 'static
{
    // Tokio somehow doesn't make it easy to just run a future _and_ return it's result,
    // well I guess that isn't what it is designed for and only makes sense if you do
    // have this kind of small mostly sync examples... (In a server setup you should
    // have a tokio core running "somewhere" and then spawn the mail future there which
    // most likely would require a (oneshot) channel for sending back the response anyway).
    let (send, mut recv) = oneshot::channel();
    tokio::run(f.then(|inner_res| {
        let _ = send.send(inner_res);
        Ok(())
    }));

    recv.try_recv()
        .expect("[BUG]somehow send mail future wasn't run to completion/error but aborted")
        .expect("[BUG]somehow send mail future wasn't run to completion/error when `tokio::run` returned")
}



fn main() {

    let err =
        match run() {
            Ok(()) => { return; },
            Err(err) => err
        };

    for err in err.iter_chain() {
        eprintln!("Err: {}", err);
        eprintln!("Debug: {:?}", err);
        if let Some(backtrace) = err.backtrace() {
            eprintln!("Backtrace: {}", backtrace);
        } else {
            eprintln!("Backtrace: None");
        }
    }

}
