Note: Japanese text only.

# 迷路をひたすら進むだけ: MazeTraversal
題名通り、ただひたすら進むだけです。  
乱数で迷路を作ってみたかったのです。  
▲を操って出口（上端で虹色の◆がくるくるしてる）を目指してください。  
うろうろしてる障害物に見つかると（色が赤になります）追いかけてきます。  
で、触るとHPがモリモリ減ります。HPゼロでゲームオーバー。  
コインは拾っても拾わなくてもOK。気分に合わせてどうぞ。
## WASM版
https://hyoi.github.io/maze_traversal/
## 操作方法
`⇧` `⇩` `⇦` `⇨` キーで上下左右に移動。   
`Esc`キーで一時停止(Pause‥‥使い道ないけど)。   
`Alt`＋`Enter`でフルスクリーンとウインドウモード切替（デスクトップアプリの場合）。
## コンパイル方法
デスクトップアプリにするなら`cargo run -r`でOK。  
`cargo run`だと、デバッグ用の表示が追加されます。
```
cargo run -r
```
WASMの場合は、bevy 0.6 から bevy_webgl2 に頼らなくても良くなりました。
```
cargo build -r --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./target --target web --no-typescript ./target/wasm32-unknown-unknown/release/maze_traversal.wasm
```
※`wasm-bindgen`コマンドの各ディレクトリーは作業環境に合わせてください。   
※WASMのコンパイルには事前にRustのtargetの追加とwasm-bindgenのインストールが必要です。  
※wasm-bindgenを実行するとバージョン違いで警告が出ることがあります。その時は素直にバージョン上げましょう。  
```
rustup target install wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
```
　[Unofficial Bevy Cheat Book - 13.5. Browser (WebAssembly)](https://bevy-cheatbook.github.io/platforms/wasm.html)をご参考に。
## お世話になりました
- [bevy](https://bevyengine.org/)と[その仲間たち](https://crates.io/search?q=bevy)
  - [bevy-web-resizer](https://github.com/frewsxcv/bevy-web-resizer)
  - [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Google Fonts](https://fonts.google.com/)
  - [Orbitron](https://fonts.google.com/specimen/Orbitron)
  - [Reggae One](https://fonts.google.com/specimen/Reggae+One?subset=japanese)
- [ドット絵ダウンロードサイト DOTOWN](https://dotown.maeda-design-room.net/)
  - Rustだから蟹 <img src="./assets/sprites/kani_DOTOWN.png" width="22" height="16" style="vertical-align: bottom;">
## 宿題
- ~~障害物が追いかけてくるようにしたい。~~ (v0.2.2)
- 障害物の追跡アルゴリズムをもっとちゃんとしたい。
- 追いつかれたら、RPGっぽい戦闘イベントにしたい。
- ~~当たり判定が手抜きすぎなので直す。いくら何でもヒドイ。~~  (v0.2.1)
- SEを鳴らしたい。
- 全部 なおしたい なおしたい 病（リファクタリングにいたる病）