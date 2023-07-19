pkgname=los
pkgnamelong=lights_out_solver
pkgver=1.0.0
pkgrel=1
pkgdesc="CLI program created in Rust to solve Lights Out Puzzle"
arch=('x86_64')
url='https://github.com/kikawet/lights_out_solver'
license=('MIT')
makedepends=('cargo')
source=("$pkgnamelong-$pkgver.tar.gz::https://github.com/kikawet/$pkgnamelong/archive/v$pkgver.tar.gz")
sha512sums=('6df82e78d4d17cfed2b89be45128a8c3e76b2dcaae3b1625b4853aa65155e465b55feb57e650f66a2a108030f38424e3b2bd5e0912f97d5f16bafbd394807854')

prepare() {
    cd "$pkgnamelong-$pkgver"

    export RUSTUP_TOOLCHAIN=stable
    cargo update
    cargo fetch --target "$CARCH-unknown-linux-gnu"

    cd -
}

build() {
    cd "$pkgnamelong-$pkgver"

    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features

    cd -
}

check() {
    cd "$pkgnamelong-$pkgver"

    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features

    cd -
}

package() {
    cd "$pkgnamelong-$pkgver"

    install -Dm0755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 "$pkgname.1" "$pkgdir/usr/share/man/man1/$pkgname.1"
    install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README"
    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
