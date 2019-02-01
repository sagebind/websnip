workflow "Main" {
  on = "push"
  resolves = ["Test"]
}

action "Test" {
  uses = "docker://rust"
  args = ["sh", "-c", "apt update && wget https://github.com/wkhtmltopdf/wkhtmltopdf/releases/download/0.12.5/wkhtmltox_0.12.5-1.stretch_amd64.deb && apt install -y ./*.deb && cargo build"]
}
