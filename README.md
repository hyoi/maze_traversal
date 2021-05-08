Note: Japanese text only.

# 迷路をひたすら進むだけ: MazeTraversal
題名通り、ただただ進むだけで～す (^_^;) 。  
乱数で迷路を作ってみたかったのです。  
▲を操って、出口（◆がくるくるしてるとこ）を目指してください。
## 操作方法
カーソルキーで上下左右に移動。（使い道はないけどEscで一時停止）。   
Alt＋Enterでフルスクリーンとウインドウモード切替（Not WASM版）。
## WASM版
https://hyoi.github.io/maze_traversal/
## Rustのコンパイル版
[bevy_webgl2_app_template](https://github.com/mrk-its/bevy_webgl2_app_template)をお借りしたので、cargo-makeを使います。   
```
cargo make --profile release run    
```
WASM版の場合は、
```
cargo make --profile release serve
```
※事前にRustのtargetの追加とか必要です、たぶんきっとおそらく
## お世話になりました
- [bevy](https://bevyengine.org/)と[その仲間たち](https://crates.io/search?q=bevy)
  - [bevy_webgl2_app_template](https://github.com/mrk-its/bevy_webgl2_app_template)
  - [bevy_prototype_lyon](https://github.com/Nilirad/bevy_prototype_lyon/)
  - [bevy_egui](https://github.com/mvlabat/bevy_egui)
  - [Unofficial Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [Google Fonts](https://fonts.google.com/)
  - [Orbitron](https://fonts.google.com/specimen/Orbitron)
## 宿題
- 宝箱とかコボルトとか、配置したらRPGにできそう…
