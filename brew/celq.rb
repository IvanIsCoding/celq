class Celq < Formula
  desc "A Common Expression Language (CEL) CLI Tool"
  homepage "https://github.com/IvanIsCoding/celq"
  version "{{CELQ_VERSION}}"
  license "MIT OR Apache-2.0"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/IvanIsCoding/celq/releases/download/v{{CELQ_VERSION}}/celq-aarch64-apple-darwin.tar.gz"
      sha256 "{{CELQ_SHA256_ARM64}}"
    end
    if Hardware::CPU.intel?
      url "https://github.com/IvanIsCoding/celq/releases/download/v{{CELQ_VERSION}}/celq-x86_64-apple-darwin.tar.gz"
      sha256 "{{CELQ_SHA256_X86_64}}"
    end
  else
    # Fallback to source for Linux
    url "https://github.com/IvanIsCoding/celq/archive/refs/tags/{{CELQ_VERSION}}.tar.gz"
    sha256 "{{CELQ_SHA256_SOURCE}}"
    depends_on "rust" => :build
  end

  def install
    if OS.mac?
      # Install pre-built binary
      bin.install "celq"
    else
      # Build from source
      system "cargo", "install", "--locked", "--root", prefix, "--path", "."
    end
  end

  test do
    output = shell_output("#{bin}/celq -n --arg='fruit:string=apple' 'fruit.contains(\"a\")'")
    assert_match "true", output
  end
end