Name:           linux-boot-doctor
Version:        0.1.1
Release:        1%{?dist}
Summary:        Diagnose slow Linux boots from systemd-analyze evidence

License:        MIT
URL:            https://github.com/adeshsuryan/linux-boot-doctor
Source0:        https://github.com/adeshsuryan/linux-boot-doctor/archive/refs/tags/v%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust
Requires:       glibc

%description
Read-only CLI that interprets systemd-analyze boot evidence with budgets
and CI-friendly JSON output.

%prep
%autosetup -n linux-boot-doctor-%{version}

%build
cargo build --release -p linux-boot-doctor

%install
install -D -m 0755 target/release/linux-boot-doctor %{buildroot}%{_bindir}/linux-boot-doctor
install -D -m 0644 LICENSE %{buildroot}%{_licensedir}/%{name}/LICENSE
install -D -m 0644 README.md %{buildroot}%{_pkgdocdir}/README.md

%files
%license LICENSE
%doc README.md
%{_bindir}/linux-boot-doctor

%changelog
* Wed Oct 29 2026 Adesh Suryan <adeshsuryan2005@gmail.com> - 0.1.0-1
- Initial COPR package
