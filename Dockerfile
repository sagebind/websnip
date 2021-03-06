FROM debian:jessie
MAINTAINER Stephen Coakley <me@stephencoakley.com>

RUN apt-get update && \
    apt-get install -y libfontconfig libxext6 libxrender1 wget xz-utils && \
    wget -q http://download.gna.org/wkhtmltopdf/0.12/0.12.3/wkhtmltox-0.12.3_linux-generic-amd64.tar.xz && \
    tar -xf wkhtmltox-0.12.3_linux-generic-amd64.tar.xz && \
    cp -R wkhtmltox/* /usr && \
    apt-get clean && \
    rm -rf wkhtmltox wkhtmltox-0.12.3_linux-generic-amd64.tar.xz /var/lib/apt/lists/* /tmp/* /var/tmp/*

COPY target/release/websnip /bin/websnip

EXPOSE 80

CMD ["/bin/websnip"]

