<img src="/docs/assets/ServerSpot.png"/>

## Install

One command on a Linux server:

```bash
curl -fsSL https://raw.githubusercontent.com/serverspot/serverspot/install-script/install.sh | bash
```

The installer walks through Docker, then either:

- **Domain + HTTPS** — point an A record at the server. Traefik + Let's Encrypt on ports 80 and 443
- **IP / local port** — no domain, bind a single port (default `8080`)

Use ↑↓ and Enter to pick options, or type `1` / `2`.

Already cloned this repo?

```bash
chmod +x install.sh
./install.sh
```

Unattended (domain):

```bash
SERVERSPOT_MODE=proxy \
SERVERSPOT_DOMAIN=panel.example.com \
SERVERSPOT_EMAIL=you@example.com \
./install.sh --yes
```

Unattended (port only):

```bash
SERVERSPOT_MODE=local SERVERSPOT_PORT=8080 ./install.sh --yes
```

Manual Compose (after copying `.env.example` → `.env` and setting `SURREAL_PASS`):

```bash
cp .env.example .env
docker compose up -d --build
```

Domain + Traefik: copy `deploy/.env.proxy.example` instead, set `DOMAIN` and `ACME_EMAIL`, then `docker compose up -d --build`.

Default install path is `/opt/serverspot` (or this repo, if you run `./install.sh` from a checkout). Afterwards:

```bash
cd /opt/serverspot
docker compose logs -f
docker compose down          # stop
docker compose down -v       # stop and delete the database volume
```

Whole bunch of words here eventually.
