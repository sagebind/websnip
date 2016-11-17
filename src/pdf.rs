use std::path::PathBuf;
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::Mutex;
use std::thread::{self, Builder, Thread};
use wkhtmltopdf::PdfApplication;


pub struct PdfGenerator {
    sender: Mutex<Sender<Request>>,
}

impl PdfGenerator {
    pub fn new() -> PdfGenerator {
        PdfGenerator {
            sender: Mutex::new(Self::spawn()),
        }
    }

    pub fn generate<S: Into<String>, P: Into<PathBuf>>(&self, title: S, html: S, path: P) -> Result<PathBuf, ()> {
        let sender = self.sender.lock().unwrap();

        let path = path.into();

        let request = Request {
            title: title.into(),
            html: html.into(),
            path: path.clone(),
            requestor: thread::current(),
        };

        sender.send(request).expect("error sending pdf request");
        thread::park();

        if path.exists() {
            Ok(path)
        } else {
            Err(())
        }
    }

    fn spawn() -> Sender<Request> {
        let (sender, receiver) = mpsc::channel();
        info!("starting pdf renderer thread");

        Builder::new().name("wkhtmltopdf renderer".into()).spawn(move || {
            info!("pdf renderer thread ready");
            Self::renderer(receiver)
        }).unwrap();

        sender
    }

    fn renderer(receiver: Receiver<Request>) {
        let mut application = PdfApplication::new().expect("error starting PDF application");

        while let Ok(request) = receiver.recv() {
            info!("generating pdf: {}", request.path.to_str().unwrap());

            let mut pdf = match {
                application.builder()
                    .title(&request.title)
                    .build_from_html(&request.html)
            } {
                Ok(pdf) => pdf,
                Err(e) => {
                    warn!("error generating pdf: {:?}", e);
                    request.requestor.unpark();
                    continue;
                }
            };

            if let Err(e) = pdf.save(&request.path) {
                warn!("error saving pdf: {:?}", e);
            }

            request.requestor.unpark();
        }
    }
}

struct Request {
    title: String,
    html: String,
    path: PathBuf,
    requestor: Thread,
}
