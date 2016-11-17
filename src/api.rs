use chrono::*;
use iron::headers::{AccessControlAllowOrigin, ContentType};
use iron::middleware::Handler;
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::prelude::*;
use iron::status::Status;
use mercury::Mercury;
use pdf::PdfGenerator;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;


pub struct ApiHandler {
    mercury: Mercury,
    pdf_generator: PdfGenerator,
}

impl ApiHandler {
    pub fn new() -> ApiHandler {
        ApiHandler {
            mercury: Mercury::default(),
            pdf_generator: PdfGenerator::new(),
        }
    }
}

impl Handler for ApiHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let query = request.url.query();
        info!("query: {:?}", query);

        if query.is_none() {
            return Ok(Response::with((Status::BadRequest, "Invalid query")));
        }

        // Fetch the article data.
        let query = query.unwrap();
        let article = match self.mercury.parse(query) {
            Some(a) => a,
            None => return Ok(Response::with((Status::BadRequest, "Could not read article"))),
        };

        // Generate a temporary path for the pdf file.
        let mut path = PathBuf::from("/tmp");
        path.push(format!("article-{}.pdf", UTC::now().format("%s")));

        // Generate a pdf.
        let path = match self.pdf_generator.generate(article.title, article.content, path) {
            Ok(p) => p,
            Err(_) => return Ok(Response::with((Status::InternalServerError, "Error"))),
        };

        // Read the pdf to memory.
        let mut data = Vec::new();
        {
            let mut file = File::open(&path).unwrap();
            file.read_to_end(&mut data).unwrap();
        }
        fs::remove_file(&path).unwrap();

        // Generate HTTP response.
        let mut response = Response::new();
        response.status = Some(Status::Ok);
        response.headers.set(AccessControlAllowOrigin::Any);
        response.headers
            .set(ContentType(Mime(TopLevel::Application, SubLevel::Ext("pdf".into()), vec![])));
        response.body = Some(Box::new(data));

        Ok(response)
    }
}
