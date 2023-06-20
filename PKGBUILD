# Maintainer: HE7086 <me@heyi7086.com>
pkgname=battery-tracker-git
pkgver=r1.5fbdc92
pkgrel=1
pkgdesc="Battery tracker"
arch=("x86_64")
url="https://github.com/HE7086/battery-tracker"
license=("MIT")
makedepends=("git" "cargo")
source=("$pkgname::git+https://github.com/HE7086/battery-tracker.git")
sha256sums=('SKIP')

pkgver() {
    cd "$pkgname"
    printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

prepare() {
    cd "$pkgname"
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$pkgname"
    cargo build --frozen --release
}

package() {
    cd "$pkgname"
    cargo install --offline --no-track --root "$pkgdir/usr/" --path .

    install -dm755 "$pkgdir/var/lib/battery-tracker"
    install -Dm755 battery-tracker.service "$pkgdir/usr/lib/systemd/system/battery-tracker.service"
}
