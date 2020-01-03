//! Pick commandA
use super::Command;
// use crate::{cache::Cache, helper::Digit};
use crate::err::Error;
use clap::{SubCommand, App, Arg, ArgMatches};

/// Abstract pick command
pub struct PickCommand;

static QUERY_HELP: &'static str = r#"Fliter questions by conditions:
Uppercase means negative
e = easy     E = m+h
m = medium   M = e+h
h = hard     H = e+m
d = done     D = not done
l = locked   L = not locked
s = starred  S = not starred"#;

impl Command for PickCommand {
    /// `pick` usage
    fn usage<'a, 'pick>() -> App<'a, 'pick> {
        SubCommand::with_name("pick")
            .about("Pick a problem")
            .visible_alias("p")
            .arg(Arg::with_name("id")
                 .help("Problem id")
                 .takes_value(true)
            ).arg(Arg::with_name("query")
                  .short("q")
                  .long("query")
                  .takes_value(true)
                  .help(QUERY_HELP)
            )
    }

    /// `pick` handler
    fn handler(m: &ArgMatches) -> Result<(), Error>{
        use crate::cache::Cache;
        use rand::Rng;
        
        let cache = Cache::new().unwrap();
        let mut problems = cache.get_problems()?;
        if problems.len() == 0 {
            cache.clone().download_problems()?;
            Self::handler(m)?
        }

        if m.is_present("query") {
            let query = m.value_of("query")?;
            for p in query.chars() {
                match p {
                    'l' => problems.retain(|x| x.locked),
                    'L' => problems.retain(|x| !x.locked),
                    's' => problems.retain(|x| x.starred),
                    'S' => problems.retain(|x| !x.starred),
                    'e' => problems.retain(|x| x.level == 1),
                    'E' => problems.retain(|x| x.level != 1),
                    'm' => problems.retain(|x| x.level == 2),
                    'M' => problems.retain(|x| x.level != 2),
                    'h' => problems.retain(|x| x.level == 3),
                    'H' => problems.retain(|x| x.level != 3),
                    'd' => problems.retain(|x| x.status == "ac".to_string()),
                    'D' => problems.retain(|x| x.status != "ac".to_string()),
                    _ => {}
                }
            }
        }

        if let Some(id) =  m.value_of("id") {
            problems.retain(|x| x.fid.to_string() == id);
        }

        let problem = &problems[rand::thread_rng().gen_range(0, problems.len())];

        let r = cache.get_desc(problem.fid);
        if r.is_err() {
            if let Error::FeatureError(_) = r.err()? {
                Self::handler(m)?;
            }
        } else {
            println!("{:#?}", r);
        }

        Ok(())
    }
}
