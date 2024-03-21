use poise::serenity_prelude as serenity;
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Joins the call
#[poise::command(slash_command, prefix_command)]
async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.cache().guild(ctx.partial_guild().await.unwrap().id).unwrap();
    let voice_channel_id = guild_id
            .voice_states
            .get(&ctx.author().id)
            .and_then(|vs| vs.channel_id)
            .unwrap();

    let manager = songbird::get(poise::Context::serenity_context(ctx))
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();



    println!("Message sent in server: {:?}", voice_channel_id);
    Ok(())
}

// async fn join(ctx: &Context, msg: &Message) -> CommandResult {
//     let (guild_id, channel_id) = {
//         let guild = msg.guild(&ctx.cache).unwrap();
//         let channel_id = guild
//             .voice_states
//             .get(&msg.author.id)
//             .and_then(|voice_state| voice_state.channel_id);

//         (guild.id, channel_id)
//     };

    // let connect_to = match channel_id {
    //     Some(channel) => channel,
    //     None => {
    //         check_msg(msg.reply(ctx, "Not in a voice channel").await);

    //         return Ok(());
    //     },
    // };

//     let manager = songbird::get(ctx)
//         .await
//         .expect("Songbird Voice client placed in at initialisation.")
//         .clone();

//     if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
//         // Attach an event handler to see notifications of all track errors.
//         let mut handler = handler_lock.lock().await;
//         handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
//     }

//     Ok(())
// }

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), join()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
