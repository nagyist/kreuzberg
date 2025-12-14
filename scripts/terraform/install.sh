#!/usr/bin/env bash
set -euo pipefail

OPENTOFU_VERSION="${OPENTOFU_VERSION:-1.8.5}"
TFLINT_VERSION="${TFLINT_VERSION:-v0.54.0}"

detect_os() {
	case "$(uname -s)" in
	Darwin)
		echo "darwin"
		;;
	Linux)
		echo "linux"
		;;
	*)
		echo "Unsupported OS: $(uname -s)" >&2
		exit 1
		;;
	esac
}

detect_arch() {
	case "$(uname -m)" in
	x86_64 | amd64)
		echo "amd64"
		;;
	arm64 | aarch64)
		echo "arm64"
		;;
	*)
		echo "Unsupported architecture: $(uname -m)" >&2
		exit 1
		;;
	esac
}

install_opentofu() {
	echo "=== Installing OpenTofu ${OPENTOFU_VERSION} ==="

	if command -v tofu >/dev/null 2>&1; then
		current_version=$(tofu version -json 2>/dev/null | grep -o '"terraform_version":"[^"]*"' | cut -d'"' -f4 || echo "unknown")
		if [[ "$current_version" == "$OPENTOFU_VERSION" ]]; then
			echo "OpenTofu $OPENTOFU_VERSION already installed"
			return 0
		fi
	fi

	os=$(detect_os)
	arch=$(detect_arch)

	if [[ "$os" == "darwin" ]] && command -v brew >/dev/null 2>&1; then
		echo "Installing via Homebrew..."
		brew install opentofu || brew upgrade opentofu
	else
		echo "Installing OpenTofu binary for ${os}_${arch}..."
		tofu_url="https://github.com/opentofu/opentofu/releases/download/v${OPENTOFU_VERSION}/tofu_${OPENTOFU_VERSION}_${os}_${arch}.zip"

		tmp_dir=$(mktemp -d)
		trap 'rm -rf "$tmp_dir"' EXIT

		if command -v curl >/dev/null 2>&1; then
			curl -fsSL "$tofu_url" -o "$tmp_dir/tofu.zip"
		elif command -v wget >/dev/null 2>&1; then
			wget -q "$tofu_url" -O "$tmp_dir/tofu.zip"
		else
			echo "Error: Neither curl nor wget found" >&2
			exit 1
		fi

		unzip -q "$tmp_dir/tofu.zip" -d "$tmp_dir"
		install_dir="${HOME}/.local/bin"
		mkdir -p "$install_dir"
		mv "$tmp_dir/tofu" "$install_dir/tofu"
		chmod +x "$install_dir/tofu"

		if [[ ":$PATH:" != *":$install_dir:"* ]]; then
			echo "Warning: $install_dir not in PATH. Add it to your shell profile:" >&2
			echo "  export PATH=\"\$PATH:$install_dir\"" >&2
		fi
	fi

	if command -v tofu >/dev/null 2>&1; then
		echo "OpenTofu installed: $(tofu version | head -n1)"
	else
		echo "Error: OpenTofu installation failed" >&2
		exit 1
	fi
}

install_tflint() {
	echo "=== Installing tflint ${TFLINT_VERSION} ==="

	if command -v tflint >/dev/null 2>&1; then
		current_version=$(tflint --version | head -n1 | grep -o 'version [^ ]*' | cut -d' ' -f2 || echo "unknown")
		if [[ "$current_version" == "$TFLINT_VERSION" ]]; then
			echo "tflint $TFLINT_VERSION already installed"
			return 0
		fi
	fi

	os=$(detect_os)
	arch=$(detect_arch)

	if [[ "$os" == "darwin" ]] && command -v brew >/dev/null 2>&1; then
		echo "Installing via Homebrew..."
		brew install tflint || brew upgrade tflint
	else
		echo "Installing tflint binary for ${os}_${arch}..."
		tflint_url="https://github.com/terraform-linters/tflint/releases/download/${TFLINT_VERSION}/tflint_${os}_${arch}.zip"

		tmp_dir=$(mktemp -d)
		trap 'rm -rf "$tmp_dir"' EXIT

		if command -v curl >/dev/null 2>&1; then
			curl -fsSL "$tflint_url" -o "$tmp_dir/tflint.zip"
		elif command -v wget >/dev/null 2>&1; then
			wget -q "$tflint_url" -O "$tmp_dir/tflint.zip"
		else
			echo "Error: Neither curl nor wget found" >&2
			exit 1
		fi

		unzip -q "$tmp_dir/tflint.zip" -d "$tmp_dir"
		install_dir="${HOME}/.local/bin"
		mkdir -p "$install_dir"
		mv "$tmp_dir/tflint" "$install_dir/tflint"
		chmod +x "$install_dir/tflint"

		if [[ ":$PATH:" != *":$install_dir:"* ]]; then
			echo "Warning: $install_dir not in PATH. Add it to your shell profile:" >&2
			echo "  export PATH=\"\$PATH:$install_dir\"" >&2
		fi
	fi

	if command -v tflint >/dev/null 2>&1; then
		echo "tflint installed: $(tflint --version | head -n1)"
	else
		echo "Error: tflint installation failed" >&2
		exit 1
	fi
}

main() {
	install_opentofu
	install_tflint
	echo "=== Terraform tooling installation complete ==="
}

main "$@"
