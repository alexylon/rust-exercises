#[allow(unused_imports)]
use fluent::{FluentBundle, FluentValue, FluentResource, FluentArgs, FluentMessage};
use std::fs::File;
use std::{fs, io};
use std::io::prelude::*;
use std::path::Path;
use std::env;
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
#[allow(unused_imports)]
use accept_language::{intersection, parse};
use fluent::FluentError;

// Used to provide a locale for the bundle.
use unic_langid::{langid, LanguageIdentifier};

/// We need a generic file read helper function to read the localization resource file.
///
/// The resource files are stored in `./i18n/resources/{locale}` directory.
fn read_file(path: &Path) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

/// This helper function allows us to read the list of available locales
/// by reading the list of directories in `./i18n/resources`.
///
/// It is expected that every directory inside it has a name that is a valid BCP47 language tag.
fn get_available_locales() -> Result<Vec<LanguageIdentifier>, io::Error> {
    let mut locales = vec![];

    let mut dir = env::current_dir()?;
    dir.push("resources");
    let res_dir = fs::read_dir(dir)?;
    for entry in res_dir {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    if let Some(name) = name.to_str() {
                        if let Ok(langid) = name.parse() {
                            locales.push(langid);
                        } else {
                            eprintln!("Parsing failed.");
                        }
                    }
                }
            }
        }
    }
    return Ok(locales);
}

static L10N_RESOURCES: &[&str] = &["simple.ftl"];

pub fn i18n_test(locales: String, name: &str) -> String {
    println!("locales: {}", &locales);
    let user_languages = parse(&locales);
    println!("user_languages: {:?}", &user_languages);

    let mut requested_locales: Vec<LanguageIdentifier> = vec![];
    for user_language in user_languages {
        if let Ok(langid) = user_language.parse() {
            requested_locales.push(langid);
        } else {
            eprintln!("Parsing failed: ParserError(InvalidLanguage)")
        }
    }

    // Negotiate it against the available ones
    let default_locale = langid!("en-GB");
    let available_locales = get_available_locales().expect("Retrieving available locales failed.");
    let resolved_locales = negotiate_languages(
        &requested_locales,
        &available_locales,
        Some(&default_locale),
        NegotiationStrategy::Filtering,
    );

    // Create a new Fluent FluentBundle using the resolved locales.
    let mut bundle = FluentBundle::new(resolved_locales.clone().into_iter().cloned().collect());

    if let Some(current_locale) = resolved_locales
        .get(0)
        .cloned() {
        // Load the localization resource
        for path in L10N_RESOURCES {
            if let Ok(mut full_path) = env::current_dir() {
                full_path.push("resources");
                full_path.push(current_locale.to_string());
                full_path.push(path);
                if let Ok(source) = read_file(&full_path) {
                    if let Ok(resource) = FluentResource::try_new(source) {
                        match bundle
                            .add_resource(resource) {
                            Ok(_) => {}
                            Err(e) => { eprintln!("Failed to add FTL resources to the bundle: {:?}", e); }
                        }
                    } else {
                        eprintln!("Could not parse an FTL string.");
                    }
                } else {
                    eprintln!("Failed to read file.");
                }
            } else {
                eprintln!("Failed to retrieve current dir.");
            }
        }
    } else {
        eprintln!("At least one locale should match.");
    }

    let value1 = get_message(&bundle, "hello-world");
    println!("{}", value1);

    let mut args = FluentArgs::new();
    args.set("name", FluentValue::from(name));
    let value2 = get_message_args(&bundle, "intro", &args);
    println!("{}", value2);

    format!("{}\n{}", value1, value2)
}

fn get_message(bundle: &FluentBundle<FluentResource>, message_id: &str) -> String {
    get_value(bundle, None, message_id)
}

fn get_message_args(bundle: &FluentBundle<FluentResource>, message_id: &str, args: &FluentArgs) -> String {
    get_value(bundle, Some(args), message_id)
}

fn get_value(bundle: &FluentBundle<FluentResource>, args: Option<&FluentArgs>, message_id: &str) -> String {
    match bundle.get_message(message_id) {
        None => { message_id.to_string() }
        Some(message) => {
            match message.value() {
                Some(pattern) => {
                    let value = bundle.format_pattern(&pattern, args, &mut vec![]);
                    format!("{}", value)
                }
                None => { message_id.to_string() }
            }
        }
    }
}