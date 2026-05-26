pkgname=keskos-welcome
pkgver=0.1.0
pkgrel=1
pkgdesc="KeskOS first-boot welcome app and fallback setup console"
arch=(x86_64)
url="https://github.com/memegeko/keskos"
license=(GPL-3.0-or-later)
options=(!debug)
depends=(
  gtk3
  iputils
  networkmanager
  pyside6
  python
  xdg-utils
)
makedepends=(
  cargo
  pkgconf
  rust
)
provides=(kesk-welcome)
conflicts=(kesk-welcome)
replaces=(kesk-welcome)
backup=(
  etc/xdg/autostart/kesk-welcome.desktop
  etc/xdg/autostart/keskos-first-run.desktop
)
optdepends=(
  'keskos-browser-startpage: ships the staged homepage and browser theming assets used by Kesk Welcome'
  'keskos-settings: enables the full browser/theme helper integration shown by Kesk Welcome'
  'keskos-tools: enables the routed `kesk welcome` command and repair helpers linked from the flow'
)
source=()
sha256sums=()

build() {
  local app_dir="${startdir}/files/kesk-welcome"

  cd "${app_dir}"
  CARGO_TARGET_DIR="${srcdir}/target" cargo build --release --locked
}

package() {
  local srcroot="${startdir}/files"
  local app_dir="${srcroot}/kesk-welcome"

  install -D -m 755 "${srcdir}/target/release/kesk-welcome" "${pkgdir}/usr/bin/kesk-welcome"
  install -D -m 644 "${app_dir}/packaging/kesk-welcome.desktop" "${pkgdir}/usr/share/applications/kesk-welcome.desktop"
  install -D -m 644 "${app_dir}/packaging/kesk-welcome-autostart.desktop" "${pkgdir}/etc/xdg/autostart/kesk-welcome.desktop"
  install -D -m 644 "${app_dir}/LICENSE" "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"

  install -D -m 755 "${srcroot}/airootfs/usr/bin/keskos-first-run" "${pkgdir}/usr/bin/keskos-first-run"
  cp -a "${srcroot}/airootfs/usr/lib/keskos-first-run" "${pkgdir}/usr/lib/"
  install -D -m 644 "${srcroot}/airootfs/usr/share/applications/keskos-first-run.desktop" "${pkgdir}/usr/share/applications/keskos-first-run.desktop"
  install -D -m 644 "${srcroot}/airootfs/etc/xdg/autostart/keskos-first-run.desktop" "${pkgdir}/etc/xdg/autostart/keskos-first-run.desktop"
  install -D -m 644 "${srcroot}/airootfs/etc/skel/.config/autostart/keskos-first-run.desktop" "${pkgdir}/etc/skel/.config/autostart/keskos-first-run.desktop"
}
