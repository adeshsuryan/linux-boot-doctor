# Homebrew formula template for adeshsuryan/homebrew-tap
# After each GitHub Release, update url/sha256 for the darwin or linux bottle you ship.
# linux-boot-doctor is primarily a Linux binary; on macOS prefer --input mode.

class LinuxBootDoctor < Formula
  desc "Diagnose slow Linux boots from systemd-analyze evidence"
  homepage "https://github.com/adeshsuryan/linux-boot-doctor"
  version "0.1.0"
  license "MIT"

  on_linux do
    on_intel do
      url "https://github.com/adeshsuryan/linux-boot-doctor/releases/download/v0.1.0/linux-boot-doctor-0.1.0-linux-amd64.tar.gz"
      # sha256 "REPLACE_AFTER_RELEASE"
    end
    on_arm do
      url "https://github.com/adeshsuryan/linux-boot-doctor/releases/download/v0.1.0/linux-boot-doctor-0.1.0-linux-arm64.tar.gz"
      # sha256 "REPLACE_AFTER_RELEASE"
    end
  end

  on_macos do
    # Build from source on macOS for --input / CI fixture use
    url "https://github.com/adeshsuryan/linux-boot-doctor/archive/refs/tags/v0.1.0.tar.gz"
    # sha256 "REPLACE_SOURCE_TARBALL_AFTER_TAG"
    depends_on "rust" => :build
  end

  def install
    if OS.mac?
      system "cargo", "install", "--locked", "--root", prefix, "--path", "crates/boot-doctor-cli"
    else
      bin.install "linux-boot-doctor"
    end
  end

  test do
    assert_match "linux-boot-doctor", shell_output("#{bin}/linux-boot-doctor --version")
  end
end
