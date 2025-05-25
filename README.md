# CLI CORE
CLI Coreは、コマンドラインアプリケーション開発のための多機能なRust、dllライブラリです。
コマンドラインツールを素早く開発するために必要な基本的な機能を提供します。

## 特徴
- 多言語対応：Rustと、C/C++などの言語から使用可能
- 色を付けたログの出力：視認性の高いカラー付きログの生成が可能
- 進捗表示：アニメーション付きかつ、現在の進捗が一目でわかる
- コマンドライン引数解析：シンプルで柔軟な引数パーサー
- 対話機能：プロンプト、確認、選択、パスワードが可能
- テンプレート管理：メッセージテンプレートの柔軟な管理
- 設定管理：TOML形式の設定ファイルの読み込みが可能
- エラー処理：構造化されたエラーハンドリング

## インストール

Rustプロジェクトでの使用

Cargo.tomlに依存関係を追加
```
[dependencies]
cli_core = { path = "パスを指定" }
```

または、GitHubから

```
[dependencies]
cli_core = { git = "https://github.com/Piliman22/CLI_Core" }
```

C/C++プロジェクトでの使用

```
cargo build --release
```

生成された.dll/.so/.dylibファイルとcli_core.hをプロジェクトに追加

リンカーオプションでライブラリをリンク

## 使用例

Rust
```
use cli_core::{log_info, log_success, log_error, get_template, prompt, confirm};

fn main() {
    log_info("アプリケーションを開始します...");
    
    let name = match prompt("お名前を入力してください: ") {
        Ok(name) => name,
        Err(_) => {
            log_error("入力エラーが発生しました");
            return;
        }
    };
    
    log_info(&format!("ようこそ、{}さん！", name));
    
    if confirm("続行しますか？", true).unwrap_or(false) {
        log_success("操作が正常に完了しました");
    } else {
        if let Some(msg) = get_template("cancelled") {
            log_info(&msg);
        }
    }
}
```

C
```
#include "cli_core.h"
#include <stdio.h>

int main() {
    cli_log_info("アプリケーションを開始します...");
    
    char* name = cli_prompt("お名前を入力してください: ");
    if (name) {
        printf("ようこそ、%sさん！\n", name);
        cli_free_string(name);
    } else {
        cli_log_error("入力エラーが発生しました");
        return 1;
    }
    
    if (cli_confirm("続行しますか？", true)) {
        cli_log_success("操作が正常に完了しました");
    } else {
        char* cancelled_msg = cli_get_template("cancelled");
        if (cancelled_msg) {
            cli_log_info(cancelled_msg);
            cli_free_string(cancelled_msg);
        }
    }
    
    return 0;
}
```

## ライセンス
このプロジェクトはMITライセンスの下で公開されています。

## 貢献
バグレポート、機能リクエスト、プルリクエストなどの貢献を歓迎します。