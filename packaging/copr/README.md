# Fedora COPR

1. Create project `adeshsuryan/linux-boot-doctor` on https://copr.fedorainfracloud.org/
2. Point SCM build at this GitHub repo and `packaging/copr/linux-boot-doctor.spec`
3. Users:

```bash
sudo dnf copr enable adeshsuryan/linux-boot-doctor
sudo dnf install linux-boot-doctor
```
