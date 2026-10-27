<p align="center">
  <img src="assets/icon.png" alt="Gacho" width="96">
</p>

<h1 align="center">Gacho</h1>

<p align="center">
  Comic Book ZIP viewer
</p>

<p align="center">
  <img alt="GitHub release (latest by date)" src="https://img.shields.io/github/v/release/yoshitaka-k/keiga">&nbsp;
  <img alt="License" src="https://img.shields.io/github/license/yoshitaka-k/keiga">&nbsp;
  <img alt="GitHub top language" src="https://img.shields.io/github/languages/top/yoshitaka-k/keiga">&nbsp;
  <img alt="Lines of code" src="https://www.aschey.tech/tokei/github/yoshitaka-k/keiga">&nbsp;
  <img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/yoshitaka-k/keiga">
</p>

Rust の勉強がてら、自分用に CBZビューア ってことで、Gacho（画帳）を作ってみようと思って作成なぅ。

フォルダやアーカイブ、画像をドロップすると、その本を開きます。同じフォルダのアーカイブは自然順につながっていて、最後のページの次で次の本へ、最初のページの前で前の本へ移ります。数字は数値として比べ、全角の ASCII は半角に揃えてから並べます。

<p align="center">
  <img src="assets/readme/preview.png" alt="Gacho screenshot" width="480">
</p>

## Supported formats

開けるのは次の拡張子です。アーカイブの中も、ばらの画像も同じ形式です。

| Extension                | How it opens |
| ------------------------ | ------------ |
| `.zip` / `.cbz`          | 1冊として開く。中の画像がページになる |
| `.jpg` / `.jpeg`         | 同じフォルダの画像を、自然順に1冊にする |
| `.png` / `.bmp` / `.gif` | 同上 |
| `.webp` / `.avif`        | 同上 |

ファイルを選ぶダイアログはアーカイブだけです。画像のフォルダはドロップするか、フォルダを開いてください。

アーカイブ内の隠しファイルと `__MACOSX` は読み飛ばします。ファイル名は Shift_JIS として解釈します。

## Usage

- フォルダ、アーカイブ、画像を **ドラッグ＆ドロップ**（複数なら先頭だけ）
- 右上のファイルボタンからアーカイブを開く
- 右上のフォルダボタンからフォルダを開く
- OS の「このアプリで開く」や、起動時に渡したパスでも開く

画像を開くと、そのフォルダの画像が1冊になり、開いたファイルのページから表示します。フォルダを開いたときは、中にアーカイブがあれば自然順の先頭を開き、なければフォルダ内の画像を1冊にします。

上部のタイトルは、ファイル名（画像フォルダならフォルダ名）から取ります。次の形なら、タイトルの部分だけを使います。

- `(カテゴリ)[著者]タイトル(ジャンル)`
- その末尾に `[備考]` が付いたもの（`[備考]` は複数でも可）
- `[著者]タイトル`
- 先頭に `(カテゴリ)` だけ付いた `[著者]タイトル`

右上のクリアボタン、または ⌘W / Ctrl+W で、開いている本を閉じます。

## Pages

見開きと表紙の扱いは設定の Page Layout / Cover Layout です。変更は本を開き直すまで反映されません。

| Layout          | Behavior |
| --------------- | -------- |
| Single Page     | 1ページずつ（既定） |
| Two-Page Spread | 縦長を見開きにする。幅 ÷ 高さが 1.2 を超えるページは1ページのまま |
| Single Cover    | 先頭を表紙として単独表示し、続きを見開きにする（既定） |
| No Single Cover | 先頭から見開きにする |

並びは Read From に従います。Right to Left（既定）では、見開きの右が若いページです。

下部は `#ページ / 総ページ` とファイル名です。見開きは `1-2` のようにまとめて出します。スライダーは見開き単位で移動し、Right to Left では右端が先頭です。矢印の右のボタンは先頭、左のボタンは末尾です（Right to Left）。すでにその端にいて前後の本があるときは、その本へ移ります。

## Mouse & keyboard

ページ送りです。Right to Left では左が次、右が前です。Left to Right では逆になります。本の端では、同じフォルダの前後のアーカイブへ移ります。

| Input                   | Behavior |
| ----------------------- | -------- |
| Click on left half / ←  | 次のページ（Right to Left） |
| Click on right half / → | 前のページ（Right to Left） |
| 下部の左向き矢印           | ← と同じ |
| 下部の右向き矢印           | → と同じ |
| 下部の左端ボタン           | 末尾へ（Right to Left）。すでに末尾なら次の本 |
| 下部の右端ボタン           | 先頭へ（Right to Left）。すでに先頭なら前の本 |

アプリ全体のショートカットです。macOS は ⌘、その他は Ctrl です。

| Input              | Behavior |
| ------------------ | -------- |
| ⌘O / Ctrl+O        | アーカイブを開く |
| ⌘⇧O / Ctrl+Shift+O | フォルダを開く |
| ⌘, / Ctrl+,        | 設定を開く |
| ⌘W / Ctrl+W        | 開いている本を閉じる。設定ウィンドウでは設定を閉じる |

## Settings

歯車アイコン、または ⌘, / Ctrl+, から設定ウィンドウを開きます。閉じるのは、そのウィンドウでの ⌘W / Ctrl+W です。値は次回起動時に復元されます。

### General

| Setting              | Behavior |
| -------------------- | -------- |
| Page Layout          | Single Page / Two-Page Spread（既定 Single Page） |
| Cover Layout         | Single Cover / No Single Cover（既定 Single Cover）。Page Layout とあわせて、本を開き直すまで反映されない |
| Read From            | Right to Left / Left to Right（既定 Right to Left） |
| Remembered Last Page | Yes / No（既定 No）。Yes のとき、読んでいるページを残す |
| Open Last Page       | Yes / No（既定 No）。Remembered Last Page が Yes のときだけ選べる。本を開き直すまで反映されない |
| Preloading           | 0–10。前後の見開きを先に読む数（既定 5） |

Remembered Last Page が Yes のとき、ページを送るたびに位置を残し、次回以降も使えます。アーカイブはファイル、ばらの画像は親フォルダが単位です。Open Last Page が Yes なら、本を開いたときと前後の本へ移ったときに、残してあるページから開きます。画像ファイルを直接開いたときは、その画像のページから表示します。

### About

バージョン・ライセンス・リポジトリと、GitHub Releases へのアップデート確認があります。

## License

[Apache-2.0](https://github.com/yoshitaka-k/gacho/blob/main/LICENSE)
