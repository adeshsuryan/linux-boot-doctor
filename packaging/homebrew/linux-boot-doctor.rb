class LinuxBootDoctor < Formula
  desc "Diagnose slow Linux boots from systemd-analyze evidence"
  homepage "https://github.com/adeshsuryan/linux-boot-doctor"
  version "0.1.1"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/adeshsuryan/linux-boot-doctor/releases/download/v0.1.1/linux-boot-doctor-0.1.1-darwin-arm64.tar.gz"
      sha256 "6f5b91fb45eb1829e6e1906ed13a83b7ac6eeaaa73f7d8d6e9ca9f54605973c5"
    end
    on_intel do
      url "https://github.com/adeshsuryan/linux-boot-doctor/releases/download/v0.1.1/linux-boot-doctor-0.1.1-darwin-amd64.tar.gz"
      sha256 "ab3eaa11cb8da59d849f6c6c76f21946410c1d6de3d1292df3979ac7b7c05cd2"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/adeshsuryan/linux-boot-doctor/releases/download/v0.1.1/linux-boot-doctor-0.1.1-linux-amd64.tar.gz"
      sha256 "200fb4b18f650f4ef3eed2b73885801bb475c812651513e2f118b2c53dc9b103"
    end
    on_arm do
      url "https://github.com/adeshsuryan/linux-boot-doctor/releases/download/v0.1.1/linux-boot-doctor-0.1.1-linux-arm64.tar.gz"
      sha256 "8163f6a0f2f39969ffd26f825e10a7f324af9871081e2ede39ba185c60b6913b"
    end
  end

  def install
    bin.install "linux-boot-doctor"
  end

  test do
    assert_match "linux-boot-doctor", shell_output("#{bin}/linux-boot-doctor --version")
  end
end
