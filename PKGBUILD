pkgname='los'
pkgver=1.0.0
pkgrel=1
pkgdesc="CLI program created in Rust to solve Lights Out Puzzle"
makedepends=('cargo')
arch=('x86_64')
url='https://github.com/kikawet/lights_out_solver'
license=('MIT')

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
    #install -Dm644 $pkgname.1 "$pkgdir"/usr/share/man/man1/
}
