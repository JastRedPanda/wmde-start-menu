# Установка на deb/rpm-дистрибутивах без сборки из сырцов вручную

Пилот: `wmde-start-menu`. Автор собирает пакеты у себя локально, как сейчас `makepkg -si`.

Опорные платформы — Debian stable и Fedora: `.deb` собираем на Debian (встаёт и на Ubuntu/Mint),
`.rpm` — на Fedora (встаёт и на RHEL/openSUSE). Наоборот не гарантируется.

## Собрать

```sh
# зависимости сборки: rustc>=1.93 cargo just clang lld pkgconf wayland-protocols + nfpm
cargo install --locked nfpm  # или бинарник с github.com/goreleaser/nfpm/releases

just deb   # -> target/wmde-start-menu_*.deb
just rpm   # -> target/wmde-start-menu-*.rpm
```

Версия берётся из git-тега (`git describe --tags`, fallback `0.0.14`).
По умолчанию сборка под чистый `x86-64` (чтобы не ловить `SIGILL` на старом железе и не нарушать политики Debian/Fedora).
Опционально под `x86-64-v3` (как в Arch-репозитории):

```sh
WMDE_CPU=x86-64-v3 just deb
WMDE_CPU=x86-64-v3 just rpm
```

## Поставить

```sh
# Debian/Ubuntu/Mint
sudo apt install ./target/wmde-start-menu_*.deb

# Fedora/RHEL/openSUSE
sudo dnf install ./target/wmde-start-menu-*.rpm
```

Рантайм-зависимости подтянутся сами (см. `packaging/nfpm.yaml`).
Для работы как апплета нужна `cosmic-panel`.

## Офлайн-сборка (без сети к git-зависимостям)

Проект тянет git-зависимости (`Lin-WMDE/libcosmic`, `pop-os/*`, `smithay`) — сборочным джейлам без сети это ломает шею.
`vendor.tar` в git не лежит (см. `.gitignore`), генерим на машине с сетью один раз:

```sh
just vendor          # -> vendor.tar (cargo vendor + .cargo/config.toml)
just deb-vendored    # сборка --frozen --offline из vendor.tar -> target/*.deb
just rpm-vendored    # аналогично -> target/*.rpm
```

На машину без сети привозим `vendor.tar` файлом и жмём те же `just deb-vendored` / `just rpm-vendored` — качать из сети ничего не полезет.
