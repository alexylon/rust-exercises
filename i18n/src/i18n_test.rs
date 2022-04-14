#[allow(unused_imports)]
use fluent::{FluentBundle, FluentValue, FluentResource, FluentArgs, FluentMessage};
use std::fs::File;
use std::{fs, io};
use std::io::prelude::*;
use std::path::Path;
use std::env;
use fluent_langneg::{negotiate_languages, NegotiationStrategy};

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
                        let langid = name.parse().expect("Parsing failed.");
                        locales.push(langid);
                    }
                }
            }
        }
    }
    return Ok(locales);
}

static L10N_RESOURCES: &[&str] = &["simple.ftl"];

pub fn i18n_test(locale: String, name: &str) -> String {
    let mut requested: Vec<LanguageIdentifier> = vec![];
    if let Ok(langid) = locale.parse() {
        requested.push(langid);
    } else {
        println!("Parsing failed: ParserError(InvalidLanguage)")
    }

    // Negotiate it against the available ones
    let default_locale = langid!("en-GB");
    let available = get_available_locales().expect("Retrieving available locales failed.");
    let resolved_locales = negotiate_languages(
        &requested,
        &available,
        Some(&default_locale),
        NegotiationStrategy::Filtering,
    );

    let current_locale = resolved_locales
        .get(0)
        .cloned()
        .expect("At least one locale should match.");

    // Create a new Fluent FluentBundle using the resolved locales.
    let mut bundle = FluentBundle::new(resolved_locales.into_iter().cloned().collect());

    // Load the localization resource
    for path in L10N_RESOURCES {
        let mut full_path = env::current_dir().expect("Failed to retrieve current dir.");
        full_path.push("resources");
        full_path.push(current_locale.to_string());
        full_path.push(path);
        let source = read_file(&full_path).expect("Failed to read file.");
        let resource = FluentResource::try_new(source).expect("Could not parse an FTL string.");
        bundle
            .add_resource(resource)
            .expect("Failed to add FTL resources to the bundle.");
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