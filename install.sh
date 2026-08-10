#!/usr/bin/env bash
set -euo pipefail

REPO_URL="${SERVERSPOT_REPO:-https://github.com/serverspot/serverspot.git}"
REPO_REF="${SERVERSPOT_REF:-install-script}"
TTY="/dev/tty"

C_RESET="\033[0m"
C_DIM="\033[2m"
C_BOLD="\033[1m"
C_CYAN="\033[36m"
C_GREEN="\033[32m"
C_YELLOW="\033[33m"
C_RED="\033[31m"
C_WHITE="\033[97m"

YES=0
MODE="${SERVERSPOT_MODE:-}"
DOMAIN="${SERVERSPOT_DOMAIN:-}"
ACME_EMAIL="${SERVERSPOT_EMAIL:-}"
INSTALL_DIR="${SERVERSPOT_DIR:-}"
EXPOSE_PORT="${SERVERSPOT_PORT:-8080}"
SURREAL_PASS="${SERVERSPOT_DB_PASS:-}"
SURREAL_USER="${SERVERSPOT_DB_USER:-root}"
SURREAL_NS="${SERVERSPOT_NS:-serverspot}"

for arg in "$@"; do
  case "$arg" in
    -y|--yes) YES=1 ;;
    -h|--help)
      printf '%s\n' "Usage: install.sh [--yes]" \
        "" \
        "Interactive install (default), or set these and pass --yes:" \
        "  SERVERSPOT_MODE=proxy|local" \
        "  SERVERSPOT_DOMAIN=panel.example.com" \
        "  SERVERSPOT_EMAIL=you@example.com" \
        "  SERVERSPOT_DIR=/opt/serverspot" \
        "  SERVERSPOT_PORT=8080" \
        "  SERVERSPOT_DB_PASS=..." \
        "  SERVERSPOT_REPO=https://github.com/serverspot/serverspot.git" \
        "  SERVERSPOT_REF=install-script"
      exit 0
      ;;
  esac
done

have_tty() {
  [[ -r "$TTY" && -w "$TTY" ]]
}

die() {
  tput cnorm >"$TTY" 2>/dev/null || true
  printf "${C_RED}✖ %s${C_RESET}\n" "$*" >&2
  exit 1
}

info() { printf "${C_CYAN}▸${C_RESET} %s\n" "$*"; }
ok() { printf "${C_GREEN}✔${C_RESET} %s\n" "$*"; }
warn() { printf "${C_YELLOW}!${C_RESET} %s\n" "$*"; }

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "Missing required command: $1"
}

banner() {
  printf '\n'
  printf "${C_CYAN}${C_BOLD}"
  printf '  ┌─────────────────────────────────────────┐\n'
  printf '  │              ServerSpot                 │\n'
  printf '  │     one-command Docker installer        │\n'
  printf '  └─────────────────────────────────────────┘\n'
  printf "${C_RESET}\n"
}

read_tty() {
  local prompt="$1"
  local default="${2-}"
  local secret="${3-0}"
  local value=""
  if [[ "$YES" -eq 1 ]]; then
    printf '%s\n' "$default"
    return
  fi
  have_tty || die "No terminal available. Re-run with --yes and env vars, or from a real TTY."
  if [[ -n "$default" ]]; then
    printf "${C_WHITE}%s${C_RESET} ${C_DIM}[%s]${C_RESET} " "$prompt" "$default" >"$TTY"
  else
    printf "${C_WHITE}%s${C_RESET} " "$prompt" >"$TTY"
  fi
  if [[ "$secret" == "1" ]]; then
    read -r -s value <"$TTY" || true
    printf '\n' >"$TTY"
  else
    read -r value <"$TTY" || true
  fi
  if [[ -z "$value" ]]; then
    printf '%s\n' "$default"
  else
    printf '%s\n' "$value"
  fi
}

choose() {
  local prompt="$1"
  shift
  local -a opts=("$@")
  local count=${#opts[@]}
  local selected=0
  local i key rest

  if [[ "$YES" -eq 1 ]]; then
    printf '%s\n' "${opts[0]}"
    return
  fi
  have_tty || die "No terminal available for the menu."

  printf '\n'"${C_BOLD}%s${C_RESET}\n" "$prompt" >"$TTY"
  printf "${C_DIM}  ↑↓ move · Enter select · 1-%d jump${C_RESET}\n\n" "$count" >"$TTY"

  tput civis >"$TTY" 2>/dev/null || true

  while true; do
    for ((i = 0; i < count; i++)); do
      if ((i == selected)); then
        printf "  ${C_CYAN}${C_BOLD}❯ %s${C_RESET}\n" "${opts[$i]}" >"$TTY"
      else
        printf "    %s\n" "${opts[$i]}" >"$TTY"
      fi
    done

    IFS= read -rsn1 key <"$TTY" || key=""
    if [[ "$key" == $'\x1b' ]]; then
      read -rsn2 rest <"$TTY" || rest=""
      key+="$rest"
    fi

    case "$key" in
      $'\x1b[A' | k)
        selected=$(((selected - 1 + count) % count))
        ;;
      $'\x1b[B' | j)
        selected=$(((selected + 1) % count))
        ;;
      '' | $'\n' | $'\r')
        tput cnorm >"$TTY" 2>/dev/null || true
        printf '%s\n' "${opts[$selected]}"
        for ((i = 0; i < count + 3; i++)); do printf '\033[A\033[2K' >"$TTY"; done
        printf "  ${C_GREEN}✔${C_RESET} %s  ${C_DIM}%s${C_RESET}\n\n" "$prompt" "${opts[$selected]}" >"$TTY"
        return
        ;;
      [1-9])
        if ((key >= 1 && key <= count)); then
          selected=$((key - 1))
        fi
        ;;
      q | Q)
        die "Cancelled."
        ;;
    esac
    printf '\033[%dA' "$count" >"$TTY"
  done
}

public_ip() {
  local ip=""
  ip="$(curl -4 -fsS --max-time 4 https://ifconfig.me 2>/dev/null || true)"
  if [[ -z "$ip" ]]; then
    ip="$(curl -4 -fsS --max-time 4 https://api.ipify.org 2>/dev/null || true)"
  fi
  printf '%s\n' "$ip"
}

resolve_host() {
  local host="$1"
  local ip=""

  if command -v getent >/dev/null 2>&1; then
    ip="$(getent ahosts "$host" 2>/dev/null | awk '/STREAM/ {print $1; exit}')" || true
    if [[ -z "$ip" ]]; then
      ip="$(getent ahostsv4 "$host" 2>/dev/null | awk '{print $1; exit}')" || true
    fi
    if [[ -z "$ip" ]]; then
      ip="$(getent hosts "$host" 2>/dev/null | awk '{print $1; exit}')" || true
    fi
  fi
  if [[ -z "$ip" ]] && command -v dig >/dev/null 2>&1; then
    ip="$(dig +short A "$host" 2>/dev/null | awk '/^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$/ {print; exit}')" || true
  fi
  if [[ -z "$ip" ]] && command -v python3 >/dev/null 2>&1; then
    ip="$(python3 -c "import socket; print(socket.getaddrinfo('${host}', None, socket.AF_INET)[0][4][0])" 2>/dev/null)" || true
  fi
  printf '%s\n' "$ip"
}

valid_domain() {
  [[ "$1" =~ ^[A-Za-z0-9]([A-Za-z0-9-]{0,61}[A-Za-z0-9])?(\.[A-Za-z0-9]([A-Za-z0-9-]{0,61}[A-Za-z0-9])?)+$ ]]
}

valid_email() {
  [[ "$1" =~ ^[^@[:space:]]+@[^@[:space:]]+\.[^@[:space:]]+$ ]]
}

gen_pass() {
  if command -v openssl >/dev/null 2>&1; then
    openssl rand -hex 16
    return
  fi
  dd if=/dev/urandom bs=16 count=1 2>/dev/null | od -An -tx1 | tr -d ' \n'
}

docker_bin() {
  if docker compose version >/dev/null 2>&1; then
    printf '%s\n' "docker compose"
  elif command -v docker-compose >/dev/null 2>&1; then
    printf '%s\n' "docker-compose"
  else
    die "Docker Compose is not available."
  fi
}

run_docker() {
  local dir="$1"
  shift
  if docker info >/dev/null 2>&1; then
    (cd "$dir" && "$@")
  else
    as_root sh -c "cd \"$dir\" && $*"
  fi
}

write_if_possible() {
  local file="$1"
  local dir
  dir="$(dirname "$file")"
  if [[ -w "$dir" && ( ! -e "$file" || -w "$file" ) ]]; then
    cat >"$file"
    chmod 600 "$file"
  else
    as_root tee "$file" >/dev/null
    as_root chmod 600 "$file"
  fi
}

as_root() {
  if [[ "$(id -u)" -eq 0 ]]; then
    "$@"
  elif command -v sudo >/dev/null 2>&1; then
    sudo "$@"
  else
    die "Need root to run: $*"
  fi
}

ensure_docker() {
  if command -v docker >/dev/null 2>&1 && docker info >/dev/null 2>&1; then
    ok "Docker is ready."
    return
  fi

  if command -v docker >/dev/null 2>&1; then
    warn "Docker is installed but this user cannot talk to the daemon."
    if [[ "$YES" -eq 1 ]]; then
      die "Fix Docker permissions, then re-run."
    fi
    local next
    next="$(choose "Docker is installed but not usable. What next?" \
      "Install / repair Docker with the official script (needs sudo)" \
      "Quit so I can fix it myself")"
    [[ "$next" == Quit* ]] && die "Fix Docker access, then re-run ./install.sh"
  else
    info "Docker is not installed."
    if [[ "$YES" -ne 1 ]]; then
      local next
      next="$(choose "Install Docker now? (official get.docker.com script)" \
        "Yes, install Docker" \
        "No, I'll handle Docker myself")"
      [[ "$next" == No* ]] && die "Install Docker, then re-run ./install.sh"
    fi
  fi

  need_cmd curl
  as_root sh -c 'curl -fsSL https://get.docker.com | sh'
  if [[ "$(id -u)" -ne 0 && -n "${SUDO_USER:-}" ]]; then
    as_root usermod -aG docker "$SUDO_USER" || true
  elif [[ "$(id -u)" -ne 0 ]]; then
    as_root usermod -aG docker "$(id -un)" || true
    warn "You may need to log out and back in (or run: newgrp docker)."
  fi

  if ! docker info >/dev/null 2>&1; then
    as_root sh -c 'until docker info >/dev/null 2>&1; do sleep 1; done'
  fi
  docker info >/dev/null 2>&1 || die "Docker is still not reachable. Log out/in and re-run."
  ok "Docker installed."
}

in_repo() {
  [[ -f "$1/docker-compose.yml" && -f "$1/Dockerfile" && -d "$1/database" && -f "$1/src/main.rs" ]]
}

script_source_dir() {
  local src="${BASH_SOURCE[0]:-}"
  [[ -n "$src" && -f "$src" ]] || return 1
  cd "$(dirname "$src")" && pwd
}

prepare_tree() {
  local dest="$1"
  local src=""
  src="$(script_source_dir || true)"

  if [[ -n "$src" ]] && in_repo "$src"; then
    if [[ "$src" == "$dest" ]]; then
      ok "Using this checkout."
      return
    fi
    info "Copying ServerSpot into ${dest}"
    if [[ -w "$(dirname "$dest")" ]]; then
      mkdir -p "$dest"
      cp -a "$src"/. "$dest"/
    else
      as_root mkdir -p "$dest"
      as_root sh -c "cp -a \"$src\"/. \"$dest\"/"
    fi
    return
  fi

  need_cmd git
  info "Cloning ServerSpot (${REPO_REF}) → ${dest}"
  if [[ -w "$(dirname "$dest")" || -w "$dest" ]]; then
    mkdir -p "$dest"
    if [[ -d "$dest/.git" ]]; then
      git -C "$dest" fetch --depth 1 origin "$REPO_REF"
      git -C "$dest" checkout --force FETCH_HEAD
    else
      rm -rf "${dest:?}/"* "${dest}"/.[!.]* 2>/dev/null || true
      git clone --depth 1 --branch "$REPO_REF" "$REPO_URL" "$dest"
    fi
  else
    as_root mkdir -p "$dest"
    if [[ -d "$dest/.git" ]]; then
      as_root git -C "$dest" fetch --depth 1 origin "$REPO_REF"
      as_root git -C "$dest" checkout --force FETCH_HEAD
    else
      as_root rm -rf "$dest"
      as_root git clone --depth 1 --branch "$REPO_REF" "$REPO_URL" "$dest"
    fi
  fi
}

write_env() {
  local dest="$1"
  local mode="$2"
  local bind compose_file
  if [[ "$mode" == "proxy" ]]; then
    bind="127.0.0.1"
    compose_file="docker-compose.yml:deploy/proxy.yml"
  else
    bind="0.0.0.0"
    compose_file="docker-compose.yml"
  fi

  write_if_possible "$dest/.env" <<EOF
SURREAL_USER=${SURREAL_USER}
SURREAL_PASS=${SURREAL_PASS}
SURREAL_NS=${SURREAL_NS}
SERVERSPOT_BIND=${bind}
EXPOSE_PORT=${EXPOSE_PORT}
COMPOSE_FILE=${compose_file}
DOMAIN=${DOMAIN}
ACME_EMAIL=${ACME_EMAIL}
EOF
}

port_busy() {
  local port="$1"
  if command -v ss >/dev/null 2>&1; then
    ss -ltn 2>/dev/null | grep -Eq ":${port}[[:space:]]" && return 0 || true
  fi
  if command -v lsof >/dev/null 2>&1; then
    lsof -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1 && return 0 || true
  fi
  return 1
}

main() {
  banner

  if [[ "$(uname -s)" != "Linux" && "$YES" -ne 1 ]]; then
    warn "This installer targets a Linux server. You can still use local/port mode for a trial."
  fi

  local here=""
  here="$(script_source_dir || true)"
  local default_dir="/opt/serverspot"
  if [[ -n "$here" ]] && in_repo "$here"; then
    default_dir="$here"
  fi

  if [[ -z "$INSTALL_DIR" ]]; then
    if [[ "$YES" -eq 1 ]]; then
      INSTALL_DIR="$default_dir"
    else
      local loc
      if [[ -n "$here" ]] && in_repo "$here" && [[ "$here" != "/opt/serverspot" ]]; then
        loc="$(choose "Where should ServerSpot live?" \
          "This folder (${here})" \
          "/opt/serverspot" \
          "Somewhere else…")"
        case "$loc" in
          This*) INSTALL_DIR="$here" ;;
          /opt/*) INSTALL_DIR="/opt/serverspot" ;;
          *) INSTALL_DIR="$(read_tty "Path" "$default_dir")" ;;
        esac
      else
        loc="$(choose "Where should ServerSpot live?" \
          "/opt/serverspot" \
          "Somewhere else…")"
        if [[ "$loc" == /opt/* ]]; then
          INSTALL_DIR="/opt/serverspot"
        else
          INSTALL_DIR="$(read_tty "Path" "$default_dir")"
        fi
      fi
    fi
  fi
  INSTALL_DIR="$(cd / && readlink -f "$INSTALL_DIR" 2>/dev/null || printf '%s' "$INSTALL_DIR")"

  if [[ -z "$MODE" ]]; then
    if [[ "$YES" -eq 1 ]]; then
      die "Set SERVERSPOT_MODE=proxy or SERVERSPOT_MODE=local with --yes."
    fi
    local reach
    reach="$(choose "How should people open ServerSpot?" \
      "Domain + HTTPS   Traefik, Let's Encrypt, ports 80 & 443" \
      "IP / local port  no domain, bind a single port")"
    if [[ "$reach" == Domain* ]]; then
      MODE="proxy"
    else
      MODE="local"
    fi
  fi

  if [[ "$MODE" == "proxy" ]]; then
    while [[ -z "$DOMAIN" ]] || ! valid_domain "$DOMAIN"; do
      DOMAIN="$(read_tty "Domain (e.g. panel.example.com)" "${DOMAIN}")"
      valid_domain "$DOMAIN" || warn "That doesn't look like a hostname."
      [[ "$YES" -eq 1 ]] && ! valid_domain "$DOMAIN" && die "Invalid SERVERSPOT_DOMAIN."
    done
    while [[ -z "$ACME_EMAIL" ]] || ! valid_email "$ACME_EMAIL"; do
      ACME_EMAIL="$(read_tty "Email for HTTPS certificates" "${ACME_EMAIL}")"
      valid_email "$ACME_EMAIL" || warn "That doesn't look like an email."
      [[ "$YES" -eq 1 ]] && ! valid_email "$ACME_EMAIL" && die "Invalid SERVERSPOT_EMAIL."
    done

    local ip
    ip="$(public_ip)"
    printf '\n'
    info "Point an A record to this machine, then wait a minute for DNS."
    if [[ -n "$ip" ]]; then
      printf "    ${C_BOLD}%s${C_RESET}  →  ${C_CYAN}%s${C_RESET}\n\n" "$DOMAIN" "$ip"
    else
      printf "    ${C_BOLD}%s${C_RESET}  →  ${C_DIM}(this server's public IP)${C_RESET}\n\n" "$DOMAIN"
    fi

    if [[ "$YES" -ne 1 ]]; then
      local dns_now
      dns_now="$(choose "Is DNS already pointing here?" \
        "Yes, check it now" \
        "Skip — I'll finish DNS after install")"
      if [[ "$dns_now" == Yes* ]]; then
        local resolved="" tries=0
        info "Checking DNS for ${DOMAIN}…"
        while ((tries < 18)); do
          resolved="$(resolve_host "$DOMAIN" || true)"
          resolved="${resolved//$'\r'/}"
          resolved="${resolved//$'\n'/}"
          if [[ -n "$ip" && "$resolved" == "$ip" ]]; then
            ok "DNS looks good (${resolved})."
            break
          fi
          if [[ -n "$resolved" && -z "$ip" ]]; then
            ok "DNS resolves to ${resolved}."
            break
          fi
          tries=$((tries + 1))
          printf "${C_DIM}  waiting for DNS… (%s)${C_RESET}\r" "${resolved:-none yet}" >"$TTY" || true
          sleep 5
        done
        printf '\n'
        if [[ -n "$ip" && "$resolved" != "$ip" ]]; then
          warn "DNS is ${resolved:-unset}, this server is ${ip}. Certificates will fail until that matches."
          local keep
          keep="$(choose "Continue anyway?" "Continue" "Cancel")"
          [[ "$keep" == Cancel ]] && die "Cancelled."
        fi
      fi
    fi

    if port_busy 80 || port_busy 443; then
      warn "Port 80 or 443 is already in use. Traefik needs both."
      if [[ "$YES" -ne 1 ]]; then
        local keep
        keep="$(choose "Continue anyway?" "Continue" "Cancel")"
        [[ "$keep" == Cancel ]] && die "Cancelled."
      fi
    fi
  else
    if [[ "$YES" -ne 1 ]]; then
      EXPOSE_PORT="$(read_tty "Host port" "$EXPOSE_PORT")"
    fi
    [[ "$EXPOSE_PORT" =~ ^[0-9]+$ ]] || die "Port must be a number."
    if port_busy "$EXPOSE_PORT"; then
      warn "Port ${EXPOSE_PORT} looks busy."
    fi
  fi

  if [[ -z "$SURREAL_PASS" ]]; then
    if [[ "$YES" -eq 1 ]]; then
      SURREAL_PASS="$(gen_pass)"
    else
      local pw_choice
      pw_choice="$(choose "Database password" \
        "Generate a strong password" \
        "Set my own")"
      if [[ "$pw_choice" == Generate* ]]; then
        SURREAL_PASS="$(gen_pass)"
      else
        SURREAL_PASS="$(read_tty "Database password" "" 1)"
        [[ -n "$SURREAL_PASS" ]] || die "Password cannot be empty."
      fi
    fi
  fi

  printf '\n'"${C_BOLD}Ready to install${C_RESET}\n"
  printf "  folder     %s\n" "$INSTALL_DIR"
  if [[ "$MODE" == "proxy" ]]; then
    printf "  access     https://%s\n" "$DOMAIN"
    printf "  certs      Let's Encrypt (%s)\n" "$ACME_EMAIL"
  else
    printf "  access     http://<this-host>:%s\n" "$EXPOSE_PORT"
  fi
  printf "  database   password stored in %s/.env\n\n" "$INSTALL_DIR"

  if [[ "$YES" -ne 1 ]]; then
    local go
    go="$(choose "Start install?" "Start" "Cancel")"
    [[ "$go" == Cancel ]] && die "Cancelled."
  fi

  ensure_docker
  prepare_tree "$INSTALL_DIR"
  write_env "$INSTALL_DIR" "$MODE"

  info "First build compiles ServerSpot inside Docker. Grab a coffee."
  local dc
  dc="$(docker_bin)"
  run_docker "$INSTALL_DIR" $dc pull || true
  run_docker "$INSTALL_DIR" $dc up -d --build

  printf '\n'
  ok "ServerSpot is up."
  if [[ "$MODE" == "proxy" ]]; then
    printf "  Open  ${C_CYAN}${C_BOLD}https://%s${C_RESET}\n" "$DOMAIN"
    printf "  Certs can take a minute the first time.\n"
  else
    local tip
    tip="$(public_ip)"
    [[ -z "$tip" ]] && tip="127.0.0.1"
    printf "  Open  ${C_CYAN}${C_BOLD}http://%s:%s${C_RESET}\n" "$tip" "$EXPOSE_PORT"
  fi
  printf '\n'"${C_DIM}  cd %s && docker compose --env-file .env logs -f${C_RESET}\n" "$INSTALL_DIR"
  printf "${C_DIM}  stop:  docker compose --env-file .env down${C_RESET}\n"
  printf "${C_DIM}  wipe:  docker compose --env-file .env down -v${C_RESET}\n\n"
}

main "$@"
