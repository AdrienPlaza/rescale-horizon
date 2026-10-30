# Rescale — mod natif pour Horizon

Résolution étirée (« stretched resolution ») pour [Horizon](https://playhorizon.dev) :
le monde est dessiné dans un ratio choisi (4:3, 5:4, 16:10, 16:9 ou un ratio
personnalisé) pendant que la fenêtre garde le sien. L'image est donc étirée
horizontalement, exactement comme le fait le mod Fabric
[Rescale](https://modrinth.com/mod/rescale) de TCD1234 sur Minecraft Java.

La main (vue première personne) et le HUD gardent leur propre projection : ils
restent normaux, comme dans le mod d'origine.

## Installation (ami·e)

Prérequis : le launcher **Horizon** à jour (Windows), une version du jeu
installée et lancée au moins une fois.

1. Ouvre le launcher, va sur **Mods**, choisis la source **Horizon**, puis
   **Add a native mod from GitHub** (« Ajouter un mod natif depuis GitHub »).
2. Colle ce lien, puis valide :

   ```
   https://github.com/AdrienPlaza/rescale-horizon
   ```

3. Le launcher lit le dépôt, télécharge la source et l'installe. Il prévient
   que les mods natifs ne sont pas sandboxés : c'est normal, tout le code est
   dans ce dépôt, tu peux le lire.
4. Dans **Settings > Native mods**, clique **Build** (ou « I trust these mods:
   build »). Le premier build télécharge Rust et prend 20 à 30 minutes ; les
   suivants prennent quelques minutes.
5. Lance l'instance **Native mods**. En jeu : **F6** change le ratio, et
   **Options > Mods > Rescale** donne les mêmes réglages en boutons.

Le mod se construit sur le Nightly `nightly-2026-10-30` (commit
`1da354b`), celui sur lequel il a été testé : le launcher garde la source de
chaque Nightly, donc ça fonctionne même si ton launcher est plus récent.

## Si F6 ne fait rien

Dans l'ordre, et c'est presque toujours l'un des trois premiers :

1. **Tu joues la bonne instance.** Les mods natifs ne tournent que dans
   l'instance **Native mods** (en haut à gauche du launcher). L'instance
   normale ne les a pas.
2. **Le mod est activé.** En jeu, **Options > Mods** : la ligne « Rescale
   0.2.0 » doit être là, avec son interrupteur sur ON. Si elle n'est pas là,
   c'est que le build ne contient pas le mod.
3. **Tu as rebuild après l'installation.** Installer ne suffit pas :
   **Settings > Native mods > Build** (le bouton rouge « I trust these mods:
   build »). Le build crée une version `native-…` ; c'est elle que lance
   l'instance.
4. **Tu es à jour.** `horizon-launcher mods update-github rescale` puis
   **Build** à nouveau.
5. **Le repère visuel** : à l'arrivée dans un monde, une ligne dans le chat
   dit « Rescale 0.2.0: F6 changes the ratio ». Si elle n'apparaît pas, le
   mod n'est pas dans le build ; si elle apparaît et que F6 ne fait rien,
   regarde la touche liée dans **Options > Commandes**.

## Réglages

- **Touche** : `F6` par défaut, rebindable dans **Options > Commandes**
  (chercher « Rescale »). Elle fait défiler
  `4:3 → 5:4 → 16:10 → 16:9 → custom → off`.
  Le premier appui donne donc **4:3**, un changement bien visible : c'est
  volontaire, un premier appui sur 16:9 ne change rien sur un écran 16:9.
- **Options > Mods > Rescale** : le mode courant, les quatre préréglages, un
  ratio personnalisé (`±0.05` par clic) et « off ».
- Les réglages sont dans
  `%APPDATA%\horizon\horizon-mods\.horizon-mod-data\rescale\` (`mode`, `custom`).
- Le ratio s'affiche en haut à gauche pendant 2,5 s après un changement.
- À l'arrivée dans un monde, une ligne dans le chat rappelle que le mod est
  chargé : si tu ne la vois pas, le build ne contient pas le mod (voir plus bas).

## Comment c'est fait

Un bundle [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) :

| Injection | Cible | Effet |
|---|---|---|
| `projection` | `horizon_client_render::gfx::view::View::screen` (MODIFY_VARIABLE sur l'argument `aspect`) | dessine le niveau au ratio choisi |
| `frame` | `native_frame` | lit les réglages une fois, les applique |
| `key` | `native_key` | fait défiler les modes |
| `list`, `settings`, `click` | `native_mods`, `native_settings`, `native_settings_click` | la ligne et la page Options > Mods |
| `hud` | `native_hud` | le ratio, pendant 1,5 s |

Deux modules de support : `rescale` (dans `horizon-client-render`, l'atomique
que lit le hook de projection) et `rescale_client` (dans `horizon-client`, les
réglages, la touche et la page).

L'injection `projection` est une **cible brute**, pas un hook nommé : c'est
pour ça que `[compatibility] horizon_revision` est épinglé dans
`horizon-mixin.toml`. Si une mise à jour de Horizon déplace `View::screen`, le
build échoue et il faut retrouver la cible.

### Construire en local

```sh
horizon-launcher native-build "<ce dossier>" --instance native-mods --trust-native-code
```

## Fichiers

```
horizon-mixin.toml   le manifeste (id, version, cible du build, hooks)
projection.rs        le hook de projection (le ratio)
frame.rs key.rs      la boucle et la touche
list.rs settings.rs click.rs hud.rs
render_state.rs      le ratio partagé (module `rescale`)
client_state.rs      les réglages (module `rescale_client`)
dist/*.hnmod         le paquet prêt à installer (même contenu, zippé)
```

## Licence et crédits

GPL-3.0-only, comme le mod dont il reprend l'idée.

Ce mod est une réimplémentation indépendante en Rust pour Horizon de l'idée du
mod Fabric **Rescale** de **TCD1234** (<https://github.com/TCD1234/Rescale>),
lui-même en GPL-3.0-only. Aucun code du mod d'origine n'est inclus : seule la
logique (un ratio appliqué à la projection de la caméra) est réimplémentée.

Ni fait ni approuvé par le projet Horizon.
