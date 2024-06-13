VERSION 0.8
IMPORT github.com/Jake-Shadle/xwin:0871d7dbbbf36839f3cba53de8879851670a6246 AS xwin

builder:
    FROM xwin+xwin

    RUN mkdir /build
    WORKDIR /build

    RUN apt-get update && apt-get install --no-install-recommends -y jq
    COPY --keep-ts --if-exists rust-toolchain.toml ./
    IF [ ! -f ./rust-toolchain.toml ]
        RUN rustup default stable
    END
    RUN rustup target add x86_64-pc-windows-msvc
    RUN ln $CARGO_HOME/bin/* /usr/bin/

    SAVE IMAGE --cache-hint builder

prefetch:
    FROM +builder

    ARG --required cargo_home_cache_id
    ARG --required CARGO_REGISTRIES_KTRA_INDEX
    COPY --keep-ts --dir --if-exists .cargo ./
    COPY --keep-ts Cargo.toml ./
    COPY --keep-ts --if-exists Cargo.lock ./

    RUN mkdir -p src && touch src/lib.rs

    RUN --mount=type=cache,mode=0777,id=$cargo_home_cache_id,sharing=shared,target=$CARGO_HOME \
        cargo --color=always fetch --target x86_64-pc-windows-msvc

    SAVE IMAGE --cache-hint prefetch

addon:
    ARG cargo_home_cache_id="earthly-cargo-cache"
    ARG CARGO_REGISTRIES_KTRA_INDEX = https://git.0x0f.net/0x0f/ktra-crates.git
    FROM --pass-args +prefetch

    ARG EARTHLY_TARGET_PROJECT_NO_TAG
    ARG EARTHLY_TARGET_NAME
    ARG target_cache_id="${EARTHLY_CARGO_HOME_CACHE_ID}#${EARTHLY_TARGET_PROJECT_NO_TAG}#${EARTHLY_TARGET_NAME}"
    ARG target_folder="target/x86_64-pc-windows-msvc/release"

    ARG CARGO_TARGET_NAME = $(cargo metadata -q --no-deps --filter-platform x86_64-pc-windows-msvc | jq -r '.packages[0].name' | sed 's/-/_/g')
    ARG CARGO_EXTRA_ARGS

    COPY --keep-ts --dir . ./
    RUN --mount=type=cache,mode=0777,id=$cargo_home_cache_id,sharing=shared,target=$CARGO_HOME \
        --mount=type=cache,mode=0777,id=$target_cache_id,sharing=locked,target=target \
        cargo build --release --color=always --target x86_64-pc-windows-msvc ${CARGO_EXTRA_ARGS} && \
        (rm ${CARGO_TARGET_NAME}.dll || true) && \
        (rm ${CARGO_TARGET_NAME}.pdb || true) && \
        cp ${target_folder}/${CARGO_TARGET_NAME}.dll ./ && \
        (cp ${target_folder}/${CARGO_TARGET_NAME}.pdb ./ || true)

    SAVE ARTIFACT ${CARGO_TARGET_NAME}.dll
    SAVE ARTIFACT --if-exists ${CARGO_TARGET_NAME}.pdb
