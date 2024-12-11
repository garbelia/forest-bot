use crate::{Context, Error};
use giproc::Index;
mod giproc;

/// Show this help menu. Can also be used with !help
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom:
                "This is the initial release of the Greenness Index bot for the Forest discord.",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Calculate any nation's Greenness Index
#[poise::command(prefix_command)]
pub async fn greenindex(
    ctx: Context<'_>,
    #[description = "Calculate any nation's Greenness Index"] nation: String,
) -> Result<(), Error> {
    let gival = giproc::gindexcalc(nation, Index::Green).await?;
    println!("{:?}", gival);
    if gival.status == "200 OK" {
        let response = format!(
            "<:greenness:1316460167436435466> {}'s Greenness Index is {}",
            gival.fullname, gival.index
        );
        ctx.say(response).await?;
    } else if gival.status == "404 Not Found" {
        let response = ":bangbang: That nation does not exist!";
        ctx.say(response).await?;
    } else {
        let response = ":bangbang: An unexpected error has occurred connecting to the NationStates API. Try again in a minute!";
        ctx.say(response).await?;
    }
    Ok(())
}

/// Calculate any nation's Festivity Index
#[poise::command(prefix_command)]
pub async fn festindex(
    ctx: Context<'_>,
    #[description = "Calculate any nation's Festivity Index"] nation: String,
) -> Result<(), Error> {
    let gival = giproc::gindexcalc(nation, Index::Fest).await?;
    println!("{:?}", gival);
    if gival.status == "200 OK" {
        let response = format!("{}'s Festivity Index is {}", gival.fullname, gival.index);
        ctx.say(response).await?;
    } else if gival.status == "404 Not Found" {
        let response = ":bangbang: That nation does not exist!";
        ctx.say(response).await?;
    } else {
        let response = ":bangbang: An unexpected error has occurred connecting to the NationStates API. Try again in a minute!";
        ctx.say(response).await?;
    }
    Ok(())
}
