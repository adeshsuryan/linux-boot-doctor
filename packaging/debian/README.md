# Debian / Ubuntu PPA

1. Build a source package from this tree (`debuild -S`) once Launchpad credentials exist.
2. Create PPA `ppa:adeshsuryan/linux-boot-doctor` on Launchpad.
3. `dput` the source upload.
4. Users:

```bash
sudo add-apt-repository ppa:adeshsuryan/linux-boot-doctor
sudo apt update
sudo apt install linux-boot-doctor
```

Until the PPA is live, prefer GitHub Releases `.tar.gz` installs.
