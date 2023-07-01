# Maintainer: HE7086 <me@heyi7086.com>
pkgname=battery-tracker-git
_pkgname=battery-tracker
pkgver=r12.3744fe3
pkgrel=1
pkgdesc="Battery tracker"
arch=("x86_64")
url="https://github.com/HE7086/battery-tracker"
license=("MIT")
makedepends=("git" "cargo")
source=("$_pkgname::git+https://github.com/HE7086/battery-tracker.git")
sha256sums=('SKIP')

pkgver() {
    cd "$_pkgname"
    printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

prepare() {
    cd "$_pkgname"
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$_pkgname"
    cargo build --frozen --release
}

package() {
    cd "$_pkgname"
    cargo install --offline --no-track --root "$pkgdir/usr/" --path .

    install -dm755 "$pkgdir/var/lib/battery-tracker"
    install -Dm755 battery-tracker.service "$pkgdir/usr/lib/systemd/system/battery-tracker.service"
}
