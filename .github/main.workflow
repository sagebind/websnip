workflow "Main" {
  on = "push"
  resolves = ["Test"]
}

action "Test" {
  uses = "docker://rust"
  args = ["sh", "-c", "apt update && apt install -y wkhtmltopdf && cargo test"]
}
