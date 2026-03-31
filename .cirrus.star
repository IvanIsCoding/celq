FREEBSD_IMAGE = "freebsd-14-4"

def main(ctx):
    return [
        {
            "name": "FreeBSD Build and Test",
            "only_if": "$CIRRUS_BUILD_SOURCE == 'api'",
            "freebsd_instance": {
                "image_family": FREEBSD_IMAGE
            },
            "install_script": [
                "pkg install -y rust"
            ],
            "cargo_cache": {
                "folder": "~/.cargo/registry",
                "fingerprint_script": "cat Cargo.lock"
            },
            "build_script": [
                "cargo build"
            ],
            "test_script": [
                "cargo test"
            ]
        }
    ]