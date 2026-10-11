# Maintainer: WMDE <https://wmde.fun>
# Contributor: Kamil Lihan <k.lihan@outlook.com> (original cosmic-ext-classic-menu)
#
# Builds our fork Lin-WMDE/wmde-start-menu (branch `wmde`).
pkgname=wmde-start-menu
pkgver=0.0.14
pkgrel=3
pkgdesc="WMDE Start Menu - a classic Windows-style launcher applet for the COSMIC desktop (fork of cosmic-ext-classic-menu)"
arch=('x86_64')
url="https://wmde.fun"
license=('GPL-3.0-only')
# depends: readelf NEEDED gives glibc/gcc-libs/libxkbcommon; wayland, mesa,
# fontconfig and freetype2 are dlopen'd by libcosmic at runtime. Verify with namcap.
depends=('glibc' 'gcc-libs' 'libxkbcommon' 'wayland' 'mesa' 'fontconfig' 'freetype2')
# makedepends: same toolchain/libs that build the fork in Docker (Dockerfile.build).
makedepends=('rust' 'cargo' 'just' 'git' 'clang' 'lld' 'pkgconf' 'mesa' 'wayland'
             'libxkbcommon' 'fontconfig' 'freetype2' 'expat')
optdepends=('wmde-panel: run as a WMDE panel applet')
# NOTE: Cargo.toml uses path deps to sibling checkouts (../libcosmic,
# ../wmde-settings-daemon, ../wmde-applets). The build harness arranges them next
# to $srcdir; a standalone makepkg run without that layout fails dependency resolution.
source=("$pkgname::git+https://github.com/Lin-WMDE/wmde-start-menu.git#branch=wmde")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/$pkgname"
  # WMDE version: <COSMIC major.minor>.<WMDE release>.<commits since the nearest tag>.g<short>.
  local desc
  desc=$(git describe --long --tags --abbrev=7 2>/dev/null || true)
  if [ -n "$desc" ]; then
    printf '1.10.0.%s.g%s' "$(printf '%s' "$desc" | sed -E 's/.*-([0-9]+)-g[0-9a-f]+$/\1/')" "$(git rev-parse --short=7 HEAD)"
  else
    printf '1.10.0.%s.g%s' "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"
  fi
}

build() {
  cd "$srcdir/$pkgname"
  # x86-64-v3 (AVX2/BMI2) baseline for the WMDE repo; runs on Haswell+ (and the VM).
  export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C target-cpu=x86-64-v3"
  just build-release
}

package() {
  cd "$srcdir/$pkgname"
  # installs wmde-start-menu-applet to /usr/bin and the
  # fun.wmde.start-menu .desktop/metainfo/icon/applet-buttons to /usr/share
  just rootdir="$pkgdir" prefix=/usr install
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
