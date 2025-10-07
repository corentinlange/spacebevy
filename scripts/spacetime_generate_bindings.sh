#!/usr/bin/env bash
set -e

# Aller à la racine du projet (une étape au-dessus du dossier contenant ce script)
cd "$(dirname "$0")/.."

# Exécuter la commande spacetime
spacetime generate \
  --lang rust \
  --out-dir client/src/net/module_bindings \
  --project-path server
