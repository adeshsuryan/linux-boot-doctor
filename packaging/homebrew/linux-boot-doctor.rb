class LinuxBootDoctor < Formula
  desc "Diagnose slow Linux boots from systemd-analyze evidence"
  homepage "https://github.com/adeshsuryan/linux-boot-doctor"
  version "0.1.0"
  license "MIT"

  on_linux do
    on_intel do
      url "https://github.com/adeshsuryan/linux-boot-doctor/releases/download/v0.1.0/linux-boot-doctor-0.1.0-linux-amd64.tar.gz"
      sha256 "3fbebd612bf406e9e870bb388019e518e2735bf06f948db798f7974fc6af4c3d"
    end
    on_arm do
      url "https://github.com/adeshsuryan/linux-boot-doctor/releases/download/v0.1.0/linux-boot-doctor-0.1.0-linux-arm64.tar.gz"
      sha256 "a3b2f909df32eb94ea1c2ff4041185cf4f9909526a317bebd176fb4c6417e9d3"
    end
  end

  on_macos do
    url "https://github.com/adeshsuryan/linux-boot-doctor/archive/refs/tags/v0.1.0.tar.gz"
    sha256 "17d16e715a7a6f91f16aef40f9498f0107bbc392f4930fa7cd228881a8bdeff1"
    depends_on "rust" => :build
  end

  def install
    if OS.mac?
      system "cargo", "install", "--root", prefix, "--path", "crates/boot-doctor-cli"
    else
      bin.install "linux-boot-doctor"
    end
  end

  test do
    assert_match "linux-boot-doctor", shell_output("#{bin}/linux-boot-doctor --version")
  end
end
