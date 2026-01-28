class Celq < Formula
  desc "A Common Expression Language (CEL) CLI Tool"
  homepage "https://github.com/IvanIsCoding/celq"
  version "{{CELQ_VERSION}}"
  license "MIT OR Apache-2.0"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/IvanIsCoding/celq/releases/download/v{{CELQ_VERSION}}/celq-macos-aarch64.tar.gz"
      sha256 "{{CELQ_SHA256_MACOS_ARM64}}"
    end
    if Hardware::CPU.intel?
      url "https://github.com/IvanIsCoding/celq/releases/download/v{{CELQ_VERSION}}/celq-macos-x86_64.tar.gz"
      sha256 "{{CELQ_SHA256_MACOS_X86_64}}"
    end
  end
  
  if OS.linux?
    if Hardware::CPU.arm?
      url "https://github.com/IvanIsCoding/celq/releases/download/v{{CELQ_VERSION}}/celq-linux-aarch64-gnu.tar.gz"
      sha256 "{{CELQ_SHA256_LINUX_ARM64}}"
    end
    if Hardware::CPU.intel?
      url "https://github.com/IvanIsCoding/celq/releases/download/v{{CELQ_VERSION}}/celq-linux-x86_64-gnu.tar.gz"
      sha256 "{{CELQ_SHA256_LINUX_X86_64}}"
    end
  end

  def install
    bin.install "celq"
  end

  test do
    output = shell_output("#{bin}/celq -n --arg='fruit:string=apple' 'fruit.contains(\"a\")'")
    assert_match "true", output
  end
end