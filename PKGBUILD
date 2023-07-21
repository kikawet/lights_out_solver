pkgname=los
pkgnamelong=lights_out_solver
pkgver=1.1.1
pkgrel=1
pkgdesc="CLI program created in Rust to solve Lights Out Puzzle"
arch=('x86_64')
url='https://github.com/kikawet/lights_out_solver'
license=('MIT')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/kikawet/$pkgnamelong/archive/v$pkgver.tar.gz")
sha512sums=("https://github.com/kikawet/$pkgnamelong/archive/checksums.txt")

prepare() {
    cd "$pkgname-$pkgver"

    export RUSTUP_TOOLCHAIN=stable
    cargo update
    cargo fetch --target "$CARCH-unknown-linux-gnu"

    cd -
}

build() {
    cd "$pkgname-$pkgver"

    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features

    cd -
}

check() {
    cd "$pkgname-$pkgver"

    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features

    cd -
}

package() {
    cd "$pkgname-$pkgver"

    install -Dm0755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 "$pkgname.1" "$pkgdir/usr/share/man/man1/$pkgname.1"
    install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README"
    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
