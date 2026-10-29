# Template — sha256 filled after each GitHub Release in adeshsuryan/homebrew-tap
class LinuxBootDoctor < Formula
  desc "Diagnose slow Linux boots from systemd-analyze evidence"
  homepage "https://github.com/adeshsuryan/linux-boot-doctor"
  version "0.1.1"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/adeshsuryan/linux-boot-doctor/releases/download/v0.1.1/linux-boot-doctor-0.1.1-darwin-arm64.tar.gz"
      # sha256 filled after release
    end
    on_intel do
      url "https://github.com/adeshsuryan/linux-boot-doctor/releases/download/v0.1.1/linux-boot-doctor-0.1.1-darwin-amd64.tar.gz"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/adeshsuryan/linux-boot-doctor/releases/download/v0.1.1/linux-boot-doctor-0.1.1-linux-amd64.tar.gz"
    end
    on_arm do
      url "https://github.com/adeshsuryan/linux-boot-doctor/releases/download/v0.1.1/linux-boot-doctor-0.1.1-linux-arm64.tar.gz"
    end
  end

  def install
    bin.install "linux-boot-doctor"
  end

  test do
    assert_match "linux-boot-doctor", shell_output("#{bin}/linux-boot-doctor --version")
  end
end
