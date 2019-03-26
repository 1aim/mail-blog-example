use std::{
    collections::HashMap
};

use serde::Serialize;
use failure::Error;
use futures::Future;

use mail::{
    Mail, IRI,
    Context, Resource, Source,
    headers,
    headers::{_From, _To},
    template::{
        load_toml_template_from_path,
        Template, TemplateExt,
        handlebars::Handlebars,
        LoadedTemplateData,
        TemplateData
    }
};

#[derive(Debug, Serialize)]
pub struct HelloWorldData {
    pub name: String,
    pub target: String
}

/// A more type-safe wrapper around the `Template<Handlebars>` instance.
///
/// We don't need to do anything like this but having type-safe wrappers
/// around a single template or a group of templates which should only
/// be feed some specific input data is quite a useful pattern.
pub struct HelloWorldTemplate(Template<Handlebars>);

impl HelloWorldTemplate {

    const PATH: &'static str = "./templates/hello_world/template.toml";

    pub fn load(ctx: &impl Context) -> Result<Self, Error> {
        let template = load_toml_template_from_path(
            Handlebars::new(),
            Self::PATH.into(),
            ctx
        ).wait()?;

        Ok(HelloWorldTemplate(template))
    }

    /// Create a mail using the template.
    ///
    /// Note that passing in a invalid mail address through `from`
    /// to `to` an error is returned.
    ///
    /// Note that we could support additional input data types,
    /// but we don't want that here, so we only have `HelloWorldData`.
    pub fn create_mail(
        &self,
        from: &str,
        to: &str,
        data: HelloWorldData,
        ctx: &impl Context
    ) -> Result<Mail, Error> {
        let mut mail = self.0.render(data.into(), ctx)?;
        // This could also contain headers like `Reply-To`.
        mail.insert_headers(headers! {
            _From: [from,],
            _To: [to,]
        }?);
        Ok(mail)
    }
}

pub struct AvatarTemplate(Template<Handlebars>);

impl AvatarTemplate {

    const PATH: &'static str = "./templates/avatar/template.toml";

    pub fn load(ctx: &impl Context) -> Result<Self, Error> {
        let template = load_toml_template_from_path(
            Handlebars::new(),
            Self::PATH.into(),
            ctx
        ).wait()?;

        Ok(AvatarTemplate(template))
    }

    /// Create a mail using the template.
    ///
    /// Note that passing in a invalid mail address through `from`
    /// to `to` an error is returned.
    ///
    /// Note that we could support additional input data types,
    /// but we don't want that here, so we only have `HelloWorldData`.
    pub fn create_mail(
        &self,
        from: &str,
        to: &str,
        avatar_iri: IRI,
        ctx: &impl Context
    ) -> Result<Mail, Error> {
        let data = Self::prepare_data(avatar_iri, ctx)?;
        let mut mail = self.0.render(data, ctx)?;
        // This could also contain headers like `Reply-To`.
        mail.insert_headers(headers! {
            _From: [from,],
            _To: [to,]
        }?);
        Ok(mail)
    }
}


// We don't have any additional data.
#[derive(Debug, Serialize)]
pub struct AvatarData;

impl AvatarTemplate {

    /// Creates a `TemplateData` instance from the input data (the `IRI`) and "load" it.
    ///
    /// We need to load it as only loaded data can reliably have a `Content-Id` due to
    /// the semantics of it.
    pub fn prepare_data(iri: IRI, ctx: &impl Context) -> Result<LoadedTemplateData<AvatarData>, Error> {
        let mut embeddings = HashMap::new();
        embeddings.insert("avatar".to_owned(), Resource::Source(Source {
            iri: iri,
            use_media_type: Default::default(),
            use_file_name: Default::default()
        }));

        let template_data = TemplateData {
            data: AvatarData.into(),
            attachments: Vec::new(),
            inline_embeddings: embeddings
        };

        // We have to make sure it is at last loaded
        // else we can not know it's content id.
        template_data.load(ctx).wait()
    }
}
